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
use rusoto_core::pagination::{all_pages, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto::xml::error::*;
use rusoto_core::proto::xml::util::{
    self as xml_util, deserialize_elements, find_start_element, skip_tree,
};
use rusoto_core::proto::xml::util::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[cfg(feature = "deserialize_structs")]
use serde::Deserialize;
#[cfg(feature = "serialize_structs")]
use serde::Serialize;
use serde_urlencoded;
use std::str::FromStr;
use xml::EventReader;

impl ElastiCacheClient {
    fn new_params(&self, operation_name: &str) -> Params {
        let mut params = Params::new();

        params.put("Action", operation_name);
        params.put("Version", "2015-02-02");

        params
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

/// <p>Represents the input of an AddTagsToResource operation.</p>
/// see [ElastiCache::add_tags_to_resource]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the resource to which the tags are to be added, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>. ElastiCache resources are <i>cluster</i> and <i>snapshot</i>.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    pub resource_name: String,
    /// <p>A list of cost allocation tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value.</p>
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

#[allow(dead_code)]
struct AllowedNodeGroupIdDeserializer;
impl AllowedNodeGroupIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents the allowed node types you can use to modify your cluster or replication group.</p>
/// see [ElastiCache::list_allowed_node_type_modifications]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AllowedNodeTypeModificationsMessage {
    /// <p>A string list, each element of which specifies a cache node type which you can use to scale your cluster or replication group. When scaling down a Redis cluster or replication group using ModifyCacheCluster or ModifyReplicationGroup, use a value from this list for the CacheNodeType parameter. </p>
    pub scale_down_modifications: Option<Vec<String>>,
    /// <p>A string list, each element of which specifies a cache node type which you can use to scale your cluster or replication group.</p> <p>When scaling up a Redis cluster or replication group using <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code>, use a value from this list for the <code>CacheNodeType</code> parameter.</p>
    pub scale_up_modifications: Option<Vec<String>>,
}

#[allow(dead_code)]
struct AllowedNodeTypeModificationsMessageDeserializer;
impl AllowedNodeTypeModificationsMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AllowedNodeTypeModificationsMessage, XmlParseError> {
        deserialize_elements::<_, AllowedNodeTypeModificationsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ScaleDownModifications" => {
                        obj.scale_down_modifications.get_or_insert(vec![]).extend(
                            NodeTypeListDeserializer::deserialize("ScaleDownModifications", stack)?,
                        );
                    }
                    "ScaleUpModifications" => {
                        obj.scale_up_modifications.get_or_insert(vec![]).extend(
                            NodeTypeListDeserializer::deserialize("ScaleUpModifications", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct AuthTokenUpdateStatusDeserializer;
impl AuthTokenUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Indicates whether the user requires a password to authenticate.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Authentication {
    /// <p>The number of passwords belonging to the user. The maximum is two.</p>
    pub password_count: Option<i64>,
    /// <p>Indicates whether the user requires a password to authenticate.</p>
    pub type_: Option<String>,
}

#[allow(dead_code)]
struct AuthenticationDeserializer;
impl AuthenticationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Authentication, XmlParseError> {
        deserialize_elements::<_, Authentication, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "PasswordCount" => {
                    obj.password_count = Some(IntegerOptionalDeserializer::deserialize(
                        "PasswordCount",
                        stack,
                    )?);
                }
                "Type" => {
                    obj.type_ = Some(AuthenticationTypeDeserializer::deserialize("Type", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct AuthenticationTypeDeserializer;
impl AuthenticationTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents the input of an AuthorizeCacheSecurityGroupIngress operation.</p>
/// see [ElastiCache::authorize_cache_security_group_ingress]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AuthorizeCacheSecurityGroupIngressMessage {
    /// <p>The cache security group that allows network ingress.</p>
    pub cache_security_group_name: String,
    /// <p>The Amazon EC2 security group to be authorized for ingress to the cache security group.</p>
    pub ec2_security_group_name: String,
    /// <p>The AWS account number of the Amazon EC2 security group owner. Note that this is not the same thing as an AWS access key ID - you must provide a valid AWS account number for this parameter.</p>
    pub ec2_security_group_owner_id: String,
}

/// Serialize `AuthorizeCacheSecurityGroupIngressMessage` contents to a `SignedRequest`.
struct AuthorizeCacheSecurityGroupIngressMessageSerializer;
impl AuthorizeCacheSecurityGroupIngressMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AuthorizeCacheSecurityGroupIngressMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSecurityGroupName"),
            &obj.cache_security_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "EC2SecurityGroupName"),
            &obj.ec2_security_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
            &obj.ec2_security_group_owner_id,
        );
    }
}

/// see [ElastiCache::authorize_cache_security_group_ingress]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AuthorizeCacheSecurityGroupIngressResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[allow(dead_code)]
struct AuthorizeCacheSecurityGroupIngressResultDeserializer;
impl AuthorizeCacheSecurityGroupIngressResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<AuthorizeCacheSecurityGroupIngressResult, XmlParseError> {
        deserialize_elements::<_, AuthorizeCacheSecurityGroupIngressResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSecurityGroup" => {
                        obj.cache_security_group =
                            Some(CacheSecurityGroupDeserializer::deserialize(
                                "CacheSecurityGroup",
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
#[allow(dead_code)]
struct AutomaticFailoverStatusDeserializer;
impl AutomaticFailoverStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes an Availability Zone in which the cluster is launched.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The name of the Availability Zone.</p>
    pub name: Option<String>,
}

#[allow(dead_code)]
struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct AvailabilityZonesListDeserializer;
impl AvailabilityZonesListDeserializer {
    #[allow(dead_code, unused_variables)]
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

/// Serialize `AvailabilityZonesList` contents to a `SignedRequest`.
struct AvailabilityZonesListSerializer;
impl AvailabilityZonesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// see [ElastiCache::batch_apply_update_action]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchApplyUpdateActionMessage {
    /// <p>The cache cluster IDs</p>
    pub cache_cluster_ids: Option<Vec<String>>,
    /// <p>The replication group IDs</p>
    pub replication_group_ids: Option<Vec<String>>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: String,
}

/// Serialize `BatchApplyUpdateActionMessage` contents to a `SignedRequest`.
struct BatchApplyUpdateActionMessageSerializer;
impl BatchApplyUpdateActionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchApplyUpdateActionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_ids {
            CacheClusterIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheClusterIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replication_group_ids {
            ReplicationGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplicationGroupIds"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ServiceUpdateName"),
            &obj.service_update_name,
        );
    }
}

/// see [ElastiCache::batch_stop_update_action]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchStopUpdateActionMessage {
    /// <p>The cache cluster IDs</p>
    pub cache_cluster_ids: Option<Vec<String>>,
    /// <p>The replication group IDs</p>
    pub replication_group_ids: Option<Vec<String>>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: String,
}

/// Serialize `BatchStopUpdateActionMessage` contents to a `SignedRequest`.
struct BatchStopUpdateActionMessageSerializer;
impl BatchStopUpdateActionMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &BatchStopUpdateActionMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_ids {
            CacheClusterIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheClusterIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replication_group_ids {
            ReplicationGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplicationGroupIds"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ServiceUpdateName"),
            &obj.service_update_name,
        );
    }
}

#[allow(dead_code)]
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct BooleanOptionalDeserializer;
impl BooleanOptionalDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<bool, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(bool::from_str(&s).unwrap()))
    }
}
/// <p>Contains all of the attributes of a specific cluster.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheCluster {
    /// <p>The ARN (Amazon Resource Name) of the cache cluster.</p>
    pub arn: Option<String>,
    /// <p>A flag that enables encryption at-rest when set to <code>true</code>.</p> <p>You cannot modify the value of <code>AtRestEncryptionEnabled</code> after the cluster is created. To enable at-rest encryption on a cluster you must set <code>AtRestEncryptionEnabled</code> to <code>true</code> when you create a cluster.</p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p>
    pub at_rest_encryption_enabled: Option<bool>,
    /// <p>A flag that enables using an <code>AuthToken</code> (password) when issuing Redis commands.</p> <p>Default: <code>false</code> </p>
    pub auth_token_enabled: Option<bool>,
    /// <p>The date the auth token was last modified</p>
    pub auth_token_last_modified_date: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The date and time when the cluster was created.</p>
    pub cache_cluster_create_time: Option<String>,
    /// <p>The user-supplied identifier of the cluster. This identifier is a unique key that identifies a cluster.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The current state of this cluster, one of the following values: <code>available</code>, <code>creating</code>, <code>deleted</code>, <code>deleting</code>, <code>incompatible-network</code>, <code>modifying</code>, <code>rebooting cluster nodes</code>, <code>restore-failed</code>, or <code>snapshotting</code>.</p>
    pub cache_cluster_status: Option<String>,
    /// <p><p>The name of the compute and memory capacity node type for the cluster.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>A list of cache nodes that are members of the cluster.</p>
    pub cache_nodes: Option<Vec<CacheNode>>,
    /// <p>Status of the cache parameter group.</p>
    pub cache_parameter_group: Option<CacheParameterGroupStatus>,
    /// <p>A list of cache security group elements, composed of name and status sub-elements.</p>
    pub cache_security_groups: Option<Vec<CacheSecurityGroupMembership>>,
    /// <p>The name of the cache subnet group associated with the cluster.</p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>The URL of the web page where you can download the latest ElastiCache client library.</p>
    pub client_download_landing_page: Option<String>,
    /// <p>Represents a Memcached cluster endpoint which, if Automatic Discovery is enabled on the cluster, can be used by an application to connect to any node in the cluster. The configuration endpoint will always have <code>.cfg</code> in it.</p> <p>Example: <code>mem-3.9dvc4r<u>.cfg</u>.usw2.cache.amazonaws.com:11211</code> </p>
    pub configuration_endpoint: Option<Endpoint>,
    /// <p>The name of the cache engine (<code>memcached</code> or <code>redis</code>) to be used for this cluster.</p>
    pub engine: Option<String>,
    /// <p>The version of the cache engine that is used in this cluster.</p>
    pub engine_version: Option<String>,
    /// <p>Describes a notification topic and its status. Notification topics are used for publishing ElastiCache events to subscribers using Amazon Simple Notification Service (SNS). </p>
    pub notification_configuration: Option<NotificationConfiguration>,
    /// <p>The number of cache nodes in the cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>
    pub num_cache_nodes: Option<i64>,
    pub pending_modified_values: Option<PendingModifiedValues>,
    /// <p>The name of the Availability Zone in which the cluster is located or "Multiple" if the cache nodes are located in different Availability Zones.</p>
    pub preferred_availability_zone: Option<String>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The outpost ARN in which the cache cluster is created.</p>
    pub preferred_outpost_arn: Option<String>,
    /// <p>The replication group to which this cluster belongs. If this field is empty, the cluster is not associated with any replication group.</p>
    pub replication_group_id: Option<String>,
    /// <p>A list of VPC Security Groups associated with the cluster.</p>
    pub security_groups: Option<Vec<SecurityGroupMembership>>,
    /// <p><p>The number of days for which ElastiCache retains automatic cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <important> <p> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p> </important></p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your cluster.</p> <p>Example: <code>05:00-09:00</code> </p>
    pub snapshot_window: Option<String>,
    /// <p>A flag that enables in-transit encryption when set to <code>true</code>.</p> <p>You cannot modify the value of <code>TransitEncryptionEnabled</code> after the cluster is created. To enable in-transit encryption on a cluster you must set <code>TransitEncryptionEnabled</code> to <code>true</code> when you create a cluster.</p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p>
    pub transit_encryption_enabled: Option<bool>,
}

#[allow(dead_code)]
struct CacheClusterDeserializer;
impl CacheClusterDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheCluster, XmlParseError> {
        deserialize_elements::<_, CacheCluster, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "AtRestEncryptionEnabled" => {
                    obj.at_rest_encryption_enabled = Some(
                        BooleanOptionalDeserializer::deserialize("AtRestEncryptionEnabled", stack)?,
                    );
                }
                "AuthTokenEnabled" => {
                    obj.auth_token_enabled = Some(BooleanOptionalDeserializer::deserialize(
                        "AuthTokenEnabled",
                        stack,
                    )?);
                }
                "AuthTokenLastModifiedDate" => {
                    obj.auth_token_last_modified_date = Some(TStampDeserializer::deserialize(
                        "AuthTokenLastModifiedDate",
                        stack,
                    )?);
                }
                "AutoMinorVersionUpgrade" => {
                    obj.auto_minor_version_upgrade = Some(BooleanDeserializer::deserialize(
                        "AutoMinorVersionUpgrade",
                        stack,
                    )?);
                }
                "CacheClusterCreateTime" => {
                    obj.cache_cluster_create_time = Some(TStampDeserializer::deserialize(
                        "CacheClusterCreateTime",
                        stack,
                    )?);
                }
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "CacheClusterStatus" => {
                    obj.cache_cluster_status = Some(StringDeserializer::deserialize(
                        "CacheClusterStatus",
                        stack,
                    )?);
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "CacheNodes" => {
                    obj.cache_nodes
                        .get_or_insert(vec![])
                        .extend(CacheNodeListDeserializer::deserialize("CacheNodes", stack)?);
                }
                "CacheParameterGroup" => {
                    obj.cache_parameter_group =
                        Some(CacheParameterGroupStatusDeserializer::deserialize(
                            "CacheParameterGroup",
                            stack,
                        )?);
                }
                "CacheSecurityGroups" => {
                    obj.cache_security_groups.get_or_insert(vec![]).extend(
                        CacheSecurityGroupMembershipListDeserializer::deserialize(
                            "CacheSecurityGroups",
                            stack,
                        )?,
                    );
                }
                "CacheSubnetGroupName" => {
                    obj.cache_subnet_group_name = Some(StringDeserializer::deserialize(
                        "CacheSubnetGroupName",
                        stack,
                    )?);
                }
                "ClientDownloadLandingPage" => {
                    obj.client_download_landing_page = Some(StringDeserializer::deserialize(
                        "ClientDownloadLandingPage",
                        stack,
                    )?);
                }
                "ConfigurationEndpoint" => {
                    obj.configuration_endpoint = Some(EndpointDeserializer::deserialize(
                        "ConfigurationEndpoint",
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
                "NotificationConfiguration" => {
                    obj.notification_configuration =
                        Some(NotificationConfigurationDeserializer::deserialize(
                            "NotificationConfiguration",
                            stack,
                        )?);
                }
                "NumCacheNodes" => {
                    obj.num_cache_nodes = Some(IntegerOptionalDeserializer::deserialize(
                        "NumCacheNodes",
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
                "PreferredAvailabilityZone" => {
                    obj.preferred_availability_zone = Some(StringDeserializer::deserialize(
                        "PreferredAvailabilityZone",
                        stack,
                    )?);
                }
                "PreferredMaintenanceWindow" => {
                    obj.preferred_maintenance_window = Some(StringDeserializer::deserialize(
                        "PreferredMaintenanceWindow",
                        stack,
                    )?);
                }
                "PreferredOutpostArn" => {
                    obj.preferred_outpost_arn = Some(StringDeserializer::deserialize(
                        "PreferredOutpostArn",
                        stack,
                    )?);
                }
                "ReplicationGroupId" => {
                    obj.replication_group_id = Some(StringDeserializer::deserialize(
                        "ReplicationGroupId",
                        stack,
                    )?);
                }
                "SecurityGroups" => {
                    obj.security_groups.get_or_insert(vec![]).extend(
                        SecurityGroupMembershipListDeserializer::deserialize(
                            "SecurityGroups",
                            stack,
                        )?,
                    );
                }
                "SnapshotRetentionLimit" => {
                    obj.snapshot_retention_limit = Some(IntegerOptionalDeserializer::deserialize(
                        "SnapshotRetentionLimit",
                        stack,
                    )?);
                }
                "SnapshotWindow" => {
                    obj.snapshot_window =
                        Some(StringDeserializer::deserialize("SnapshotWindow", stack)?);
                }
                "TransitEncryptionEnabled" => {
                    obj.transit_encryption_enabled =
                        Some(BooleanOptionalDeserializer::deserialize(
                            "TransitEncryptionEnabled",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `CacheClusterIdList` contents to a `SignedRequest`.
struct CacheClusterIdListSerializer;
impl CacheClusterIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct CacheClusterListDeserializer;
impl CacheClusterListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheCluster>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheCluster" {
                obj.push(CacheClusterDeserializer::deserialize(
                    "CacheCluster",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeCacheClusters</code> operation.</p>
/// see [ElastiCache::describe_cache_clusters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheClusterMessage {
    /// <p>A list of clusters. Each item in the list contains detailed information about one cluster.</p>
    pub cache_clusters: Option<Vec<CacheCluster>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl CacheClusterMessage {
    fn pagination_page_opt(self) -> Option<Vec<CacheCluster>> {
        Some(self.cache_clusters.as_ref()?.clone())
    }
}

impl PagedOutput for CacheClusterMessage {
    type Item = CacheCluster;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<CacheCluster> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheClusterMessageDeserializer;
impl CacheClusterMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheClusterMessage, XmlParseError> {
        deserialize_elements::<_, CacheClusterMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheClusters" => {
                    obj.cache_clusters.get_or_insert(vec![]).extend(
                        CacheClusterListDeserializer::deserialize("CacheClusters", stack)?,
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
/// <p>Provides all of the details about a particular cache engine version.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheEngineVersion {
    /// <p>The description of the cache engine.</p>
    pub cache_engine_description: Option<String>,
    /// <p>The description of the cache engine version.</p>
    pub cache_engine_version_description: Option<String>,
    /// <p>The name of the cache parameter group family associated with this cache engine.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p>
    pub cache_parameter_group_family: Option<String>,
    /// <p>The name of the cache engine.</p>
    pub engine: Option<String>,
    /// <p>The version number of the cache engine.</p>
    pub engine_version: Option<String>,
}

#[allow(dead_code)]
struct CacheEngineVersionDeserializer;
impl CacheEngineVersionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheEngineVersion, XmlParseError> {
        deserialize_elements::<_, CacheEngineVersion, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheEngineDescription" => {
                    obj.cache_engine_description = Some(StringDeserializer::deserialize(
                        "CacheEngineDescription",
                        stack,
                    )?);
                }
                "CacheEngineVersionDescription" => {
                    obj.cache_engine_version_description = Some(StringDeserializer::deserialize(
                        "CacheEngineVersionDescription",
                        stack,
                    )?);
                }
                "CacheParameterGroupFamily" => {
                    obj.cache_parameter_group_family = Some(StringDeserializer::deserialize(
                        "CacheParameterGroupFamily",
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
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CacheEngineVersionListDeserializer;
impl CacheEngineVersionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheEngineVersion>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheEngineVersion" {
                obj.push(CacheEngineVersionDeserializer::deserialize(
                    "CacheEngineVersion",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <a>DescribeCacheEngineVersions</a> operation.</p>
/// see [ElastiCache::describe_cache_engine_versions]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheEngineVersionMessage {
    /// <p>A list of cache engine version details. Each element in the list contains detailed information about one cache engine version.</p>
    pub cache_engine_versions: Option<Vec<CacheEngineVersion>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl CacheEngineVersionMessage {
    fn pagination_page_opt(self) -> Option<Vec<CacheEngineVersion>> {
        Some(self.cache_engine_versions.as_ref()?.clone())
    }
}

impl PagedOutput for CacheEngineVersionMessage {
    type Item = CacheEngineVersion;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<CacheEngineVersion> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheEngineVersionMessageDeserializer;
impl CacheEngineVersionMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheEngineVersionMessage, XmlParseError> {
        deserialize_elements::<_, CacheEngineVersionMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheEngineVersions" => {
                        obj.cache_engine_versions.get_or_insert(vec![]).extend(
                            CacheEngineVersionListDeserializer::deserialize(
                                "CacheEngineVersions",
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
/// <p><p>Represents an individual cache node within a cluster. Each cache node runs its own instance of the cluster&#39;s protocol-compliant caching software - either Memcached or Redis.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheNode {
    /// <p>The date and time when the cache node was created.</p>
    pub cache_node_create_time: Option<String>,
    /// <p>The cache node identifier. A node ID is a numeric identifier (0001, 0002, etc.). The combination of cluster ID and node ID uniquely identifies every cache node used in a customer's AWS account.</p>
    pub cache_node_id: Option<String>,
    /// <p>The current state of this cache node, one of the following values: <code>available</code>, <code>creating</code>, <code>rebooting</code>, or <code>deleting</code>.</p>
    pub cache_node_status: Option<String>,
    /// <p>The Availability Zone where this node was created and now resides.</p>
    pub customer_availability_zone: Option<String>,
    /// <p>The customer outpost ARN of the cache node.</p>
    pub customer_outpost_arn: Option<String>,
    /// <p>The hostname for connecting to this cache node.</p>
    pub endpoint: Option<Endpoint>,
    /// <p>The status of the parameter group applied to this cache node.</p>
    pub parameter_group_status: Option<String>,
    /// <p>The ID of the primary node to which this read replica node is synchronized. If this field is empty, this node is not associated with a primary cluster.</p>
    pub source_cache_node_id: Option<String>,
}

#[allow(dead_code)]
struct CacheNodeDeserializer;
impl CacheNodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheNode, XmlParseError> {
        deserialize_elements::<_, CacheNode, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheNodeCreateTime" => {
                    obj.cache_node_create_time = Some(TStampDeserializer::deserialize(
                        "CacheNodeCreateTime",
                        stack,
                    )?);
                }
                "CacheNodeId" => {
                    obj.cache_node_id =
                        Some(StringDeserializer::deserialize("CacheNodeId", stack)?);
                }
                "CacheNodeStatus" => {
                    obj.cache_node_status =
                        Some(StringDeserializer::deserialize("CacheNodeStatus", stack)?);
                }
                "CustomerAvailabilityZone" => {
                    obj.customer_availability_zone = Some(StringDeserializer::deserialize(
                        "CustomerAvailabilityZone",
                        stack,
                    )?);
                }
                "CustomerOutpostArn" => {
                    obj.customer_outpost_arn = Some(StringDeserializer::deserialize(
                        "CustomerOutpostArn",
                        stack,
                    )?);
                }
                "Endpoint" => {
                    obj.endpoint = Some(EndpointDeserializer::deserialize("Endpoint", stack)?);
                }
                "ParameterGroupStatus" => {
                    obj.parameter_group_status = Some(StringDeserializer::deserialize(
                        "ParameterGroupStatus",
                        stack,
                    )?);
                }
                "SourceCacheNodeId" => {
                    obj.source_cache_node_id =
                        Some(StringDeserializer::deserialize("SourceCacheNodeId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CacheNodeIdsListDeserializer;
impl CacheNodeIdsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheNodeId" {
                obj.push(StringDeserializer::deserialize("CacheNodeId", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `CacheNodeIdsList` contents to a `SignedRequest`.
struct CacheNodeIdsListSerializer;
impl CacheNodeIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct CacheNodeListDeserializer;
impl CacheNodeListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheNode>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheNode" {
                obj.push(CacheNodeDeserializer::deserialize("CacheNode", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A parameter that has a different value for each cache node type it is applied to. For example, in a Redis cluster, a <code>cache.m1.large</code> cache node type would have a larger <code>maxmemory</code> value than a <code>cache.m1.small</code> type.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheNodeTypeSpecificParameter {
    /// <p>The valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>A list of cache node types and their corresponding values for this parameter.</p>
    pub cache_node_type_specific_values: Option<Vec<CacheNodeTypeSpecificValue>>,
    /// <p>Indicates whether a change to the parameter is applied immediately or requires a reboot for the change to be applied. You can force a reboot or wait until the next maintenance window's reboot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Rebooting.html">Rebooting a Cluster</a>.</p>
    pub change_type: Option<String>,
    /// <p>The valid data type for the parameter.</p>
    pub data_type: Option<String>,
    /// <p>A description of the parameter.</p>
    pub description: Option<String>,
    /// <p>Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed.</p>
    pub is_modifiable: Option<bool>,
    /// <p>The earliest cache engine version to which the parameter can apply.</p>
    pub minimum_engine_version: Option<String>,
    /// <p>The name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>The source of the parameter value.</p>
    pub source: Option<String>,
}

#[allow(dead_code)]
struct CacheNodeTypeSpecificParameterDeserializer;
impl CacheNodeTypeSpecificParameterDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheNodeTypeSpecificParameter, XmlParseError> {
        deserialize_elements::<_, CacheNodeTypeSpecificParameter, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AllowedValues" => {
                        obj.allowed_values =
                            Some(StringDeserializer::deserialize("AllowedValues", stack)?);
                    }
                    "CacheNodeTypeSpecificValues" => {
                        obj.cache_node_type_specific_values
                            .get_or_insert(vec![])
                            .extend(CacheNodeTypeSpecificValueListDeserializer::deserialize(
                                "CacheNodeTypeSpecificValues",
                                stack,
                            )?);
                    }
                    "ChangeType" => {
                        obj.change_type =
                            Some(ChangeTypeDeserializer::deserialize("ChangeType", stack)?);
                    }
                    "DataType" => {
                        obj.data_type = Some(StringDeserializer::deserialize("DataType", stack)?);
                    }
                    "Description" => {
                        obj.description =
                            Some(StringDeserializer::deserialize("Description", stack)?);
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
                    "Source" => {
                        obj.source = Some(StringDeserializer::deserialize("Source", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct CacheNodeTypeSpecificParametersListDeserializer;
impl CacheNodeTypeSpecificParametersListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheNodeTypeSpecificParameter>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheNodeTypeSpecificParameter" {
                obj.push(CacheNodeTypeSpecificParameterDeserializer::deserialize(
                    "CacheNodeTypeSpecificParameter",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A value that applies only to a certain cache node type.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheNodeTypeSpecificValue {
    /// <p>The cache node type for which this value applies.</p>
    pub cache_node_type: Option<String>,
    /// <p>The value for the cache node type.</p>
    pub value: Option<String>,
}

#[allow(dead_code)]
struct CacheNodeTypeSpecificValueDeserializer;
impl CacheNodeTypeSpecificValueDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheNodeTypeSpecificValue, XmlParseError> {
        deserialize_elements::<_, CacheNodeTypeSpecificValue, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheNodeType" => {
                        obj.cache_node_type =
                            Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                    }
                    "Value" => {
                        obj.value = Some(StringDeserializer::deserialize("Value", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct CacheNodeTypeSpecificValueListDeserializer;
impl CacheNodeTypeSpecificValueListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheNodeTypeSpecificValue>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheNodeTypeSpecificValue" {
                obj.push(CacheNodeTypeSpecificValueDeserializer::deserialize(
                    "CacheNodeTypeSpecificValue",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The status of the service update on the cache node</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheNodeUpdateStatus {
    /// <p>The node ID of the cache cluster</p>
    pub cache_node_id: Option<String>,
    /// <p>The deletion date of the node</p>
    pub node_deletion_date: Option<String>,
    /// <p>The end date of the update for a node</p>
    pub node_update_end_date: Option<String>,
    /// <p>Reflects whether the update was initiated by the customer or automatically applied</p>
    pub node_update_initiated_by: Option<String>,
    /// <p>The date when the update is triggered</p>
    pub node_update_initiated_date: Option<String>,
    /// <p>The start date of the update for a node</p>
    pub node_update_start_date: Option<String>,
    /// <p>The update status of the node</p>
    pub node_update_status: Option<String>,
    /// <p>The date when the NodeUpdateStatus was last modified&gt;</p>
    pub node_update_status_modified_date: Option<String>,
}

#[allow(dead_code)]
struct CacheNodeUpdateStatusDeserializer;
impl CacheNodeUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheNodeUpdateStatus, XmlParseError> {
        deserialize_elements::<_, CacheNodeUpdateStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheNodeId" => {
                    obj.cache_node_id =
                        Some(StringDeserializer::deserialize("CacheNodeId", stack)?);
                }
                "NodeDeletionDate" => {
                    obj.node_deletion_date =
                        Some(TStampDeserializer::deserialize("NodeDeletionDate", stack)?);
                }
                "NodeUpdateEndDate" => {
                    obj.node_update_end_date =
                        Some(TStampDeserializer::deserialize("NodeUpdateEndDate", stack)?);
                }
                "NodeUpdateInitiatedBy" => {
                    obj.node_update_initiated_by =
                        Some(NodeUpdateInitiatedByDeserializer::deserialize(
                            "NodeUpdateInitiatedBy",
                            stack,
                        )?);
                }
                "NodeUpdateInitiatedDate" => {
                    obj.node_update_initiated_date = Some(TStampDeserializer::deserialize(
                        "NodeUpdateInitiatedDate",
                        stack,
                    )?);
                }
                "NodeUpdateStartDate" => {
                    obj.node_update_start_date = Some(TStampDeserializer::deserialize(
                        "NodeUpdateStartDate",
                        stack,
                    )?);
                }
                "NodeUpdateStatus" => {
                    obj.node_update_status = Some(NodeUpdateStatusDeserializer::deserialize(
                        "NodeUpdateStatus",
                        stack,
                    )?);
                }
                "NodeUpdateStatusModifiedDate" => {
                    obj.node_update_status_modified_date = Some(TStampDeserializer::deserialize(
                        "NodeUpdateStatusModifiedDate",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct CacheNodeUpdateStatusListDeserializer;
impl CacheNodeUpdateStatusListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheNodeUpdateStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheNodeUpdateStatus" {
                obj.push(CacheNodeUpdateStatusDeserializer::deserialize(
                    "CacheNodeUpdateStatus",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>CreateCacheParameterGroup</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheParameterGroup {
    /// <p>The ARN (Amazon Resource Name) of the cache parameter group.</p>
    pub arn: Option<String>,
    /// <p>The name of the cache parameter group family that this cache parameter group is compatible with.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p>
    pub cache_parameter_group_family: Option<String>,
    /// <p>The name of the cache parameter group.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>The description for this cache parameter group.</p>
    pub description: Option<String>,
    /// <p>Indicates whether the parameter group is associated with a Global Datastore</p>
    pub is_global: Option<bool>,
}

#[allow(dead_code)]
struct CacheParameterGroupDeserializer;
impl CacheParameterGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheParameterGroup, XmlParseError> {
        deserialize_elements::<_, CacheParameterGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "CacheParameterGroupFamily" => {
                    obj.cache_parameter_group_family = Some(StringDeserializer::deserialize(
                        "CacheParameterGroupFamily",
                        stack,
                    )?);
                }
                "CacheParameterGroupName" => {
                    obj.cache_parameter_group_name = Some(StringDeserializer::deserialize(
                        "CacheParameterGroupName",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                "IsGlobal" => {
                    obj.is_global = Some(BooleanDeserializer::deserialize("IsGlobal", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeCacheParameters</code> operation.</p>
/// see [ElastiCache::describe_cache_parameters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheParameterGroupDetails {
    /// <p>A list of parameters specific to a particular cache node type. Each element in the list contains detailed information about one parameter.</p>
    pub cache_node_type_specific_parameters: Option<Vec<CacheNodeTypeSpecificParameter>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
    /// <p>A list of <a>Parameter</a> instances.</p>
    pub parameters: Option<Vec<Parameter>>,
}

impl CacheParameterGroupDetails {
    fn pagination_page_opt(self) -> Option<Vec<Parameter>> {
        Some(self.parameters.as_ref()?.clone())
    }
}

impl PagedOutput for CacheParameterGroupDetails {
    type Item = Parameter;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Parameter> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheParameterGroupDetailsDeserializer;
impl CacheParameterGroupDetailsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheParameterGroupDetails, XmlParseError> {
        deserialize_elements::<_, CacheParameterGroupDetails, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheNodeTypeSpecificParameters" => {
                        obj.cache_node_type_specific_parameters
                            .get_or_insert(vec![])
                            .extend(
                                CacheNodeTypeSpecificParametersListDeserializer::deserialize(
                                    "CacheNodeTypeSpecificParameters",
                                    stack,
                                )?,
                            );
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
            },
        )
    }
}
#[allow(dead_code)]
struct CacheParameterGroupListDeserializer;
impl CacheParameterGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheParameterGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheParameterGroup" {
                obj.push(CacheParameterGroupDeserializer::deserialize(
                    "CacheParameterGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p><p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>ModifyCacheParameterGroup</code> </p> </li> <li> <p> <code>ResetCacheParameterGroup</code> </p> </li> </ul></p>
/// see [ElastiCache::modify_cache_parameter_group]
/// see [ElastiCache::reset_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheParameterGroupNameMessage {
    /// <p>The name of the cache parameter group.</p>
    pub cache_parameter_group_name: Option<String>,
}

#[allow(dead_code)]
struct CacheParameterGroupNameMessageDeserializer;
impl CacheParameterGroupNameMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheParameterGroupNameMessage, XmlParseError> {
        deserialize_elements::<_, CacheParameterGroupNameMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheParameterGroupName" => {
                        obj.cache_parameter_group_name = Some(StringDeserializer::deserialize(
                            "CacheParameterGroupName",
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
/// <p>Status of the cache parameter group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheParameterGroupStatus {
    /// <p>A list of the cache node IDs which need to be rebooted for parameter changes to be applied. A node ID is a numeric identifier (0001, 0002, etc.).</p>
    pub cache_node_ids_to_reboot: Option<Vec<String>>,
    /// <p>The name of the cache parameter group.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>The status of parameter updates.</p>
    pub parameter_apply_status: Option<String>,
}

#[allow(dead_code)]
struct CacheParameterGroupStatusDeserializer;
impl CacheParameterGroupStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheParameterGroupStatus, XmlParseError> {
        deserialize_elements::<_, CacheParameterGroupStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheNodeIdsToReboot" => {
                        obj.cache_node_ids_to_reboot.get_or_insert(vec![]).extend(
                            CacheNodeIdsListDeserializer::deserialize(
                                "CacheNodeIdsToReboot",
                                stack,
                            )?,
                        );
                    }
                    "CacheParameterGroupName" => {
                        obj.cache_parameter_group_name = Some(StringDeserializer::deserialize(
                            "CacheParameterGroupName",
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
            },
        )
    }
}
/// <p>Represents the output of a <code>DescribeCacheParameterGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_parameter_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheParameterGroupsMessage {
    /// <p>A list of cache parameter groups. Each element in the list contains detailed information about one cache parameter group.</p>
    pub cache_parameter_groups: Option<Vec<CacheParameterGroup>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl CacheParameterGroupsMessage {
    fn pagination_page_opt(self) -> Option<Vec<CacheParameterGroup>> {
        Some(self.cache_parameter_groups.as_ref()?.clone())
    }
}

impl PagedOutput for CacheParameterGroupsMessage {
    type Item = CacheParameterGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<CacheParameterGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheParameterGroupsMessageDeserializer;
impl CacheParameterGroupsMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheParameterGroupsMessage, XmlParseError> {
        deserialize_elements::<_, CacheParameterGroupsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheParameterGroups" => {
                        obj.cache_parameter_groups.get_or_insert(vec![]).extend(
                            CacheParameterGroupListDeserializer::deserialize(
                                "CacheParameterGroups",
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
/// <p><p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>AuthorizeCacheSecurityGroupIngress</code> </p> </li> <li> <p> <code>CreateCacheSecurityGroup</code> </p> </li> <li> <p> <code>RevokeCacheSecurityGroupIngress</code> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheSecurityGroup {
    /// <p>The ARN of the cache security group,</p>
    pub arn: Option<String>,
    /// <p>The name of the cache security group.</p>
    pub cache_security_group_name: Option<String>,
    /// <p>The description of the cache security group.</p>
    pub description: Option<String>,
    /// <p>A list of Amazon EC2 security groups that are associated with this cache security group.</p>
    pub ec2_security_groups: Option<Vec<EC2SecurityGroup>>,
    /// <p>The AWS account ID of the cache security group owner.</p>
    pub owner_id: Option<String>,
}

#[allow(dead_code)]
struct CacheSecurityGroupDeserializer;
impl CacheSecurityGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheSecurityGroup, XmlParseError> {
        deserialize_elements::<_, CacheSecurityGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "CacheSecurityGroupName" => {
                    obj.cache_security_group_name = Some(StringDeserializer::deserialize(
                        "CacheSecurityGroupName",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                "EC2SecurityGroups" => {
                    obj.ec2_security_groups.get_or_insert(vec![]).extend(
                        EC2SecurityGroupListDeserializer::deserialize("EC2SecurityGroups", stack)?,
                    );
                }
                "OwnerId" => {
                    obj.owner_id = Some(StringDeserializer::deserialize("OwnerId", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a cluster's status within a particular cache security group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheSecurityGroupMembership {
    /// <p>The name of the cache security group.</p>
    pub cache_security_group_name: Option<String>,
    /// <p>The membership status in the cache security group. The status changes when a cache security group is modified, or when the cache security groups assigned to a cluster are modified.</p>
    pub status: Option<String>,
}

#[allow(dead_code)]
struct CacheSecurityGroupMembershipDeserializer;
impl CacheSecurityGroupMembershipDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheSecurityGroupMembership, XmlParseError> {
        deserialize_elements::<_, CacheSecurityGroupMembership, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSecurityGroupName" => {
                        obj.cache_security_group_name = Some(StringDeserializer::deserialize(
                            "CacheSecurityGroupName",
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
#[allow(dead_code)]
struct CacheSecurityGroupMembershipListDeserializer;
impl CacheSecurityGroupMembershipListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheSecurityGroupMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheSecurityGroup" {
                obj.push(CacheSecurityGroupMembershipDeserializer::deserialize(
                    "CacheSecurityGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeCacheSecurityGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_security_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheSecurityGroupMessage {
    /// <p>A list of cache security groups. Each element in the list contains detailed information about one group.</p>
    pub cache_security_groups: Option<Vec<CacheSecurityGroup>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl CacheSecurityGroupMessage {
    fn pagination_page_opt(self) -> Option<Vec<CacheSecurityGroup>> {
        Some(self.cache_security_groups.as_ref()?.clone())
    }
}

impl PagedOutput for CacheSecurityGroupMessage {
    type Item = CacheSecurityGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<CacheSecurityGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheSecurityGroupMessageDeserializer;
impl CacheSecurityGroupMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheSecurityGroupMessage, XmlParseError> {
        deserialize_elements::<_, CacheSecurityGroupMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSecurityGroups" => {
                        obj.cache_security_groups.get_or_insert(vec![]).extend(
                            CacheSecurityGroupsDeserializer::deserialize(
                                "CacheSecurityGroups",
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

/// Serialize `CacheSecurityGroupNameList` contents to a `SignedRequest`.
struct CacheSecurityGroupNameListSerializer;
impl CacheSecurityGroupNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct CacheSecurityGroupsDeserializer;
impl CacheSecurityGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheSecurityGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheSecurityGroup" {
                obj.push(CacheSecurityGroupDeserializer::deserialize(
                    "CacheSecurityGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p><p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>CreateCacheSubnetGroup</code> </p> </li> <li> <p> <code>ModifyCacheSubnetGroup</code> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheSubnetGroup {
    /// <p>The ARN (Amazon Resource Name) of the cache subnet group.</p>
    pub arn: Option<String>,
    /// <p>The description of the cache subnet group.</p>
    pub cache_subnet_group_description: Option<String>,
    /// <p>The name of the cache subnet group.</p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>A list of subnets associated with the cache subnet group.</p>
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet group.</p>
    pub vpc_id: Option<String>,
}

#[allow(dead_code)]
struct CacheSubnetGroupDeserializer;
impl CacheSubnetGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheSubnetGroup, XmlParseError> {
        deserialize_elements::<_, CacheSubnetGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "CacheSubnetGroupDescription" => {
                    obj.cache_subnet_group_description = Some(StringDeserializer::deserialize(
                        "CacheSubnetGroupDescription",
                        stack,
                    )?);
                }
                "CacheSubnetGroupName" => {
                    obj.cache_subnet_group_name = Some(StringDeserializer::deserialize(
                        "CacheSubnetGroupName",
                        stack,
                    )?);
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
/// <p>Represents the output of a <code>DescribeCacheSubnetGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_subnet_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CacheSubnetGroupMessage {
    /// <p>A list of cache subnet groups. Each element in the list contains detailed information about one group.</p>
    pub cache_subnet_groups: Option<Vec<CacheSubnetGroup>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl CacheSubnetGroupMessage {
    fn pagination_page_opt(self) -> Option<Vec<CacheSubnetGroup>> {
        Some(self.cache_subnet_groups.as_ref()?.clone())
    }
}

impl PagedOutput for CacheSubnetGroupMessage {
    type Item = CacheSubnetGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<CacheSubnetGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct CacheSubnetGroupMessageDeserializer;
impl CacheSubnetGroupMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CacheSubnetGroupMessage, XmlParseError> {
        deserialize_elements::<_, CacheSubnetGroupMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSubnetGroups" => {
                        obj.cache_subnet_groups.get_or_insert(vec![]).extend(
                            CacheSubnetGroupsDeserializer::deserialize("CacheSubnetGroups", stack)?,
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
#[allow(dead_code)]
struct CacheSubnetGroupsDeserializer;
impl CacheSubnetGroupsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<CacheSubnetGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "CacheSubnetGroup" {
                obj.push(CacheSubnetGroupDeserializer::deserialize(
                    "CacheSubnetGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ChangeTypeDeserializer;
impl ChangeTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ClusterIdListDeserializer;
impl ClusterIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ClusterId" {
                obj.push(StringDeserializer::deserialize("ClusterId", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::complete_migration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteMigrationMessage {
    /// <p>Forces the migration to stop without ensuring that data is in sync. It is recommended to use this option only to abort the migration and not recommended when application wants to continue migration to ElastiCache.</p>
    pub force: Option<bool>,
    /// <p>The ID of the replication group to which data is being migrated.</p>
    pub replication_group_id: String,
}

/// Serialize `CompleteMigrationMessage` contents to a `SignedRequest`.
struct CompleteMigrationMessageSerializer;
impl CompleteMigrationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CompleteMigrationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.force {
            params.put(&format!("{}{}", prefix, "Force"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
    }
}

/// see [ElastiCache::complete_migration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CompleteMigrationResponse {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct CompleteMigrationResponseDeserializer;
impl CompleteMigrationResponseDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CompleteMigrationResponse, XmlParseError> {
        deserialize_elements::<_, CompleteMigrationResponse, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// <p>Node group (shard) configuration options when adding or removing replicas. Each node group (shard) configuration has the following members: NodeGroupId, NewReplicaCount, and PreferredAvailabilityZones. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ConfigureShard {
    /// <p><p>The number of replicas you want in this node group at the end of this operation. The maximum value for <code>NewReplicaCount</code> is 5. The minimum value depends upon the type of Redis replication group you are working with.</p> <p>The minimum number of replicas in a shard or replication group is:</p> <ul> <li> <p>Redis (cluster mode disabled)</p> <ul> <li> <p>If Multi-AZ: 1</p> </li> <li> <p>If Multi-AZ: 0</p> </li> </ul> </li> <li> <p>Redis (cluster mode enabled): 0 (though you will not be able to failover to a replica if your primary node fails)</p> </li> </ul></p>
    pub new_replica_count: i64,
    /// <p>The 4-digit id for the node group you are configuring. For Redis (cluster mode disabled) replication groups, the node group id is always 0001. To find a Redis (cluster mode enabled)'s node group's (shard's) id, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/shard-find-id.html">Finding a Shard's Id</a>.</p>
    pub node_group_id: String,
    /// <p>A list of <code>PreferredAvailabilityZone</code> strings that specify which availability zones the replication group's nodes are to be in. The nummber of <code>PreferredAvailabilityZone</code> values must equal the value of <code>NewReplicaCount</code> plus 1 to account for the primary node. If this member of <code>ReplicaConfiguration</code> is omitted, ElastiCache for Redis selects the availability zone for each of the replicas.</p>
    pub preferred_availability_zones: Option<Vec<String>>,
    /// <p>The outpost ARNs in which the cache cluster is created.</p>
    pub preferred_outpost_arns: Option<Vec<String>>,
}

/// Serialize `ConfigureShard` contents to a `SignedRequest`.
struct ConfigureShardSerializer;
impl ConfigureShardSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ConfigureShard) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "NewReplicaCount"),
            &obj.new_replica_count,
        );
        params.put(&format!("{}{}", prefix, "NodeGroupId"), &obj.node_group_id);
        if let Some(ref field_value) = obj.preferred_availability_zones {
            PreferredAvailabilityZoneListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PreferredAvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_outpost_arns {
            PreferredOutpostArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PreferredOutpostArn"),
                field_value,
            );
        }
    }
}

/// <p>Represents the input of a <code>CopySnapshotMessage</code> operation.</p>
/// see [ElastiCache::copy_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CopySnapshotMessage {
    /// <p>The ID of the KMS key used to encrypt the target snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>The name of an existing snapshot from which to make a copy.</p>
    pub source_snapshot_name: String,
    /// <p>The Amazon S3 bucket to which the snapshot is exported. This parameter is used only when exporting a snapshot for external access.</p> <p>When using this parameter to export a snapshot, be sure Amazon ElastiCache has the needed permissions to this S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the <i>Amazon ElastiCache User Guide</i>.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Snapshots.Exporting.html">Exporting a Snapshot</a> in the <i>Amazon ElastiCache User Guide</i>.</p>
    pub target_bucket: Option<String>,
    /// <p>A name for the snapshot copy. ElastiCache does not permit overwriting a snapshot, therefore this name must be unique within its context - ElastiCache or an Amazon S3 bucket if exporting.</p>
    pub target_snapshot_name: String,
}

/// Serialize `CopySnapshotMessage` contents to a `SignedRequest`.
struct CopySnapshotMessageSerializer;
impl CopySnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CopySnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "SourceSnapshotName"),
            &obj.source_snapshot_name,
        );
        if let Some(ref field_value) = obj.target_bucket {
            params.put(&format!("{}{}", prefix, "TargetBucket"), &field_value);
        }
        params.put(
            &format!("{}{}", prefix, "TargetSnapshotName"),
            &obj.target_snapshot_name,
        );
    }
}

/// see [ElastiCache::copy_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CopySnapshotResult {
    pub snapshot: Option<Snapshot>,
}

#[allow(dead_code)]
struct CopySnapshotResultDeserializer;
impl CopySnapshotResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CopySnapshotResult, XmlParseError> {
        deserialize_elements::<_, CopySnapshotResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Snapshot" => {
                    obj.snapshot = Some(SnapshotDeserializer::deserialize("Snapshot", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents the input of a CreateCacheCluster operation.</p>
/// see [ElastiCache::create_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCacheClusterMessage {
    /// <p>Specifies whether the nodes in this Memcached cluster are created in a single Availability Zone or created across multiple Availability Zones in the cluster's region.</p> <p>This parameter is only supported for Memcached clusters.</p> <p>If the <code>AZMode</code> and <code>PreferredAvailabilityZones</code> are not specified, ElastiCache assumes <code>single-az</code> mode.</p>
    pub az_mode: Option<String>,
    /// <p> <b>Reserved parameter.</b> The password used to access a password protected server.</p> <p>Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters.</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length.</p> </li> <li> <p>The only permitted printable special characters are !, &amp;, #, $, ^, &lt;, &gt;, and -. Other printable special characters cannot be used in the AUTH token.</p> </li> </ul> <p>For more information, see <a href="http://redis.io/commands/AUTH">AUTH password</a> at http://redis.io/commands/AUTH.</p>
    pub auth_token: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p><p>The node group (shard) identifier. This parameter is stored as a lowercase string.</p> <p> <b>Constraints:</b> </p> <ul> <li> <p>A name must contain from 1 to 50 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub cache_cluster_id: String,
    /// <p><p>The compute and memory capacity of the nodes in the node group (shard).</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>The name of the parameter group to associate with this cluster. If this argument is omitted, the default parameter group for the specified engine is used. You cannot use any parameter group which has <code>cluster-enabled='yes'</code> when creating a cluster.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>A list of security group names to associate with this cluster.</p> <p>Use this parameter only when you are creating a cluster outside of an Amazon Virtual Private Cloud (Amazon VPC).</p>
    pub cache_security_group_names: Option<Vec<String>>,
    /// <p><p>The name of the subnet group to be used for the cluster.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p> <important> <p>If you&#39;re going to launch your cluster in an Amazon VPC, you need to create a subnet group before you start creating a cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SubnetGroups.html">Subnets and Subnet Groups</a>.</p> </important></p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>The name of the cache engine to be used for this cluster.</p> <p>Valid values for this parameter are: <code>memcached</code> | <code>redis</code> </p>
    pub engine: Option<String>,
    /// <p>The version number of the cache engine to be used for this cluster. To view the supported cache engine versions, use the DescribeCacheEngineVersions operation.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SelectEngine.html#VersionManagement">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cluster or replication group and create it anew with the earlier engine version. </p>
    pub engine_version: Option<String>,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be the same as the cluster owner.</p> </note></p>
    pub notification_topic_arn: Option<String>,
    /// <p>The initial number of cache nodes that the cluster has.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p> <p>If you need more than 20 nodes for your Memcached cluster, please fill out the ElastiCache Limit Increase Request form at <a href="http://aws.amazon.com/contact-us/elasticache-node-limit-request/">http://aws.amazon.com/contact-us/elasticache-node-limit-request/</a>.</p>
    pub num_cache_nodes: Option<i64>,
    /// <p>Specifies whether the nodes in the cluster are created in a single outpost or across multiple outposts.</p>
    pub outpost_mode: Option<String>,
    /// <p>The port number on which each of the cache nodes accepts connections.</p>
    pub port: Option<i64>,
    /// <p>The EC2 Availability Zone in which the cluster is created.</p> <p>All nodes belonging to this cluster are placed in the preferred Availability Zone. If you want to create your nodes across multiple Availability Zones, use <code>PreferredAvailabilityZones</code>.</p> <p>Default: System chosen Availability Zone.</p>
    pub preferred_availability_zone: Option<String>,
    /// <p>A list of the Availability Zones in which cache nodes are created. The order of the zones in the list is not important.</p> <p>This option is only supported on Memcached.</p> <note> <p>If you are creating your cluster in an Amazon VPC (recommended) you can only locate nodes in Availability Zones that are associated with the subnets in the selected subnet group.</p> <p>The number of Availability Zones listed must equal the value of <code>NumCacheNodes</code>.</p> </note> <p>If you want all the nodes in the same Availability Zone, use <code>PreferredAvailabilityZone</code> instead, or repeat the Availability Zone multiple times in the list.</p> <p>Default: System chosen Availability Zones.</p>
    pub preferred_availability_zones: Option<Vec<String>>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The outpost ARN in which the cache cluster is created.</p>
    pub preferred_outpost_arn: Option<String>,
    /// <p>The outpost ARNs in which the cache cluster is created.</p>
    pub preferred_outpost_arns: Option<Vec<String>>,
    /// <p><p>The ID of the replication group to which this cluster should belong. If this parameter is specified, the cluster is added to the specified replication group as a read replica; otherwise, the cluster is a standalone primary that is not part of any replication group.</p> <p>If the specified replication group is Multi-AZ enabled and the Availability Zone is not specified, the cluster is created in Availability Zones that provide the best spread of read replicas across Availability Zones.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note></p>
    pub replication_group_id: Option<String>,
    /// <p>One or more VPC security groups associated with the cluster.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A single-element string list containing an Amazon Resource Name (ARN) that uniquely identifies a Redis RDB snapshot file stored in Amazon S3. The snapshot file is used to populate the node group (shard). The Amazon S3 object name in the ARN cannot contain any commas.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code> </p>
    pub snapshot_arns: Option<Vec<String>>,
    /// <p><p>The name of a Redis snapshot from which to restore data into the new node group (shard). The snapshot status changes to <code>restoring</code> while the new node group (shard) is being created.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note></p>
    pub snapshot_name: Option<String>,
    /// <p>The number of days for which ElastiCache retains automatic snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot taken today is retained for 5 days before being deleted.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Default: 0 (i.e., automatic backups are disabled for this cache cluster).</p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p><p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note></p>
    pub snapshot_window: Option<String>,
    /// <p>A list of cost allocation tags to be added to this resource.</p>
    pub tags: Option<Vec<Tag>>,
}

/// Serialize `CreateCacheClusterMessage` contents to a `SignedRequest`.
struct CreateCacheClusterMessageSerializer;
impl CreateCacheClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateCacheClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.az_mode {
            params.put(&format!("{}{}", prefix, "AZMode"), &field_value);
        }
        if let Some(ref field_value) = obj.auth_token {
            params.put(&format!("{}{}", prefix, "AuthToken"), &field_value);
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "CacheClusterId"),
            &obj.cache_cluster_id,
        );
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheSubnetGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicArn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.num_cache_nodes {
            params.put(&format!("{}{}", prefix, "NumCacheNodes"), &field_value);
        }
        if let Some(ref field_value) = obj.outpost_mode {
            params.put(&format!("{}{}", prefix, "OutpostMode"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_availability_zone {
            params.put(
                &format!("{}{}", prefix, "PreferredAvailabilityZone"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_availability_zones {
            PreferredAvailabilityZoneListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PreferredAvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_outpost_arn {
            params.put(
                &format!("{}{}", prefix, "PreferredOutpostArn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_outpost_arns {
            PreferredOutpostArnListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PreferredOutpostArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroupId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_arns {
            SnapshotArnsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SnapshotArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(
                &format!("{}{}", prefix, "SnapshotRetentionLimit"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
    }
}

/// see [ElastiCache::create_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

#[allow(dead_code)]
struct CreateCacheClusterResultDeserializer;
impl CreateCacheClusterResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCacheClusterResult, XmlParseError> {
        deserialize_elements::<_, CreateCacheClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheCluster" => {
                        obj.cache_cluster = Some(CacheClusterDeserializer::deserialize(
                            "CacheCluster",
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
/// <p>Represents the input of a <code>CreateCacheParameterGroup</code> operation.</p>
/// see [ElastiCache::create_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCacheParameterGroupMessage {
    /// <p>The name of the cache parameter group family that the cache parameter group can be used with.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p>
    pub cache_parameter_group_family: String,
    /// <p>A user-specified name for the cache parameter group.</p>
    pub cache_parameter_group_name: String,
    /// <p>A user-specified description for the cache parameter group.</p>
    pub description: String,
}

/// Serialize `CreateCacheParameterGroupMessage` contents to a `SignedRequest`.
struct CreateCacheParameterGroupMessageSerializer;
impl CreateCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupFamily"),
            &obj.cache_parameter_group_family,
        );
        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupName"),
            &obj.cache_parameter_group_name,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
    }
}

/// see [ElastiCache::create_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCacheParameterGroupResult {
    pub cache_parameter_group: Option<CacheParameterGroup>,
}

#[allow(dead_code)]
struct CreateCacheParameterGroupResultDeserializer;
impl CreateCacheParameterGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCacheParameterGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateCacheParameterGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheParameterGroup" => {
                        obj.cache_parameter_group =
                            Some(CacheParameterGroupDeserializer::deserialize(
                                "CacheParameterGroup",
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
/// <p>Represents the input of a <code>CreateCacheSecurityGroup</code> operation.</p>
/// see [ElastiCache::create_cache_security_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCacheSecurityGroupMessage {
    /// <p>A name for the cache security group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Cannot be the word "Default".</p> <p>Example: <code>mysecuritygroup</code> </p>
    pub cache_security_group_name: String,
    /// <p>A description for the cache security group.</p>
    pub description: String,
}

/// Serialize `CreateCacheSecurityGroupMessage` contents to a `SignedRequest`.
struct CreateCacheSecurityGroupMessageSerializer;
impl CreateCacheSecurityGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateCacheSecurityGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSecurityGroupName"),
            &obj.cache_security_group_name,
        );
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);
    }
}

/// see [ElastiCache::create_cache_security_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCacheSecurityGroupResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[allow(dead_code)]
struct CreateCacheSecurityGroupResultDeserializer;
impl CreateCacheSecurityGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCacheSecurityGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateCacheSecurityGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSecurityGroup" => {
                        obj.cache_security_group =
                            Some(CacheSecurityGroupDeserializer::deserialize(
                                "CacheSecurityGroup",
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
/// <p>Represents the input of a <code>CreateCacheSubnetGroup</code> operation.</p>
/// see [ElastiCache::create_cache_subnet_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCacheSubnetGroupMessage {
    /// <p>A description for the cache subnet group.</p>
    pub cache_subnet_group_description: String,
    /// <p>A name for the cache subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p> <p>Example: <code>mysubnetgroup</code> </p>
    pub cache_subnet_group_name: String,
    /// <p>A list of VPC subnet IDs for the cache subnet group.</p>
    pub subnet_ids: Vec<String>,
}

/// Serialize `CreateCacheSubnetGroupMessage` contents to a `SignedRequest`.
struct CreateCacheSubnetGroupMessageSerializer;
impl CreateCacheSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateCacheSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSubnetGroupDescription"),
            &obj.cache_subnet_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "CacheSubnetGroupName"),
            &obj.cache_subnet_group_name,
        );
        SubnetIdentifierListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "SubnetIdentifier"),
            &obj.subnet_ids,
        );
    }
}

/// see [ElastiCache::create_cache_subnet_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateCacheSubnetGroupResult {
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

#[allow(dead_code)]
struct CreateCacheSubnetGroupResultDeserializer;
impl CreateCacheSubnetGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateCacheSubnetGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateCacheSubnetGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSubnetGroup" => {
                        obj.cache_subnet_group = Some(CacheSubnetGroupDeserializer::deserialize(
                            "CacheSubnetGroup",
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
/// see [ElastiCache::create_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateGlobalReplicationGroupMessage {
    /// <p>Provides details of the Global Datastore</p>
    pub global_replication_group_description: Option<String>,
    /// <p>The suffix name of a Global Datastore. Amazon ElastiCache automatically applies a prefix to the Global Datastore ID when it is created. Each AWS Region has its own prefix. For instance, a Global Datastore ID created in the US-West-1 region will begin with "dsdfu" along with the suffix name you provide. The suffix, combined with the auto-generated prefix, guarantees uniqueness of the Global Datastore name across multiple regions. </p> <p>For a full list of AWS Regions and their respective Global Datastore iD prefixes, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Redis-Global-Datastores-CLI.html">Using the AWS CLI with Global Datastores </a>.</p>
    pub global_replication_group_id_suffix: String,
    /// <p>The name of the primary cluster that accepts writes and will replicate updates to the secondary cluster.</p>
    pub primary_replication_group_id: String,
}

/// Serialize `CreateGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct CreateGlobalReplicationGroupMessageSerializer;
impl CreateGlobalReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateGlobalReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.global_replication_group_description {
            params.put(
                &format!("{}{}", prefix, "GlobalReplicationGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupIdSuffix"),
            &obj.global_replication_group_id_suffix,
        );
        params.put(
            &format!("{}{}", prefix, "PrimaryReplicationGroupId"),
            &obj.primary_replication_group_id,
        );
    }
}

/// see [ElastiCache::create_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct CreateGlobalReplicationGroupResultDeserializer;
impl CreateGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// <p>Represents the input of a <code>CreateReplicationGroup</code> operation.</p>
/// see [ElastiCache::create_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReplicationGroupMessage {
    /// <p>A flag that enables encryption at rest when set to <code>true</code>.</p> <p>You cannot modify the value of <code>AtRestEncryptionEnabled</code> after the replication group is created. To enable encryption at rest on a replication group you must set <code>AtRestEncryptionEnabled</code> to <code>true</code> when you create the replication group. </p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p>
    pub at_rest_encryption_enabled: Option<bool>,
    /// <p> <b>Reserved parameter.</b> The password used to access a password protected server.</p> <p> <code>AuthToken</code> can be specified only on replication groups where <code>TransitEncryptionEnabled</code> is <code>true</code>.</p> <important> <p>For HIPAA compliance, you must specify <code>TransitEncryptionEnabled</code> as <code>true</code>, an <code>AuthToken</code>, and a <code>CacheSubnetGroup</code>.</p> </important> <p>Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters.</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length.</p> </li> <li> <p>The only permitted printable special characters are !, &amp;, #, $, ^, &lt;, &gt;, and -. Other printable special characters cannot be used in the AUTH token.</p> </li> </ul> <p>For more information, see <a href="http://redis.io/commands/AUTH">AUTH password</a> at http://redis.io/commands/AUTH.</p>
    pub auth_token: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Specifies whether a read-only replica is automatically promoted to read/write primary if the existing primary fails.</p> <p> <code>AutomaticFailoverEnabled</code> must be enabled for Redis (cluster mode enabled) replication groups.</p> <p>Default: false</p>
    pub automatic_failover_enabled: Option<bool>,
    /// <p><p>The compute and memory capacity of the nodes in the node group (shard).</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p><p>The name of the parameter group to associate with this replication group. If this argument is omitted, the default cache parameter group for the specified engine is used.</p> <note> <p>If you are restoring to an engine version that is different than the original, you must specify the default version of that version. For example, <code>CacheParameterGroupName=default.redis4.0</code>.</p> </note> <p>If you are running Redis version 3.2.4 or later, only one node group (shard), and want to use a default parameter group, we recommend that you specify the parameter group by name. </p> <ul> <li> <p>To create a Redis (cluster mode disabled) replication group, use <code>CacheParameterGroupName=default.redis3.2</code>.</p> </li> <li> <p>To create a Redis (cluster mode enabled) replication group, use <code>CacheParameterGroupName=default.redis3.2.cluster.on</code>.</p> </li> </ul></p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>A list of cache security group names to associate with this replication group.</p>
    pub cache_security_group_names: Option<Vec<String>>,
    /// <p><p>The name of the cache subnet group to be used for the replication group.</p> <important> <p>If you&#39;re going to launch your cluster in an Amazon VPC, you need to create a subnet group before you start creating a cluster. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SubnetGroups.html">Subnets and Subnet Groups</a>.</p> </important></p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>The name of the cache engine to be used for the clusters in this replication group.</p>
    pub engine: Option<String>,
    /// <p>The version number of the cache engine to be used for the clusters in this replication group. To view the supported cache engine versions, use the <code>DescribeCacheEngineVersions</code> operation.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SelectEngine.html#VersionManagement">Selecting a Cache Engine and Version</a>) in the <i>ElastiCache User Guide</i>, but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cluster or replication group and create it anew with the earlier engine version. </p>
    pub engine_version: Option<String>,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: Option<String>,
    /// <p>The ID of the KMS key used to encrypt the disk in the cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/AutoFailover.html">Minimizing Downtime: Multi-AZ</a>.</p>
    pub multi_az_enabled: Option<bool>,
    /// <p>A list of node group (shard) configuration options. Each node group (shard) configuration has the following members: <code>PrimaryAvailabilityZone</code>, <code>ReplicaAvailabilityZones</code>, <code>ReplicaCount</code>, and <code>Slots</code>.</p> <p>If you're creating a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group, you can use this parameter to individually configure each node group (shard), or you can omit this parameter. However, it is required when seeding a Redis (cluster mode enabled) cluster from a S3 rdb file. You must configure each node group (shard) using this parameter because you must specify the slots for each node group.</p>
    pub node_group_configuration: Option<Vec<NodeGroupConfiguration>>,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be the same as the cluster owner.</p> </note></p>
    pub notification_topic_arn: Option<String>,
    /// <p>The number of clusters this replication group initially has.</p> <p>This parameter is not used if there is more than one node group (shard). You should use <code>ReplicasPerNodeGroup</code> instead.</p> <p>If <code>AutomaticFailoverEnabled</code> is <code>true</code>, the value of this parameter must be at least 2. If <code>AutomaticFailoverEnabled</code> is <code>false</code> you can omit this parameter (it will default to 1), or you can explicitly set it to a value between 2 and 6.</p> <p>The maximum permitted value for <code>NumCacheClusters</code> is 6 (1 primary plus 5 replicas).</p>
    pub num_cache_clusters: Option<i64>,
    /// <p>An optional parameter that specifies the number of node groups (shards) for this Redis (cluster mode enabled) replication group. For Redis (cluster mode disabled) either omit this parameter or set it to 1.</p> <p>Default: 1</p>
    pub num_node_groups: Option<i64>,
    /// <p>The port number on which each member of the replication group accepts connections.</p>
    pub port: Option<i64>,
    /// <p>A list of EC2 Availability Zones in which the replication group's clusters are created. The order of the Availability Zones in the list is the order in which clusters are allocated. The primary cluster is created in the first AZ in the list.</p> <p>This parameter is not used if there is more than one node group (shard). You should use <code>NodeGroupConfiguration</code> instead.</p> <note> <p>If you are creating your replication group in an Amazon VPC (recommended), you can only locate clusters in Availability Zones associated with the subnets in the selected subnet group.</p> <p>The number of Availability Zones listed must equal the value of <code>NumCacheClusters</code>.</p> </note> <p>Default: system chosen Availability Zones.</p>
    pub preferred_cache_cluster_a_zs: Option<Vec<String>>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The identifier of the cluster that serves as the primary for this replication group. This cluster must already exist and have a status of <code>available</code>.</p> <p>This parameter is not required if <code>NumCacheClusters</code>, <code>NumNodeGroups</code>, or <code>ReplicasPerNodeGroup</code> is specified.</p>
    pub primary_cluster_id: Option<String>,
    /// <p>An optional parameter that specifies the number of replica nodes in each node group (shard). Valid values are 0 to 5.</p>
    pub replicas_per_node_group: Option<i64>,
    /// <p>A user-created description for the replication group.</p>
    pub replication_group_description: String,
    /// <p><p>The replication group identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>A name must contain from 1 to 40 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    pub replication_group_id: String,
    /// <p>One or more Amazon VPC security groups associated with this replication group.</p> <p>Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of Amazon Resource Names (ARN) that uniquely identify the Redis RDB snapshot files stored in Amazon S3. The snapshot files are used to populate the new replication group. The Amazon S3 object name in the ARN cannot contain any commas. The new replication group will have the number of node groups (console: shards) specified by the parameter <i>NumNodeGroups</i> or the number of node groups configured by <i>NodeGroupConfiguration</i> regardless of the number of ARNs specified here.</p> <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code> </p>
    pub snapshot_arns: Option<Vec<String>>,
    /// <p>The name of a snapshot from which to restore data into the new replication group. The snapshot status changes to <code>restoring</code> while the new replication group is being created.</p>
    pub snapshot_name: Option<String>,
    /// <p>The number of days for which ElastiCache retains automatic snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <p>Default: 0 (i.e., automatic backups are disabled for this cluster).</p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p>
    pub snapshot_window: Option<String>,
    /// <p>A list of cost allocation tags to be added to this resource. Tags are comma-separated key,value pairs (e.g. Key=<code>myKey</code>, Value=<code>myKeyValue</code>. You can include multiple tags as shown following: Key=<code>myKey</code>, Value=<code>myKeyValue</code> Key=<code>mySecondKey</code>, Value=<code>mySecondKeyValue</code>.</p>
    pub tags: Option<Vec<Tag>>,
    /// <p><p>A flag that enables in-transit encryption when set to <code>true</code>.</p> <p>You cannot modify the value of <code>TransitEncryptionEnabled</code> after the cluster is created. To enable in-transit encryption on a cluster you must set <code>TransitEncryptionEnabled</code> to <code>true</code> when you create a cluster.</p> <p>This parameter is valid only if the <code>Engine</code> parameter is <code>redis</code>, the <code>EngineVersion</code> parameter is <code>3.2.6</code>, <code>4.x</code> or later, and the cluster is being created in an Amazon VPC.</p> <p>If you enable in-transit encryption, you must also specify a value for <code>CacheSubnetGroup</code>.</p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p> <important> <p>For HIPAA compliance, you must specify <code>TransitEncryptionEnabled</code> as <code>true</code>, an <code>AuthToken</code>, and a <code>CacheSubnetGroup</code>.</p> </important></p>
    pub transit_encryption_enabled: Option<bool>,
    /// <p>The list of user groups to associate with the replication group.</p>
    pub user_group_ids: Option<Vec<String>>,
}

/// Serialize `CreateReplicationGroupMessage` contents to a `SignedRequest`.
struct CreateReplicationGroupMessageSerializer;
impl CreateReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.at_rest_encryption_enabled {
            params.put(
                &format!("{}{}", prefix, "AtRestEncryptionEnabled"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.auth_token {
            params.put(&format!("{}{}", prefix, "AuthToken"), &field_value);
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.automatic_failover_enabled {
            params.put(
                &format!("{}{}", prefix, "AutomaticFailoverEnabled"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheSubnetGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.global_replication_group_id {
            params.put(
                &format!("{}{}", prefix, "GlobalReplicationGroupId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az_enabled {
            params.put(&format!("{}{}", prefix, "MultiAZEnabled"), &field_value);
        }
        if let Some(ref field_value) = obj.node_group_configuration {
            NodeGroupConfigurationListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NodeGroupConfiguration"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicArn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.num_cache_clusters {
            params.put(&format!("{}{}", prefix, "NumCacheClusters"), &field_value);
        }
        if let Some(ref field_value) = obj.num_node_groups {
            params.put(&format!("{}{}", prefix, "NumNodeGroups"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_cache_cluster_a_zs {
            AvailabilityZonesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.primary_cluster_id {
            params.put(&format!("{}{}", prefix, "PrimaryClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.replicas_per_node_group {
            params.put(
                &format!("{}{}", prefix, "ReplicasPerNodeGroup"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupDescription"),
            &obj.replication_group_description,
        );
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroupId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_arns {
            SnapshotArnsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SnapshotArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(
                &format!("{}{}", prefix, "SnapshotRetentionLimit"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tag"), field_value);
        }
        if let Some(ref field_value) = obj.transit_encryption_enabled {
            params.put(
                &format!("{}{}", prefix, "TransitEncryptionEnabled"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.user_group_ids {
            UserGroupIdListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserGroupIds"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::create_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct CreateReplicationGroupResultDeserializer;
impl CreateReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, CreateReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// <p>Represents the input of a <code>CreateSnapshot</code> operation.</p>
/// see [ElastiCache::create_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateSnapshotMessage {
    /// <p>The identifier of an existing cluster. The snapshot is created from this cluster.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The ID of the KMS key used to encrypt the snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>The identifier of an existing replication group. The snapshot is created from this replication group.</p>
    pub replication_group_id: Option<String>,
    /// <p>A name for the snapshot being created.</p>
    pub snapshot_name: String,
}

/// Serialize `CreateSnapshotMessage` contents to a `SignedRequest`.
struct CreateSnapshotMessageSerializer;
impl CreateSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_id {
            params.put(&format!("{}{}", prefix, "CacheClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.kms_key_id {
            params.put(&format!("{}{}", prefix, "KmsKeyId"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "SnapshotName"), &obj.snapshot_name);
    }
}

/// see [ElastiCache::create_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct CreateSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

#[allow(dead_code)]
struct CreateSnapshotResultDeserializer;
impl CreateSnapshotResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<CreateSnapshotResult, XmlParseError> {
        deserialize_elements::<_, CreateSnapshotResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Snapshot" => {
                    obj.snapshot = Some(SnapshotDeserializer::deserialize("Snapshot", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::create_user_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserGroupMessage {
    /// <p>Must be Redis. </p>
    pub engine: String,
    /// <p>The ID of the user group.</p>
    pub user_group_id: String,
    /// <p>The list of user IDs that belong to the user group.</p>
    pub user_ids: Option<Vec<String>>,
}

/// Serialize `CreateUserGroupMessage` contents to a `SignedRequest`.
struct CreateUserGroupMessageSerializer;
impl CreateUserGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateUserGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        params.put(&format!("{}{}", prefix, "UserGroupId"), &obj.user_group_id);
        if let Some(ref field_value) = obj.user_ids {
            UserIdListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserIds"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::create_user]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateUserMessage {
    /// <p>Access permissions string used for this user account.</p>
    pub access_string: String,
    /// <p>Must be Redis. </p>
    pub engine: String,
    /// <p>Indicates a password is not required for this user account.</p>
    pub no_password_required: Option<bool>,
    /// <p>Passwords used for this user account. You can create up to two passwords for each user.</p>
    pub passwords: Option<Vec<String>>,
    /// <p>The ID of the user.</p>
    pub user_id: String,
    /// <p>The username of the user.</p>
    pub user_name: String,
}

/// Serialize `CreateUserMessage` contents to a `SignedRequest`.
struct CreateUserMessageSerializer;
impl CreateUserMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateUserMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "AccessString"), &obj.access_string);
        params.put(&format!("{}{}", prefix, "Engine"), &obj.engine);
        if let Some(ref field_value) = obj.no_password_required {
            params.put(&format!("{}{}", prefix, "NoPasswordRequired"), &field_value);
        }
        if let Some(ref field_value) = obj.passwords {
            PasswordListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Passwords"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "UserId"), &obj.user_id);
        params.put(&format!("{}{}", prefix, "UserName"), &obj.user_name);
    }
}

/// <p>The endpoint from which data should be migrated.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CustomerNodeEndpoint {
    /// <p>The address of the node endpoint</p>
    pub address: Option<String>,
    /// <p>The port of the node endpoint</p>
    pub port: Option<i64>,
}

/// Serialize `CustomerNodeEndpoint` contents to a `SignedRequest`.
struct CustomerNodeEndpointSerializer;
impl CustomerNodeEndpointSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CustomerNodeEndpoint) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.address {
            params.put(&format!("{}{}", prefix, "Address"), &field_value);
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value);
        }
    }
}

/// Serialize `CustomerNodeEndpointList` contents to a `SignedRequest`.
struct CustomerNodeEndpointListSerializer;
impl CustomerNodeEndpointListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<CustomerNodeEndpoint>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            CustomerNodeEndpointSerializer::serialize(params, &key, obj);
        }
    }
}

/// see [ElastiCache::decrease_node_groups_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecreaseNodeGroupsInGlobalReplicationGroupMessage {
    /// <p>Indicates that the shard reconfiguration process begins immediately. At present, the only permitted value for this parameter is true. </p>
    pub apply_immediately: bool,
    /// <p>If the value of NodeGroupCount is less than the current number of node groups (shards), then either NodeGroupsToRemove or NodeGroupsToRetain is required. NodeGroupsToRemove is a list of NodeGroupIds to remove from the cluster. ElastiCache for Redis will attempt to remove all node groups listed by NodeGroupsToRemove from the cluster. </p>
    pub global_node_groups_to_remove: Option<Vec<String>>,
    /// <p>If the value of NodeGroupCount is less than the current number of node groups (shards), then either NodeGroupsToRemove or NodeGroupsToRetain is required. NodeGroupsToRemove is a list of NodeGroupIds to remove from the cluster. ElastiCache for Redis will attempt to remove all node groups listed by NodeGroupsToRemove from the cluster. </p>
    pub global_node_groups_to_retain: Option<Vec<String>>,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
    /// <p>The number of node groups (shards) that results from the modification of the shard configuration</p>
    pub node_group_count: i64,
}

/// Serialize `DecreaseNodeGroupsInGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct DecreaseNodeGroupsInGlobalReplicationGroupMessageSerializer;
impl DecreaseNodeGroupsInGlobalReplicationGroupMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DecreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        if let Some(ref field_value) = obj.global_node_groups_to_remove {
            GlobalNodeGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "GlobalNodeGroupId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.global_node_groups_to_retain {
            GlobalNodeGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "GlobalNodeGroupId"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "NodeGroupCount"),
            &obj.node_group_count,
        );
    }
}

/// see [ElastiCache::decrease_node_groups_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DecreaseNodeGroupsInGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct DecreaseNodeGroupsInGlobalReplicationGroupResultDeserializer;
impl DecreaseNodeGroupsInGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DecreaseNodeGroupsInGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, DecreaseNodeGroupsInGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// see [ElastiCache::decrease_replica_count]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DecreaseReplicaCountMessage {
    /// <p>If <code>True</code>, the number of replica nodes is decreased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub apply_immediately: bool,
    /// <p><p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group&#39;s node groups.</p> <p>The minimum number of replicas in a shard or replication group is:</p> <ul> <li> <p>Redis (cluster mode disabled)</p> <ul> <li> <p>If Multi-AZ is enabled: 1</p> </li> <li> <p>If Multi-AZ is not enabled: 0</p> </li> </ul> </li> <li> <p>Redis (cluster mode enabled): 0 (though you will not be able to failover to a replica if your primary node fails)</p> </li> </ul></p>
    pub new_replica_count: Option<i64>,
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub replica_configuration: Option<Vec<ConfigureShard>>,
    /// <p>A list of the node ids to remove from the replication group or node group (shard).</p>
    pub replicas_to_remove: Option<Vec<String>>,
    /// <p>The id of the replication group from which you want to remove replica nodes.</p>
    pub replication_group_id: String,
}

/// Serialize `DecreaseReplicaCountMessage` contents to a `SignedRequest`.
struct DecreaseReplicaCountMessageSerializer;
impl DecreaseReplicaCountMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DecreaseReplicaCountMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        if let Some(ref field_value) = obj.new_replica_count {
            params.put(&format!("{}{}", prefix, "NewReplicaCount"), &field_value);
        }
        if let Some(ref field_value) = obj.replica_configuration {
            ReplicaConfigurationListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ConfigureShard"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replicas_to_remove {
            RemoveReplicasListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplicasToRemove"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
    }
}

/// see [ElastiCache::decrease_replica_count]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DecreaseReplicaCountResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct DecreaseReplicaCountResultDeserializer;
impl DecreaseReplicaCountResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DecreaseReplicaCountResult, XmlParseError> {
        deserialize_elements::<_, DecreaseReplicaCountResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// <p>Represents the input of a <code>DeleteCacheCluster</code> operation.</p>
/// see [ElastiCache::delete_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCacheClusterMessage {
    /// <p>The cluster identifier for the cluster to be deleted. This parameter is not case sensitive.</p>
    pub cache_cluster_id: String,
    /// <p>The user-supplied name of a final cluster snapshot. This is the unique name that identifies the snapshot. ElastiCache creates the snapshot, and then deletes the cluster immediately afterward.</p>
    pub final_snapshot_identifier: Option<String>,
}

/// Serialize `DeleteCacheClusterMessage` contents to a `SignedRequest`.
struct DeleteCacheClusterMessageSerializer;
impl DeleteCacheClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteCacheClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheClusterId"),
            &obj.cache_cluster_id,
        );
        if let Some(ref field_value) = obj.final_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalSnapshotIdentifier"),
                &field_value,
            );
        }
    }
}

/// see [ElastiCache::delete_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

#[allow(dead_code)]
struct DeleteCacheClusterResultDeserializer;
impl DeleteCacheClusterResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteCacheClusterResult, XmlParseError> {
        deserialize_elements::<_, DeleteCacheClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheCluster" => {
                        obj.cache_cluster = Some(CacheClusterDeserializer::deserialize(
                            "CacheCluster",
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
/// <p>Represents the input of a <code>DeleteCacheParameterGroup</code> operation.</p>
/// see [ElastiCache::delete_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCacheParameterGroupMessage {
    /// <p><p>The name of the cache parameter group to delete.</p> <note> <p>The specified cache security group must not be associated with any clusters.</p> </note></p>
    pub cache_parameter_group_name: String,
}

/// Serialize `DeleteCacheParameterGroupMessage` contents to a `SignedRequest`.
struct DeleteCacheParameterGroupMessageSerializer;
impl DeleteCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupName"),
            &obj.cache_parameter_group_name,
        );
    }
}

/// <p>Represents the input of a <code>DeleteCacheSecurityGroup</code> operation.</p>
/// see [ElastiCache::delete_cache_security_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCacheSecurityGroupMessage {
    /// <p><p>The name of the cache security group to delete.</p> <note> <p>You cannot delete the default security group.</p> </note></p>
    pub cache_security_group_name: String,
}

/// Serialize `DeleteCacheSecurityGroupMessage` contents to a `SignedRequest`.
struct DeleteCacheSecurityGroupMessageSerializer;
impl DeleteCacheSecurityGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteCacheSecurityGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSecurityGroupName"),
            &obj.cache_security_group_name,
        );
    }
}

/// <p>Represents the input of a <code>DeleteCacheSubnetGroup</code> operation.</p>
/// see [ElastiCache::delete_cache_subnet_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCacheSubnetGroupMessage {
    /// <p>The name of the cache subnet group to delete.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>
    pub cache_subnet_group_name: String,
}

/// Serialize `DeleteCacheSubnetGroupMessage` contents to a `SignedRequest`.
struct DeleteCacheSubnetGroupMessageSerializer;
impl DeleteCacheSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteCacheSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSubnetGroupName"),
            &obj.cache_subnet_group_name,
        );
    }
}

/// see [ElastiCache::delete_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteGlobalReplicationGroupMessage {
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
    /// <p>The primary replication group is retained as a standalone replication group. </p>
    pub retain_primary_replication_group: bool,
}

/// Serialize `DeleteGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct DeleteGlobalReplicationGroupMessageSerializer;
impl DeleteGlobalReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteGlobalReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "RetainPrimaryReplicationGroup"),
            &obj.retain_primary_replication_group,
        );
    }
}

/// see [ElastiCache::delete_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct DeleteGlobalReplicationGroupResultDeserializer;
impl DeleteGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, DeleteGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// <p>Represents the input of a <code>DeleteReplicationGroup</code> operation.</p>
/// see [ElastiCache::delete_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationGroupMessage {
    /// <p>The name of a final node group (shard) snapshot. ElastiCache creates the snapshot from the primary node in the cluster, rather than one of the replicas; this is to ensure that it captures the freshest data. After the final snapshot is taken, the replication group is immediately deleted.</p>
    pub final_snapshot_identifier: Option<String>,
    /// <p>The identifier for the cluster to be deleted. This parameter is not case sensitive.</p>
    pub replication_group_id: String,
    /// <p>If set to <code>true</code>, all of the read replicas are deleted, but the primary node is retained.</p>
    pub retain_primary_cluster: Option<bool>,
}

/// Serialize `DeleteReplicationGroupMessage` contents to a `SignedRequest`.
struct DeleteReplicationGroupMessageSerializer;
impl DeleteReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.final_snapshot_identifier {
            params.put(
                &format!("{}{}", prefix, "FinalSnapshotIdentifier"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        if let Some(ref field_value) = obj.retain_primary_cluster {
            params.put(
                &format!("{}{}", prefix, "RetainPrimaryCluster"),
                &field_value,
            );
        }
    }
}

/// see [ElastiCache::delete_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct DeleteReplicationGroupResultDeserializer;
impl DeleteReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, DeleteReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// <p>Represents the input of a <code>DeleteSnapshot</code> operation.</p>
/// see [ElastiCache::delete_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteSnapshotMessage {
    /// <p>The name of the snapshot to be deleted.</p>
    pub snapshot_name: String,
}

/// Serialize `DeleteSnapshotMessage` contents to a `SignedRequest`.
struct DeleteSnapshotMessageSerializer;
impl DeleteSnapshotMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteSnapshotMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "SnapshotName"), &obj.snapshot_name);
    }
}

/// see [ElastiCache::delete_snapshot]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DeleteSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

#[allow(dead_code)]
struct DeleteSnapshotResultDeserializer;
impl DeleteSnapshotResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DeleteSnapshotResult, XmlParseError> {
        deserialize_elements::<_, DeleteSnapshotResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Snapshot" => {
                    obj.snapshot = Some(SnapshotDeserializer::deserialize("Snapshot", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::delete_user_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserGroupMessage {
    /// <p>The ID of the user group.</p>
    pub user_group_id: String,
}

/// Serialize `DeleteUserGroupMessage` contents to a `SignedRequest`.
struct DeleteUserGroupMessageSerializer;
impl DeleteUserGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteUserGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "UserGroupId"), &obj.user_group_id);
    }
}

/// see [ElastiCache::delete_user]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteUserMessage {
    /// <p>The ID of the user.</p>
    pub user_id: String,
}

/// Serialize `DeleteUserMessage` contents to a `SignedRequest`.
struct DeleteUserMessageSerializer;
impl DeleteUserMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DeleteUserMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "UserId"), &obj.user_id);
    }
}

/// <p>Represents the input of a <code>DescribeCacheClusters</code> operation.</p>
/// see [ElastiCache::describe_cache_clusters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheClustersMessage {
    /// <p>The user-supplied cluster identifier. If this parameter is specified, only information about that specific cluster is returned. This parameter isn't case sensitive.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to show only nodes (API/CLI: clusters) that are not members of a replication group. In practice, this mean Memcached and single node Redis clusters.</p>
    pub show_cache_clusters_not_in_replication_groups: Option<bool>,
    /// <p>An optional flag that can be included in the <code>DescribeCacheCluster</code> request to retrieve information about the individual cache nodes.</p>
    pub show_cache_node_info: Option<bool>,
}

impl PagedRequest for DescribeCacheClustersMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheClustersMessage` contents to a `SignedRequest`.
struct DescribeCacheClustersMessageSerializer;
impl DescribeCacheClustersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheClustersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_id {
            params.put(&format!("{}{}", prefix, "CacheClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.show_cache_clusters_not_in_replication_groups {
            params.put(
                &format!("{}{}", prefix, "ShowCacheClustersNotInReplicationGroups"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.show_cache_node_info {
            params.put(&format!("{}{}", prefix, "ShowCacheNodeInfo"), &field_value);
        }
    }
}

/// <p>Represents the input of a <code>DescribeCacheEngineVersions</code> operation.</p>
/// see [ElastiCache::describe_cache_engine_versions]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheEngineVersionsMessage {
    /// <p><p>The name of a specific cache parameter group family to return details for.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul></p>
    pub cache_parameter_group_family: Option<String>,
    /// <p>If <code>true</code>, specifies that only the default version of the specified engine or engine and major version combination is to be returned.</p>
    pub default_only: Option<bool>,
    /// <p>The cache engine to return. Valid values: <code>memcached</code> | <code>redis</code> </p>
    pub engine: Option<String>,
    /// <p>The cache engine version to return.</p> <p>Example: <code>1.4.14</code> </p>
    pub engine_version: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeCacheEngineVersionsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheEngineVersionsMessage` contents to a `SignedRequest`.
struct DescribeCacheEngineVersionsMessageSerializer;
impl DescribeCacheEngineVersionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheEngineVersionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_parameter_group_family {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupFamily"),
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
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

/// <p>Represents the input of a <code>DescribeCacheParameterGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_parameter_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheParameterGroupsMessage {
    /// <p>The name of a specific cache parameter group to return details for.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeCacheParameterGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheParameterGroupsMessage` contents to a `SignedRequest`.
struct DescribeCacheParameterGroupsMessageSerializer;
impl DescribeCacheParameterGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheParameterGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupName"),
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

/// <p>Represents the input of a <code>DescribeCacheParameters</code> operation.</p>
/// see [ElastiCache::describe_cache_parameters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheParametersMessage {
    /// <p>The name of a specific cache parameter group to return details for.</p>
    pub cache_parameter_group_name: String,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The parameter types to return.</p> <p>Valid values: <code>user</code> | <code>system</code> | <code>engine-default</code> </p>
    pub source: Option<String>,
}

impl PagedRequest for DescribeCacheParametersMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheParametersMessage` contents to a `SignedRequest`.
struct DescribeCacheParametersMessageSerializer;
impl DescribeCacheParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupName"),
            &obj.cache_parameter_group_name,
        );
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

/// <p>Represents the input of a <code>DescribeCacheSecurityGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_security_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheSecurityGroupsMessage {
    /// <p>The name of the cache security group to return details for.</p>
    pub cache_security_group_name: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeCacheSecurityGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheSecurityGroupsMessage` contents to a `SignedRequest`.
struct DescribeCacheSecurityGroupsMessageSerializer;
impl DescribeCacheSecurityGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheSecurityGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_security_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheSecurityGroupName"),
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

/// <p>Represents the input of a <code>DescribeCacheSubnetGroups</code> operation.</p>
/// see [ElastiCache::describe_cache_subnet_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCacheSubnetGroupsMessage {
    /// <p>The name of the cache subnet group to return details for.</p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeCacheSubnetGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeCacheSubnetGroupsMessage` contents to a `SignedRequest`.
struct DescribeCacheSubnetGroupsMessageSerializer;
impl DescribeCacheSubnetGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheSubnetGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_subnet_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheSubnetGroupName"),
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

/// <p>Represents the input of a <code>DescribeEngineDefaultParameters</code> operation.</p>
/// see [ElastiCache::describe_engine_default_parameters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEngineDefaultParametersMessage {
    /// <p>The name of the cache parameter group family.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p>
    pub cache_parameter_group_family: String,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeEngineDefaultParametersMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
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
            &format!("{}{}", prefix, "CacheParameterGroupFamily"),
            &obj.cache_parameter_group_family,
        );
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
    }
}

/// see [ElastiCache::describe_engine_default_parameters]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeEngineDefaultParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

impl DescribeEngineDefaultParametersResult {
    fn pagination_page_opt(self) -> Option<Vec<Parameter>> {
        Some(self.engine_defaults.as_ref()?.parameters.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeEngineDefaultParametersResult {
    type Item = Parameter;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.engine_defaults.as_ref()?.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Parameter> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct DescribeEngineDefaultParametersResultDeserializer;
impl DescribeEngineDefaultParametersResultDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Represents the input of a <code>DescribeEvents</code> operation.</p>
/// see [ElastiCache::describe_events]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsMessage {
    /// <p>The number of minutes worth of events to retrieve.</p>
    pub duration: Option<i64>,
    /// <p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p> <p> <b>Example:</b> 2017-03-30T07:03:49.555Z</p>
    pub end_time: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The identifier of the event source for which events are returned. If not specified, all sources are included in the response.</p>
    pub source_identifier: Option<String>,
    /// <p>The event source to retrieve events for. If no value is specified, all events are returned.</p>
    pub source_type: Option<String>,
    /// <p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p> <p> <b>Example:</b> 2017-03-30T07:03:49.555Z</p>
    pub start_time: Option<String>,
}

impl PagedRequest for DescribeEventsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
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

/// see [ElastiCache::describe_global_replication_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeGlobalReplicationGroupsMessage {
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxRecords value, a marker is included in the response so that the remaining results can be retrieved. </p>
    pub max_records: Option<i64>,
    /// <p>Returns the list of members that comprise the Global Datastore.</p>
    pub show_member_info: Option<bool>,
}

impl PagedRequest for DescribeGlobalReplicationGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeGlobalReplicationGroupsMessage` contents to a `SignedRequest`.
struct DescribeGlobalReplicationGroupsMessageSerializer;
impl DescribeGlobalReplicationGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeGlobalReplicationGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.global_replication_group_id {
            params.put(
                &format!("{}{}", prefix, "GlobalReplicationGroupId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.show_member_info {
            params.put(&format!("{}{}", prefix, "ShowMemberInfo"), &field_value);
        }
    }
}

/// see [ElastiCache::describe_global_replication_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeGlobalReplicationGroupsResult {
    /// <p>Indicates the slot configuration and global identifier for each slice group.</p>
    pub global_replication_groups: Option<Vec<GlobalReplicationGroup>>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    pub marker: Option<String>,
}

impl DescribeGlobalReplicationGroupsResult {
    fn pagination_page_opt(self) -> Option<Vec<GlobalReplicationGroup>> {
        Some(self.global_replication_groups.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeGlobalReplicationGroupsResult {
    type Item = GlobalReplicationGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<GlobalReplicationGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct DescribeGlobalReplicationGroupsResultDeserializer;
impl DescribeGlobalReplicationGroupsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeGlobalReplicationGroupsResult, XmlParseError> {
        deserialize_elements::<_, DescribeGlobalReplicationGroupsResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroups" => {
                        obj.global_replication_groups.get_or_insert(vec![]).extend(
                            GlobalReplicationGroupListDeserializer::deserialize(
                                "GlobalReplicationGroups",
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
/// <p>Represents the input of a <code>DescribeReplicationGroups</code> operation.</p>
/// see [ElastiCache::describe_replication_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationGroupsMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The identifier for the replication group to be described. This parameter is not case sensitive.</p> <p>If you do not specify this parameter, information about all replication groups is returned.</p>
    pub replication_group_id: Option<String>,
}

impl PagedRequest for DescribeReplicationGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeReplicationGroupsMessage` contents to a `SignedRequest`.
struct DescribeReplicationGroupsMessageSerializer;
impl DescribeReplicationGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReplicationGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
    }
}

/// <p>Represents the input of a <code>DescribeReservedCacheNodes</code> operation.</p>
/// see [ElastiCache::describe_reserved_cache_nodes]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservedCacheNodesMessage {
    /// <p><p>The cache node type filter value. Use this parameter to show only those reservations matching the specified cache node type.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>The duration filter value, specified in years or seconds. Use this parameter to show only reservations for this duration.</p> <p>Valid Values: <code>1 | 3 | 31536000 | 94608000</code> </p>
    pub duration: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The offering type filter value. Use this parameter to show only the available offerings matching the specified offering type.</p> <p>Valid values: <code>"Light Utilization"|"Medium Utilization"|"Heavy Utilization"|"All Upfront"|"Partial Upfront"| "No Upfront"</code> </p>
    pub offering_type: Option<String>,
    /// <p>The product description filter value. Use this parameter to show only those reservations matching the specified product description.</p>
    pub product_description: Option<String>,
    /// <p>The reserved cache node identifier filter value. Use this parameter to show only the reservation that matches the specified reservation ID.</p>
    pub reserved_cache_node_id: Option<String>,
    /// <p>The offering identifier filter value. Use this parameter to show only purchased reservations matching the specified offering identifier.</p>
    pub reserved_cache_nodes_offering_id: Option<String>,
}

impl PagedRequest for DescribeReservedCacheNodesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeReservedCacheNodesMessage` contents to a `SignedRequest`.
struct DescribeReservedCacheNodesMessageSerializer;
impl DescribeReservedCacheNodesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeReservedCacheNodesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.duration {
            params.put(&format!("{}{}", prefix, "Duration"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.offering_type {
            params.put(&format!("{}{}", prefix, "OfferingType"), &field_value);
        }
        if let Some(ref field_value) = obj.product_description {
            params.put(&format!("{}{}", prefix, "ProductDescription"), &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_node_id {
            params.put(
                &format!("{}{}", prefix, "ReservedCacheNodeId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.reserved_cache_nodes_offering_id {
            params.put(
                &format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
                &field_value,
            );
        }
    }
}

/// <p>Represents the input of a <code>DescribeReservedCacheNodesOfferings</code> operation.</p>
/// see [ElastiCache::describe_reserved_cache_nodes_offerings]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReservedCacheNodesOfferingsMessage {
    /// <p><p>The cache node type filter value. Use this parameter to show only the available offerings matching the specified cache node type.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>Duration filter value, specified in years or seconds. Use this parameter to show only reservations for a given duration.</p> <p>Valid Values: <code>1 | 3 | 31536000 | 94608000</code> </p>
    pub duration: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>
    pub max_records: Option<i64>,
    /// <p>The offering type filter value. Use this parameter to show only the available offerings matching the specified offering type.</p> <p>Valid Values: <code>"Light Utilization"|"Medium Utilization"|"Heavy Utilization" |"All Upfront"|"Partial Upfront"| "No Upfront"</code> </p>
    pub offering_type: Option<String>,
    /// <p>The product description filter value. Use this parameter to show only the available offerings matching the specified product description.</p>
    pub product_description: Option<String>,
    /// <p>The offering identifier filter value. Use this parameter to show only the available offering that matches the specified reservation identifier.</p> <p>Example: <code>438012d3-4052-4cc7-b2e3-8d3372e0e706</code> </p>
    pub reserved_cache_nodes_offering_id: Option<String>,
}

impl PagedRequest for DescribeReservedCacheNodesOfferingsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeReservedCacheNodesOfferingsMessage` contents to a `SignedRequest`.
struct DescribeReservedCacheNodesOfferingsMessageSerializer;
impl DescribeReservedCacheNodesOfferingsMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &DescribeReservedCacheNodesOfferingsMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.duration {
            params.put(&format!("{}{}", prefix, "Duration"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.offering_type {
            params.put(&format!("{}{}", prefix, "OfferingType"), &field_value);
        }
        if let Some(ref field_value) = obj.product_description {
            params.put(&format!("{}{}", prefix, "ProductDescription"), &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_nodes_offering_id {
            params.put(
                &format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
                &field_value,
            );
        }
    }
}

/// see [ElastiCache::describe_service_updates]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeServiceUpdatesMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response</p>
    pub max_records: Option<i64>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
    /// <p>The status of the service update</p>
    pub service_update_status: Option<Vec<String>>,
}

impl PagedRequest for DescribeServiceUpdatesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeServiceUpdatesMessage` contents to a `SignedRequest`.
struct DescribeServiceUpdatesMessageSerializer;
impl DescribeServiceUpdatesMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeServiceUpdatesMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.service_update_name {
            params.put(&format!("{}{}", prefix, "ServiceUpdateName"), &field_value);
        }
        if let Some(ref field_value) = obj.service_update_status {
            ServiceUpdateStatusListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ServiceUpdateStatus"),
                field_value,
            );
        }
    }
}

/// <p>Represents the output of a <code>DescribeSnapshots</code> operation.</p>
/// see [ElastiCache::describe_snapshots]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeSnapshotsListMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>
    pub snapshots: Option<Vec<Snapshot>>,
}

impl DescribeSnapshotsListMessage {
    fn pagination_page_opt(self) -> Option<Vec<Snapshot>> {
        Some(self.snapshots.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeSnapshotsListMessage {
    type Item = Snapshot;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Snapshot> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct DescribeSnapshotsListMessageDeserializer;
impl DescribeSnapshotsListMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeSnapshotsListMessage, XmlParseError> {
        deserialize_elements::<_, DescribeSnapshotsListMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "Snapshots" => {
                        obj.snapshots
                            .get_or_insert(vec![])
                            .extend(SnapshotListDeserializer::deserialize("Snapshots", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>Represents the input of a <code>DescribeSnapshotsMessage</code> operation.</p>
/// see [ElastiCache::describe_snapshots]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSnapshotsMessage {
    /// <p>A user-supplied cluster identifier. If this parameter is specified, only snapshots associated with that specific cluster are described.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 50</p> <p>Constraints: minimum 20; maximum 50.</p>
    pub max_records: Option<i64>,
    /// <p>A user-supplied replication group identifier. If this parameter is specified, only snapshots associated with that specific replication group are described.</p>
    pub replication_group_id: Option<String>,
    /// <p>A Boolean value which if true, the node group (shard) configuration is included in the snapshot description.</p>
    pub show_node_group_config: Option<bool>,
    /// <p>A user-supplied name of the snapshot. If this parameter is specified, only this snapshot are described.</p>
    pub snapshot_name: Option<String>,
    /// <p>If set to <code>system</code>, the output shows snapshots that were automatically created by ElastiCache. If set to <code>user</code> the output shows snapshots that were manually created. If omitted, the output shows both automatically and manually created snapshots.</p>
    pub snapshot_source: Option<String>,
}

impl PagedRequest for DescribeSnapshotsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeSnapshotsMessage` contents to a `SignedRequest`.
struct DescribeSnapshotsMessageSerializer;
impl DescribeSnapshotsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeSnapshotsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_id {
            params.put(&format!("{}{}", prefix, "CacheClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.show_node_group_config {
            params.put(
                &format!("{}{}", prefix, "ShowNodeGroupConfig"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_source {
            params.put(&format!("{}{}", prefix, "SnapshotSource"), &field_value);
        }
    }
}

/// see [ElastiCache::describe_update_actions]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUpdateActionsMessage {
    /// <p>The cache cluster IDs</p>
    pub cache_cluster_ids: Option<Vec<String>>,
    /// <p>The Elasticache engine to which the update applies. Either Redis or Memcached </p>
    pub engine: Option<String>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response</p>
    pub max_records: Option<i64>,
    /// <p>The replication group IDs</p>
    pub replication_group_ids: Option<Vec<String>>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
    /// <p>The status of the service update</p>
    pub service_update_status: Option<Vec<String>>,
    /// <p>The range of time specified to search for service updates that are in available status</p>
    pub service_update_time_range: Option<TimeRangeFilter>,
    /// <p>Dictates whether to include node level update status in the response </p>
    pub show_node_level_update_status: Option<bool>,
    /// <p>The status of the update action.</p>
    pub update_action_status: Option<Vec<String>>,
}

impl PagedRequest for DescribeUpdateActionsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeUpdateActionsMessage` contents to a `SignedRequest`.
struct DescribeUpdateActionsMessageSerializer;
impl DescribeUpdateActionsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeUpdateActionsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_ids {
            CacheClusterIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheClusterIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_ids {
            ReplicationGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReplicationGroupIds"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.service_update_name {
            params.put(&format!("{}{}", prefix, "ServiceUpdateName"), &field_value);
        }
        if let Some(ref field_value) = obj.service_update_status {
            ServiceUpdateStatusListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ServiceUpdateStatus"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.service_update_time_range {
            TimeRangeFilterSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ServiceUpdateTimeRange"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.show_node_level_update_status {
            params.put(
                &format!("{}{}", prefix, "ShowNodeLevelUpdateStatus"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.update_action_status {
            UpdateActionStatusListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UpdateActionStatus"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::describe_user_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUserGroupsMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxRecords value, a marker is included in the response so that the remaining results can be retrieved. </p>
    pub max_records: Option<i64>,
    /// <p>The ID of the user group.</p>
    pub user_group_id: Option<String>,
}

impl PagedRequest for DescribeUserGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeUserGroupsMessage` contents to a `SignedRequest`.
struct DescribeUserGroupsMessageSerializer;
impl DescribeUserGroupsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeUserGroupsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.user_group_id {
            params.put(&format!("{}{}", prefix, "UserGroupId"), &field_value);
        }
    }
}

/// see [ElastiCache::describe_user_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeUserGroupsResult {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    pub marker: Option<String>,
    /// <p>Returns a list of user groups.</p>
    pub user_groups: Option<Vec<UserGroup>>,
}

impl DescribeUserGroupsResult {
    fn pagination_page_opt(self) -> Option<Vec<UserGroup>> {
        Some(self.user_groups.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeUserGroupsResult {
    type Item = UserGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<UserGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct DescribeUserGroupsResultDeserializer;
impl DescribeUserGroupsResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeUserGroupsResult, XmlParseError> {
        deserialize_elements::<_, DescribeUserGroupsResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "UserGroups" => {
                        obj.user_groups
                            .get_or_insert(vec![])
                            .extend(UserGroupListDeserializer::deserialize("UserGroups", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// see [ElastiCache::describe_users]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeUsersMessage {
    /// <p>The Redis engine. </p>
    pub engine: Option<String>,
    /// <p>Filter to determine the list of User IDs to return.</p>
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified MaxRecords value, a marker is included in the response so that the remaining results can be retrieved. </p>
    pub max_records: Option<i64>,
    /// <p>The ID of the user.</p>
    pub user_id: Option<String>,
}

impl PagedRequest for DescribeUsersMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// Serialize `DescribeUsersMessage` contents to a `SignedRequest`.
struct DescribeUsersMessageSerializer;
impl DescribeUsersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeUsersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.filters {
            FilterListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Filters"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"), &field_value);
        }
        if let Some(ref field_value) = obj.user_id {
            params.put(&format!("{}{}", prefix, "UserId"), &field_value);
        }
    }
}

/// see [ElastiCache::describe_users]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DescribeUsersResult {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by MaxRecords. &gt;</p>
    pub marker: Option<String>,
    /// <p>A list of users.</p>
    pub users: Option<Vec<User>>,
}

impl DescribeUsersResult {
    fn pagination_page_opt(self) -> Option<Vec<User>> {
        Some(self.users.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeUsersResult {
    type Item = User;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<User> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct DescribeUsersResultDeserializer;
impl DescribeUsersResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DescribeUsersResult, XmlParseError> {
        deserialize_elements::<_, DescribeUsersResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                "Users" => {
                    obj.users
                        .get_or_insert(vec![])
                        .extend(UserListDeserializer::deserialize("Users", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::disassociate_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateGlobalReplicationGroupMessage {
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
    /// <p>The name of the secondary cluster you wish to remove from the Global Datastore</p>
    pub replication_group_id: String,
    /// <p>The AWS region of secondary cluster you wish to remove from the Global Datastore</p>
    pub replication_group_region: String,
}

/// Serialize `DisassociateGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct DisassociateGlobalReplicationGroupMessageSerializer;
impl DisassociateGlobalReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DisassociateGlobalReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupRegion"),
            &obj.replication_group_region,
        );
    }
}

/// see [ElastiCache::disassociate_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct DisassociateGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct DisassociateGlobalReplicationGroupResultDeserializer;
impl DisassociateGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<DisassociateGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, DisassociateGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
#[allow(dead_code)]
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<f64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(f64::from_str(&s).unwrap()))
    }
}
/// <p>Provides ownership and status information for an Amazon EC2 security group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EC2SecurityGroup {
    /// <p>The name of the Amazon EC2 security group.</p>
    pub ec2_security_group_name: Option<String>,
    /// <p>The AWS account ID of the Amazon EC2 security group owner.</p>
    pub ec2_security_group_owner_id: Option<String>,
    /// <p>The status of the Amazon EC2 security group.</p>
    pub status: Option<String>,
}

#[allow(dead_code)]
struct EC2SecurityGroupDeserializer;
impl EC2SecurityGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EC2SecurityGroup, XmlParseError> {
        deserialize_elements::<_, EC2SecurityGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "EC2SecurityGroupName" => {
                    obj.ec2_security_group_name = Some(StringDeserializer::deserialize(
                        "EC2SecurityGroupName",
                        stack,
                    )?);
                }
                "EC2SecurityGroupOwnerId" => {
                    obj.ec2_security_group_owner_id = Some(StringDeserializer::deserialize(
                        "EC2SecurityGroupOwnerId",
                        stack,
                    )?);
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
#[allow(dead_code)]
struct EC2SecurityGroupListDeserializer;
impl EC2SecurityGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<EC2SecurityGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "EC2SecurityGroup" {
                obj.push(EC2SecurityGroupDeserializer::deserialize(
                    "EC2SecurityGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the information required for client programs to connect to a cache node.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Endpoint {
    /// <p>The DNS hostname of the cache node.</p>
    pub address: Option<String>,
    /// <p>The port number that the cache engine is listening on.</p>
    pub port: Option<i64>,
}

#[allow(dead_code)]
struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Endpoint, XmlParseError> {
        deserialize_elements::<_, Endpoint, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Address" => {
                    obj.address = Some(StringDeserializer::deserialize("Address", stack)?);
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
/// <p>Represents the output of a <code>DescribeEngineDefaultParameters</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EngineDefaults {
    /// <p>A list of parameters specific to a particular cache node type. Each element in the list contains detailed information about one parameter.</p>
    pub cache_node_type_specific_parameters: Option<Vec<CacheNodeTypeSpecificParameter>>,
    /// <p>Specifies the name of the cache parameter group family to which the engine default parameters apply.</p> <p>Valid values are: <code>memcached1.4</code> | <code>memcached1.5</code> | <code>memcached1.6</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> | <code>redis4.0</code> | <code>redis5.0</code> | <code>redis6.x</code> | </p>
    pub cache_parameter_group_family: Option<String>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
    /// <p>Contains a list of engine default parameters.</p>
    pub parameters: Option<Vec<Parameter>>,
}

#[allow(dead_code)]
struct EngineDefaultsDeserializer;
impl EngineDefaultsDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<EngineDefaults, XmlParseError> {
        deserialize_elements::<_, EngineDefaults, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheNodeTypeSpecificParameters" => {
                    obj.cache_node_type_specific_parameters
                        .get_or_insert(vec![])
                        .extend(
                            CacheNodeTypeSpecificParametersListDeserializer::deserialize(
                                "CacheNodeTypeSpecificParameters",
                                stack,
                            )?,
                        );
                }
                "CacheParameterGroupFamily" => {
                    obj.cache_parameter_group_family = Some(StringDeserializer::deserialize(
                        "CacheParameterGroupFamily",
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
#[allow(dead_code)]
struct EngineTypeDeserializer;
impl EngineTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents a single occurrence of something interesting within the system. Some examples of events are creating a cluster, adding or removing a cache node, or rebooting a node.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Event {
    /// <p>The date and time when the event occurred.</p>
    pub date: Option<String>,
    /// <p>The text of the event.</p>
    pub message: Option<String>,
    /// <p>The identifier for the source of the event. For example, if the event occurred at the cluster level, the identifier would be the name of the cluster.</p>
    pub source_identifier: Option<String>,
    /// <p>Specifies the origin of this event - a cluster, a parameter group, a security group, etc.</p>
    pub source_type: Option<String>,
}

#[allow(dead_code)]
struct EventDeserializer;
impl EventDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<Event, XmlParseError> {
        deserialize_elements::<_, Event, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Date" => {
                    obj.date = Some(TStampDeserializer::deserialize("Date", stack)?);
                }
                "Message" => {
                    obj.message = Some(StringDeserializer::deserialize("Message", stack)?);
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
#[allow(dead_code)]
struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Represents the output of a <code>DescribeEvents</code> operation.</p>
/// see [ElastiCache::describe_events]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct EventsMessage {
    /// <p>A list of events. Each element in the list contains detailed information about one event.</p>
    pub events: Option<Vec<Event>>,
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
}

impl EventsMessage {
    fn pagination_page_opt(self) -> Option<Vec<Event>> {
        Some(self.events.as_ref()?.clone())
    }
}

impl PagedOutput for EventsMessage {
    type Item = Event;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Event> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct EventsMessageDeserializer;
impl EventsMessageDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// see [ElastiCache::failover_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FailoverGlobalReplicationGroupMessage {
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
    /// <p>The AWS region of the primary cluster of the Global Datastore</p>
    pub primary_region: String,
    /// <p>The name of the primary replication group</p>
    pub primary_replication_group_id: String,
}

/// Serialize `FailoverGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct FailoverGlobalReplicationGroupMessageSerializer;
impl FailoverGlobalReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &FailoverGlobalReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "PrimaryRegion"),
            &obj.primary_region,
        );
        params.put(
            &format!("{}{}", prefix, "PrimaryReplicationGroupId"),
            &obj.primary_replication_group_id,
        );
    }
}

/// see [ElastiCache::failover_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct FailoverGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct FailoverGlobalReplicationGroupResultDeserializer;
impl FailoverGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<FailoverGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, FailoverGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// <p>Used to streamline results of a search based on the property being filtered.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The property being filtered. For example, UserId.</p>
    pub name: String,
    /// <p>The property values to filter on. For example, "user-123".</p>
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
            &format!("{}{}", prefix, "Values"),
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

/// <p>Indicates the slot configuration and global identifier for a slice group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GlobalNodeGroup {
    /// <p>The name of the global node group</p>
    pub global_node_group_id: Option<String>,
    /// <p>The keyspace for this node group</p>
    pub slots: Option<String>,
}

#[allow(dead_code)]
struct GlobalNodeGroupDeserializer;
impl GlobalNodeGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GlobalNodeGroup, XmlParseError> {
        deserialize_elements::<_, GlobalNodeGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "GlobalNodeGroupId" => {
                    obj.global_node_group_id =
                        Some(StringDeserializer::deserialize("GlobalNodeGroupId", stack)?);
                }
                "Slots" => {
                    obj.slots = Some(StringDeserializer::deserialize("Slots", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `GlobalNodeGroupIdList` contents to a `SignedRequest`.
struct GlobalNodeGroupIdListSerializer;
impl GlobalNodeGroupIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct GlobalNodeGroupListDeserializer;
impl GlobalNodeGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<GlobalNodeGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "GlobalNodeGroup" {
                obj.push(GlobalNodeGroupDeserializer::deserialize(
                    "GlobalNodeGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p><p>Consists of a primary cluster that accepts writes and an associated secondary cluster that resides in a different AWS region. The secondary cluster accepts only reads. The primary cluster automatically replicates updates to the secondary cluster.</p> <ul> <li> <p>The <b>GlobalReplicationGroupIdSuffix</b> represents the name of the Global Datastore, which is what you use to associate a secondary cluster.</p> </li> </ul></p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GlobalReplicationGroup {
    /// <p>The ARN (Amazon Resource Name) of the global replication group.</p>
    pub arn: Option<String>,
    /// <p>A flag that enables encryption at rest when set to <code>true</code>.</p> <p>You cannot modify the value of <code>AtRestEncryptionEnabled</code> after the replication group is created. To enable encryption at rest on a replication group you must set <code>AtRestEncryptionEnabled</code> to <code>true</code> when you create the replication group. </p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p>
    pub at_rest_encryption_enabled: Option<bool>,
    /// <p>A flag that enables using an <code>AuthToken</code> (password) when issuing Redis commands.</p> <p>Default: <code>false</code> </p>
    pub auth_token_enabled: Option<bool>,
    /// <p>The cache node type of the Global Datastore</p>
    pub cache_node_type: Option<String>,
    /// <p>A flag that indicates whether the Global Datastore is cluster enabled.</p>
    pub cluster_enabled: Option<bool>,
    /// <p>The Elasticache engine. For Redis only.</p>
    pub engine: Option<String>,
    /// <p>The Elasticache Redis engine version.</p>
    pub engine_version: Option<String>,
    /// <p>Indicates the slot configuration and global identifier for each slice group.</p>
    pub global_node_groups: Option<Vec<GlobalNodeGroup>>,
    /// <p>The optional description of the Global Datastore</p>
    pub global_replication_group_description: Option<String>,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: Option<String>,
    /// <p>The replication groups that comprise the Global Datastore.</p>
    pub members: Option<Vec<GlobalReplicationGroupMember>>,
    /// <p>The status of the Global Datastore</p>
    pub status: Option<String>,
    /// <p>A flag that enables in-transit encryption when set to true. You cannot modify the value of <code>TransitEncryptionEnabled</code> after the cluster is created. To enable in-transit encryption on a cluster you must set <code>TransitEncryptionEnabled</code> to true when you create a cluster. </p>
    pub transit_encryption_enabled: Option<bool>,
}

#[allow(dead_code)]
struct GlobalReplicationGroupDeserializer;
impl GlobalReplicationGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GlobalReplicationGroup, XmlParseError> {
        deserialize_elements::<_, GlobalReplicationGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "AtRestEncryptionEnabled" => {
                    obj.at_rest_encryption_enabled = Some(
                        BooleanOptionalDeserializer::deserialize("AtRestEncryptionEnabled", stack)?,
                    );
                }
                "AuthTokenEnabled" => {
                    obj.auth_token_enabled = Some(BooleanOptionalDeserializer::deserialize(
                        "AuthTokenEnabled",
                        stack,
                    )?);
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "ClusterEnabled" => {
                    obj.cluster_enabled = Some(BooleanOptionalDeserializer::deserialize(
                        "ClusterEnabled",
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
                "GlobalNodeGroups" => {
                    obj.global_node_groups.get_or_insert(vec![]).extend(
                        GlobalNodeGroupListDeserializer::deserialize("GlobalNodeGroups", stack)?,
                    );
                }
                "GlobalReplicationGroupDescription" => {
                    obj.global_replication_group_description =
                        Some(StringDeserializer::deserialize(
                            "GlobalReplicationGroupDescription",
                            stack,
                        )?);
                }
                "GlobalReplicationGroupId" => {
                    obj.global_replication_group_id = Some(StringDeserializer::deserialize(
                        "GlobalReplicationGroupId",
                        stack,
                    )?);
                }
                "Members" => {
                    obj.members.get_or_insert(vec![]).extend(
                        GlobalReplicationGroupMemberListDeserializer::deserialize(
                            "Members", stack,
                        )?,
                    );
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "TransitEncryptionEnabled" => {
                    obj.transit_encryption_enabled =
                        Some(BooleanOptionalDeserializer::deserialize(
                            "TransitEncryptionEnabled",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>The name of the Global Datastore and role of this replication group in the Global Datastore.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GlobalReplicationGroupInfo {
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: Option<String>,
    /// <p>The role of the replication group in a Global Datastore. Can be primary or secondary.</p>
    pub global_replication_group_member_role: Option<String>,
}

#[allow(dead_code)]
struct GlobalReplicationGroupInfoDeserializer;
impl GlobalReplicationGroupInfoDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GlobalReplicationGroupInfo, XmlParseError> {
        deserialize_elements::<_, GlobalReplicationGroupInfo, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroupId" => {
                        obj.global_replication_group_id = Some(StringDeserializer::deserialize(
                            "GlobalReplicationGroupId",
                            stack,
                        )?);
                    }
                    "GlobalReplicationGroupMemberRole" => {
                        obj.global_replication_group_member_role =
                            Some(StringDeserializer::deserialize(
                                "GlobalReplicationGroupMemberRole",
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
#[allow(dead_code)]
struct GlobalReplicationGroupListDeserializer;
impl GlobalReplicationGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<GlobalReplicationGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "GlobalReplicationGroup" {
                obj.push(GlobalReplicationGroupDeserializer::deserialize(
                    "GlobalReplicationGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A member of a Global Datastore. It contains the Replication Group Id, the AWS region and the role of the replication group. </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct GlobalReplicationGroupMember {
    /// <p>Indicates whether automatic failover is enabled for the replication group.</p>
    pub automatic_failover: Option<String>,
    /// <p>The replication group id of the Global Datastore member.</p>
    pub replication_group_id: Option<String>,
    /// <p>The AWS region of the Global Datastore member.</p>
    pub replication_group_region: Option<String>,
    /// <p>Indicates the role of the replication group, primary or secondary.</p>
    pub role: Option<String>,
    /// <p>The status of the membership of the replication group.</p>
    pub status: Option<String>,
}

#[allow(dead_code)]
struct GlobalReplicationGroupMemberDeserializer;
impl GlobalReplicationGroupMemberDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<GlobalReplicationGroupMember, XmlParseError> {
        deserialize_elements::<_, GlobalReplicationGroupMember, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AutomaticFailover" => {
                        obj.automatic_failover =
                            Some(AutomaticFailoverStatusDeserializer::deserialize(
                                "AutomaticFailover",
                                stack,
                            )?);
                    }
                    "ReplicationGroupId" => {
                        obj.replication_group_id = Some(StringDeserializer::deserialize(
                            "ReplicationGroupId",
                            stack,
                        )?);
                    }
                    "ReplicationGroupRegion" => {
                        obj.replication_group_region = Some(StringDeserializer::deserialize(
                            "ReplicationGroupRegion",
                            stack,
                        )?);
                    }
                    "Role" => {
                        obj.role = Some(StringDeserializer::deserialize("Role", stack)?);
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
#[allow(dead_code)]
struct GlobalReplicationGroupMemberListDeserializer;
impl GlobalReplicationGroupMemberListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<GlobalReplicationGroupMember>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "GlobalReplicationGroupMember" {
                obj.push(GlobalReplicationGroupMemberDeserializer::deserialize(
                    "GlobalReplicationGroupMember",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::increase_node_groups_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupMessage {
    /// <p>Indicates that the process begins immediately. At present, the only permitted value for this parameter is true.</p>
    pub apply_immediately: bool,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
    /// <p>The number of node groups you wish to add</p>
    pub node_group_count: i64,
    /// <p>Describes the replication group IDs, the AWS regions where they are stored and the shard configuration for each that comprise the Global Datastore</p>
    pub regional_configurations: Option<Vec<RegionalConfiguration>>,
}

/// Serialize `IncreaseNodeGroupsInGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct IncreaseNodeGroupsInGlobalReplicationGroupMessageSerializer;
impl IncreaseNodeGroupsInGlobalReplicationGroupMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &IncreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "NodeGroupCount"),
            &obj.node_group_count,
        );
        if let Some(ref field_value) = obj.regional_configurations {
            RegionalConfigurationListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "RegionalConfiguration"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::increase_node_groups_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IncreaseNodeGroupsInGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct IncreaseNodeGroupsInGlobalReplicationGroupResultDeserializer;
impl IncreaseNodeGroupsInGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IncreaseNodeGroupsInGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, IncreaseNodeGroupsInGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// see [ElastiCache::increase_replica_count]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct IncreaseReplicaCountMessage {
    /// <p>If <code>True</code>, the number of replica nodes is increased immediately. <code>ApplyImmediately=False</code> is not currently supported.</p>
    pub apply_immediately: bool,
    /// <p>The number of read replica nodes you want at the completion of this operation. For Redis (cluster mode disabled) replication groups, this is the number of replica nodes in the replication group. For Redis (cluster mode enabled) replication groups, this is the number of replica nodes in each of the replication group's node groups.</p>
    pub new_replica_count: Option<i64>,
    /// <p>A list of <code>ConfigureShard</code> objects that can be used to configure each shard in a Redis (cluster mode enabled) replication group. The <code>ConfigureShard</code> has three members: <code>NewReplicaCount</code>, <code>NodeGroupId</code>, and <code>PreferredAvailabilityZones</code>.</p>
    pub replica_configuration: Option<Vec<ConfigureShard>>,
    /// <p>The id of the replication group to which you want to add replica nodes.</p>
    pub replication_group_id: String,
}

/// Serialize `IncreaseReplicaCountMessage` contents to a `SignedRequest`.
struct IncreaseReplicaCountMessageSerializer;
impl IncreaseReplicaCountMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &IncreaseReplicaCountMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        if let Some(ref field_value) = obj.new_replica_count {
            params.put(&format!("{}{}", prefix, "NewReplicaCount"), &field_value);
        }
        if let Some(ref field_value) = obj.replica_configuration {
            ReplicaConfigurationListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ConfigureShard"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
    }
}

/// see [ElastiCache::increase_replica_count]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct IncreaseReplicaCountResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct IncreaseReplicaCountResultDeserializer;
impl IncreaseReplicaCountResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<IncreaseReplicaCountResult, XmlParseError> {
        deserialize_elements::<_, IncreaseReplicaCountResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
#[allow(dead_code)]
struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
    }
}
#[allow(dead_code)]
struct IntegerOptionalDeserializer;
impl IntegerOptionalDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<i64, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, |s| Ok(i64::from_str(&s).unwrap()))
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

/// <p>The input parameters for the <code>ListAllowedNodeTypeModifications</code> operation.</p>
/// see [ElastiCache::list_allowed_node_type_modifications]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListAllowedNodeTypeModificationsMessage {
    /// <p><p>The name of the cluster you want to scale up to a larger node instanced type. ElastiCache uses the cluster id to identify the current node type of this cluster and from that to create a list of node types you can scale up to.</p> <important> <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p> </important></p>
    pub cache_cluster_id: Option<String>,
    /// <p><p>The name of the replication group want to scale up to a larger node type. ElastiCache uses the replication group id to identify the current node type being used by this replication group, and from that to create a list of node types you can scale up to.</p> <important> <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p> </important></p>
    pub replication_group_id: Option<String>,
}

/// Serialize `ListAllowedNodeTypeModificationsMessage` contents to a `SignedRequest`.
struct ListAllowedNodeTypeModificationsMessageSerializer;
impl ListAllowedNodeTypeModificationsMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ListAllowedNodeTypeModificationsMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_cluster_id {
            params.put(&format!("{}{}", prefix, "CacheClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
    }
}

/// <p>The input parameters for the <code>ListTagsForResource</code> operation.</p>
/// see [ElastiCache::list_tags_for_resource]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the resource for which you want the list of tags, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
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

        params.put(&format!("{}{}", prefix, "ResourceName"), &obj.resource_name);
    }
}

/// <p>Represents the input of a <code>ModifyCacheCluster</code> operation.</p>
/// see [ElastiCache::modify_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyCacheClusterMessage {
    /// <p><p>Specifies whether the new nodes in this Memcached cluster are all created in a single Availability Zone or created across multiple Availability Zones.</p> <p>Valid values: <code>single-az</code> | <code>cross-az</code>.</p> <p>This option is only supported for Memcached clusters.</p> <note> <p>You cannot specify <code>single-az</code> if the Memcached cluster already has cache nodes in different Availability Zones. If <code>cross-az</code> is specified, existing Memcached nodes remain in their current Availability Zone.</p> <p>Only newly created nodes are located in different Availability Zones. </p> </note></p>
    pub az_mode: Option<String>,
    /// <p>If <code>true</code>, this parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the cluster.</p> <p>If <code>false</code>, changes to the cluster are applied on the next maintenance reboot, or the next failure reboot, whichever occurs first.</p> <important> <p>If you perform a <code>ModifyCacheCluster</code> before a pending modification is applied, the pending modification is replaced by the newer modification.</p> </important> <p>Valid values: <code>true</code> | <code>false</code> </p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p>Reserved parameter. The password used to access a password protected server. This parameter must be specified with the <code>auth-token-update</code> parameter. Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length</p> </li> <li> <p>Cannot contain any of the following characters: '/', '"', or '@', '%'</p> </li> </ul> <p> For more information, see AUTH password at <a href="http://redis.io/commands/AUTH">AUTH</a>.</p>
    pub auth_token: Option<String>,
    /// <p>Specifies the strategy to use to update the AUTH token. This parameter must be specified with the <code>auth-token</code> parameter. Possible values:</p> <ul> <li> <p>Rotate</p> </li> <li> <p>Set</p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/auth.html">Authenticating Users with Redis AUTH</a> </p>
    pub auth_token_update_strategy: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The cluster identifier. This value is stored as a lowercase string.</p>
    pub cache_cluster_id: String,
    /// <p>A list of cache node IDs to be removed. A node ID is a numeric identifier (0001, 0002, etc.). This parameter is only valid when <code>NumCacheNodes</code> is less than the existing number of cache nodes. The number of cache node IDs supplied in this parameter must match the difference between the existing number of cache nodes in the cluster or pending cache nodes, whichever is greater, and the value of <code>NumCacheNodes</code> in the request.</p> <p>For example: If you have 3 active cache nodes, 7 pending cache nodes, and the number of cache nodes in this <code>ModifyCacheCluster</code> call is 5, you must list 2 (7 - 5) cache node IDs to remove.</p>
    pub cache_node_ids_to_remove: Option<Vec<String>>,
    /// <p>A valid cache node type that you want to scale this cluster up to.</p>
    pub cache_node_type: Option<String>,
    /// <p>The name of the cache parameter group to apply to this cluster. This change is asynchronously applied as soon as possible for parameters when the <code>ApplyImmediately</code> parameter is specified as <code>true</code> for this request.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>A list of cache security group names to authorize on this cluster. This change is asynchronously applied as soon as possible.</p> <p>You can use this parameter only with clusters that are created outside of an Amazon Virtual Private Cloud (Amazon VPC).</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Must not be "Default".</p>
    pub cache_security_group_names: Option<Vec<String>>,
    /// <p>The upgraded version of the cache engine to be run on the cache nodes.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SelectEngine.html#VersionManagement">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cluster and create it anew with the earlier engine version. </p>
    pub engine_version: Option<String>,
    /// <p><p>The list of Availability Zones where the new Memcached cache nodes are created.</p> <p>This parameter is only valid when <code>NumCacheNodes</code> in the request is greater than the sum of the number of active cache nodes and the number of cache nodes pending creation (which may be zero). The number of Availability Zones supplied in this list must match the cache nodes being added in this request.</p> <p>This option is only supported on Memcached clusters.</p> <p>Scenarios:</p> <ul> <li> <p> <b>Scenario 1:</b> You have 3 active nodes and wish to add 2 nodes. Specify <code>NumCacheNodes=5</code> (3 + 2) and optionally specify two Availability Zones for the two new nodes.</p> </li> <li> <p> <b>Scenario 2:</b> You have 3 active nodes and 2 nodes pending creation (from the scenario 1 call) and want to add 1 more node. Specify <code>NumCacheNodes=6</code> ((3 + 2) + 1) and optionally specify an Availability Zone for the new node.</p> </li> <li> <p> <b>Scenario 3:</b> You want to cancel all pending operations. Specify <code>NumCacheNodes=3</code> to cancel all pending operations.</p> </li> </ul> <p>The Availability Zone placement of nodes pending creation cannot be modified. If you wish to cancel any nodes pending creation, add 0 nodes by setting <code>NumCacheNodes</code> to the number of current nodes.</p> <p>If <code>cross-az</code> is specified, existing Memcached nodes remain in their current Availability Zone. Only newly created nodes can be located in different Availability Zones. For guidance on how to move existing Memcached nodes to different Availability Zones, see the <b>Availability Zone Considerations</b> section of <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/mem-ug/CacheNodes.SupportedTypes.html">Cache Node Considerations for Memcached</a>.</p> <p> <b>Impact of new add/remove requests upon pending requests</b> </p> <ul> <li> <p>Scenario-1</p> <ul> <li> <p>Pending Action: Delete</p> </li> <li> <p>New Request: Delete</p> </li> <li> <p>Result: The new delete, pending or immediate, replaces the pending delete.</p> </li> </ul> </li> <li> <p>Scenario-2</p> <ul> <li> <p>Pending Action: Delete</p> </li> <li> <p>New Request: Create</p> </li> <li> <p>Result: The new create, pending or immediate, replaces the pending delete.</p> </li> </ul> </li> <li> <p>Scenario-3</p> <ul> <li> <p>Pending Action: Create</p> </li> <li> <p>New Request: Delete</p> </li> <li> <p>Result: The new delete, pending or immediate, replaces the pending create.</p> </li> </ul> </li> <li> <p>Scenario-4</p> <ul> <li> <p>Pending Action: Create</p> </li> <li> <p>New Request: Create</p> </li> <li> <p>Result: The new create is added to the pending create.</p> <important> <p> <b>Important:</b> If the new create request is <b>Apply Immediately - Yes</b>, all creates are performed immediately. If the new create request is <b>Apply Immediately - No</b>, all creates are pending.</p> </important> </li> </ul> </li> </ul></p>
    pub new_availability_zones: Option<Vec<String>>,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be same as the cluster owner.</p> </note></p>
    pub notification_topic_arn: Option<String>,
    /// <p>The status of the Amazon SNS notification topic. Notifications are sent only if the status is <code>active</code>.</p> <p>Valid values: <code>active</code> | <code>inactive</code> </p>
    pub notification_topic_status: Option<String>,
    /// <p><p>The number of cache nodes that the cluster should have. If the value for <code>NumCacheNodes</code> is greater than the sum of the number of current cache nodes and the number of cache nodes pending creation (which may be zero), more nodes are added. If the value is less than the number of existing cache nodes, nodes are removed. If the value is equal to the number of current cache nodes, any pending add or remove requests are canceled.</p> <p>If you are removing cache nodes, you must use the <code>CacheNodeIdsToRemove</code> parameter to provide the IDs of the specific cache nodes to remove.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p> <note> <p>Adding or removing Memcached cache nodes can be applied immediately or as a pending operation (see <code>ApplyImmediately</code>).</p> <p>A pending operation to modify the number of cache nodes in a cluster during its maintenance window, whether by adding or removing nodes in accordance with the scale out architecture, is not queued. The customer&#39;s latest request to add or remove nodes to the cluster overrides any previous pending operations to modify the number of cache nodes in the cluster. For example, a request to remove 2 nodes would override a previous pending operation to remove 3 nodes. Similarly, a request to add 2 nodes would override a previous pending operation to remove 3 nodes and vice versa. As Memcached cache nodes may now be provisioned in different Availability Zones with flexible cache node placement, a request to add nodes does not automatically override a previous pending operation to add nodes. The customer can modify the previous pending operation to add more nodes or explicitly cancel the pending request and retry the new request. To cancel pending operations to modify the number of cache nodes in a cluster, use the <code>ModifyCacheCluster</code> request and set <code>NumCacheNodes</code> equal to the number of cache nodes currently in the cluster.</p> </note></p>
    pub num_cache_nodes: Option<i64>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>Specifies the VPC Security Groups associated with the cluster.</p> <p>This parameter can be used only with clusters that are created in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    pub security_group_ids: Option<Vec<String>>,
    /// <p><p>The number of days for which ElastiCache retains automatic cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <note> <p>If the value of <code>SnapshotRetentionLimit</code> is set to zero (0), backups are turned off.</p> </note></p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your cluster. </p>
    pub snapshot_window: Option<String>,
}

/// Serialize `ModifyCacheClusterMessage` contents to a `SignedRequest`.
struct ModifyCacheClusterMessageSerializer;
impl ModifyCacheClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyCacheClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.az_mode {
            params.put(&format!("{}{}", prefix, "AZMode"), &field_value);
        }
        if let Some(ref field_value) = obj.apply_immediately {
            params.put(&format!("{}{}", prefix, "ApplyImmediately"), &field_value);
        }
        if let Some(ref field_value) = obj.auth_token {
            params.put(&format!("{}{}", prefix, "AuthToken"), &field_value);
        }
        if let Some(ref field_value) = obj.auth_token_update_strategy {
            params.put(
                &format!("{}{}", prefix, "AuthTokenUpdateStrategy"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "CacheClusterId"),
            &obj.cache_cluster_id,
        );
        if let Some(ref field_value) = obj.cache_node_ids_to_remove {
            CacheNodeIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheNodeId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.new_availability_zones {
            PreferredAvailabilityZoneListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "PreferredAvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicArn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_topic_status {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicStatus"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.num_cache_nodes {
            params.put(&format!("{}{}", prefix, "NumCacheNodes"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroupId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(
                &format!("{}{}", prefix, "SnapshotRetentionLimit"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
    }
}

/// see [ElastiCache::modify_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

#[allow(dead_code)]
struct ModifyCacheClusterResultDeserializer;
impl ModifyCacheClusterResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyCacheClusterResult, XmlParseError> {
        deserialize_elements::<_, ModifyCacheClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheCluster" => {
                        obj.cache_cluster = Some(CacheClusterDeserializer::deserialize(
                            "CacheCluster",
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
/// <p>Represents the input of a <code>ModifyCacheParameterGroup</code> operation.</p>
/// see [ElastiCache::modify_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyCacheParameterGroupMessage {
    /// <p>The name of the cache parameter group to modify.</p>
    pub cache_parameter_group_name: String,
    /// <p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>
    pub parameter_name_values: Vec<ParameterNameValue>,
}

/// Serialize `ModifyCacheParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyCacheParameterGroupMessageSerializer;
impl ModifyCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupName"),
            &obj.cache_parameter_group_name,
        );
        ParameterNameValueListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ParameterNameValue"),
            &obj.parameter_name_values,
        );
    }
}

/// <p>Represents the input of a <code>ModifyCacheSubnetGroup</code> operation.</p>
/// see [ElastiCache::modify_cache_subnet_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyCacheSubnetGroupMessage {
    /// <p>A description of the cache subnet group.</p>
    pub cache_subnet_group_description: Option<String>,
    /// <p>The name for the cache subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p> <p>Example: <code>mysubnetgroup</code> </p>
    pub cache_subnet_group_name: String,
    /// <p>The EC2 subnet IDs for the cache subnet group.</p>
    pub subnet_ids: Option<Vec<String>>,
}

/// Serialize `ModifyCacheSubnetGroupMessage` contents to a `SignedRequest`.
struct ModifyCacheSubnetGroupMessageSerializer;
impl ModifyCacheSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyCacheSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_subnet_group_description {
            params.put(
                &format!("{}{}", prefix, "CacheSubnetGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "CacheSubnetGroupName"),
            &obj.cache_subnet_group_name,
        );
        if let Some(ref field_value) = obj.subnet_ids {
            SubnetIdentifierListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SubnetIdentifier"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::modify_cache_subnet_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyCacheSubnetGroupResult {
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

#[allow(dead_code)]
struct ModifyCacheSubnetGroupResultDeserializer;
impl ModifyCacheSubnetGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyCacheSubnetGroupResult, XmlParseError> {
        deserialize_elements::<_, ModifyCacheSubnetGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSubnetGroup" => {
                        obj.cache_subnet_group = Some(CacheSubnetGroupDeserializer::deserialize(
                            "CacheSubnetGroup",
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
/// see [ElastiCache::modify_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyGlobalReplicationGroupMessage {
    /// <p>This parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible. Modifications to Global Replication Groups cannot be requested to be applied in PreferredMaintenceWindow. </p>
    pub apply_immediately: bool,
    /// <p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure. </p>
    pub automatic_failover_enabled: Option<bool>,
    /// <p>A valid cache node type that you want to scale this Global Datastore to.</p>
    pub cache_node_type: Option<String>,
    /// <p>The upgraded version of the cache engine to be run on the clusters in the Global Datastore. </p>
    pub engine_version: Option<String>,
    /// <p>A description of the Global Datastore</p>
    pub global_replication_group_description: Option<String>,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
}

/// Serialize `ModifyGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct ModifyGlobalReplicationGroupMessageSerializer;
impl ModifyGlobalReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyGlobalReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        if let Some(ref field_value) = obj.automatic_failover_enabled {
            params.put(
                &format!("{}{}", prefix, "AutomaticFailoverEnabled"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.global_replication_group_description {
            params.put(
                &format!("{}{}", prefix, "GlobalReplicationGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
    }
}

/// see [ElastiCache::modify_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct ModifyGlobalReplicationGroupResultDeserializer;
impl ModifyGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, ModifyGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// <p>Represents the input of a <code>ModifyReplicationGroups</code> operation.</p>
/// see [ElastiCache::modify_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReplicationGroupMessage {
    /// <p>If <code>true</code>, this parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the replication group.</p> <p>If <code>false</code>, changes to the nodes in the replication group are applied on the next maintenance reboot, or the next failure reboot, whichever occurs first.</p> <p>Valid values: <code>true</code> | <code>false</code> </p> <p>Default: <code>false</code> </p>
    pub apply_immediately: Option<bool>,
    /// <p>Reserved parameter. The password used to access a password protected server. This parameter must be specified with the <code>auth-token-update-strategy </code> parameter. Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length</p> </li> <li> <p>Cannot contain any of the following characters: '/', '"', or '@', '%'</p> </li> </ul> <p> For more information, see AUTH password at <a href="http://redis.io/commands/AUTH">AUTH</a>.</p>
    pub auth_token: Option<String>,
    /// <p>Specifies the strategy to use to update the AUTH token. This parameter must be specified with the <code>auth-token</code> parameter. Possible values:</p> <ul> <li> <p>Rotate</p> </li> <li> <p>Set</p> </li> </ul> <p> For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/auth.html">Authenticating Users with Redis AUTH</a> </p>
    pub auth_token_update_strategy: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure.</p> <p>Valid values: <code>true</code> | <code>false</code> </p>
    pub automatic_failover_enabled: Option<bool>,
    /// <p>A valid cache node type that you want to scale this replication group to.</p>
    pub cache_node_type: Option<String>,
    /// <p>The name of the cache parameter group to apply to all of the clusters in this replication group. This change is asynchronously applied as soon as possible for parameters when the <code>ApplyImmediately</code> parameter is specified as <code>true</code> for this request.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>A list of cache security group names to authorize for the clusters in this replication group. This change is asynchronously applied as soon as possible.</p> <p>This parameter can be used only with replication group containing clusters running outside of an Amazon Virtual Private Cloud (Amazon VPC).</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Must not be <code>Default</code>.</p>
    pub cache_security_group_names: Option<Vec<String>>,
    /// <p>The upgraded version of the cache engine to be run on the clusters in the replication group.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/SelectEngine.html#VersionManagement">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing replication group and create it anew with the earlier engine version. </p>
    pub engine_version: Option<String>,
    /// <p>A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/AutoFailover.html">Minimizing Downtime: Multi-AZ</a>.</p>
    pub multi_az_enabled: Option<bool>,
    /// <p><p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be same as the replication group owner. </p> </note></p>
    pub notification_topic_arn: Option<String>,
    /// <p>The status of the Amazon SNS notification topic for the replication group. Notifications are sent only if the status is <code>active</code>.</p> <p>Valid values: <code>active</code> | <code>inactive</code> </p>
    pub notification_topic_status: Option<String>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>For replication groups with a single primary, if this parameter is specified, ElastiCache promotes the specified cluster in the specified replication group to the primary role. The nodes of all other clusters in the replication group are read replicas.</p>
    pub primary_cluster_id: Option<String>,
    /// <p>Removes the user groups that can access this replication group.</p>
    pub remove_user_groups: Option<bool>,
    /// <p>A description for the replication group. Maximum length is 255 characters.</p>
    pub replication_group_description: Option<String>,
    /// <p>The identifier of the replication group to modify.</p>
    pub replication_group_id: String,
    /// <p>Specifies the VPC Security Groups associated with the clusters in the replication group.</p> <p>This parameter can be used only with replication group containing clusters running in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    pub security_group_ids: Option<Vec<String>>,
    /// <p>The number of days for which ElastiCache retains automatic node group (shard) snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <p> <b>Important</b> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of the node group (shard) specified by <code>SnapshottingClusterId</code>.</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p>
    pub snapshot_window: Option<String>,
    /// <p>The cluster ID that is used as the daily snapshot source for the replication group. This parameter cannot be set for Redis (cluster mode enabled) replication groups.</p>
    pub snapshotting_cluster_id: Option<String>,
    /// <p>A list of user group IDs.</p>
    pub user_group_ids_to_add: Option<Vec<String>>,
    /// <p>A list of users groups to remove, meaning the users in the group no longer can access thereplication group.</p>
    pub user_group_ids_to_remove: Option<Vec<String>>,
}

/// Serialize `ModifyReplicationGroupMessage` contents to a `SignedRequest`.
struct ModifyReplicationGroupMessageSerializer;
impl ModifyReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.apply_immediately {
            params.put(&format!("{}{}", prefix, "ApplyImmediately"), &field_value);
        }
        if let Some(ref field_value) = obj.auth_token {
            params.put(&format!("{}{}", prefix, "AuthToken"), &field_value);
        }
        if let Some(ref field_value) = obj.auth_token_update_strategy {
            params.put(
                &format!("{}{}", prefix, "AuthTokenUpdateStrategy"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(
                &format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.automatic_failover_enabled {
            params.put(
                &format!("{}{}", prefix, "AutomaticFailoverEnabled"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(
                &format!("{}{}", prefix, "CacheParameterGroupName"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "CacheSecurityGroupName"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.multi_az_enabled {
            params.put(&format!("{}{}", prefix, "MultiAZEnabled"), &field_value);
        }

        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicArn"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.notification_topic_status {
            params.put(
                &format!("{}{}", prefix, "NotificationTopicStatus"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(
                &format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.primary_cluster_id {
            params.put(&format!("{}{}", prefix, "PrimaryClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.remove_user_groups {
            params.put(&format!("{}{}", prefix, "RemoveUserGroups"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_description {
            params.put(
                &format!("{}{}", prefix, "ReplicationGroupDescription"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "SecurityGroupId"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(
                &format!("{}{}", prefix, "SnapshotRetentionLimit"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshotting_cluster_id {
            params.put(
                &format!("{}{}", prefix, "SnapshottingClusterId"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.user_group_ids_to_add {
            UserGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserGroupIdsToAdd"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.user_group_ids_to_remove {
            UserGroupIdListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserGroupIdsToRemove"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::modify_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct ModifyReplicationGroupResultDeserializer;
impl ModifyReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, ModifyReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// <p>Represents the input for a <code>ModifyReplicationGroupShardConfiguration</code> operation.</p>
/// see [ElastiCache::modify_replication_group_shard_configuration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReplicationGroupShardConfigurationMessage {
    /// <p>Indicates that the shard reconfiguration process begins immediately. At present, the only permitted value for this parameter is <code>true</code>.</p> <p>Value: true</p>
    pub apply_immediately: bool,
    /// <p>The number of node groups (shards) that results from the modification of the shard configuration.</p>
    pub node_group_count: i64,
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRemove</code> is a list of <code>NodeGroupId</code>s to remove from the cluster.</p> <p>ElastiCache for Redis will attempt to remove all node groups listed by <code>NodeGroupsToRemove</code> from the cluster.</p>
    pub node_groups_to_remove: Option<Vec<String>>,
    /// <p>If the value of <code>NodeGroupCount</code> is less than the current number of node groups (shards), then either <code>NodeGroupsToRemove</code> or <code>NodeGroupsToRetain</code> is required. <code>NodeGroupsToRetain</code> is a list of <code>NodeGroupId</code>s to retain in the cluster.</p> <p>ElastiCache for Redis will attempt to remove all node groups except those listed by <code>NodeGroupsToRetain</code> from the cluster.</p>
    pub node_groups_to_retain: Option<Vec<String>>,
    /// <p>The name of the Redis (cluster mode enabled) cluster (replication group) on which the shards are to be configured.</p>
    pub replication_group_id: String,
    /// <p>Specifies the preferred availability zones for each node group in the cluster. If the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards), you can use this parameter to specify the preferred availability zones of the cluster's shards. If you omit this parameter ElastiCache selects availability zones for you.</p> <p>You can specify this parameter only if the value of <code>NodeGroupCount</code> is greater than the current number of node groups (shards).</p>
    pub resharding_configuration: Option<Vec<ReshardingConfiguration>>,
}

/// Serialize `ModifyReplicationGroupShardConfigurationMessage` contents to a `SignedRequest`.
struct ModifyReplicationGroupShardConfigurationMessageSerializer;
impl ModifyReplicationGroupShardConfigurationMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &ModifyReplicationGroupShardConfigurationMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        params.put(
            &format!("{}{}", prefix, "NodeGroupCount"),
            &obj.node_group_count,
        );
        if let Some(ref field_value) = obj.node_groups_to_remove {
            NodeGroupsToRemoveListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NodeGroupToRemove"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.node_groups_to_retain {
            NodeGroupsToRetainListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "NodeGroupToRetain"),
                field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        if let Some(ref field_value) = obj.resharding_configuration {
            ReshardingConfigurationListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ReshardingConfiguration"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::modify_replication_group_shard_configuration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ModifyReplicationGroupShardConfigurationResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct ModifyReplicationGroupShardConfigurationResultDeserializer;
impl ModifyReplicationGroupShardConfigurationResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ModifyReplicationGroupShardConfigurationResult, XmlParseError> {
        deserialize_elements::<_, ModifyReplicationGroupShardConfigurationResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReplicationGroup" => {
                        obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                            "ReplicationGroup",
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
/// see [ElastiCache::modify_user_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyUserGroupMessage {
    /// <p>The ID of the user group.</p>
    pub user_group_id: String,
    /// <p>The list of user IDs to add to the user group.</p>
    pub user_ids_to_add: Option<Vec<String>>,
    /// <p>The list of user IDs to remove from the user group.</p>
    pub user_ids_to_remove: Option<Vec<String>>,
}

/// Serialize `ModifyUserGroupMessage` contents to a `SignedRequest`.
struct ModifyUserGroupMessageSerializer;
impl ModifyUserGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyUserGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "UserGroupId"), &obj.user_group_id);
        if let Some(ref field_value) = obj.user_ids_to_add {
            UserIdListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserIdsToAdd"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.user_ids_to_remove {
            UserIdListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "UserIdsToRemove"),
                field_value,
            );
        }
    }
}

/// see [ElastiCache::modify_user]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyUserMessage {
    /// <p>Access permissions string used for this user account.</p>
    pub access_string: Option<String>,
    /// <p>Adds additional user permissions to the access string.</p>
    pub append_access_string: Option<String>,
    /// <p>Indicates no password is required for the user account.</p>
    pub no_password_required: Option<bool>,
    /// <p>The passwords belonging to the user account. You are allowed up to two.</p>
    pub passwords: Option<Vec<String>>,
    /// <p>The ID of the user.</p>
    pub user_id: String,
}

/// Serialize `ModifyUserMessage` contents to a `SignedRequest`.
struct ModifyUserMessageSerializer;
impl ModifyUserMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyUserMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.access_string {
            params.put(&format!("{}{}", prefix, "AccessString"), &field_value);
        }
        if let Some(ref field_value) = obj.append_access_string {
            params.put(&format!("{}{}", prefix, "AppendAccessString"), &field_value);
        }
        if let Some(ref field_value) = obj.no_password_required {
            params.put(&format!("{}{}", prefix, "NoPasswordRequired"), &field_value);
        }
        if let Some(ref field_value) = obj.passwords {
            PasswordListInputSerializer::serialize(
                params,
                &format!("{}{}", prefix, "Passwords"),
                field_value,
            );
        }
        params.put(&format!("{}{}", prefix, "UserId"), &obj.user_id);
    }
}

#[allow(dead_code)]
struct MultiAZStatusDeserializer;
impl MultiAZStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents a collection of cache nodes in a replication group. One node in the node group is the read/write primary node. All the other nodes are read-only Replica nodes.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NodeGroup {
    /// <p>The identifier for the node group (shard). A Redis (cluster mode disabled) replication group contains only 1 node group; therefore, the node group ID is 0001. A Redis (cluster mode enabled) replication group contains 1 to 90 node groups numbered 0001 to 0090. Optionally, the user can provide the id for a node group. </p>
    pub node_group_id: Option<String>,
    /// <p>A list containing information about individual nodes within the node group (shard).</p>
    pub node_group_members: Option<Vec<NodeGroupMember>>,
    /// <p>The endpoint of the primary node in this node group (shard).</p>
    pub primary_endpoint: Option<Endpoint>,
    /// <p>The endpoint of the replica nodes in this node group (shard).</p>
    pub reader_endpoint: Option<Endpoint>,
    /// <p>The keyspace for this node group (shard).</p>
    pub slots: Option<String>,
    /// <p>The current state of this replication group - <code>creating</code>, <code>available</code>, <code>modifying</code>, <code>deleting</code>.</p>
    pub status: Option<String>,
}

#[allow(dead_code)]
struct NodeGroupDeserializer;
impl NodeGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeGroup, XmlParseError> {
        deserialize_elements::<_, NodeGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NodeGroupId" => {
                    obj.node_group_id =
                        Some(StringDeserializer::deserialize("NodeGroupId", stack)?);
                }
                "NodeGroupMembers" => {
                    obj.node_group_members.get_or_insert(vec![]).extend(
                        NodeGroupMemberListDeserializer::deserialize("NodeGroupMembers", stack)?,
                    );
                }
                "PrimaryEndpoint" => {
                    obj.primary_endpoint =
                        Some(EndpointDeserializer::deserialize("PrimaryEndpoint", stack)?);
                }
                "ReaderEndpoint" => {
                    obj.reader_endpoint =
                        Some(EndpointDeserializer::deserialize("ReaderEndpoint", stack)?);
                }
                "Slots" => {
                    obj.slots = Some(StringDeserializer::deserialize("Slots", stack)?);
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
/// <p>Node group (shard) configuration options. Each node group (shard) configuration has the following: <code>Slots</code>, <code>PrimaryAvailabilityZone</code>, <code>ReplicaAvailabilityZones</code>, <code>ReplicaCount</code>.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct NodeGroupConfiguration {
    /// <p>Either the ElastiCache for Redis supplied 4-digit id or a user supplied id for the node group these configuration values apply to.</p>
    pub node_group_id: Option<String>,
    /// <p>The Availability Zone where the primary node of this node group (shard) is launched.</p>
    pub primary_availability_zone: Option<String>,
    /// <p>The output ARN of the primary node.</p>
    pub primary_outpost_arn: Option<String>,
    /// <p>A list of Availability Zones to be used for the read replicas. The number of Availability Zones in this list must match the value of <code>ReplicaCount</code> or <code>ReplicasPerNodeGroup</code> if not specified.</p>
    pub replica_availability_zones: Option<Vec<String>>,
    /// <p>The number of read replica nodes in this node group (shard).</p>
    pub replica_count: Option<i64>,
    /// <p>The outpost ARN of the node replicas.</p>
    pub replica_outpost_arns: Option<Vec<String>>,
    /// <p>A string that specifies the keyspace for a particular node group. Keyspaces range from 0 to 16,383. The string is in the format <code>startkey-endkey</code>.</p> <p>Example: <code>"0-3999"</code> </p>
    pub slots: Option<String>,
}

#[allow(dead_code)]
struct NodeGroupConfigurationDeserializer;
impl NodeGroupConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeGroupConfiguration, XmlParseError> {
        deserialize_elements::<_, NodeGroupConfiguration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NodeGroupId" => {
                    obj.node_group_id = Some(AllowedNodeGroupIdDeserializer::deserialize(
                        "NodeGroupId",
                        stack,
                    )?);
                }
                "PrimaryAvailabilityZone" => {
                    obj.primary_availability_zone = Some(StringDeserializer::deserialize(
                        "PrimaryAvailabilityZone",
                        stack,
                    )?);
                }
                "PrimaryOutpostArn" => {
                    obj.primary_outpost_arn =
                        Some(StringDeserializer::deserialize("PrimaryOutpostArn", stack)?);
                }
                "ReplicaAvailabilityZones" => {
                    obj.replica_availability_zones.get_or_insert(vec![]).extend(
                        AvailabilityZonesListDeserializer::deserialize(
                            "ReplicaAvailabilityZones",
                            stack,
                        )?,
                    );
                }
                "ReplicaCount" => {
                    obj.replica_count = Some(IntegerOptionalDeserializer::deserialize(
                        "ReplicaCount",
                        stack,
                    )?);
                }
                "ReplicaOutpostArns" => {
                    obj.replica_outpost_arns.get_or_insert(vec![]).extend(
                        OutpostArnsListDeserializer::deserialize("ReplicaOutpostArns", stack)?,
                    );
                }
                "Slots" => {
                    obj.slots = Some(StringDeserializer::deserialize("Slots", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `NodeGroupConfiguration` contents to a `SignedRequest`.
struct NodeGroupConfigurationSerializer;
impl NodeGroupConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &NodeGroupConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.node_group_id {
            params.put(&format!("{}{}", prefix, "NodeGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.primary_availability_zone {
            params.put(
                &format!("{}{}", prefix, "PrimaryAvailabilityZone"),
                &field_value,
            );
        }
        if let Some(ref field_value) = obj.primary_outpost_arn {
            params.put(&format!("{}{}", prefix, "PrimaryOutpostArn"), &field_value);
        }
        if let Some(ref field_value) = obj.replica_availability_zones {
            AvailabilityZonesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.replica_count {
            params.put(&format!("{}{}", prefix, "ReplicaCount"), &field_value);
        }
        if let Some(ref field_value) = obj.replica_outpost_arns {
            OutpostArnsListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "OutpostArn"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.slots {
            params.put(&format!("{}{}", prefix, "Slots"), &field_value);
        }
    }
}

/// Serialize `NodeGroupConfigurationList` contents to a `SignedRequest`.
struct NodeGroupConfigurationListSerializer;
impl NodeGroupConfigurationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<NodeGroupConfiguration>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            NodeGroupConfigurationSerializer::serialize(params, &key, obj);
        }
    }
}

#[allow(dead_code)]
struct NodeGroupListDeserializer;
impl NodeGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NodeGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NodeGroup" {
                obj.push(NodeGroupDeserializer::deserialize("NodeGroup", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents a single node within a node group (shard).</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NodeGroupMember {
    /// <p>The ID of the cluster to which the node belongs.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The ID of the node within its cluster. A node ID is a numeric identifier (0001, 0002, etc.).</p>
    pub cache_node_id: Option<String>,
    /// <p>The role that is currently assigned to the node - <code>primary</code> or <code>replica</code>. This member is only applicable for Redis (cluster mode disabled) replication groups.</p>
    pub current_role: Option<String>,
    /// <p>The name of the Availability Zone in which the node is located.</p>
    pub preferred_availability_zone: Option<String>,
    /// <p>The outpost ARN of the node group member.</p>
    pub preferred_outpost_arn: Option<String>,
    /// <p>The information required for client programs to connect to a node for read operations. The read endpoint is only applicable on Redis (cluster mode disabled) clusters.</p>
    pub read_endpoint: Option<Endpoint>,
}

#[allow(dead_code)]
struct NodeGroupMemberDeserializer;
impl NodeGroupMemberDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeGroupMember, XmlParseError> {
        deserialize_elements::<_, NodeGroupMember, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "CacheNodeId" => {
                    obj.cache_node_id =
                        Some(StringDeserializer::deserialize("CacheNodeId", stack)?);
                }
                "CurrentRole" => {
                    obj.current_role = Some(StringDeserializer::deserialize("CurrentRole", stack)?);
                }
                "PreferredAvailabilityZone" => {
                    obj.preferred_availability_zone = Some(StringDeserializer::deserialize(
                        "PreferredAvailabilityZone",
                        stack,
                    )?);
                }
                "PreferredOutpostArn" => {
                    obj.preferred_outpost_arn = Some(StringDeserializer::deserialize(
                        "PreferredOutpostArn",
                        stack,
                    )?);
                }
                "ReadEndpoint" => {
                    obj.read_endpoint =
                        Some(EndpointDeserializer::deserialize("ReadEndpoint", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct NodeGroupMemberListDeserializer;
impl NodeGroupMemberListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NodeGroupMember>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NodeGroupMember" {
                obj.push(NodeGroupMemberDeserializer::deserialize(
                    "NodeGroupMember",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The status of the service update on the node group member </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NodeGroupMemberUpdateStatus {
    /// <p>The cache cluster ID</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The node ID of the cache cluster</p>
    pub cache_node_id: Option<String>,
    /// <p>The deletion date of the node</p>
    pub node_deletion_date: Option<String>,
    /// <p>The end date of the update for a node</p>
    pub node_update_end_date: Option<String>,
    /// <p>Reflects whether the update was initiated by the customer or automatically applied</p>
    pub node_update_initiated_by: Option<String>,
    /// <p>The date when the update is triggered</p>
    pub node_update_initiated_date: Option<String>,
    /// <p>The start date of the update for a node</p>
    pub node_update_start_date: Option<String>,
    /// <p>The update status of the node</p>
    pub node_update_status: Option<String>,
    /// <p>The date when the NodeUpdateStatus was last modified</p>
    pub node_update_status_modified_date: Option<String>,
}

#[allow(dead_code)]
struct NodeGroupMemberUpdateStatusDeserializer;
impl NodeGroupMemberUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeGroupMemberUpdateStatus, XmlParseError> {
        deserialize_elements::<_, NodeGroupMemberUpdateStatus, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheClusterId" => {
                        obj.cache_cluster_id =
                            Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                    }
                    "CacheNodeId" => {
                        obj.cache_node_id =
                            Some(StringDeserializer::deserialize("CacheNodeId", stack)?);
                    }
                    "NodeDeletionDate" => {
                        obj.node_deletion_date =
                            Some(TStampDeserializer::deserialize("NodeDeletionDate", stack)?);
                    }
                    "NodeUpdateEndDate" => {
                        obj.node_update_end_date =
                            Some(TStampDeserializer::deserialize("NodeUpdateEndDate", stack)?);
                    }
                    "NodeUpdateInitiatedBy" => {
                        obj.node_update_initiated_by =
                            Some(NodeUpdateInitiatedByDeserializer::deserialize(
                                "NodeUpdateInitiatedBy",
                                stack,
                            )?);
                    }
                    "NodeUpdateInitiatedDate" => {
                        obj.node_update_initiated_date = Some(TStampDeserializer::deserialize(
                            "NodeUpdateInitiatedDate",
                            stack,
                        )?);
                    }
                    "NodeUpdateStartDate" => {
                        obj.node_update_start_date = Some(TStampDeserializer::deserialize(
                            "NodeUpdateStartDate",
                            stack,
                        )?);
                    }
                    "NodeUpdateStatus" => {
                        obj.node_update_status = Some(NodeUpdateStatusDeserializer::deserialize(
                            "NodeUpdateStatus",
                            stack,
                        )?);
                    }
                    "NodeUpdateStatusModifiedDate" => {
                        obj.node_update_status_modified_date = Some(
                            TStampDeserializer::deserialize("NodeUpdateStatusModifiedDate", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct NodeGroupMemberUpdateStatusListDeserializer;
impl NodeGroupMemberUpdateStatusListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NodeGroupMemberUpdateStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NodeGroupMemberUpdateStatus" {
                obj.push(NodeGroupMemberUpdateStatusDeserializer::deserialize(
                    "NodeGroupMemberUpdateStatus",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The status of the service update on the node group </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NodeGroupUpdateStatus {
    /// <p>The ID of the node group</p>
    pub node_group_id: Option<String>,
    /// <p>The status of the service update on the node group member</p>
    pub node_group_member_update_status: Option<Vec<NodeGroupMemberUpdateStatus>>,
}

#[allow(dead_code)]
struct NodeGroupUpdateStatusDeserializer;
impl NodeGroupUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeGroupUpdateStatus, XmlParseError> {
        deserialize_elements::<_, NodeGroupUpdateStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "NodeGroupId" => {
                    obj.node_group_id =
                        Some(StringDeserializer::deserialize("NodeGroupId", stack)?);
                }
                "NodeGroupMemberUpdateStatus" => {
                    obj.node_group_member_update_status
                        .get_or_insert(vec![])
                        .extend(NodeGroupMemberUpdateStatusListDeserializer::deserialize(
                            "NodeGroupMemberUpdateStatus",
                            stack,
                        )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct NodeGroupUpdateStatusListDeserializer;
impl NodeGroupUpdateStatusListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NodeGroupUpdateStatus>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NodeGroupUpdateStatus" {
                obj.push(NodeGroupUpdateStatusDeserializer::deserialize(
                    "NodeGroupUpdateStatus",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `NodeGroupsToRemoveList` contents to a `SignedRequest`.
struct NodeGroupsToRemoveListSerializer;
impl NodeGroupsToRemoveListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `NodeGroupsToRetainList` contents to a `SignedRequest`.
struct NodeGroupsToRetainListSerializer;
impl NodeGroupsToRetainListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Represents an individual cache node in a snapshot of a cluster.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NodeSnapshot {
    /// <p>A unique identifier for the source cluster.</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The date and time when the cache node was created in the source cluster.</p>
    pub cache_node_create_time: Option<String>,
    /// <p>The cache node identifier for the node in the source cluster.</p>
    pub cache_node_id: Option<String>,
    /// <p>The size of the cache on the source cache node.</p>
    pub cache_size: Option<String>,
    /// <p>The configuration for the source node group (shard).</p>
    pub node_group_configuration: Option<NodeGroupConfiguration>,
    /// <p>A unique identifier for the source node group (shard).</p>
    pub node_group_id: Option<String>,
    /// <p>The date and time when the source node's metadata and cache data set was obtained for the snapshot.</p>
    pub snapshot_create_time: Option<String>,
}

#[allow(dead_code)]
struct NodeSnapshotDeserializer;
impl NodeSnapshotDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NodeSnapshot, XmlParseError> {
        deserialize_elements::<_, NodeSnapshot, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "CacheNodeCreateTime" => {
                    obj.cache_node_create_time = Some(TStampDeserializer::deserialize(
                        "CacheNodeCreateTime",
                        stack,
                    )?);
                }
                "CacheNodeId" => {
                    obj.cache_node_id =
                        Some(StringDeserializer::deserialize("CacheNodeId", stack)?);
                }
                "CacheSize" => {
                    obj.cache_size = Some(StringDeserializer::deserialize("CacheSize", stack)?);
                }
                "NodeGroupConfiguration" => {
                    obj.node_group_configuration =
                        Some(NodeGroupConfigurationDeserializer::deserialize(
                            "NodeGroupConfiguration",
                            stack,
                        )?);
                }
                "NodeGroupId" => {
                    obj.node_group_id =
                        Some(StringDeserializer::deserialize("NodeGroupId", stack)?);
                }
                "SnapshotCreateTime" => {
                    obj.snapshot_create_time = Some(TStampDeserializer::deserialize(
                        "SnapshotCreateTime",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct NodeSnapshotListDeserializer;
impl NodeSnapshotListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<NodeSnapshot>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "NodeSnapshot" {
                obj.push(NodeSnapshotDeserializer::deserialize(
                    "NodeSnapshot",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct NodeTypeListDeserializer;
impl NodeTypeListDeserializer {
    #[allow(dead_code, unused_variables)]
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
#[allow(dead_code)]
struct NodeUpdateInitiatedByDeserializer;
impl NodeUpdateInitiatedByDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct NodeUpdateStatusDeserializer;
impl NodeUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Describes a notification topic and its status. Notification topics are used for publishing ElastiCache events to subscribers using Amazon Simple Notification Service (SNS).</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct NotificationConfiguration {
    /// <p>The Amazon Resource Name (ARN) that identifies the topic.</p>
    pub topic_arn: Option<String>,
    /// <p>The current state of the topic.</p>
    pub topic_status: Option<String>,
}

#[allow(dead_code)]
struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<NotificationConfiguration, XmlParseError> {
        deserialize_elements::<_, NotificationConfiguration, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "TopicArn" => {
                        obj.topic_arn = Some(StringDeserializer::deserialize("TopicArn", stack)?);
                    }
                    "TopicStatus" => {
                        obj.topic_status =
                            Some(StringDeserializer::deserialize("TopicStatus", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct OutpostArnsListDeserializer;
impl OutpostArnsListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "OutpostArn" {
                obj.push(StringDeserializer::deserialize("OutpostArn", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `OutpostArnsList` contents to a `SignedRequest`.
struct OutpostArnsListSerializer;
impl OutpostArnsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Describes an individual setting that controls some aspect of ElastiCache behavior.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Parameter {
    /// <p>The valid range of values for the parameter.</p>
    pub allowed_values: Option<String>,
    /// <p>Indicates whether a change to the parameter is applied immediately or requires a reboot for the change to be applied. You can force a reboot or wait until the next maintenance window's reboot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Rebooting.html">Rebooting a Cluster</a>.</p>
    pub change_type: Option<String>,
    /// <p>The valid data type for the parameter.</p>
    pub data_type: Option<String>,
    /// <p>A description of the parameter.</p>
    pub description: Option<String>,
    /// <p>Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed.</p>
    pub is_modifiable: Option<bool>,
    /// <p>The earliest cache engine version to which the parameter can apply.</p>
    pub minimum_engine_version: Option<String>,
    /// <p>The name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>The value of the parameter.</p>
    pub parameter_value: Option<String>,
    /// <p>The source of the parameter.</p>
    pub source: Option<String>,
}

#[allow(dead_code)]
struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(dead_code, unused_variables)]
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
                "ChangeType" => {
                    obj.change_type =
                        Some(ChangeTypeDeserializer::deserialize("ChangeType", stack)?);
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
/// <p>Describes a name-value pair that is used to update the value of a parameter.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ParameterNameValue {
    /// <p>The name of the parameter.</p>
    pub parameter_name: Option<String>,
    /// <p>The value of the parameter.</p>
    pub parameter_value: Option<String>,
}

/// Serialize `ParameterNameValue` contents to a `SignedRequest`.
struct ParameterNameValueSerializer;
impl ParameterNameValueSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ParameterNameValue) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.parameter_name {
            params.put(&format!("{}{}", prefix, "ParameterName"), &field_value);
        }
        if let Some(ref field_value) = obj.parameter_value {
            params.put(&format!("{}{}", prefix, "ParameterValue"), &field_value);
        }
    }
}

/// Serialize `ParameterNameValueList` contents to a `SignedRequest`.
struct ParameterNameValueListSerializer;
impl ParameterNameValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ParameterNameValue>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ParameterNameValueSerializer::serialize(params, &key, obj);
        }
    }
}

#[allow(dead_code)]
struct ParametersListDeserializer;
impl ParametersListDeserializer {
    #[allow(dead_code, unused_variables)]
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

/// Serialize `PasswordListInput` contents to a `SignedRequest`.
struct PasswordListInputSerializer;
impl PasswordListInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct PendingAutomaticFailoverStatusDeserializer;
impl PendingAutomaticFailoverStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>A group of settings that are applied to the cluster in the future, or that are currently being applied.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PendingModifiedValues {
    /// <p>The auth token status</p>
    pub auth_token_status: Option<String>,
    /// <p>A list of cache node IDs that are being removed (or will be removed) from the cluster. A node ID is a 4-digit numeric identifier (0001, 0002, etc.).</p>
    pub cache_node_ids_to_remove: Option<Vec<String>>,
    /// <p>The cache node type that this cluster or replication group is scaled to.</p>
    pub cache_node_type: Option<String>,
    /// <p>The new cache engine version that the cluster runs.</p>
    pub engine_version: Option<String>,
    /// <p>The new number of cache nodes for the cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>
    pub num_cache_nodes: Option<i64>,
}

#[allow(dead_code)]
struct PendingModifiedValuesDeserializer;
impl PendingModifiedValuesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PendingModifiedValues, XmlParseError> {
        deserialize_elements::<_, PendingModifiedValues, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AuthTokenStatus" => {
                    obj.auth_token_status = Some(AuthTokenUpdateStatusDeserializer::deserialize(
                        "AuthTokenStatus",
                        stack,
                    )?);
                }
                "CacheNodeIdsToRemove" => {
                    obj.cache_node_ids_to_remove.get_or_insert(vec![]).extend(
                        CacheNodeIdsListDeserializer::deserialize("CacheNodeIdsToRemove", stack)?,
                    );
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "EngineVersion" => {
                    obj.engine_version =
                        Some(StringDeserializer::deserialize("EngineVersion", stack)?);
                }
                "NumCacheNodes" => {
                    obj.num_cache_nodes = Some(IntegerOptionalDeserializer::deserialize(
                        "NumCacheNodes",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `PreferredAvailabilityZoneList` contents to a `SignedRequest`.
struct PreferredAvailabilityZoneListSerializer;
impl PreferredAvailabilityZoneListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `PreferredOutpostArnList` contents to a `SignedRequest`.
struct PreferredOutpostArnListSerializer;
impl PreferredOutpostArnListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Update action that has been processed for the corresponding apply/stop request</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ProcessedUpdateAction {
    /// <p>The ID of the cache cluster</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The ID of the replication group</p>
    pub replication_group_id: Option<String>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
    /// <p>The status of the update action on the Redis cluster</p>
    pub update_action_status: Option<String>,
}

#[allow(dead_code)]
struct ProcessedUpdateActionDeserializer;
impl ProcessedUpdateActionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ProcessedUpdateAction, XmlParseError> {
        deserialize_elements::<_, ProcessedUpdateAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "ReplicationGroupId" => {
                    obj.replication_group_id = Some(StringDeserializer::deserialize(
                        "ReplicationGroupId",
                        stack,
                    )?);
                }
                "ServiceUpdateName" => {
                    obj.service_update_name =
                        Some(StringDeserializer::deserialize("ServiceUpdateName", stack)?);
                }
                "UpdateActionStatus" => {
                    obj.update_action_status = Some(UpdateActionStatusDeserializer::deserialize(
                        "UpdateActionStatus",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ProcessedUpdateActionListDeserializer;
impl ProcessedUpdateActionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ProcessedUpdateAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ProcessedUpdateAction" {
                obj.push(ProcessedUpdateActionDeserializer::deserialize(
                    "ProcessedUpdateAction",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the input of a <code>PurchaseReservedCacheNodesOffering</code> operation.</p>
/// see [ElastiCache::purchase_reserved_cache_nodes_offering]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PurchaseReservedCacheNodesOfferingMessage {
    /// <p>The number of cache node instances to reserve.</p> <p>Default: <code>1</code> </p>
    pub cache_node_count: Option<i64>,
    /// <p>A customer-specified identifier to track this reservation.</p> <note> <p>The Reserved Cache Node ID is an unique customer-specified identifier to track this reservation. If this parameter is not specified, ElastiCache automatically generates an identifier for the reservation.</p> </note> <p>Example: myreservationID</p>
    pub reserved_cache_node_id: Option<String>,
    /// <p>The ID of the reserved cache node offering to purchase.</p> <p>Example: <code>438012d3-4052-4cc7-b2e3-8d3372e0e706</code> </p>
    pub reserved_cache_nodes_offering_id: String,
}

/// Serialize `PurchaseReservedCacheNodesOfferingMessage` contents to a `SignedRequest`.
struct PurchaseReservedCacheNodesOfferingMessageSerializer;
impl PurchaseReservedCacheNodesOfferingMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PurchaseReservedCacheNodesOfferingMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_node_count {
            params.put(&format!("{}{}", prefix, "CacheNodeCount"), &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_node_id {
            params.put(
                &format!("{}{}", prefix, "ReservedCacheNodeId"),
                &field_value,
            );
        }
        params.put(
            &format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
            &obj.reserved_cache_nodes_offering_id,
        );
    }
}

/// see [ElastiCache::purchase_reserved_cache_nodes_offering]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct PurchaseReservedCacheNodesOfferingResult {
    pub reserved_cache_node: Option<ReservedCacheNode>,
}

#[allow(dead_code)]
struct PurchaseReservedCacheNodesOfferingResultDeserializer;
impl PurchaseReservedCacheNodesOfferingResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<PurchaseReservedCacheNodesOfferingResult, XmlParseError> {
        deserialize_elements::<_, PurchaseReservedCacheNodesOfferingResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ReservedCacheNode" => {
                        obj.reserved_cache_node = Some(ReservedCacheNodeDeserializer::deserialize(
                            "ReservedCacheNode",
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
/// see [ElastiCache::rebalance_slots_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebalanceSlotsInGlobalReplicationGroupMessage {
    /// <p>If <code>True</code>, redistribution is applied immediately.</p>
    pub apply_immediately: bool,
    /// <p>The name of the Global Datastore</p>
    pub global_replication_group_id: String,
}

/// Serialize `RebalanceSlotsInGlobalReplicationGroupMessage` contents to a `SignedRequest`.
struct RebalanceSlotsInGlobalReplicationGroupMessageSerializer;
impl RebalanceSlotsInGlobalReplicationGroupMessageSerializer {
    fn serialize(
        params: &mut Params,
        name: &str,
        obj: &RebalanceSlotsInGlobalReplicationGroupMessage,
    ) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ApplyImmediately"),
            &obj.apply_immediately,
        );
        params.put(
            &format!("{}{}", prefix, "GlobalReplicationGroupId"),
            &obj.global_replication_group_id,
        );
    }
}

/// see [ElastiCache::rebalance_slots_in_global_replication_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RebalanceSlotsInGlobalReplicationGroupResult {
    pub global_replication_group: Option<GlobalReplicationGroup>,
}

#[allow(dead_code)]
struct RebalanceSlotsInGlobalReplicationGroupResultDeserializer;
impl RebalanceSlotsInGlobalReplicationGroupResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RebalanceSlotsInGlobalReplicationGroupResult, XmlParseError> {
        deserialize_elements::<_, RebalanceSlotsInGlobalReplicationGroupResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "GlobalReplicationGroup" => {
                        obj.global_replication_group =
                            Some(GlobalReplicationGroupDeserializer::deserialize(
                                "GlobalReplicationGroup",
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
/// <p>Represents the input of a <code>RebootCacheCluster</code> operation.</p>
/// see [ElastiCache::reboot_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootCacheClusterMessage {
    /// <p>The cluster identifier. This parameter is stored as a lowercase string.</p>
    pub cache_cluster_id: String,
    /// <p>A list of cache node IDs to reboot. A node ID is a numeric identifier (0001, 0002, etc.). To reboot an entire cluster, specify all of the cache node IDs.</p>
    pub cache_node_ids_to_reboot: Vec<String>,
}

/// Serialize `RebootCacheClusterMessage` contents to a `SignedRequest`.
struct RebootCacheClusterMessageSerializer;
impl RebootCacheClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebootCacheClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheClusterId"),
            &obj.cache_cluster_id,
        );
        CacheNodeIdsListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "CacheNodeId"),
            &obj.cache_node_ids_to_reboot,
        );
    }
}

/// see [ElastiCache::reboot_cache_cluster]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RebootCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

#[allow(dead_code)]
struct RebootCacheClusterResultDeserializer;
impl RebootCacheClusterResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RebootCacheClusterResult, XmlParseError> {
        deserialize_elements::<_, RebootCacheClusterResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheCluster" => {
                        obj.cache_cluster = Some(CacheClusterDeserializer::deserialize(
                            "CacheCluster",
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
/// <p>Contains the specific price and frequency of a recurring charges for a reserved cache node, or for a reserved cache node offering.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RecurringCharge {
    /// <p>The monetary amount of the recurring charge.</p>
    pub recurring_charge_amount: Option<f64>,
    /// <p>The frequency of the recurring charge.</p>
    pub recurring_charge_frequency: Option<String>,
}

#[allow(dead_code)]
struct RecurringChargeDeserializer;
impl RecurringChargeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RecurringCharge, XmlParseError> {
        deserialize_elements::<_, RecurringCharge, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "RecurringChargeAmount" => {
                    obj.recurring_charge_amount = Some(DoubleDeserializer::deserialize(
                        "RecurringChargeAmount",
                        stack,
                    )?);
                }
                "RecurringChargeFrequency" => {
                    obj.recurring_charge_frequency = Some(StringDeserializer::deserialize(
                        "RecurringChargeFrequency",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct RecurringChargeListDeserializer;
impl RecurringChargeListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<RecurringCharge>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "RecurringCharge" {
                obj.push(RecurringChargeDeserializer::deserialize(
                    "RecurringCharge",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>A list of the replication groups </p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RegionalConfiguration {
    /// <p>The name of the secondary cluster</p>
    pub replication_group_id: String,
    /// <p>The AWS region where the cluster is stored</p>
    pub replication_group_region: String,
    /// <p>A list of <code>PreferredAvailabilityZones</code> objects that specifies the configuration of a node group in the resharded cluster. </p>
    pub resharding_configuration: Vec<ReshardingConfiguration>,
}

/// Serialize `RegionalConfiguration` contents to a `SignedRequest`.
struct RegionalConfigurationSerializer;
impl RegionalConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RegionalConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupRegion"),
            &obj.replication_group_region,
        );
        ReshardingConfigurationListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "ReshardingConfiguration"),
            &obj.resharding_configuration,
        );
    }
}

/// Serialize `RegionalConfigurationList` contents to a `SignedRequest`.
struct RegionalConfigurationListSerializer;
impl RegionalConfigurationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<RegionalConfiguration>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            RegionalConfigurationSerializer::serialize(params, &key, obj);
        }
    }
}

/// Serialize `RemoveReplicasList` contents to a `SignedRequest`.
struct RemoveReplicasListSerializer;
impl RemoveReplicasListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Represents the input of a <code>RemoveTagsFromResource</code> operation.</p>
/// see [ElastiCache::remove_tags_from_resource]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceMessage {
    /// <p>The Amazon Resource Name (ARN) of the resource from which you want the tags removed, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p> <p>For more information about ARNs, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    pub resource_name: String,
    /// <p>A list of <code>TagKeys</code> identifying the tags you want removed from the named resource.</p>
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

/// Serialize `ReplicaConfigurationList` contents to a `SignedRequest`.
struct ReplicaConfigurationListSerializer;
impl ReplicaConfigurationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ConfigureShard>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ConfigureShardSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>Contains all of the attributes of a specific Redis replication group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReplicationGroup {
    /// <p>The ARN (Amazon Resource Name) of the replication group.</p>
    pub arn: Option<String>,
    /// <p>A flag that enables encryption at-rest when set to <code>true</code>.</p> <p>You cannot modify the value of <code>AtRestEncryptionEnabled</code> after the cluster is created. To enable encryption at-rest on a cluster you must set <code>AtRestEncryptionEnabled</code> to <code>true</code> when you create a cluster.</p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p>
    pub at_rest_encryption_enabled: Option<bool>,
    /// <p>A flag that enables using an <code>AuthToken</code> (password) when issuing Redis commands.</p> <p>Default: <code>false</code> </p>
    pub auth_token_enabled: Option<bool>,
    /// <p>The date the auth token was last modified</p>
    pub auth_token_last_modified_date: Option<String>,
    /// <p>Indicates the status of automatic failover for this Redis replication group.</p>
    pub automatic_failover: Option<String>,
    /// <p>The name of the compute and memory capacity node type for each node in the replication group.</p>
    pub cache_node_type: Option<String>,
    /// <p>A flag indicating whether or not this replication group is cluster enabled; i.e., whether its data can be partitioned across multiple shards (API/CLI: node groups).</p> <p>Valid values: <code>true</code> | <code>false</code> </p>
    pub cluster_enabled: Option<bool>,
    /// <p>The configuration endpoint for this replication group. Use the configuration endpoint to connect to this replication group.</p>
    pub configuration_endpoint: Option<Endpoint>,
    /// <p>The user supplied description of the replication group.</p>
    pub description: Option<String>,
    /// <p>The name of the Global Datastore and role of this replication group in the Global Datastore.</p>
    pub global_replication_group_info: Option<GlobalReplicationGroupInfo>,
    /// <p>The ID of the KMS key used to encrypt the disk in the cluster.</p>
    pub kms_key_id: Option<String>,
    /// <p>The names of all the cache clusters that are part of this replication group.</p>
    pub member_clusters: Option<Vec<String>>,
    /// <p>The outpost ARNs of the replication group's member clusters.</p>
    pub member_clusters_outpost_arns: Option<Vec<String>>,
    /// <p>A flag indicating if you have Multi-AZ enabled to enhance fault tolerance. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/AutoFailover.html">Minimizing Downtime: Multi-AZ</a> </p>
    pub multi_az: Option<String>,
    /// <p>A list of node groups in this replication group. For Redis (cluster mode disabled) replication groups, this is a single-element list. For Redis (cluster mode enabled) replication groups, the list contains an entry for each node group (shard).</p>
    pub node_groups: Option<Vec<NodeGroup>>,
    /// <p>A group of settings to be applied to the replication group, either immediately or during the next maintenance window.</p>
    pub pending_modified_values: Option<ReplicationGroupPendingModifiedValues>,
    /// <p>The identifier for the replication group.</p>
    pub replication_group_id: Option<String>,
    /// <p><p>The number of days for which ElastiCache retains automatic cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <important> <p> If the value of <code>SnapshotRetentionLimit</code> is set to zero (0), backups are turned off.</p> </important></p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p><p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note></p>
    pub snapshot_window: Option<String>,
    /// <p>The cluster ID that is used as the daily snapshot source for the replication group.</p>
    pub snapshotting_cluster_id: Option<String>,
    /// <p>The current state of this replication group - <code>creating</code>, <code>available</code>, <code>modifying</code>, <code>deleting</code>, <code>create-failed</code>, <code>snapshotting</code>.</p>
    pub status: Option<String>,
    /// <p>A flag that enables in-transit encryption when set to <code>true</code>.</p> <p>You cannot modify the value of <code>TransitEncryptionEnabled</code> after the cluster is created. To enable in-transit encryption on a cluster you must set <code>TransitEncryptionEnabled</code> to <code>true</code> when you create a cluster.</p> <p> <b>Required:</b> Only available when creating a replication group in an Amazon VPC using redis version <code>3.2.6</code>, <code>4.x</code> or later.</p> <p>Default: <code>false</code> </p>
    pub transit_encryption_enabled: Option<bool>,
    /// <p>The list of user group IDs that have access to the replication group.</p>
    pub user_group_ids: Option<Vec<String>>,
}

#[allow(dead_code)]
struct ReplicationGroupDeserializer;
impl ReplicationGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationGroup, XmlParseError> {
        deserialize_elements::<_, ReplicationGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "AtRestEncryptionEnabled" => {
                    obj.at_rest_encryption_enabled = Some(
                        BooleanOptionalDeserializer::deserialize("AtRestEncryptionEnabled", stack)?,
                    );
                }
                "AuthTokenEnabled" => {
                    obj.auth_token_enabled = Some(BooleanOptionalDeserializer::deserialize(
                        "AuthTokenEnabled",
                        stack,
                    )?);
                }
                "AuthTokenLastModifiedDate" => {
                    obj.auth_token_last_modified_date = Some(TStampDeserializer::deserialize(
                        "AuthTokenLastModifiedDate",
                        stack,
                    )?);
                }
                "AutomaticFailover" => {
                    obj.automatic_failover =
                        Some(AutomaticFailoverStatusDeserializer::deserialize(
                            "AutomaticFailover",
                            stack,
                        )?);
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "ClusterEnabled" => {
                    obj.cluster_enabled = Some(BooleanOptionalDeserializer::deserialize(
                        "ClusterEnabled",
                        stack,
                    )?);
                }
                "ConfigurationEndpoint" => {
                    obj.configuration_endpoint = Some(EndpointDeserializer::deserialize(
                        "ConfigurationEndpoint",
                        stack,
                    )?);
                }
                "Description" => {
                    obj.description = Some(StringDeserializer::deserialize("Description", stack)?);
                }
                "GlobalReplicationGroupInfo" => {
                    obj.global_replication_group_info =
                        Some(GlobalReplicationGroupInfoDeserializer::deserialize(
                            "GlobalReplicationGroupInfo",
                            stack,
                        )?);
                }
                "KmsKeyId" => {
                    obj.kms_key_id = Some(StringDeserializer::deserialize("KmsKeyId", stack)?);
                }
                "MemberClusters" => {
                    obj.member_clusters.get_or_insert(vec![]).extend(
                        ClusterIdListDeserializer::deserialize("MemberClusters", stack)?,
                    );
                }
                "MemberClustersOutpostArns" => {
                    obj.member_clusters_outpost_arns
                        .get_or_insert(vec![])
                        .extend(ReplicationGroupOutpostArnListDeserializer::deserialize(
                            "MemberClustersOutpostArns",
                            stack,
                        )?);
                }
                "MultiAZ" => {
                    obj.multi_az = Some(MultiAZStatusDeserializer::deserialize("MultiAZ", stack)?);
                }
                "NodeGroups" => {
                    obj.node_groups
                        .get_or_insert(vec![])
                        .extend(NodeGroupListDeserializer::deserialize("NodeGroups", stack)?);
                }
                "PendingModifiedValues" => {
                    obj.pending_modified_values = Some(
                        ReplicationGroupPendingModifiedValuesDeserializer::deserialize(
                            "PendingModifiedValues",
                            stack,
                        )?,
                    );
                }
                "ReplicationGroupId" => {
                    obj.replication_group_id = Some(StringDeserializer::deserialize(
                        "ReplicationGroupId",
                        stack,
                    )?);
                }
                "SnapshotRetentionLimit" => {
                    obj.snapshot_retention_limit = Some(IntegerOptionalDeserializer::deserialize(
                        "SnapshotRetentionLimit",
                        stack,
                    )?);
                }
                "SnapshotWindow" => {
                    obj.snapshot_window =
                        Some(StringDeserializer::deserialize("SnapshotWindow", stack)?);
                }
                "SnapshottingClusterId" => {
                    obj.snapshotting_cluster_id = Some(StringDeserializer::deserialize(
                        "SnapshottingClusterId",
                        stack,
                    )?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "TransitEncryptionEnabled" => {
                    obj.transit_encryption_enabled =
                        Some(BooleanOptionalDeserializer::deserialize(
                            "TransitEncryptionEnabled",
                            stack,
                        )?);
                }
                "UserGroupIds" => {
                    obj.user_group_ids.get_or_insert(vec![]).extend(
                        UserGroupIdListDeserializer::deserialize("UserGroupIds", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}

/// Serialize `ReplicationGroupIdList` contents to a `SignedRequest`.
struct ReplicationGroupIdListSerializer;
impl ReplicationGroupIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct ReplicationGroupListDeserializer;
impl ReplicationGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReplicationGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReplicationGroup" {
                obj.push(ReplicationGroupDeserializer::deserialize(
                    "ReplicationGroup",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeReplicationGroups</code> operation.</p>
/// see [ElastiCache::describe_replication_groups]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReplicationGroupMessage {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
    /// <p>A list of replication groups. Each item in the list contains detailed information about one replication group.</p>
    pub replication_groups: Option<Vec<ReplicationGroup>>,
}

impl ReplicationGroupMessage {
    fn pagination_page_opt(self) -> Option<Vec<ReplicationGroup>> {
        Some(self.replication_groups.as_ref()?.clone())
    }
}

impl PagedOutput for ReplicationGroupMessage {
    type Item = ReplicationGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReplicationGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct ReplicationGroupMessageDeserializer;
impl ReplicationGroupMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationGroupMessage, XmlParseError> {
        deserialize_elements::<_, ReplicationGroupMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "ReplicationGroups" => {
                        obj.replication_groups.get_or_insert(vec![]).extend(
                            ReplicationGroupListDeserializer::deserialize(
                                "ReplicationGroups",
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
#[allow(dead_code)]
struct ReplicationGroupOutpostArnListDeserializer;
impl ReplicationGroupOutpostArnListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReplicationGroupOutpostArn" {
                obj.push(StringDeserializer::deserialize(
                    "ReplicationGroupOutpostArn",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The settings to be applied to the Redis replication group, either immediately or during the next maintenance window.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReplicationGroupPendingModifiedValues {
    /// <p>The auth token status</p>
    pub auth_token_status: Option<String>,
    /// <p>Indicates the status of automatic failover for this Redis replication group.</p>
    pub automatic_failover_status: Option<String>,
    /// <p>The primary cluster ID that is applied immediately (if <code>--apply-immediately</code> was specified), or during the next maintenance window.</p>
    pub primary_cluster_id: Option<String>,
    /// <p>The status of an online resharding operation.</p>
    pub resharding: Option<ReshardingStatus>,
    /// <p>The user groups being modified.</p>
    pub user_groups: Option<UserGroupsUpdateStatus>,
}

#[allow(dead_code)]
struct ReplicationGroupPendingModifiedValuesDeserializer;
impl ReplicationGroupPendingModifiedValuesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReplicationGroupPendingModifiedValues, XmlParseError> {
        deserialize_elements::<_, ReplicationGroupPendingModifiedValues, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "AuthTokenStatus" => {
                        obj.auth_token_status =
                            Some(AuthTokenUpdateStatusDeserializer::deserialize(
                                "AuthTokenStatus",
                                stack,
                            )?);
                    }
                    "AutomaticFailoverStatus" => {
                        obj.automatic_failover_status =
                            Some(PendingAutomaticFailoverStatusDeserializer::deserialize(
                                "AutomaticFailoverStatus",
                                stack,
                            )?);
                    }
                    "PrimaryClusterId" => {
                        obj.primary_cluster_id =
                            Some(StringDeserializer::deserialize("PrimaryClusterId", stack)?);
                    }
                    "Resharding" => {
                        obj.resharding = Some(ReshardingStatusDeserializer::deserialize(
                            "Resharding",
                            stack,
                        )?);
                    }
                    "UserGroups" => {
                        obj.user_groups = Some(UserGroupsUpdateStatusDeserializer::deserialize(
                            "UserGroups",
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
/// <p>Represents the output of a <code>PurchaseReservedCacheNodesOffering</code> operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReservedCacheNode {
    /// <p>The number of cache nodes that have been reserved.</p>
    pub cache_node_count: Option<i64>,
    /// <p><p>The cache node type for the reserved cache nodes.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>The duration of the reservation in seconds.</p>
    pub duration: Option<i64>,
    /// <p>The fixed price charged for this reserved cache node.</p>
    pub fixed_price: Option<f64>,
    /// <p>The offering type of this reserved cache node.</p>
    pub offering_type: Option<String>,
    /// <p>The description of the reserved cache node.</p>
    pub product_description: Option<String>,
    /// <p>The recurring price charged to run this reserved cache node.</p>
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>The Amazon Resource Name (ARN) of the reserved cache node.</p> <p>Example: <code>arn:aws:elasticache:us-east-1:123456789012:reserved-instance:ri-2017-03-27-08-33-25-582</code> </p>
    pub reservation_arn: Option<String>,
    /// <p>The unique identifier for the reservation.</p>
    pub reserved_cache_node_id: Option<String>,
    /// <p>The offering identifier.</p>
    pub reserved_cache_nodes_offering_id: Option<String>,
    /// <p>The time the reservation started.</p>
    pub start_time: Option<String>,
    /// <p>The state of the reserved cache node.</p>
    pub state: Option<String>,
    /// <p>The hourly price charged for this reserved cache node.</p>
    pub usage_price: Option<f64>,
}

#[allow(dead_code)]
struct ReservedCacheNodeDeserializer;
impl ReservedCacheNodeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedCacheNode, XmlParseError> {
        deserialize_elements::<_, ReservedCacheNode, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheNodeCount" => {
                    obj.cache_node_count =
                        Some(IntegerDeserializer::deserialize("CacheNodeCount", stack)?);
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "Duration" => {
                    obj.duration = Some(IntegerDeserializer::deserialize("Duration", stack)?);
                }
                "FixedPrice" => {
                    obj.fixed_price = Some(DoubleDeserializer::deserialize("FixedPrice", stack)?);
                }
                "OfferingType" => {
                    obj.offering_type =
                        Some(StringDeserializer::deserialize("OfferingType", stack)?);
                }
                "ProductDescription" => {
                    obj.product_description = Some(StringDeserializer::deserialize(
                        "ProductDescription",
                        stack,
                    )?);
                }
                "RecurringCharges" => {
                    obj.recurring_charges.get_or_insert(vec![]).extend(
                        RecurringChargeListDeserializer::deserialize("RecurringCharges", stack)?,
                    );
                }
                "ReservationARN" => {
                    obj.reservation_arn =
                        Some(StringDeserializer::deserialize("ReservationARN", stack)?);
                }
                "ReservedCacheNodeId" => {
                    obj.reserved_cache_node_id = Some(StringDeserializer::deserialize(
                        "ReservedCacheNodeId",
                        stack,
                    )?);
                }
                "ReservedCacheNodesOfferingId" => {
                    obj.reserved_cache_nodes_offering_id = Some(StringDeserializer::deserialize(
                        "ReservedCacheNodesOfferingId",
                        stack,
                    )?);
                }
                "StartTime" => {
                    obj.start_time = Some(TStampDeserializer::deserialize("StartTime", stack)?);
                }
                "State" => {
                    obj.state = Some(StringDeserializer::deserialize("State", stack)?);
                }
                "UsagePrice" => {
                    obj.usage_price = Some(DoubleDeserializer::deserialize("UsagePrice", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ReservedCacheNodeListDeserializer;
impl ReservedCacheNodeListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReservedCacheNode>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReservedCacheNode" {
                obj.push(ReservedCacheNodeDeserializer::deserialize(
                    "ReservedCacheNode",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeReservedCacheNodes</code> operation.</p>
/// see [ElastiCache::describe_reserved_cache_nodes]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReservedCacheNodeMessage {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
    /// <p>A list of reserved cache nodes. Each element in the list contains detailed information about one node.</p>
    pub reserved_cache_nodes: Option<Vec<ReservedCacheNode>>,
}

impl ReservedCacheNodeMessage {
    fn pagination_page_opt(self) -> Option<Vec<ReservedCacheNode>> {
        Some(self.reserved_cache_nodes.as_ref()?.clone())
    }
}

impl PagedOutput for ReservedCacheNodeMessage {
    type Item = ReservedCacheNode;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReservedCacheNode> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct ReservedCacheNodeMessageDeserializer;
impl ReservedCacheNodeMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedCacheNodeMessage, XmlParseError> {
        deserialize_elements::<_, ReservedCacheNodeMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "ReservedCacheNodes" => {
                        obj.reserved_cache_nodes.get_or_insert(vec![]).extend(
                            ReservedCacheNodeListDeserializer::deserialize(
                                "ReservedCacheNodes",
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
/// <p>Describes all of the attributes of a reserved cache node offering.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReservedCacheNodesOffering {
    /// <p><p>The cache node type for the reserved cache node.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>The duration of the offering. in seconds.</p>
    pub duration: Option<i64>,
    /// <p>The fixed price charged for this offering.</p>
    pub fixed_price: Option<f64>,
    /// <p>The offering type.</p>
    pub offering_type: Option<String>,
    /// <p>The cache engine used by the offering.</p>
    pub product_description: Option<String>,
    /// <p>The recurring price charged to run this reserved cache node.</p>
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    /// <p>A unique identifier for the reserved cache node offering.</p>
    pub reserved_cache_nodes_offering_id: Option<String>,
    /// <p>The hourly price charged for this offering.</p>
    pub usage_price: Option<f64>,
}

#[allow(dead_code)]
struct ReservedCacheNodesOfferingDeserializer;
impl ReservedCacheNodesOfferingDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedCacheNodesOffering, XmlParseError> {
        deserialize_elements::<_, ReservedCacheNodesOffering, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheNodeType" => {
                        obj.cache_node_type =
                            Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                    }
                    "Duration" => {
                        obj.duration = Some(IntegerDeserializer::deserialize("Duration", stack)?);
                    }
                    "FixedPrice" => {
                        obj.fixed_price =
                            Some(DoubleDeserializer::deserialize("FixedPrice", stack)?);
                    }
                    "OfferingType" => {
                        obj.offering_type =
                            Some(StringDeserializer::deserialize("OfferingType", stack)?);
                    }
                    "ProductDescription" => {
                        obj.product_description = Some(StringDeserializer::deserialize(
                            "ProductDescription",
                            stack,
                        )?);
                    }
                    "RecurringCharges" => {
                        obj.recurring_charges.get_or_insert(vec![]).extend(
                            RecurringChargeListDeserializer::deserialize(
                                "RecurringCharges",
                                stack,
                            )?,
                        );
                    }
                    "ReservedCacheNodesOfferingId" => {
                        obj.reserved_cache_nodes_offering_id = Some(
                            StringDeserializer::deserialize("ReservedCacheNodesOfferingId", stack)?,
                        );
                    }
                    "UsagePrice" => {
                        obj.usage_price =
                            Some(DoubleDeserializer::deserialize("UsagePrice", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct ReservedCacheNodesOfferingListDeserializer;
impl ReservedCacheNodesOfferingListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ReservedCacheNodesOffering>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ReservedCacheNodesOffering" {
                obj.push(ReservedCacheNodesOfferingDeserializer::deserialize(
                    "ReservedCacheNodesOffering",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Represents the output of a <code>DescribeReservedCacheNodesOfferings</code> operation.</p>
/// see [ElastiCache::describe_reserved_cache_nodes_offerings]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReservedCacheNodesOfferingMessage {
    /// <p>Provides an identifier to allow retrieval of paginated results.</p>
    pub marker: Option<String>,
    /// <p>A list of reserved cache node offerings. Each element in the list contains detailed information about one offering.</p>
    pub reserved_cache_nodes_offerings: Option<Vec<ReservedCacheNodesOffering>>,
}

impl ReservedCacheNodesOfferingMessage {
    fn pagination_page_opt(self) -> Option<Vec<ReservedCacheNodesOffering>> {
        Some(self.reserved_cache_nodes_offerings.as_ref()?.clone())
    }
}

impl PagedOutput for ReservedCacheNodesOfferingMessage {
    type Item = ReservedCacheNodesOffering;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReservedCacheNodesOffering> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct ReservedCacheNodesOfferingMessageDeserializer;
impl ReservedCacheNodesOfferingMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReservedCacheNodesOfferingMessage, XmlParseError> {
        deserialize_elements::<_, ReservedCacheNodesOfferingMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "Marker" => {
                        obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                    }
                    "ReservedCacheNodesOfferings" => {
                        obj.reserved_cache_nodes_offerings
                            .get_or_insert(vec![])
                            .extend(ReservedCacheNodesOfferingListDeserializer::deserialize(
                                "ReservedCacheNodesOfferings",
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
/// <p>Represents the input of a <code>ResetCacheParameterGroup</code> operation.</p>
/// see [ElastiCache::reset_cache_parameter_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ResetCacheParameterGroupMessage {
    /// <p>The name of the cache parameter group to reset.</p>
    pub cache_parameter_group_name: String,
    /// <p>An array of parameter names to reset to their default values. If <code>ResetAllParameters</code> is <code>true</code>, do not use <code>ParameterNameValues</code>. If <code>ResetAllParameters</code> is <code>false</code>, you must specify the name of at least one parameter to reset.</p>
    pub parameter_name_values: Option<Vec<ParameterNameValue>>,
    /// <p>If <code>true</code>, all parameters in the cache parameter group are reset to their default values. If <code>false</code>, only the parameters listed by <code>ParameterNameValues</code> are reset to their default values.</p> <p>Valid values: <code>true</code> | <code>false</code> </p>
    pub reset_all_parameters: Option<bool>,
}

/// Serialize `ResetCacheParameterGroupMessage` contents to a `SignedRequest`.
struct ResetCacheParameterGroupMessageSerializer;
impl ResetCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheParameterGroupName"),
            &obj.cache_parameter_group_name,
        );
        if let Some(ref field_value) = obj.parameter_name_values {
            ParameterNameValueListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "ParameterNameValue"),
                field_value,
            );
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(&format!("{}{}", prefix, "ResetAllParameters"), &field_value);
        }
    }
}

/// <p>A list of <code>PreferredAvailabilityZones</code> objects that specifies the configuration of a node group in the resharded cluster.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReshardingConfiguration {
    /// <p>Either the ElastiCache for Redis supplied 4-digit id or a user supplied id for the node group these configuration values apply to.</p>
    pub node_group_id: Option<String>,
    /// <p>A list of preferred availability zones for the nodes in this cluster.</p>
    pub preferred_availability_zones: Option<Vec<String>>,
}

/// Serialize `ReshardingConfiguration` contents to a `SignedRequest`.
struct ReshardingConfigurationSerializer;
impl ReshardingConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ReshardingConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.node_group_id {
            params.put(&format!("{}{}", prefix, "NodeGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.preferred_availability_zones {
            AvailabilityZonesListSerializer::serialize(
                params,
                &format!("{}{}", prefix, "AvailabilityZone"),
                field_value,
            );
        }
    }
}

/// Serialize `ReshardingConfigurationList` contents to a `SignedRequest`.
struct ReshardingConfigurationListSerializer;
impl ReshardingConfigurationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<ReshardingConfiguration>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ReshardingConfigurationSerializer::serialize(params, &key, obj);
        }
    }
}

/// <p>The status of an online resharding operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ReshardingStatus {
    /// <p>Represents the progress of an online resharding operation.</p>
    pub slot_migration: Option<SlotMigration>,
}

#[allow(dead_code)]
struct ReshardingStatusDeserializer;
impl ReshardingStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ReshardingStatus, XmlParseError> {
        deserialize_elements::<_, ReshardingStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "SlotMigration" => {
                    obj.slot_migration = Some(SlotMigrationDeserializer::deserialize(
                        "SlotMigration",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents the input of a <code>RevokeCacheSecurityGroupIngress</code> operation.</p>
/// see [ElastiCache::revoke_cache_security_group_ingress]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RevokeCacheSecurityGroupIngressMessage {
    /// <p>The name of the cache security group to revoke ingress from.</p>
    pub cache_security_group_name: String,
    /// <p>The name of the Amazon EC2 security group to revoke access from.</p>
    pub ec2_security_group_name: String,
    /// <p>The AWS account number of the Amazon EC2 security group owner. Note that this is not the same thing as an AWS access key ID - you must provide a valid AWS account number for this parameter.</p>
    pub ec2_security_group_owner_id: String,
}

/// Serialize `RevokeCacheSecurityGroupIngressMessage` contents to a `SignedRequest`.
struct RevokeCacheSecurityGroupIngressMessageSerializer;
impl RevokeCacheSecurityGroupIngressMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RevokeCacheSecurityGroupIngressMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(
            &format!("{}{}", prefix, "CacheSecurityGroupName"),
            &obj.cache_security_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "EC2SecurityGroupName"),
            &obj.ec2_security_group_name,
        );
        params.put(
            &format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
            &obj.ec2_security_group_owner_id,
        );
    }
}

/// see [ElastiCache::revoke_cache_security_group_ingress]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct RevokeCacheSecurityGroupIngressResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

#[allow(dead_code)]
struct RevokeCacheSecurityGroupIngressResultDeserializer;
impl RevokeCacheSecurityGroupIngressResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<RevokeCacheSecurityGroupIngressResult, XmlParseError> {
        deserialize_elements::<_, RevokeCacheSecurityGroupIngressResult, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheSecurityGroup" => {
                        obj.cache_security_group =
                            Some(CacheSecurityGroupDeserializer::deserialize(
                                "CacheSecurityGroup",
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

/// Serialize `SecurityGroupIdsList` contents to a `SignedRequest`.
struct SecurityGroupIdsListSerializer;
impl SecurityGroupIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// <p>Represents a single cache security group and its status.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SecurityGroupMembership {
    /// <p>The identifier of the cache security group.</p>
    pub security_group_id: Option<String>,
    /// <p>The status of the cache security group membership. The status changes whenever a cache security group is modified, or when the cache security groups assigned to a cluster are modified.</p>
    pub status: Option<String>,
}

#[allow(dead_code)]
struct SecurityGroupMembershipDeserializer;
impl SecurityGroupMembershipDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SecurityGroupMembership, XmlParseError> {
        deserialize_elements::<_, SecurityGroupMembership, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "SecurityGroupId" => {
                        obj.security_group_id =
                            Some(StringDeserializer::deserialize("SecurityGroupId", stack)?);
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
#[allow(dead_code)]
struct SecurityGroupMembershipListDeserializer;
impl SecurityGroupMembershipListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<SecurityGroupMembership>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(SecurityGroupMembershipDeserializer::deserialize(
                    "member", stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>An update that you can apply to your Redis clusters.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceUpdate {
    /// <p>Indicates whether the service update will be automatically applied once the recommended apply-by date has expired. </p>
    pub auto_update_after_recommended_apply_by_date: Option<bool>,
    /// <p>The Elasticache engine to which the update applies. Either Redis or Memcached</p>
    pub engine: Option<String>,
    /// <p>The Elasticache engine version to which the update applies. Either Redis or Memcached engine version</p>
    pub engine_version: Option<String>,
    /// <p>The estimated length of time the service update will take</p>
    pub estimated_update_time: Option<String>,
    /// <p>Provides details of the service update</p>
    pub service_update_description: Option<String>,
    /// <p>The date after which the service update is no longer available</p>
    pub service_update_end_date: Option<String>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
    /// <p>The recommendend date to apply the service update in order to ensure compliance. For information on compliance, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/elasticache-compliance.html#elasticache-compliance-self-service">Self-Service Security Updates for Compliance</a>.</p>
    pub service_update_recommended_apply_by_date: Option<String>,
    /// <p>The date when the service update is initially available</p>
    pub service_update_release_date: Option<String>,
    /// <p>The severity of the service update</p>
    pub service_update_severity: Option<String>,
    /// <p>The status of the service update</p>
    pub service_update_status: Option<String>,
    /// <p>Reflects the nature of the service update</p>
    pub service_update_type: Option<String>,
}

#[allow(dead_code)]
struct ServiceUpdateDeserializer;
impl ServiceUpdateDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceUpdate, XmlParseError> {
        deserialize_elements::<_, ServiceUpdate, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "AutoUpdateAfterRecommendedApplyByDate" => {
                    obj.auto_update_after_recommended_apply_by_date =
                        Some(BooleanOptionalDeserializer::deserialize(
                            "AutoUpdateAfterRecommendedApplyByDate",
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
                "EstimatedUpdateTime" => {
                    obj.estimated_update_time = Some(StringDeserializer::deserialize(
                        "EstimatedUpdateTime",
                        stack,
                    )?);
                }
                "ServiceUpdateDescription" => {
                    obj.service_update_description = Some(StringDeserializer::deserialize(
                        "ServiceUpdateDescription",
                        stack,
                    )?);
                }
                "ServiceUpdateEndDate" => {
                    obj.service_update_end_date = Some(TStampDeserializer::deserialize(
                        "ServiceUpdateEndDate",
                        stack,
                    )?);
                }
                "ServiceUpdateName" => {
                    obj.service_update_name =
                        Some(StringDeserializer::deserialize("ServiceUpdateName", stack)?);
                }
                "ServiceUpdateRecommendedApplyByDate" => {
                    obj.service_update_recommended_apply_by_date =
                        Some(TStampDeserializer::deserialize(
                            "ServiceUpdateRecommendedApplyByDate",
                            stack,
                        )?);
                }
                "ServiceUpdateReleaseDate" => {
                    obj.service_update_release_date = Some(TStampDeserializer::deserialize(
                        "ServiceUpdateReleaseDate",
                        stack,
                    )?);
                }
                "ServiceUpdateSeverity" => {
                    obj.service_update_severity =
                        Some(ServiceUpdateSeverityDeserializer::deserialize(
                            "ServiceUpdateSeverity",
                            stack,
                        )?);
                }
                "ServiceUpdateStatus" => {
                    obj.service_update_status = Some(ServiceUpdateStatusDeserializer::deserialize(
                        "ServiceUpdateStatus",
                        stack,
                    )?);
                }
                "ServiceUpdateType" => {
                    obj.service_update_type = Some(ServiceUpdateTypeDeserializer::deserialize(
                        "ServiceUpdateType",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ServiceUpdateListDeserializer;
impl ServiceUpdateListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<ServiceUpdate>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "ServiceUpdate" {
                obj.push(ServiceUpdateDeserializer::deserialize(
                    "ServiceUpdate",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct ServiceUpdateSeverityDeserializer;
impl ServiceUpdateSeverityDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct ServiceUpdateStatusDeserializer;
impl ServiceUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

/// Serialize `ServiceUpdateStatusList` contents to a `SignedRequest`.
struct ServiceUpdateStatusListSerializer;
impl ServiceUpdateStatusListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct ServiceUpdateTypeDeserializer;
impl ServiceUpdateTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// see [ElastiCache::describe_service_updates]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct ServiceUpdatesMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>A list of service updates</p>
    pub service_updates: Option<Vec<ServiceUpdate>>,
}

impl ServiceUpdatesMessage {
    fn pagination_page_opt(self) -> Option<Vec<ServiceUpdate>> {
        Some(self.service_updates.as_ref()?.clone())
    }
}

impl PagedOutput for ServiceUpdatesMessage {
    type Item = ServiceUpdate;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ServiceUpdate> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct ServiceUpdatesMessageDeserializer;
impl ServiceUpdatesMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<ServiceUpdatesMessage, XmlParseError> {
        deserialize_elements::<_, ServiceUpdatesMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                "ServiceUpdates" => {
                    obj.service_updates.get_or_insert(vec![]).extend(
                        ServiceUpdateListDeserializer::deserialize("ServiceUpdates", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct SlaMetDeserializer;
impl SlaMetDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents the progress of an online resharding operation.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SlotMigration {
    /// <p>The percentage of the slot migration that is complete.</p>
    pub progress_percentage: Option<f64>,
}

#[allow(dead_code)]
struct SlotMigrationDeserializer;
impl SlotMigrationDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SlotMigration, XmlParseError> {
        deserialize_elements::<_, SlotMigration, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ProgressPercentage" => {
                    obj.progress_percentage = Some(DoubleDeserializer::deserialize(
                        "ProgressPercentage",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Represents a copy of an entire Redis cluster as of the time when the snapshot was taken.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Snapshot {
    /// <p>The ARN (Amazon Resource Name) of the snapshot.</p>
    pub arn: Option<String>,
    /// <p>This parameter is currently disabled.</p>
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>Indicates the status of automatic failover for the source Redis replication group.</p>
    pub automatic_failover: Option<String>,
    /// <p>The date and time when the source cluster was created.</p>
    pub cache_cluster_create_time: Option<String>,
    /// <p>The user-supplied identifier of the source cluster.</p>
    pub cache_cluster_id: Option<String>,
    /// <p><p>The name of the compute and memory capacity node type for the source cluster.</p> <p>The following node types are supported by ElastiCache. Generally speaking, the current generation types provide more memory and computational power at lower cost when compared to their equivalent previous generation counterparts.</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: </p> <p> <b>M6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.m6g.large</code>, <code>cache.m6g.xlarge</code>, <code>cache.m6g.2xlarge</code>, <code>cache.m6g.4xlarge</code>, <code>cache.m6g.8xlarge</code>, <code>cache.m6g.12xlarge</code>, <code>cache.m6g.16xlarge</code> </p> <note> <p>At this time, M6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>M5 node types:</b> <code>cache.m5.large</code>, <code>cache.m5.xlarge</code>, <code>cache.m5.2xlarge</code>, <code>cache.m5.4xlarge</code>, <code>cache.m5.12xlarge</code>, <code>cache.m5.24xlarge</code> </p> <p> <b>M4 node types:</b> <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> <p> <b>T3 node types:</b> <code>cache.t3.micro</code>, <code>cache.t3.small</code>, <code>cache.t3.medium</code> </p> <p> <b>T2 node types:</b> <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>T1 node types:</b> <code>cache.t1.micro</code> </p> <p> <b>M1 node types:</b> <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> <p> <b>M3 node types:</b> <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized:</p> <ul> <li> <p>Previous generation: (not recommended)</p> <p> <b>C1 node types:</b> <code>cache.c1.xlarge</code> </p> </li> </ul> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: </p> <p> <b>R6g node types</b> (available only for Redis engine version 5.0.6 onward and for Memcached engine version 1.5.16 onward).</p> <p> <code>cache.r6g.large</code>, <code>cache.r6g.xlarge</code>, <code>cache.r6g.2xlarge</code>, <code>cache.r6g.4xlarge</code>, <code>cache.r6g.8xlarge</code>, <code>cache.r6g.12xlarge</code>, <code>cache.r6g.16xlarge</code> </p> <note> <p>At this time, R6g node types are available in the following regions: us-east-1, us-west-2, us-east-2, eu-central-1, eu-west-1 and ap-northeast-1.</p> </note> <p> <b>R5 node types:</b> <code>cache.r5.large</code>, <code>cache.r5.xlarge</code>, <code>cache.r5.2xlarge</code>, <code>cache.r5.4xlarge</code>, <code>cache.r5.12xlarge</code>, <code>cache.r5.24xlarge</code> </p> <p> <b>R4 node types:</b> <code>cache.r4.large</code>, <code>cache.r4.xlarge</code>, <code>cache.r4.2xlarge</code>, <code>cache.r4.4xlarge</code>, <code>cache.r4.8xlarge</code>, <code>cache.r4.16xlarge</code> </p> </li> <li> <p>Previous generation: (not recommended)</p> <p> <b>M2 node types:</b> <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> <p> <b>R3 node types:</b> <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Additional node type info</b> </p> <ul> <li> <p>All current generation instance types are created in Amazon VPC by default.</p> </li> <li> <p>Redis append-only files (AOF) are not supported for T1 or T2 instances.</p> </li> <li> <p>Redis Multi-AZ with automatic failover is not supported on T1 instances.</p> </li> <li> <p>Redis configuration variables <code>appendonly</code> and <code>appendfsync</code> are not supported on Redis version 2.8.22 and later.</p> </li> </ul></p>
    pub cache_node_type: Option<String>,
    /// <p>The cache parameter group that is associated with the source cluster.</p>
    pub cache_parameter_group_name: Option<String>,
    /// <p>The name of the cache subnet group associated with the source cluster.</p>
    pub cache_subnet_group_name: Option<String>,
    /// <p>The name of the cache engine (<code>memcached</code> or <code>redis</code>) used by the source cluster.</p>
    pub engine: Option<String>,
    /// <p>The version of the cache engine version that is used by the source cluster.</p>
    pub engine_version: Option<String>,
    /// <p>The ID of the KMS key used to encrypt the snapshot.</p>
    pub kms_key_id: Option<String>,
    /// <p>A list of the cache nodes in the source cluster.</p>
    pub node_snapshots: Option<Vec<NodeSnapshot>>,
    /// <p>The number of cache nodes in the source cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>
    pub num_cache_nodes: Option<i64>,
    /// <p>The number of node groups (shards) in this snapshot. When restoring from a snapshot, the number of node groups (shards) in the snapshot and in the restored replication group must be the same.</p>
    pub num_node_groups: Option<i64>,
    /// <p>The port number used by each cache nodes in the source cluster.</p>
    pub port: Option<i64>,
    /// <p>The name of the Availability Zone in which the source cluster is located.</p>
    pub preferred_availability_zone: Option<String>,
    /// <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>
    pub preferred_maintenance_window: Option<String>,
    /// <p>The ARN (Amazon Resource Name) of the preferred outpost.</p>
    pub preferred_outpost_arn: Option<String>,
    /// <p>A description of the source replication group.</p>
    pub replication_group_description: Option<String>,
    /// <p>The unique identifier of the source replication group.</p>
    pub replication_group_id: Option<String>,
    /// <p>The name of a snapshot. For an automatic snapshot, the name is system-generated. For a manual snapshot, this is the user-provided name.</p>
    pub snapshot_name: Option<String>,
    /// <p>For an automatic snapshot, the number of days for which ElastiCache retains the snapshot before deleting it.</p> <p>For manual snapshots, this field reflects the <code>SnapshotRetentionLimit</code> for the source cluster when the snapshot was created. This field is otherwise ignored: Manual snapshots do not expire, and can only be deleted using the <code>DeleteSnapshot</code> operation. </p> <p> <b>Important</b> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p>
    pub snapshot_retention_limit: Option<i64>,
    /// <p>Indicates whether the snapshot is from an automatic backup (<code>automated</code>) or was created manually (<code>manual</code>).</p>
    pub snapshot_source: Option<String>,
    /// <p>The status of the snapshot. Valid values: <code>creating</code> | <code>available</code> | <code>restoring</code> | <code>copying</code> | <code>deleting</code>.</p>
    pub snapshot_status: Option<String>,
    /// <p>The daily time range during which ElastiCache takes daily snapshots of the source cluster.</p>
    pub snapshot_window: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the topic used by the source cluster for publishing notifications.</p>
    pub topic_arn: Option<String>,
    /// <p>The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet group for the source cluster.</p>
    pub vpc_id: Option<String>,
}

#[allow(dead_code)]
struct SnapshotDeserializer;
impl SnapshotDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Snapshot, XmlParseError> {
        deserialize_elements::<_, Snapshot, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "AutoMinorVersionUpgrade" => {
                    obj.auto_minor_version_upgrade = Some(BooleanDeserializer::deserialize(
                        "AutoMinorVersionUpgrade",
                        stack,
                    )?);
                }
                "AutomaticFailover" => {
                    obj.automatic_failover =
                        Some(AutomaticFailoverStatusDeserializer::deserialize(
                            "AutomaticFailover",
                            stack,
                        )?);
                }
                "CacheClusterCreateTime" => {
                    obj.cache_cluster_create_time = Some(TStampDeserializer::deserialize(
                        "CacheClusterCreateTime",
                        stack,
                    )?);
                }
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "CacheNodeType" => {
                    obj.cache_node_type =
                        Some(StringDeserializer::deserialize("CacheNodeType", stack)?);
                }
                "CacheParameterGroupName" => {
                    obj.cache_parameter_group_name = Some(StringDeserializer::deserialize(
                        "CacheParameterGroupName",
                        stack,
                    )?);
                }
                "CacheSubnetGroupName" => {
                    obj.cache_subnet_group_name = Some(StringDeserializer::deserialize(
                        "CacheSubnetGroupName",
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
                "NodeSnapshots" => {
                    obj.node_snapshots.get_or_insert(vec![]).extend(
                        NodeSnapshotListDeserializer::deserialize("NodeSnapshots", stack)?,
                    );
                }
                "NumCacheNodes" => {
                    obj.num_cache_nodes = Some(IntegerOptionalDeserializer::deserialize(
                        "NumCacheNodes",
                        stack,
                    )?);
                }
                "NumNodeGroups" => {
                    obj.num_node_groups = Some(IntegerOptionalDeserializer::deserialize(
                        "NumNodeGroups",
                        stack,
                    )?);
                }
                "Port" => {
                    obj.port = Some(IntegerOptionalDeserializer::deserialize("Port", stack)?);
                }
                "PreferredAvailabilityZone" => {
                    obj.preferred_availability_zone = Some(StringDeserializer::deserialize(
                        "PreferredAvailabilityZone",
                        stack,
                    )?);
                }
                "PreferredMaintenanceWindow" => {
                    obj.preferred_maintenance_window = Some(StringDeserializer::deserialize(
                        "PreferredMaintenanceWindow",
                        stack,
                    )?);
                }
                "PreferredOutpostArn" => {
                    obj.preferred_outpost_arn = Some(StringDeserializer::deserialize(
                        "PreferredOutpostArn",
                        stack,
                    )?);
                }
                "ReplicationGroupDescription" => {
                    obj.replication_group_description = Some(StringDeserializer::deserialize(
                        "ReplicationGroupDescription",
                        stack,
                    )?);
                }
                "ReplicationGroupId" => {
                    obj.replication_group_id = Some(StringDeserializer::deserialize(
                        "ReplicationGroupId",
                        stack,
                    )?);
                }
                "SnapshotName" => {
                    obj.snapshot_name =
                        Some(StringDeserializer::deserialize("SnapshotName", stack)?);
                }
                "SnapshotRetentionLimit" => {
                    obj.snapshot_retention_limit = Some(IntegerOptionalDeserializer::deserialize(
                        "SnapshotRetentionLimit",
                        stack,
                    )?);
                }
                "SnapshotSource" => {
                    obj.snapshot_source =
                        Some(StringDeserializer::deserialize("SnapshotSource", stack)?);
                }
                "SnapshotStatus" => {
                    obj.snapshot_status =
                        Some(StringDeserializer::deserialize("SnapshotStatus", stack)?);
                }
                "SnapshotWindow" => {
                    obj.snapshot_window =
                        Some(StringDeserializer::deserialize("SnapshotWindow", stack)?);
                }
                "TopicArn" => {
                    obj.topic_arn = Some(StringDeserializer::deserialize("TopicArn", stack)?);
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

/// Serialize `SnapshotArnsList` contents to a `SignedRequest`.
struct SnapshotArnsListSerializer;
impl SnapshotArnsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct SnapshotListDeserializer;
impl SnapshotListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<Snapshot>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "Snapshot" {
                obj.push(SnapshotDeserializer::deserialize("Snapshot", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct SourceTypeDeserializer;
impl SourceTypeDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// see [ElastiCache::start_migration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartMigrationMessage {
    /// <p>List of endpoints from which data should be migrated. For Redis (cluster mode disabled), list should have only one element.</p>
    pub customer_node_endpoint_list: Vec<CustomerNodeEndpoint>,
    /// <p>The ID of the replication group to which data should be migrated.</p>
    pub replication_group_id: String,
}

/// Serialize `StartMigrationMessage` contents to a `SignedRequest`.
struct StartMigrationMessageSerializer;
impl StartMigrationMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &StartMigrationMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        CustomerNodeEndpointListSerializer::serialize(
            params,
            &format!("{}{}", prefix, "CustomerNodeEndpointList"),
            &obj.customer_node_endpoint_list,
        );
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
    }
}

/// see [ElastiCache::start_migration]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct StartMigrationResponse {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct StartMigrationResponseDeserializer;
impl StartMigrationResponseDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<StartMigrationResponse, XmlParseError> {
        deserialize_elements::<_, StartMigrationResponse, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ReplicationGroup" => {
                    obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                        "ReplicationGroup",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct StringDeserializer;
impl StringDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>Represents the subnet associated with a cluster. This parameter refers to subnets defined in Amazon Virtual Private Cloud (Amazon VPC) and used with ElastiCache.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct Subnet {
    /// <p>The Availability Zone associated with the subnet.</p>
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>The unique identifier for the subnet.</p>
    pub subnet_identifier: Option<String>,
    /// <p>The outpost ARN of the subnet.</p>
    pub subnet_outpost: Option<SubnetOutpost>,
}

#[allow(dead_code)]
struct SubnetDeserializer;
impl SubnetDeserializer {
    #[allow(dead_code, unused_variables)]
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
                "SubnetOutpost" => {
                    obj.subnet_outpost = Some(SubnetOutpostDeserializer::deserialize(
                        "SubnetOutpost",
                        stack,
                    )?);
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

#[allow(dead_code)]
struct SubnetListDeserializer;
impl SubnetListDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>The ID of the outpost subnet.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct SubnetOutpost {
    /// <p>The outpost ARN of the subnet.</p>
    pub subnet_outpost_arn: Option<String>,
}

#[allow(dead_code)]
struct SubnetOutpostDeserializer;
impl SubnetOutpostDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<SubnetOutpost, XmlParseError> {
        deserialize_elements::<_, SubnetOutpost, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "SubnetOutpostArn" => {
                    obj.subnet_outpost_arn =
                        Some(StringDeserializer::deserialize("SubnetOutpostArn", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct TStampDeserializer;
impl TStampDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
/// <p>A cost allocation Tag that can be added to an ElastiCache cluster or replication group. Tags are composed of a Key/Value pair. A tag with a null Value is permitted.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The key for the tag. May not be null.</p>
    pub key: Option<String>,
    /// <p>The tag's value. May be null.</p>
    pub value: Option<String>,
}

#[allow(dead_code)]
struct TagDeserializer;
impl TagDeserializer {
    #[allow(dead_code, unused_variables)]
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

#[allow(dead_code)]
struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(dead_code, unused_variables)]
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

/// <p>Represents the output from the <code>AddTagsToResource</code>, <code>ListTagsForResource</code>, and <code>RemoveTagsFromResource</code> operations.</p>
/// see [ElastiCache::add_tags_to_resource]
/// see [ElastiCache::list_tags_for_resource]
/// see [ElastiCache::remove_tags_from_resource]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TagListMessage {
    /// <p>A list of cost allocation tags as key-value pairs.</p>
    pub tag_list: Option<Vec<Tag>>,
}

#[allow(dead_code)]
struct TagListMessageDeserializer;
impl TagListMessageDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// see [ElastiCache::test_failover]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestFailoverMessage {
    /// <p>The name of the node group (called shard in the console) in this replication group on which automatic failover is to be tested. You may test automatic failover on up to 5 node groups in any rolling 24-hour period.</p>
    pub node_group_id: String,
    /// <p>The name of the replication group (console: cluster) whose automatic failover is being tested by this operation.</p>
    pub replication_group_id: String,
}

/// Serialize `TestFailoverMessage` contents to a `SignedRequest`.
struct TestFailoverMessageSerializer;
impl TestFailoverMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TestFailoverMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "NodeGroupId"), &obj.node_group_id);
        params.put(
            &format!("{}{}", prefix, "ReplicationGroupId"),
            &obj.replication_group_id,
        );
    }
}

/// see [ElastiCache::test_failover]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct TestFailoverResult {
    pub replication_group: Option<ReplicationGroup>,
}

#[allow(dead_code)]
struct TestFailoverResultDeserializer;
impl TestFailoverResultDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<TestFailoverResult, XmlParseError> {
        deserialize_elements::<_, TestFailoverResult, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ReplicationGroup" => {
                    obj.replication_group = Some(ReplicationGroupDeserializer::deserialize(
                        "ReplicationGroup",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// <p>Filters update actions from the service updates that are in available status during the time range.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TimeRangeFilter {
    /// <p>The end time of the time range filter</p>
    pub end_time: Option<String>,
    /// <p>The start time of the time range filter</p>
    pub start_time: Option<String>,
}

/// Serialize `TimeRangeFilter` contents to a `SignedRequest`.
struct TimeRangeFilterSerializer;
impl TimeRangeFilterSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &TimeRangeFilter) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.start_time {
            params.put(&format!("{}{}", prefix, "StartTime"), &field_value);
        }
    }
}

#[allow(dead_code)]
struct UGReplicationGroupIdListDeserializer;
impl UGReplicationGroupIdListDeserializer {
    #[allow(dead_code, unused_variables)]
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
/// <p>Update action that has failed to be processed for the corresponding apply/stop request</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UnprocessedUpdateAction {
    /// <p>The ID of the cache cluster</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The error message that describes the reason the request was not processed</p>
    pub error_message: Option<String>,
    /// <p>The error type for requests that are not processed</p>
    pub error_type: Option<String>,
    /// <p>The replication group ID</p>
    pub replication_group_id: Option<String>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
}

#[allow(dead_code)]
struct UnprocessedUpdateActionDeserializer;
impl UnprocessedUpdateActionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UnprocessedUpdateAction, XmlParseError> {
        deserialize_elements::<_, UnprocessedUpdateAction, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "CacheClusterId" => {
                        obj.cache_cluster_id =
                            Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                    }
                    "ErrorMessage" => {
                        obj.error_message =
                            Some(StringDeserializer::deserialize("ErrorMessage", stack)?);
                    }
                    "ErrorType" => {
                        obj.error_type = Some(StringDeserializer::deserialize("ErrorType", stack)?);
                    }
                    "ReplicationGroupId" => {
                        obj.replication_group_id = Some(StringDeserializer::deserialize(
                            "ReplicationGroupId",
                            stack,
                        )?);
                    }
                    "ServiceUpdateName" => {
                        obj.service_update_name =
                            Some(StringDeserializer::deserialize("ServiceUpdateName", stack)?);
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
#[allow(dead_code)]
struct UnprocessedUpdateActionListDeserializer;
impl UnprocessedUpdateActionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<UnprocessedUpdateAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "UnprocessedUpdateAction" {
                obj.push(UnprocessedUpdateActionDeserializer::deserialize(
                    "UnprocessedUpdateAction",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>The status of the service update for a specific replication group</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateAction {
    /// <p>The ID of the cache cluster</p>
    pub cache_cluster_id: Option<String>,
    /// <p>The status of the service update on the cache node</p>
    pub cache_node_update_status: Option<Vec<CacheNodeUpdateStatus>>,
    /// <p>The Elasticache engine to which the update applies. Either Redis or Memcached</p>
    pub engine: Option<String>,
    /// <p>The estimated length of time for the update to complete</p>
    pub estimated_update_time: Option<String>,
    /// <p>The status of the service update on the node group</p>
    pub node_group_update_status: Option<Vec<NodeGroupUpdateStatus>>,
    /// <p>The progress of the service update on the replication group</p>
    pub nodes_updated: Option<String>,
    /// <p>The ID of the replication group</p>
    pub replication_group_id: Option<String>,
    /// <p>The unique ID of the service update</p>
    pub service_update_name: Option<String>,
    /// <p>The recommended date to apply the service update to ensure compliance. For information on compliance, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/elasticache-compliance.html#elasticache-compliance-self-service">Self-Service Security Updates for Compliance</a>.</p>
    pub service_update_recommended_apply_by_date: Option<String>,
    /// <p>The date the update is first available</p>
    pub service_update_release_date: Option<String>,
    /// <p>The severity of the service update</p>
    pub service_update_severity: Option<String>,
    /// <p>The status of the service update</p>
    pub service_update_status: Option<String>,
    /// <p>Reflects the nature of the service update </p>
    pub service_update_type: Option<String>,
    /// <p>If yes, all nodes in the replication group have been updated by the recommended apply-by date. If no, at least one node in the replication group have not been updated by the recommended apply-by date. If N/A, the replication group was created after the recommended apply-by date.</p>
    pub sla_met: Option<String>,
    /// <p>The date that the service update is available to a replication group</p>
    pub update_action_available_date: Option<String>,
    /// <p>The status of the update action</p>
    pub update_action_status: Option<String>,
    /// <p>The date when the UpdateActionStatus was last modified</p>
    pub update_action_status_modified_date: Option<String>,
}

#[allow(dead_code)]
struct UpdateActionDeserializer;
impl UpdateActionDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateAction, XmlParseError> {
        deserialize_elements::<_, UpdateAction, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "CacheClusterId" => {
                    obj.cache_cluster_id =
                        Some(StringDeserializer::deserialize("CacheClusterId", stack)?);
                }
                "CacheNodeUpdateStatus" => {
                    obj.cache_node_update_status.get_or_insert(vec![]).extend(
                        CacheNodeUpdateStatusListDeserializer::deserialize(
                            "CacheNodeUpdateStatus",
                            stack,
                        )?,
                    );
                }
                "Engine" => {
                    obj.engine = Some(StringDeserializer::deserialize("Engine", stack)?);
                }
                "EstimatedUpdateTime" => {
                    obj.estimated_update_time = Some(StringDeserializer::deserialize(
                        "EstimatedUpdateTime",
                        stack,
                    )?);
                }
                "NodeGroupUpdateStatus" => {
                    obj.node_group_update_status.get_or_insert(vec![]).extend(
                        NodeGroupUpdateStatusListDeserializer::deserialize(
                            "NodeGroupUpdateStatus",
                            stack,
                        )?,
                    );
                }
                "NodesUpdated" => {
                    obj.nodes_updated =
                        Some(StringDeserializer::deserialize("NodesUpdated", stack)?);
                }
                "ReplicationGroupId" => {
                    obj.replication_group_id = Some(StringDeserializer::deserialize(
                        "ReplicationGroupId",
                        stack,
                    )?);
                }
                "ServiceUpdateName" => {
                    obj.service_update_name =
                        Some(StringDeserializer::deserialize("ServiceUpdateName", stack)?);
                }
                "ServiceUpdateRecommendedApplyByDate" => {
                    obj.service_update_recommended_apply_by_date =
                        Some(TStampDeserializer::deserialize(
                            "ServiceUpdateRecommendedApplyByDate",
                            stack,
                        )?);
                }
                "ServiceUpdateReleaseDate" => {
                    obj.service_update_release_date = Some(TStampDeserializer::deserialize(
                        "ServiceUpdateReleaseDate",
                        stack,
                    )?);
                }
                "ServiceUpdateSeverity" => {
                    obj.service_update_severity =
                        Some(ServiceUpdateSeverityDeserializer::deserialize(
                            "ServiceUpdateSeverity",
                            stack,
                        )?);
                }
                "ServiceUpdateStatus" => {
                    obj.service_update_status = Some(ServiceUpdateStatusDeserializer::deserialize(
                        "ServiceUpdateStatus",
                        stack,
                    )?);
                }
                "ServiceUpdateType" => {
                    obj.service_update_type = Some(ServiceUpdateTypeDeserializer::deserialize(
                        "ServiceUpdateType",
                        stack,
                    )?);
                }
                "SlaMet" => {
                    obj.sla_met = Some(SlaMetDeserializer::deserialize("SlaMet", stack)?);
                }
                "UpdateActionAvailableDate" => {
                    obj.update_action_available_date = Some(TStampDeserializer::deserialize(
                        "UpdateActionAvailableDate",
                        stack,
                    )?);
                }
                "UpdateActionStatus" => {
                    obj.update_action_status = Some(UpdateActionStatusDeserializer::deserialize(
                        "UpdateActionStatus",
                        stack,
                    )?);
                }
                "UpdateActionStatusModifiedDate" => {
                    obj.update_action_status_modified_date = Some(TStampDeserializer::deserialize(
                        "UpdateActionStatusModifiedDate",
                        stack,
                    )?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct UpdateActionListDeserializer;
impl UpdateActionListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<UpdateAction>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "UpdateAction" {
                obj.push(UpdateActionDeserializer::deserialize(
                    "UpdateAction",
                    stack,
                )?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::batch_apply_update_action]
/// see [ElastiCache::batch_stop_update_action]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateActionResultsMessage {
    /// <p>Update actions that have been processed successfully</p>
    pub processed_update_actions: Option<Vec<ProcessedUpdateAction>>,
    /// <p>Update actions that haven't been processed successfully</p>
    pub unprocessed_update_actions: Option<Vec<UnprocessedUpdateAction>>,
}

#[allow(dead_code)]
struct UpdateActionResultsMessageDeserializer;
impl UpdateActionResultsMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateActionResultsMessage, XmlParseError> {
        deserialize_elements::<_, UpdateActionResultsMessage, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "ProcessedUpdateActions" => {
                        obj.processed_update_actions.get_or_insert(vec![]).extend(
                            ProcessedUpdateActionListDeserializer::deserialize(
                                "ProcessedUpdateActions",
                                stack,
                            )?,
                        );
                    }
                    "UnprocessedUpdateActions" => {
                        obj.unprocessed_update_actions.get_or_insert(vec![]).extend(
                            UnprocessedUpdateActionListDeserializer::deserialize(
                                "UnprocessedUpdateActions",
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
#[allow(dead_code)]
struct UpdateActionStatusDeserializer;
impl UpdateActionStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}

/// Serialize `UpdateActionStatusList` contents to a `SignedRequest`.
struct UpdateActionStatusListSerializer;
impl UpdateActionStatusListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// see [ElastiCache::describe_update_actions]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UpdateActionsMessage {
    /// <p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    pub marker: Option<String>,
    /// <p>Returns a list of update actions</p>
    pub update_actions: Option<Vec<UpdateAction>>,
}

impl UpdateActionsMessage {
    fn pagination_page_opt(self) -> Option<Vec<UpdateAction>> {
        Some(self.update_actions.as_ref()?.clone())
    }
}

impl PagedOutput for UpdateActionsMessage {
    type Item = UpdateAction;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<UpdateAction> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

#[allow(dead_code)]
struct UpdateActionsMessageDeserializer;
impl UpdateActionsMessageDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UpdateActionsMessage, XmlParseError> {
        deserialize_elements::<_, UpdateActionsMessage, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "Marker" => {
                    obj.marker = Some(StringDeserializer::deserialize("Marker", stack)?);
                }
                "UpdateActions" => {
                    obj.update_actions.get_or_insert(vec![]).extend(
                        UpdateActionListDeserializer::deserialize("UpdateActions", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::create_user]
/// see [ElastiCache::delete_user]
/// see [ElastiCache::modify_user]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct User {
    /// <p>The Amazon Resource Name (ARN) of the user account.</p>
    pub arn: Option<String>,
    /// <p>Access permissions string used for this user account.</p>
    pub access_string: Option<String>,
    /// <p>Denotes whether the user requires a password to authenticate.</p>
    pub authentication: Option<Authentication>,
    /// <p>Must be Redis. </p>
    pub engine: Option<String>,
    /// <p>Indicates the user status. Can be "active", "modifying" or "deleting".</p>
    pub status: Option<String>,
    /// <p>Returns a list of the user group IDs the user belongs to.</p>
    pub user_group_ids: Option<Vec<String>>,
    /// <p>The ID of the user.</p>
    pub user_id: Option<String>,
    /// <p>The username of the user.</p>
    pub user_name: Option<String>,
}

#[allow(dead_code)]
struct UserDeserializer;
impl UserDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<User, XmlParseError> {
        deserialize_elements::<_, User, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "AccessString" => {
                    obj.access_string =
                        Some(StringDeserializer::deserialize("AccessString", stack)?);
                }
                "Authentication" => {
                    obj.authentication = Some(AuthenticationDeserializer::deserialize(
                        "Authentication",
                        stack,
                    )?);
                }
                "Engine" => {
                    obj.engine = Some(EngineTypeDeserializer::deserialize("Engine", stack)?);
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "UserGroupIds" => {
                    obj.user_group_ids.get_or_insert(vec![]).extend(
                        UserGroupIdListDeserializer::deserialize("UserGroupIds", stack)?,
                    );
                }
                "UserId" => {
                    obj.user_id = Some(StringDeserializer::deserialize("UserId", stack)?);
                }
                "UserName" => {
                    obj.user_name = Some(StringDeserializer::deserialize("UserName", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
/// see [ElastiCache::create_user_group]
/// see [ElastiCache::delete_user_group]
/// see [ElastiCache::modify_user_group]
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UserGroup {
    /// <p>The Amazon Resource Name (ARN) of the user group.</p>
    pub arn: Option<String>,
    /// <p>Must be Redis. </p>
    pub engine: Option<String>,
    /// <p>A list of updates being applied to the user groups.</p>
    pub pending_changes: Option<UserGroupPendingChanges>,
    /// <p>A list of replication groups that the user group can access.</p>
    pub replication_groups: Option<Vec<String>>,
    /// <p>Indicates user group status. Can be "creating", "active", "modifying", "deleting".</p>
    pub status: Option<String>,
    /// <p>The ID of the user group.</p>
    pub user_group_id: Option<String>,
    /// <p>The list of user IDs that belong to the user group.</p>
    pub user_ids: Option<Vec<String>>,
}

#[allow(dead_code)]
struct UserGroupDeserializer;
impl UserGroupDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UserGroup, XmlParseError> {
        deserialize_elements::<_, UserGroup, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "ARN" => {
                    obj.arn = Some(StringDeserializer::deserialize("ARN", stack)?);
                }
                "Engine" => {
                    obj.engine = Some(EngineTypeDeserializer::deserialize("Engine", stack)?);
                }
                "PendingChanges" => {
                    obj.pending_changes = Some(UserGroupPendingChangesDeserializer::deserialize(
                        "PendingChanges",
                        stack,
                    )?);
                }
                "ReplicationGroups" => {
                    obj.replication_groups.get_or_insert(vec![]).extend(
                        UGReplicationGroupIdListDeserializer::deserialize(
                            "ReplicationGroups",
                            stack,
                        )?,
                    );
                }
                "Status" => {
                    obj.status = Some(StringDeserializer::deserialize("Status", stack)?);
                }
                "UserGroupId" => {
                    obj.user_group_id =
                        Some(StringDeserializer::deserialize("UserGroupId", stack)?);
                }
                "UserIds" => {
                    obj.user_ids
                        .get_or_insert(vec![])
                        .extend(UserIdListDeserializer::deserialize("UserIds", stack)?);
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct UserGroupIdDeserializer;
impl UserGroupIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct UserGroupIdListDeserializer;
impl UserGroupIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(UserGroupIdDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `UserGroupIdList` contents to a `SignedRequest`.
struct UserGroupIdListSerializer;
impl UserGroupIdListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

/// Serialize `UserGroupIdListInput` contents to a `SignedRequest`.
struct UserGroupIdListInputSerializer;
impl UserGroupIdListInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct UserGroupListDeserializer;
impl UserGroupListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<UserGroup>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(UserGroupDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}
/// <p>Returns the updates being applied to the user group.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UserGroupPendingChanges {
    /// <p>The list of user IDs to add.</p>
    pub user_ids_to_add: Option<Vec<String>>,
    /// <p>The list of user group IDs ro remove.</p>
    pub user_ids_to_remove: Option<Vec<String>>,
}

#[allow(dead_code)]
struct UserGroupPendingChangesDeserializer;
impl UserGroupPendingChangesDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UserGroupPendingChanges, XmlParseError> {
        deserialize_elements::<_, UserGroupPendingChanges, _>(
            tag_name,
            stack,
            |name, stack, obj| {
                match name {
                    "UserIdsToAdd" => {
                        obj.user_ids_to_add
                            .get_or_insert(vec![])
                            .extend(UserIdListDeserializer::deserialize("UserIdsToAdd", stack)?);
                    }
                    "UserIdsToRemove" => {
                        obj.user_ids_to_remove.get_or_insert(vec![]).extend(
                            UserIdListDeserializer::deserialize("UserIdsToRemove", stack)?,
                        );
                    }
                    _ => skip_tree(stack),
                }
                Ok(())
            },
        )
    }
}
/// <p>The status of the user group update.</p>
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serialize_structs", derive(Serialize))]
pub struct UserGroupsUpdateStatus {
    /// <p>The list of user group IDs to add.</p>
    pub user_group_ids_to_add: Option<Vec<String>>,
    /// <p>The list of user group IDs to remove.</p>
    pub user_group_ids_to_remove: Option<Vec<String>>,
}

#[allow(dead_code)]
struct UserGroupsUpdateStatusDeserializer;
impl UserGroupsUpdateStatusDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<UserGroupsUpdateStatus, XmlParseError> {
        deserialize_elements::<_, UserGroupsUpdateStatus, _>(tag_name, stack, |name, stack, obj| {
            match name {
                "UserGroupIdsToAdd" => {
                    obj.user_group_ids_to_add.get_or_insert(vec![]).extend(
                        UserGroupIdListDeserializer::deserialize("UserGroupIdsToAdd", stack)?,
                    );
                }
                "UserGroupIdsToRemove" => {
                    obj.user_group_ids_to_remove.get_or_insert(vec![]).extend(
                        UserGroupIdListDeserializer::deserialize("UserGroupIdsToRemove", stack)?,
                    );
                }
                _ => skip_tree(stack),
            }
            Ok(())
        })
    }
}
#[allow(dead_code)]
struct UserIdDeserializer;
impl UserIdDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(tag_name: &str, stack: &mut T) -> Result<String, XmlParseError> {
        xml_util::deserialize_primitive(tag_name, stack, Ok)
    }
}
#[allow(dead_code)]
struct UserIdListDeserializer;
impl UserIdListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<String>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(UserIdDeserializer::deserialize("member", stack)?);
            } else {
                skip_tree(stack);
            }
            Ok(())
        })
    }
}

/// Serialize `UserIdListInput` contents to a `SignedRequest`.
struct UserIdListInputSerializer;
impl UserIdListInputSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &Vec<String>) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[allow(dead_code)]
struct UserListDeserializer;
impl UserListDeserializer {
    #[allow(dead_code, unused_variables)]
    fn deserialize<T: Peek + Next>(
        tag_name: &str,
        stack: &mut T,
    ) -> Result<Vec<User>, XmlParseError> {
        deserialize_elements::<_, Vec<_>, _>(tag_name, stack, |name, stack, obj| {
            if name == "member" {
                obj.push(UserDeserializer::deserialize("member", stack)?);
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
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// <p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 50.</p>
    TagQuotaPerResourceExceeded(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            AddTagsToResourceError::CacheClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidARN" => {
                        return RusotoError::Service(AddTagsToResourceError::InvalidARNFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(AddTagsToResourceError::SnapshotNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "TagQuotaPerResourceExceeded" => {
                        return RusotoError::Service(
                            AddTagsToResourceError::TagQuotaPerResourceExceeded(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AddTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToResourceError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::InvalidARNFault(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
            AddTagsToResourceError::TagQuotaPerResourceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AddTagsToResourceError {}
/// Errors returned by AuthorizeCacheSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum AuthorizeCacheSecurityGroupIngressError {
    /// <p>The specified Amazon EC2 security group is already authorized for the specified cache security group.</p>
    AuthorizationAlreadyExistsFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl AuthorizeCacheSecurityGroupIngressError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AuthorizeCacheSecurityGroupIngressError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "AuthorizationAlreadyExists" => return RusotoError::Service(AuthorizeCacheSecurityGroupIngressError::AuthorizationAlreadyExistsFault(parsed_error.message)),"CacheSecurityGroupNotFound" => return RusotoError::Service(AuthorizeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(parsed_error.message)),"InvalidCacheSecurityGroupState" => return RusotoError::Service(AuthorizeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(AuthorizeCacheSecurityGroupIngressError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(AuthorizeCacheSecurityGroupIngressError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for AuthorizeCacheSecurityGroupIngressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AuthorizeCacheSecurityGroupIngressError::AuthorizationAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AuthorizeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            AuthorizeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            AuthorizeCacheSecurityGroupIngressError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            AuthorizeCacheSecurityGroupIngressError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AuthorizeCacheSecurityGroupIngressError {}
/// Errors returned by BatchApplyUpdateAction
#[derive(Debug, PartialEq)]
pub enum BatchApplyUpdateActionError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The service update doesn't exist</p>
    ServiceUpdateNotFoundFault(String),
}

impl BatchApplyUpdateActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchApplyUpdateActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            BatchApplyUpdateActionError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceUpdateNotFoundFault" => {
                        return RusotoError::Service(
                            BatchApplyUpdateActionError::ServiceUpdateNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for BatchApplyUpdateActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchApplyUpdateActionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            BatchApplyUpdateActionError::ServiceUpdateNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchApplyUpdateActionError {}
/// Errors returned by BatchStopUpdateAction
#[derive(Debug, PartialEq)]
pub enum BatchStopUpdateActionError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The service update doesn't exist</p>
    ServiceUpdateNotFoundFault(String),
}

impl BatchStopUpdateActionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchStopUpdateActionError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            BatchStopUpdateActionError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "ServiceUpdateNotFoundFault" => {
                        return RusotoError::Service(
                            BatchStopUpdateActionError::ServiceUpdateNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for BatchStopUpdateActionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchStopUpdateActionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            BatchStopUpdateActionError::ServiceUpdateNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for BatchStopUpdateActionError {}
/// Errors returned by CompleteMigration
#[derive(Debug, PartialEq)]
pub enum CompleteMigrationError {
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The designated replication group is not available for data migration.</p>
    ReplicationGroupNotUnderMigrationFault(String),
}

impl CompleteMigrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteMigrationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            CompleteMigrationError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CompleteMigrationError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotUnderMigrationFault" => {
                        return RusotoError::Service(
                            CompleteMigrationError::ReplicationGroupNotUnderMigrationFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CompleteMigrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteMigrationError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CompleteMigrationError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CompleteMigrationError::ReplicationGroupNotUnderMigrationFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CompleteMigrationError {}
/// Errors returned by CopySnapshot
#[derive(Debug, PartialEq)]
pub enum CopySnapshotError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The current state of the snapshot does not allow the requested operation to occur.</p>
    InvalidSnapshotStateFault(String),
    /// <p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl CopySnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CopySnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            CopySnapshotError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(CopySnapshotError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSnapshotState" => {
                        return RusotoError::Service(CopySnapshotError::InvalidSnapshotStateFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(CopySnapshotError::SnapshotAlreadyExistsFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(CopySnapshotError::SnapshotNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotQuotaExceededFault" => {
                        return RusotoError::Service(CopySnapshotError::SnapshotQuotaExceededFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CopySnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CopySnapshotError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::InvalidSnapshotStateFault(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::SnapshotAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
            CopySnapshotError::SnapshotQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CopySnapshotError {}
/// Errors returned by CreateCacheCluster
#[derive(Debug, PartialEq)]
pub enum CreateCacheClusterError {
    /// <p>You already have a cluster with the given identifier.</p>
    CacheClusterAlreadyExistsFault(String),
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 50.</p>
    TagQuotaPerResourceExceeded(String),
}

impl CreateCacheClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCacheClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterAlreadyExists" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::CacheClusterAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::CacheSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ClusterQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::ClusterQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCacheClusterCapacity" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::InsufficientCacheClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForClusterExceeded" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::NodeQuotaForClusterExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::NodeQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TagQuotaPerResourceExceeded" => {
                        return RusotoError::Service(
                            CreateCacheClusterError::TagQuotaPerResourceExceeded(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCacheClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCacheClusterError::CacheClusterAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::CacheSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::ClusterQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateCacheClusterError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::NodeQuotaForClusterExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheClusterError::TagQuotaPerResourceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCacheClusterError {}
/// Errors returned by CreateCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheParameterGroupError {
    /// <p>A cache parameter group with the requested name already exists.</p>
    CacheParameterGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum number of cache security groups.</p>
    CacheParameterGroupQuotaExceededFault(String),
    /// <p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl CreateCacheParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCacheParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateCacheParameterGroupError::CacheParameterGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheParameterGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateCacheParameterGroupError::CacheParameterGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheParameterGroupState" => {
                        return RusotoError::Service(
                            CreateCacheParameterGroupError::InvalidCacheParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            CreateCacheParameterGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            CreateCacheParameterGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCacheParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCacheParameterGroupError::CacheParameterGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheParameterGroupError::CacheParameterGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheParameterGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheParameterGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCacheParameterGroupError {}
/// Errors returned by CreateCacheSecurityGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheSecurityGroupError {
    /// <p>A cache security group with the specified name already exists.</p>
    CacheSecurityGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache security groups.</p>
    CacheSecurityGroupQuotaExceededFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl CreateCacheSecurityGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCacheSecurityGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateCacheSecurityGroupError::CacheSecurityGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "QuotaExceeded.CacheSecurityGroup" => {
                        return RusotoError::Service(
                            CreateCacheSecurityGroupError::CacheSecurityGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            CreateCacheSecurityGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            CreateCacheSecurityGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCacheSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCacheSecurityGroupError::CacheSecurityGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSecurityGroupError::CacheSecurityGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSecurityGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSecurityGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateCacheSecurityGroupError {}
/// Errors returned by CreateCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheSubnetGroupError {
    /// <p>The requested cache subnet group name is already in use by an existing cache subnet group.</p>
    CacheSubnetGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache subnet groups.</p>
    CacheSubnetGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a cache subnet group.</p>
    CacheSubnetQuotaExceededFault(String),
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    /// <p>At least one subnet ID does not match the other subnet IDs. This mismatch typically occurs when a user sets one subnet ID to a regional Availability Zone and a different one to an outpost. Or when a user sets the subnet ID to an Outpost when not subscribed on this service.</p>
    SubnetNotAllowedFault(String),
}

impl CreateCacheSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCacheSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateCacheSubnetGroupError::CacheSubnetGroupAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSubnetGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateCacheSubnetGroupError::CacheSubnetGroupQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSubnetQuotaExceededFault" => {
                        return RusotoError::Service(
                            CreateCacheSubnetGroupError::CacheSubnetQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(CreateCacheSubnetGroupError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "SubnetNotAllowedFault" => {
                        return RusotoError::Service(
                            CreateCacheSubnetGroupError::SubnetNotAllowedFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateCacheSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCacheSubnetGroupError::CacheSubnetGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSubnetGroupError::CacheSubnetGroupQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSubnetGroupError::CacheSubnetQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCacheSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateCacheSubnetGroupError::SubnetNotAllowedFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCacheSubnetGroupError {}
/// Errors returned by CreateGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum CreateGlobalReplicationGroupError {
    /// <p>The Global Datastore name already exists.</p>
    GlobalReplicationGroupAlreadyExistsFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The specified service linked role (SLR) was not found.</p>
    ServiceLinkedRoleNotFoundFault(String),
}

impl CreateGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "GlobalReplicationGroupAlreadyExistsFault" => return RusotoError::Service(
                        CreateGlobalReplicationGroupError::GlobalReplicationGroupAlreadyExistsFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            CreateGlobalReplicationGroupError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            CreateGlobalReplicationGroupError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateGlobalReplicationGroupError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleNotFoundFault" => {
                        return RusotoError::Service(
                            CreateGlobalReplicationGroupError::ServiceLinkedRoleNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateGlobalReplicationGroupError::GlobalReplicationGroupAlreadyExistsFault(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateGlobalReplicationGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGlobalReplicationGroupError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGlobalReplicationGroupError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateGlobalReplicationGroupError::ServiceLinkedRoleNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateGlobalReplicationGroupError {}
/// Errors returned by CreateReplicationGroup
#[derive(Debug, PartialEq)]
pub enum CreateReplicationGroupError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user group is not in an active state.</p>
    InvalidUserGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum allowed number of node groups (shards) in a single replication group. The default maximum is 90</p>
    NodeGroupsPerReplicationGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group already exists.</p>
    ReplicationGroupAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 50.</p>
    TagQuotaPerResourceExceeded(String),
    /// <p>The user group was not found or does not exist</p>
    UserGroupNotFoundFault(String),
}

impl CreateReplicationGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "CacheClusterNotFound" => return RusotoError::Service(CreateReplicationGroupError::CacheClusterNotFoundFault(parsed_error.message)),"CacheParameterGroupNotFound" => return RusotoError::Service(CreateReplicationGroupError::CacheParameterGroupNotFoundFault(parsed_error.message)),"CacheSecurityGroupNotFound" => return RusotoError::Service(CreateReplicationGroupError::CacheSecurityGroupNotFoundFault(parsed_error.message)),"CacheSubnetGroupNotFoundFault" => return RusotoError::Service(CreateReplicationGroupError::CacheSubnetGroupNotFoundFault(parsed_error.message)),"ClusterQuotaForCustomerExceeded" => return RusotoError::Service(CreateReplicationGroupError::ClusterQuotaForCustomerExceededFault(parsed_error.message)),"GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(CreateReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InsufficientCacheClusterCapacity" => return RusotoError::Service(CreateReplicationGroupError::InsufficientCacheClusterCapacityFault(parsed_error.message)),"InvalidCacheClusterState" => return RusotoError::Service(CreateReplicationGroupError::InvalidCacheClusterStateFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(CreateReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(CreateReplicationGroupError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(CreateReplicationGroupError::InvalidParameterValue(parsed_error.message)),"InvalidUserGroupState" => return RusotoError::Service(CreateReplicationGroupError::InvalidUserGroupStateFault(parsed_error.message)),"InvalidVPCNetworkStateFault" => return RusotoError::Service(CreateReplicationGroupError::InvalidVPCNetworkStateFault(parsed_error.message)),"NodeGroupsPerReplicationGroupQuotaExceeded" => return RusotoError::Service(CreateReplicationGroupError::NodeGroupsPerReplicationGroupQuotaExceededFault(parsed_error.message)),"NodeQuotaForClusterExceeded" => return RusotoError::Service(CreateReplicationGroupError::NodeQuotaForClusterExceededFault(parsed_error.message)),"NodeQuotaForCustomerExceeded" => return RusotoError::Service(CreateReplicationGroupError::NodeQuotaForCustomerExceededFault(parsed_error.message)),"ReplicationGroupAlreadyExists" => return RusotoError::Service(CreateReplicationGroupError::ReplicationGroupAlreadyExistsFault(parsed_error.message)),"TagQuotaPerResourceExceeded" => return RusotoError::Service(CreateReplicationGroupError::TagQuotaPerResourceExceeded(parsed_error.message)),"UserGroupNotFound" => return RusotoError::Service(CreateReplicationGroupError::UserGroupNotFoundFault(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReplicationGroupError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::CacheSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::ClusterQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateReplicationGroupError::InvalidUserGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::NodeGroupsPerReplicationGroupQuotaExceededFault(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateReplicationGroupError::NodeQuotaForClusterExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::ReplicationGroupAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::TagQuotaPerResourceExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationGroupError::UserGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateReplicationGroupError {}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    /// <p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl CreateSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            CreateSnapshotError::CacheClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            CreateSnapshotError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            CreateSnapshotError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(CreateSnapshotError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            CreateSnapshotError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            CreateSnapshotError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            CreateSnapshotError::SnapshotAlreadyExistsFault(parsed_error.message),
                        )
                    }
                    "SnapshotFeatureNotSupportedFault" => {
                        return RusotoError::Service(
                            CreateSnapshotError::SnapshotFeatureNotSupportedFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceededFault" => {
                        return RusotoError::Service(
                            CreateSnapshotError::SnapshotQuotaExceededFault(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateSnapshotError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidCacheClusterStateFault(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSnapshotError::ReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::SnapshotAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateSnapshotError::SnapshotFeatureNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateSnapshotError::SnapshotQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateSnapshotError {}
/// Errors returned by CreateUser
#[derive(Debug, PartialEq)]
pub enum CreateUserError {
    /// <p>A user with this username already exists.</p>
    DuplicateUserNameFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>A user with this ID already exists.</p>
    UserAlreadyExistsFault(String),
    /// <p>The quota of users has been exceeded.</p>
    UserQuotaExceededFault(String),
}

impl CreateUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DuplicateUserName" => {
                        return RusotoError::Service(CreateUserError::DuplicateUserNameFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(CreateUserError::InvalidParameterCombination(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(CreateUserError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "UserAlreadyExists" => {
                        return RusotoError::Service(CreateUserError::UserAlreadyExistsFault(
                            parsed_error.message,
                        ))
                    }
                    "UserQuotaExceeded" => {
                        return RusotoError::Service(CreateUserError::UserQuotaExceededFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserError::DuplicateUserNameFault(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            CreateUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateUserError::UserAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateUserError::UserQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserError {}
/// Errors returned by CreateUserGroup
#[derive(Debug, PartialEq)]
pub enum CreateUserGroupError {
    /// <p>You must add default user to a user group.</p>
    DefaultUserRequired(String),
    /// <p>A user with this username already exists.</p>
    DuplicateUserNameFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user group with this ID already exists.</p>
    UserGroupAlreadyExistsFault(String),
    /// <p>The number of users exceeds the user group limit.</p>
    UserGroupQuotaExceededFault(String),
    /// <p>The user does not exist or could not be found.</p>
    UserNotFoundFault(String),
}

impl CreateUserGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateUserGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DefaultUserRequired" => {
                        return RusotoError::Service(CreateUserGroupError::DefaultUserRequired(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateUserName" => {
                        return RusotoError::Service(CreateUserGroupError::DuplicateUserNameFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(CreateUserGroupError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "UserGroupAlreadyExists" => {
                        return RusotoError::Service(
                            CreateUserGroupError::UserGroupAlreadyExistsFault(parsed_error.message),
                        )
                    }
                    "UserGroupQuotaExceeded" => {
                        return RusotoError::Service(
                            CreateUserGroupError::UserGroupQuotaExceededFault(parsed_error.message),
                        )
                    }
                    "UserNotFound" => {
                        return RusotoError::Service(CreateUserGroupError::UserNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for CreateUserGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateUserGroupError::DefaultUserRequired(ref cause) => write!(f, "{}", cause),
            CreateUserGroupError::DuplicateUserNameFault(ref cause) => write!(f, "{}", cause),
            CreateUserGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateUserGroupError::UserGroupAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateUserGroupError::UserGroupQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateUserGroupError::UserNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateUserGroupError {}
/// Errors returned by DecreaseNodeGroupsInGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum DecreaseNodeGroupsInGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DecreaseNodeGroupsInGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DecreaseNodeGroupsInGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(DecreaseNodeGroupsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DecreaseNodeGroupsInGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            DecreaseNodeGroupsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => write!(f, "{}", cause),
DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
DecreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for DecreaseNodeGroupsInGlobalReplicationGroupError {}
/// Errors returned by DecreaseReplicaCount
#[derive(Debug, PartialEq)]
pub enum DecreaseReplicaCountError {
    /// <p>The request cannot be processed because it would exceed the allowed number of clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The operation was not performed because no changes were required.</p>
    NoOperationFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum allowed number of node groups (shards) in a single replication group. The default maximum is 90</p>
    NodeGroupsPerReplicationGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The specified service linked role (SLR) was not found.</p>
    ServiceLinkedRoleNotFoundFault(String),
}

impl DecreaseReplicaCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DecreaseReplicaCountError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ClusterQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::ClusterQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCacheClusterCapacity" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InsufficientCacheClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoOperationFault" => {
                        return RusotoError::Service(DecreaseReplicaCountError::NoOperationFault(
                            parsed_error.message,
                        ))
                    }
                    "NodeGroupsPerReplicationGroupQuotaExceeded" => return RusotoError::Service(
                        DecreaseReplicaCountError::NodeGroupsPerReplicationGroupQuotaExceededFault(
                            parsed_error.message,
                        ),
                    ),
                    "NodeQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::NodeQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceLinkedRoleNotFoundFault" => {
                        return RusotoError::Service(
                            DecreaseReplicaCountError::ServiceLinkedRoleNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DecreaseReplicaCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DecreaseReplicaCountError::ClusterQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DecreaseReplicaCountError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::NoOperationFault(ref cause) => write!(f, "{}", cause),
            DecreaseReplicaCountError::NodeGroupsPerReplicationGroupQuotaExceededFault(
                ref cause,
            ) => write!(f, "{}", cause),
            DecreaseReplicaCountError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DecreaseReplicaCountError::ServiceLinkedRoleNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DecreaseReplicaCountError {}
/// Errors returned by DeleteCacheCluster
#[derive(Debug, PartialEq)]
pub enum DeleteCacheClusterError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    /// <p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl DeleteCacheClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCacheClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "SnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::SnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotFeatureNotSupportedFault" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::SnapshotFeatureNotSupportedFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceededFault" => {
                        return RusotoError::Service(
                            DeleteCacheClusterError::SnapshotQuotaExceededFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCacheClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCacheClusterError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            DeleteCacheClusterError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheClusterError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheClusterError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteCacheClusterError::SnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheClusterError::SnapshotFeatureNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheClusterError::SnapshotQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCacheClusterError {}
/// Errors returned by DeleteCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheParameterGroupError {
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DeleteCacheParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCacheParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DeleteCacheParameterGroupError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheParameterGroupState" => {
                        return RusotoError::Service(
                            DeleteCacheParameterGroupError::InvalidCacheParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DeleteCacheParameterGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DeleteCacheParameterGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCacheParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheParameterGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheParameterGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCacheParameterGroupError {}
/// Errors returned by DeleteCacheSecurityGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheSecurityGroupError {
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DeleteCacheSecurityGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCacheSecurityGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            DeleteCacheSecurityGroupError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheSecurityGroupState" => {
                        return RusotoError::Service(
                            DeleteCacheSecurityGroupError::InvalidCacheSecurityGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DeleteCacheSecurityGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DeleteCacheSecurityGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCacheSecurityGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCacheSecurityGroupError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheSecurityGroupError::InvalidCacheSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheSecurityGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCacheSecurityGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCacheSecurityGroupError {}
/// Errors returned by DeleteCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheSubnetGroupError {
    /// <p>The requested cache subnet group is currently in use.</p>
    CacheSubnetGroupInUse(String),
    /// <p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
}

impl DeleteCacheSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCacheSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupInUse" => {
                        return RusotoError::Service(
                            DeleteCacheSubnetGroupError::CacheSubnetGroupInUse(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DeleteCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteCacheSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCacheSubnetGroupError::CacheSubnetGroupInUse(ref cause) => write!(f, "{}", cause),
            DeleteCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteCacheSubnetGroupError {}
/// Errors returned by DeleteGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum DeleteGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DeleteGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "GlobalReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DeleteGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidGlobalReplicationGroupState" => return RusotoError::Service(
                        DeleteGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DeleteGlobalReplicationGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            DeleteGlobalReplicationGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteGlobalReplicationGroupError {}
/// Errors returned by DeleteReplicationGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationGroupError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    /// <p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
}

impl DeleteReplicationGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotAlreadyExistsFault" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::SnapshotAlreadyExistsFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotFeatureNotSupportedFault" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::SnapshotFeatureNotSupportedFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "SnapshotQuotaExceededFault" => {
                        return RusotoError::Service(
                            DeleteReplicationGroupError::SnapshotQuotaExceededFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteReplicationGroupError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationGroupError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationGroupError::SnapshotAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationGroupError::SnapshotFeatureNotSupportedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationGroupError::SnapshotQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteReplicationGroupError {}
/// Errors returned by DeleteSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The current state of the snapshot does not allow the requested operation to occur.</p>
    InvalidSnapshotStateFault(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
}

impl DeleteSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteSnapshotError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DeleteSnapshotError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DeleteSnapshotError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidSnapshotState" => {
                        return RusotoError::Service(
                            DeleteSnapshotError::InvalidSnapshotStateFault(parsed_error.message),
                        )
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(DeleteSnapshotError::SnapshotNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteSnapshotError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::InvalidSnapshotStateFault(ref cause) => write!(f, "{}", cause),
            DeleteSnapshotError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteSnapshotError {}
/// Errors returned by DeleteUser
#[derive(Debug, PartialEq)]
pub enum DeleteUserError {
    /// <p><p/></p>
    DefaultUserAssociatedToUserGroupFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user is not in active state.</p>
    InvalidUserStateFault(String),
    /// <p>The user does not exist or could not be found.</p>
    UserNotFoundFault(String),
}

impl DeleteUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DefaultUserAssociatedToUserGroup" => {
                        return RusotoError::Service(
                            DeleteUserError::DefaultUserAssociatedToUserGroupFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DeleteUserError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidUserState" => {
                        return RusotoError::Service(DeleteUserError::InvalidUserStateFault(
                            parsed_error.message,
                        ))
                    }
                    "UserNotFound" => {
                        return RusotoError::Service(DeleteUserError::UserNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserError::DefaultUserAssociatedToUserGroupFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteUserError::InvalidUserStateFault(ref cause) => write!(f, "{}", cause),
            DeleteUserError::UserNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserError {}
/// Errors returned by DeleteUserGroup
#[derive(Debug, PartialEq)]
pub enum DeleteUserGroupError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user group is not in an active state.</p>
    InvalidUserGroupStateFault(String),
    /// <p>The user group was not found or does not exist</p>
    UserGroupNotFoundFault(String),
}

impl DeleteUserGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteUserGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DeleteUserGroupError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidUserGroupState" => {
                        return RusotoError::Service(
                            DeleteUserGroupError::InvalidUserGroupStateFault(parsed_error.message),
                        )
                    }
                    "UserGroupNotFound" => {
                        return RusotoError::Service(DeleteUserGroupError::UserGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DeleteUserGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteUserGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteUserGroupError::InvalidUserGroupStateFault(ref cause) => write!(f, "{}", cause),
            DeleteUserGroupError::UserGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteUserGroupError {}
/// Errors returned by DescribeCacheClusters
#[derive(Debug, PartialEq)]
pub enum DescribeCacheClustersError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeCacheClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCacheClustersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            DescribeCacheClustersError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeCacheClustersError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeCacheClustersError::InvalidParameterValue(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheClustersError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheClustersError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheClustersError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCacheClustersError {}
/// Errors returned by DescribeCacheEngineVersions
#[derive(Debug, PartialEq)]
pub enum DescribeCacheEngineVersionsError {}

impl DescribeCacheEngineVersionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCacheEngineVersionsError> {
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheEngineVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeCacheEngineVersionsError {}
/// Errors returned by DescribeCacheParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheParameterGroupsError {
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeCacheParameterGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCacheParameterGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeCacheParameterGroupsError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeCacheParameterGroupsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeCacheParameterGroupsError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheParameterGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheParameterGroupsError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheParameterGroupsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheParameterGroupsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCacheParameterGroupsError {}
/// Errors returned by DescribeCacheParameters
#[derive(Debug, PartialEq)]
pub enum DescribeCacheParametersError {
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeCacheParametersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCacheParametersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeCacheParametersError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeCacheParametersError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeCacheParametersError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheParametersError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheParametersError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheParametersError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCacheParametersError {}
/// Errors returned by DescribeCacheSecurityGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheSecurityGroupsError {
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeCacheSecurityGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCacheSecurityGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeCacheSecurityGroupsError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeCacheSecurityGroupsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeCacheSecurityGroupsError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheSecurityGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheSecurityGroupsError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheSecurityGroupsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeCacheSecurityGroupsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCacheSecurityGroupsError {}
/// Errors returned by DescribeCacheSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheSubnetGroupsError {
    /// <p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
}

impl DescribeCacheSubnetGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCacheSubnetGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeCacheSubnetGroupsError::CacheSubnetGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeCacheSubnetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCacheSubnetGroupsError::CacheSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeCacheSubnetGroupsError {}
/// Errors returned by DescribeEngineDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultParametersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

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
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeEngineDefaultParametersError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeEngineDefaultParametersError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEngineDefaultParametersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEngineDefaultParametersError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeEngineDefaultParametersError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeEngineDefaultParametersError {}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeEventsError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DescribeEventsError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventsError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            DescribeEventsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEventsError {}
/// Errors returned by DescribeGlobalReplicationGroups
#[derive(Debug, PartialEq)]
pub enum DescribeGlobalReplicationGroupsError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeGlobalReplicationGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeGlobalReplicationGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(
                        DescribeGlobalReplicationGroupsError::GlobalReplicationGroupNotFoundFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeGlobalReplicationGroupsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeGlobalReplicationGroupsError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeGlobalReplicationGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeGlobalReplicationGroupsError::GlobalReplicationGroupNotFoundFault(
                ref cause,
            ) => write!(f, "{}", cause),
            DescribeGlobalReplicationGroupsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeGlobalReplicationGroupsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeGlobalReplicationGroupsError {}
/// Errors returned by DescribeReplicationGroups
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationGroupsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
}

impl DescribeReplicationGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReplicationGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeReplicationGroupsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeReplicationGroupsError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeReplicationGroupsError::ReplicationGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeReplicationGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationGroupsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReplicationGroupsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReplicationGroupsError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationGroupsError {}
/// Errors returned by DescribeReservedCacheNodes
#[derive(Debug, PartialEq)]
pub enum DescribeReservedCacheNodesError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested reserved cache node was not found.</p>
    ReservedCacheNodeNotFoundFault(String),
}

impl DescribeReservedCacheNodesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReservedCacheNodesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeReservedCacheNodesError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeReservedCacheNodesError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReservedCacheNodeNotFound" => {
                        return RusotoError::Service(
                            DescribeReservedCacheNodesError::ReservedCacheNodeNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeReservedCacheNodesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservedCacheNodesError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedCacheNodesError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedCacheNodesError::ReservedCacheNodeNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReservedCacheNodesError {}
/// Errors returned by DescribeReservedCacheNodesOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeReservedCacheNodesOfferingsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested cache node offering does not exist.</p>
    ReservedCacheNodesOfferingNotFoundFault(String),
}

impl DescribeReservedCacheNodesOfferingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReservedCacheNodesOfferingsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "InvalidParameterCombination" => return RusotoError::Service(DescribeReservedCacheNodesOfferingsError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(DescribeReservedCacheNodesOfferingsError::InvalidParameterValue(parsed_error.message)),"ReservedCacheNodesOfferingNotFound" => return RusotoError::Service(DescribeReservedCacheNodesOfferingsError::ReservedCacheNodesOfferingNotFoundFault(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeReservedCacheNodesOfferingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReservedCacheNodesOfferingsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedCacheNodesOfferingsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReservedCacheNodesOfferingsError::ReservedCacheNodesOfferingNotFoundFault(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeReservedCacheNodesOfferingsError {}
/// Errors returned by DescribeServiceUpdates
#[derive(Debug, PartialEq)]
pub enum DescribeServiceUpdatesError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The service update doesn't exist</p>
    ServiceUpdateNotFoundFault(String),
}

impl DescribeServiceUpdatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeServiceUpdatesError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeServiceUpdatesError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeServiceUpdatesError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ServiceUpdateNotFoundFault" => {
                        return RusotoError::Service(
                            DescribeServiceUpdatesError::ServiceUpdateNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeServiceUpdatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeServiceUpdatesError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeServiceUpdatesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeServiceUpdatesError::ServiceUpdateNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeServiceUpdatesError {}
/// Errors returned by DescribeSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotsError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
}

impl DescribeSnapshotsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSnapshotsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            DescribeSnapshotsError::CacheClusterNotFoundFault(parsed_error.message),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeSnapshotsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(DescribeSnapshotsError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(DescribeSnapshotsError::SnapshotNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeSnapshotsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSnapshotsError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeSnapshotsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeSnapshotsError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSnapshotsError {}
/// Errors returned by DescribeUpdateActions
#[derive(Debug, PartialEq)]
pub enum DescribeUpdateActionsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DescribeUpdateActionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUpdateActionsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeUpdateActionsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            DescribeUpdateActionsError::InvalidParameterValue(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeUpdateActionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUpdateActionsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUpdateActionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUpdateActionsError {}
/// Errors returned by DescribeUserGroups
#[derive(Debug, PartialEq)]
pub enum DescribeUserGroupsError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The user group was not found or does not exist</p>
    UserGroupNotFoundFault(String),
}

impl DescribeUserGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUserGroupsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeUserGroupsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UserGroupNotFound" => {
                        return RusotoError::Service(
                            DescribeUserGroupsError::UserGroupNotFoundFault(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeUserGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUserGroupsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeUserGroupsError::UserGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUserGroupsError {}
/// Errors returned by DescribeUsers
#[derive(Debug, PartialEq)]
pub enum DescribeUsersError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The user does not exist or could not be found.</p>
    UserNotFoundFault(String),
}

impl DescribeUsersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeUsersError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            DescribeUsersError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "UserNotFound" => {
                        return RusotoError::Service(DescribeUsersError::UserNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DescribeUsersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeUsersError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            DescribeUsersError::UserNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeUsersError {}
/// Errors returned by DisassociateGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum DisassociateGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl DisassociateGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(DisassociateGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(DisassociateGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(DisassociateGlobalReplicationGroupError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(DisassociateGlobalReplicationGroupError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for DisassociateGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(
                ref cause,
            ) => write!(f, "{}", cause),
            DisassociateGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            DisassociateGlobalReplicationGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateGlobalReplicationGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateGlobalReplicationGroupError {}
/// Errors returned by FailoverGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum FailoverGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl FailoverGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<FailoverGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(FailoverGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(FailoverGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(FailoverGlobalReplicationGroupError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(FailoverGlobalReplicationGroupError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for FailoverGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FailoverGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            FailoverGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            FailoverGlobalReplicationGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            FailoverGlobalReplicationGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for FailoverGlobalReplicationGroupError {}
/// Errors returned by IncreaseNodeGroupsInGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum IncreaseNodeGroupsInGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl IncreaseNodeGroupsInGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<IncreaseNodeGroupsInGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(IncreaseNodeGroupsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(IncreaseNodeGroupsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(IncreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for IncreaseNodeGroupsInGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            IncreaseNodeGroupsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
IncreaseNodeGroupsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => write!(f, "{}", cause),
IncreaseNodeGroupsInGlobalReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for IncreaseNodeGroupsInGlobalReplicationGroupError {}
/// Errors returned by IncreaseReplicaCount
#[derive(Debug, PartialEq)]
pub enum IncreaseReplicaCountError {
    /// <p>The request cannot be processed because it would exceed the allowed number of clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The KMS key supplied is not valid.</p>
    InvalidKMSKeyFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The operation was not performed because no changes were required.</p>
    NoOperationFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum allowed number of node groups (shards) in a single replication group. The default maximum is 90</p>
    NodeGroupsPerReplicationGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
}

impl IncreaseReplicaCountError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<IncreaseReplicaCountError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "ClusterQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::ClusterQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCacheClusterCapacity" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InsufficientCacheClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidKMSKeyFault" => {
                        return RusotoError::Service(IncreaseReplicaCountError::InvalidKMSKeyFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NoOperationFault" => {
                        return RusotoError::Service(IncreaseReplicaCountError::NoOperationFault(
                            parsed_error.message,
                        ))
                    }
                    "NodeGroupsPerReplicationGroupQuotaExceeded" => return RusotoError::Service(
                        IncreaseReplicaCountError::NodeGroupsPerReplicationGroupQuotaExceededFault(
                            parsed_error.message,
                        ),
                    ),
                    "NodeQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::NodeQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            IncreaseReplicaCountError::ReplicationGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for IncreaseReplicaCountError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            IncreaseReplicaCountError::ClusterQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::InvalidKMSKeyFault(ref cause) => write!(f, "{}", cause),
            IncreaseReplicaCountError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            IncreaseReplicaCountError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::NoOperationFault(ref cause) => write!(f, "{}", cause),
            IncreaseReplicaCountError::NodeGroupsPerReplicationGroupQuotaExceededFault(
                ref cause,
            ) => write!(f, "{}", cause),
            IncreaseReplicaCountError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            IncreaseReplicaCountError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for IncreaseReplicaCountError {}
/// Errors returned by ListAllowedNodeTypeModifications
#[derive(Debug, PartialEq)]
pub enum ListAllowedNodeTypeModificationsError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
}

impl ListAllowedNodeTypeModificationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListAllowedNodeTypeModificationsError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            ListAllowedNodeTypeModificationsError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ListAllowedNodeTypeModificationsError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ListAllowedNodeTypeModificationsError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ListAllowedNodeTypeModificationsError::ReplicationGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListAllowedNodeTypeModificationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAllowedNodeTypeModificationsError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAllowedNodeTypeModificationsError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAllowedNodeTypeModificationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListAllowedNodeTypeModificationsError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListAllowedNodeTypeModificationsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidARN" => {
                        return RusotoError::Service(ListTagsForResourceError::InvalidARNFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            ListTagsForResourceError::SnapshotNotFoundFault(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ListTagsForResourceError::InvalidARNFault(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ModifyCacheCluster
#[derive(Debug, PartialEq)]
pub enum ModifyCacheClusterError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
}

impl ModifyCacheClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyCacheClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCacheClusterCapacity" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InsufficientCacheClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheSecurityGroupState" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InvalidCacheSecurityGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InvalidParameterValue(parsed_error.message),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForClusterExceeded" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::NodeQuotaForClusterExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            ModifyCacheClusterError::NodeQuotaForCustomerExceededFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyCacheClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyCacheClusterError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyCacheClusterError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::InvalidCacheSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ModifyCacheClusterError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::NodeQuotaForClusterExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheClusterError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyCacheClusterError {}
/// Errors returned by ModifyCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyCacheParameterGroupError {
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl ModifyCacheParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyCacheParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyCacheParameterGroupError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheParameterGroupState" => {
                        return RusotoError::Service(
                            ModifyCacheParameterGroupError::InvalidCacheParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidGlobalReplicationGroupState" => {
                        return RusotoError::Service(
                            ModifyCacheParameterGroupError::InvalidGlobalReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ModifyCacheParameterGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ModifyCacheParameterGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyCacheParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheParameterGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheParameterGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheParameterGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyCacheParameterGroupError {}
/// Errors returned by ModifyCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyCacheSubnetGroupError {
    /// <p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of subnets in a cache subnet group.</p>
    CacheSubnetQuotaExceededFault(String),
    /// <p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    /// <p>The requested subnet is being used by another cache subnet group.</p>
    SubnetInUse(String),
    /// <p>At least one subnet ID does not match the other subnet IDs. This mismatch typically occurs when a user sets one subnet ID to a regional Availability Zone and a different one to an outpost. Or when a user sets the subnet ID to an Outpost when not subscribed on this service.</p>
    SubnetNotAllowedFault(String),
}

impl ModifyCacheSubnetGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyCacheSubnetGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSubnetQuotaExceededFault" => {
                        return RusotoError::Service(
                            ModifyCacheSubnetGroupError::CacheSubnetQuotaExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidSubnet" => {
                        return RusotoError::Service(ModifyCacheSubnetGroupError::InvalidSubnet(
                            parsed_error.message,
                        ))
                    }
                    "SubnetInUse" => {
                        return RusotoError::Service(ModifyCacheSubnetGroupError::SubnetInUse(
                            parsed_error.message,
                        ))
                    }
                    "SubnetNotAllowedFault" => {
                        return RusotoError::Service(
                            ModifyCacheSubnetGroupError::SubnetNotAllowedFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyCacheSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheSubnetGroupError::CacheSubnetQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyCacheSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            ModifyCacheSubnetGroupError::SubnetInUse(ref cause) => write!(f, "{}", cause),
            ModifyCacheSubnetGroupError::SubnetNotAllowedFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyCacheSubnetGroupError {}
/// Errors returned by ModifyGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum ModifyGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl ModifyGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "GlobalReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidGlobalReplicationGroupState" => return RusotoError::Service(
                        ModifyGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ModifyGlobalReplicationGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            ModifyGlobalReplicationGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyGlobalReplicationGroupError {}
/// Errors returned by ModifyReplicationGroup
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationGroupError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    /// <p>The KMS key supplied is not valid.</p>
    InvalidKMSKeyFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The user group is not in an active state.</p>
    InvalidUserGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The user group was not found or does not exist</p>
    UserGroupNotFoundFault(String),
}

impl ModifyReplicationGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InsufficientCacheClusterCapacity" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InsufficientCacheClusterCapacityFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidCacheClusterStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheSecurityGroupState" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidCacheSecurityGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidKMSKeyFault" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidKMSKeyFault(parsed_error.message),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidParameterValue(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidUserGroupState" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidUserGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidVPCNetworkStateFault" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::InvalidVPCNetworkStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForClusterExceeded" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::NodeQuotaForClusterExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeQuotaForCustomerExceeded" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::NodeQuotaForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::ReplicationGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "UserGroupNotFound" => {
                        return RusotoError::Service(
                            ModifyReplicationGroupError::UserGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReplicationGroupError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InsufficientCacheClusterCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidCacheSecurityGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidKMSKeyFault(ref cause) => write!(f, "{}", cause),
            ModifyReplicationGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ModifyReplicationGroupError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidUserGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::InvalidVPCNetworkStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::NodeQuotaForClusterExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::NodeQuotaForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::ReplicationGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationGroupError::UserGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyReplicationGroupError {}
/// Errors returned by ModifyReplicationGroupShardConfiguration
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationGroupShardConfigurationError {
    /// <p>The requested cache node type is not available in the specified Availability Zone. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ErrorMessages.html#ErrorMessages.INSUFFICIENT_CACHE_CLUSTER_CAPACITY">InsufficientCacheClusterCapacity</a> in the ElastiCache User Guide.</p>
    InsufficientCacheClusterCapacityFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The KMS key supplied is not valid.</p>
    InvalidKMSKeyFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    /// <p>The request cannot be processed because it would exceed the maximum allowed number of node groups (shards) in a single replication group. The default maximum is 90</p>
    NodeGroupsPerReplicationGroupQuotaExceededFault(String),
    /// <p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
}

impl ModifyReplicationGroupShardConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyReplicationGroupShardConfigurationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "InsufficientCacheClusterCapacity" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InsufficientCacheClusterCapacityFault(parsed_error.message)),"InvalidCacheClusterState" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidCacheClusterStateFault(parsed_error.message)),"InvalidKMSKeyFault" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidKMSKeyFault(parsed_error.message)),"InvalidParameterCombination" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidParameterValue(parsed_error.message)),"InvalidReplicationGroupState" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidReplicationGroupStateFault(parsed_error.message)),"InvalidVPCNetworkStateFault" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::InvalidVPCNetworkStateFault(parsed_error.message)),"NodeGroupsPerReplicationGroupQuotaExceeded" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::NodeGroupsPerReplicationGroupQuotaExceededFault(parsed_error.message)),"NodeQuotaForCustomerExceeded" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::NodeQuotaForCustomerExceededFault(parsed_error.message)),"ReplicationGroupNotFoundFault" => return RusotoError::Service(ModifyReplicationGroupShardConfigurationError::ReplicationGroupNotFoundFault(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyReplicationGroupShardConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            ModifyReplicationGroupShardConfigurationError::InsufficientCacheClusterCapacityFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidCacheClusterStateFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidKMSKeyFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidReplicationGroupStateFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::InvalidVPCNetworkStateFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::NodeGroupsPerReplicationGroupQuotaExceededFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::NodeQuotaForCustomerExceededFault(ref cause) => write!(f, "{}", cause),
ModifyReplicationGroupShardConfigurationError::ReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for ModifyReplicationGroupShardConfigurationError {}
/// Errors returned by ModifyUser
#[derive(Debug, PartialEq)]
pub enum ModifyUserError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user is not in active state.</p>
    InvalidUserStateFault(String),
    /// <p>The user does not exist or could not be found.</p>
    UserNotFoundFault(String),
}

impl ModifyUserError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyUserError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(ModifyUserError::InvalidParameterCombination(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(ModifyUserError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidUserState" => {
                        return RusotoError::Service(ModifyUserError::InvalidUserStateFault(
                            parsed_error.message,
                        ))
                    }
                    "UserNotFound" => {
                        return RusotoError::Service(ModifyUserError::UserNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyUserError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyUserError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            ModifyUserError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ModifyUserError::InvalidUserStateFault(ref cause) => write!(f, "{}", cause),
            ModifyUserError::UserNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyUserError {}
/// Errors returned by ModifyUserGroup
#[derive(Debug, PartialEq)]
pub enum ModifyUserGroupError {
    /// <p>You must add default user to a user group.</p>
    DefaultUserRequired(String),
    /// <p>A user with this username already exists.</p>
    DuplicateUserNameFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The user group is not in an active state.</p>
    InvalidUserGroupStateFault(String),
    /// <p>The user group was not found or does not exist</p>
    UserGroupNotFoundFault(String),
    /// <p>The user does not exist or could not be found.</p>
    UserNotFoundFault(String),
}

impl ModifyUserGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyUserGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "DefaultUserRequired" => {
                        return RusotoError::Service(ModifyUserGroupError::DefaultUserRequired(
                            parsed_error.message,
                        ))
                    }
                    "DuplicateUserName" => {
                        return RusotoError::Service(ModifyUserGroupError::DuplicateUserNameFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ModifyUserGroupError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(ModifyUserGroupError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidUserGroupState" => {
                        return RusotoError::Service(
                            ModifyUserGroupError::InvalidUserGroupStateFault(parsed_error.message),
                        )
                    }
                    "UserGroupNotFound" => {
                        return RusotoError::Service(ModifyUserGroupError::UserGroupNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "UserNotFound" => {
                        return RusotoError::Service(ModifyUserGroupError::UserNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ModifyUserGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyUserGroupError::DefaultUserRequired(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::DuplicateUserNameFault(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::InvalidUserGroupStateFault(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::UserGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyUserGroupError::UserNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyUserGroupError {}
/// Errors returned by PurchaseReservedCacheNodesOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseReservedCacheNodesOfferingError {
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>You already have a reservation with the given identifier.</p>
    ReservedCacheNodeAlreadyExistsFault(String),
    /// <p>The request cannot be processed because it would exceed the user's cache node quota.</p>
    ReservedCacheNodeQuotaExceededFault(String),
    /// <p>The requested cache node offering does not exist.</p>
    ReservedCacheNodesOfferingNotFoundFault(String),
}

impl PurchaseReservedCacheNodesOfferingError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PurchaseReservedCacheNodesOfferingError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "InvalidParameterCombination" => return RusotoError::Service(PurchaseReservedCacheNodesOfferingError::InvalidParameterCombination(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(PurchaseReservedCacheNodesOfferingError::InvalidParameterValue(parsed_error.message)),"ReservedCacheNodeAlreadyExists" => return RusotoError::Service(PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeAlreadyExistsFault(parsed_error.message)),"ReservedCacheNodeQuotaExceeded" => return RusotoError::Service(PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeQuotaExceededFault(parsed_error.message)),"ReservedCacheNodesOfferingNotFound" => return RusotoError::Service(PurchaseReservedCacheNodesOfferingError::ReservedCacheNodesOfferingNotFoundFault(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for PurchaseReservedCacheNodesOfferingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PurchaseReservedCacheNodesOfferingError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseReservedCacheNodesOfferingError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeAlreadyExistsFault(
                ref cause,
            ) => write!(f, "{}", cause),
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeQuotaExceededFault(
                ref cause,
            ) => write!(f, "{}", cause),
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodesOfferingNotFoundFault(
                ref cause,
            ) => write!(f, "{}", cause),
        }
    }
}
impl Error for PurchaseReservedCacheNodesOfferingError {}
/// Errors returned by RebalanceSlotsInGlobalReplicationGroup
#[derive(Debug, PartialEq)]
pub enum RebalanceSlotsInGlobalReplicationGroupError {
    /// <p>The Global Datastore does not exist</p>
    GlobalReplicationGroupNotFoundFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl RebalanceSlotsInGlobalReplicationGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RebalanceSlotsInGlobalReplicationGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                                    "GlobalReplicationGroupNotFoundFault" => return RusotoError::Service(RebalanceSlotsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(parsed_error.message)),"InvalidGlobalReplicationGroupState" => return RusotoError::Service(RebalanceSlotsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(parsed_error.message)),"InvalidParameterValue" => return RusotoError::Service(RebalanceSlotsInGlobalReplicationGroupError::InvalidParameterValue(parsed_error.message)),_ => {}
                                }
            }
        }
        RusotoError::Unknown(res)
    }

    fn deserialize<T>(stack: &mut T) -> Result<XmlError, XmlParseError>
    where
        T: Peek + Next,
    {
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RebalanceSlotsInGlobalReplicationGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
                            RebalanceSlotsInGlobalReplicationGroupError::GlobalReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
RebalanceSlotsInGlobalReplicationGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => write!(f, "{}", cause),
RebalanceSlotsInGlobalReplicationGroupError::InvalidParameterValue(ref cause) => write!(f, "{}", cause)
                        }
    }
}
impl Error for RebalanceSlotsInGlobalReplicationGroupError {}
/// Errors returned by RebootCacheCluster
#[derive(Debug, PartialEq)]
pub enum RebootCacheClusterError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
}

impl RebootCacheClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootCacheClusterError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            RebootCacheClusterError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            RebootCacheClusterError::InvalidCacheClusterStateFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RebootCacheClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootCacheClusterError::CacheClusterNotFoundFault(ref cause) => write!(f, "{}", cause),
            RebootCacheClusterError::InvalidCacheClusterStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RebootCacheClusterError {}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>The requested cluster ID does not refer to an existing cluster.</p>
    CacheClusterNotFoundFault(String),
    /// <p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    /// <p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// <p>The requested tag was not found on this resource.</p>
    TagNotFoundFault(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheClusterNotFound" => {
                        return RusotoError::Service(
                            RemoveTagsFromResourceError::CacheClusterNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidARN" => {
                        return RusotoError::Service(RemoveTagsFromResourceError::InvalidARNFault(
                            parsed_error.message,
                        ))
                    }
                    "SnapshotNotFoundFault" => {
                        return RusotoError::Service(
                            RemoveTagsFromResourceError::SnapshotNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "TagNotFound" => {
                        return RusotoError::Service(RemoveTagsFromResourceError::TagNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromResourceError::CacheClusterNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveTagsFromResourceError::InvalidARNFault(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::SnapshotNotFoundFault(ref cause) => write!(f, "{}", cause),
            RemoveTagsFromResourceError::TagNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromResourceError {}
/// Errors returned by ResetCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetCacheParameterGroupError {
    /// <p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    /// <p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    /// <p>The Global Datastore is not available or in primary-only state.</p>
    InvalidGlobalReplicationGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl ResetCacheParameterGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ResetCacheParameterGroupError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFound" => {
                        return RusotoError::Service(
                            ResetCacheParameterGroupError::CacheParameterGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheParameterGroupState" => {
                        return RusotoError::Service(
                            ResetCacheParameterGroupError::InvalidCacheParameterGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidGlobalReplicationGroupState" => {
                        return RusotoError::Service(
                            ResetCacheParameterGroupError::InvalidGlobalReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            ResetCacheParameterGroupError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            ResetCacheParameterGroupError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for ResetCacheParameterGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ResetCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetCacheParameterGroupError::InvalidGlobalReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetCacheParameterGroupError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            ResetCacheParameterGroupError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ResetCacheParameterGroupError {}
/// Errors returned by RevokeCacheSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum RevokeCacheSecurityGroupIngressError {
    /// <p>The specified Amazon EC2 security group is not authorized for the specified cache security group.</p>
    AuthorizationNotFoundFault(String),
    /// <p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    /// <p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
}

impl RevokeCacheSecurityGroupIngressError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RevokeCacheSecurityGroupIngressError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "AuthorizationNotFound" => {
                        return RusotoError::Service(
                            RevokeCacheSecurityGroupIngressError::AuthorizationNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "CacheSecurityGroupNotFound" => {
                        return RusotoError::Service(
                            RevokeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheSecurityGroupState" => return RusotoError::Service(
                        RevokeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(
                            parsed_error.message,
                        ),
                    ),
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            RevokeCacheSecurityGroupIngressError::InvalidParameterCombination(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(
                            RevokeCacheSecurityGroupIngressError::InvalidParameterValue(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for RevokeCacheSecurityGroupIngressError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RevokeCacheSecurityGroupIngressError::AuthorizationNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RevokeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RevokeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(
                ref cause,
            ) => write!(f, "{}", cause),
            RevokeCacheSecurityGroupIngressError::InvalidParameterCombination(ref cause) => {
                write!(f, "{}", cause)
            }
            RevokeCacheSecurityGroupIngressError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RevokeCacheSecurityGroupIngressError {}
/// Errors returned by StartMigration
#[derive(Debug, PartialEq)]
pub enum StartMigrationError {
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The targeted replication group is not available. </p>
    ReplicationGroupAlreadyUnderMigrationFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
}

impl StartMigrationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartMigrationError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "InvalidParameterValue" => {
                        return RusotoError::Service(StartMigrationError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            StartMigrationError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupAlreadyUnderMigrationFault" => {
                        return RusotoError::Service(
                            StartMigrationError::ReplicationGroupAlreadyUnderMigrationFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            StartMigrationError::ReplicationGroupNotFoundFault(
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for StartMigrationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartMigrationError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartMigrationError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartMigrationError::ReplicationGroupAlreadyUnderMigrationFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartMigrationError::ReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartMigrationError {}
/// Errors returned by TestFailover
#[derive(Debug, PartialEq)]
pub enum TestFailoverError {
    /// <p>The customer has exceeded the allowed rate of API calls.</p>
    APICallRateForCustomerExceededFault(String),
    /// <p>The requested cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// <p>The KMS key supplied is not valid.</p>
    InvalidKMSKeyFault(String),
    /// <p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    /// <p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    /// <p>The node group specified by the <code>NodeGroupId</code> parameter could not be found. Please verify that the node group exists and that you spelled the <code>NodeGroupId</code> value correctly.</p>
    NodeGroupNotFoundFault(String),
    /// <p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// <p>The <code>TestFailover</code> action is not available.</p>
    TestFailoverNotAvailableFault(String),
}

impl TestFailoverError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestFailoverError> {
        {
            let reader = EventReader::new(res.body.as_ref());
            let mut stack = XmlResponse::new(reader.into_iter().peekable());
            find_start_element(&mut stack);
            if let Ok(parsed_error) = Self::deserialize(&mut stack) {
                match &parsed_error.code[..] {
                    "APICallRateForCustomerExceeded" => {
                        return RusotoError::Service(
                            TestFailoverError::APICallRateForCustomerExceededFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "InvalidCacheClusterState" => {
                        return RusotoError::Service(
                            TestFailoverError::InvalidCacheClusterStateFault(parsed_error.message),
                        )
                    }
                    "InvalidKMSKeyFault" => {
                        return RusotoError::Service(TestFailoverError::InvalidKMSKeyFault(
                            parsed_error.message,
                        ))
                    }
                    "InvalidParameterCombination" => {
                        return RusotoError::Service(
                            TestFailoverError::InvalidParameterCombination(parsed_error.message),
                        )
                    }
                    "InvalidParameterValue" => {
                        return RusotoError::Service(TestFailoverError::InvalidParameterValue(
                            parsed_error.message,
                        ))
                    }
                    "InvalidReplicationGroupState" => {
                        return RusotoError::Service(
                            TestFailoverError::InvalidReplicationGroupStateFault(
                                parsed_error.message,
                            ),
                        )
                    }
                    "NodeGroupNotFoundFault" => {
                        return RusotoError::Service(TestFailoverError::NodeGroupNotFoundFault(
                            parsed_error.message,
                        ))
                    }
                    "ReplicationGroupNotFoundFault" => {
                        return RusotoError::Service(
                            TestFailoverError::ReplicationGroupNotFoundFault(parsed_error.message),
                        )
                    }
                    "TestFailoverNotAvailableFault" => {
                        return RusotoError::Service(
                            TestFailoverError::TestFailoverNotAvailableFault(parsed_error.message),
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
        xml_util::start_element("ErrorResponse", stack)?;
        XmlErrorDeserializer::deserialize("Error", stack)
    }
}
impl fmt::Display for TestFailoverError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestFailoverError::APICallRateForCustomerExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            TestFailoverError::InvalidCacheClusterStateFault(ref cause) => write!(f, "{}", cause),
            TestFailoverError::InvalidKMSKeyFault(ref cause) => write!(f, "{}", cause),
            TestFailoverError::InvalidParameterCombination(ref cause) => write!(f, "{}", cause),
            TestFailoverError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            TestFailoverError::InvalidReplicationGroupStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            TestFailoverError::NodeGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            TestFailoverError::ReplicationGroupNotFoundFault(ref cause) => write!(f, "{}", cause),
            TestFailoverError::TestFailoverNotAvailableFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestFailoverError {}
/// Trait representing the capabilities of the Amazon ElastiCache API. Amazon ElastiCache clients implement this trait.
#[async_trait]
pub trait ElastiCache: Clone + Sync + Send + 'static {
    /// <p>Adds up to 50 cost allocation tags to the named resource. A cost allocation tag is a key-value pair where the key and value are case-sensitive. You can use cost allocation tags to categorize and track your AWS costs.</p> <p> When you apply tags to your ElastiCache resources, AWS generates a cost allocation report as a comma-separated value (CSV) file with your usage and costs aggregated by your tags. You can apply tags that represent business categories (such as cost centers, application names, or owners) to organize your costs across multiple services. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Tagging.html">Using Cost Allocation Tags in Amazon ElastiCache</a> in the <i>ElastiCache User Guide</i>.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<TagListMessage, RusotoError<AddTagsToResourceError>>;

    /// <p><p>Allows network ingress to a cache security group. Applications using ElastiCache must be running on Amazon EC2, and Amazon EC2 security groups are used as the authorization mechanism.</p> <note> <p>You cannot authorize ingress from an Amazon EC2 security group in one region to an ElastiCache cluster in another region.</p> </note></p>
    async fn authorize_cache_security_group_ingress(
        &self,
        input: AuthorizeCacheSecurityGroupIngressMessage,
    ) -> Result<
        AuthorizeCacheSecurityGroupIngressResult,
        RusotoError<AuthorizeCacheSecurityGroupIngressError>,
    >;

    /// <p>Apply the service update. For more information on service updates and applying them, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/applying-updates.html">Applying Service Updates</a>.</p>
    async fn batch_apply_update_action(
        &self,
        input: BatchApplyUpdateActionMessage,
    ) -> Result<UpdateActionResultsMessage, RusotoError<BatchApplyUpdateActionError>>;

    /// <p>Stop the service update. For more information on service updates and stopping them, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/stopping-self-service-updates.html">Stopping Service Updates</a>.</p>
    async fn batch_stop_update_action(
        &self,
        input: BatchStopUpdateActionMessage,
    ) -> Result<UpdateActionResultsMessage, RusotoError<BatchStopUpdateActionError>>;

    /// <p>Complete the migration of data.</p>
    async fn complete_migration(
        &self,
        input: CompleteMigrationMessage,
    ) -> Result<CompleteMigrationResponse, RusotoError<CompleteMigrationError>>;

    /// <p><p>Makes a copy of an existing snapshot.</p> <note> <p>This operation is valid for Redis only.</p> </note> <important> <p>Users or groups that have permissions to use the <code>CopySnapshot</code> operation can create their own Amazon S3 buckets and copy snapshots to it. To control access to your snapshots, use an IAM policy to control who has the ability to use the <code>CopySnapshot</code> operation. For more information about using IAM to control the use of ElastiCache operations, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html">Exporting Snapshots</a> and <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/IAM.html">Authentication &amp; Access Control</a>.</p> </important> <p>You could receive the following error messages.</p> <p class="title"> <b>Error Messages</b> </p> <ul> <li> <p> <b>Error Message:</b> The S3 bucket %s is outside of the region.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s does not exist.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s is not owned by the authenticated user.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The authenticated user does not have sufficient permissions to perform the desired activity.</p> <p> <b>Solution:</b> Contact your system administrator to get the needed permissions.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s already contains an object with key %s.</p> <p> <b>Solution:</b> Give the <code>TargetSnapshotName</code> a new and unique value. If exporting a snapshot, you could alternatively create a new Amazon S3 bucket and use this same value for <code>TargetSnapshotName</code>.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add List and Read permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted WRITE permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add Upload/Delete permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ_ACP permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add View Permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> </ul></p>
    async fn copy_snapshot(
        &self,
        input: CopySnapshotMessage,
    ) -> Result<CopySnapshotResult, RusotoError<CopySnapshotError>>;

    /// <p>Creates a cluster. All nodes in the cluster run the same protocol-compliant cache engine software, either Memcached or Redis.</p> <p>This operation is not supported for Redis (cluster mode enabled) clusters.</p>
    async fn create_cache_cluster(
        &self,
        input: CreateCacheClusterMessage,
    ) -> Result<CreateCacheClusterResult, RusotoError<CreateCacheClusterError>>;

    /// <p><p>Creates a new Amazon ElastiCache cache parameter group. An ElastiCache cache parameter group is a collection of parameters and their values that are applied to all of the nodes in any cluster or replication group using the CacheParameterGroup.</p> <p>A newly created CacheParameterGroup is an exact duplicate of the default parameter group for the CacheParameterGroupFamily. To customize the newly created CacheParameterGroup you can change the values of specific parameters. For more information, see:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyCacheParameterGroup.html">ModifyCacheParameterGroup</a> in the ElastiCache API Reference.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ParameterGroups.html">Parameters and Parameter Groups</a> in the ElastiCache User Guide.</p> </li> </ul></p>
    async fn create_cache_parameter_group(
        &self,
        input: CreateCacheParameterGroupMessage,
    ) -> Result<CreateCacheParameterGroupResult, RusotoError<CreateCacheParameterGroupError>>;

    /// <p>Creates a new cache security group. Use a cache security group to control access to one or more clusters.</p> <p>Cache security groups are only used when you are creating a cluster outside of an Amazon Virtual Private Cloud (Amazon VPC). If you are creating a cluster inside of a VPC, use a cache subnet group instead. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateCacheSubnetGroup.html">CreateCacheSubnetGroup</a>.</p>
    async fn create_cache_security_group(
        &self,
        input: CreateCacheSecurityGroupMessage,
    ) -> Result<CreateCacheSecurityGroupResult, RusotoError<CreateCacheSecurityGroupError>>;

    /// <p>Creates a new cache subnet group.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    async fn create_cache_subnet_group(
        &self,
        input: CreateCacheSubnetGroupMessage,
    ) -> Result<CreateCacheSubnetGroupResult, RusotoError<CreateCacheSubnetGroupError>>;

    /// <p><p>Global Datastore for Redis offers fully managed, fast, reliable and secure cross-region replication. Using Global Datastore for Redis, you can create cross-region read replica clusters for ElastiCache for Redis to enable low-latency reads and disaster recovery across regions. For more information, see <a href="/AmazonElastiCache/latest/red-ug/Redis-Global-Datastores.html">Replication Across Regions Using Global Datastore</a>. </p> <ul> <li> <p>The <b>GlobalReplicationGroupIdSuffix</b> is the name of the Global Datastore.</p> </li> <li> <p>The <b>PrimaryReplicationGroupId</b> represents the name of the primary cluster that accepts writes and will replicate updates to the secondary cluster.</p> </li> </ul></p>
    async fn create_global_replication_group(
        &self,
        input: CreateGlobalReplicationGroupMessage,
    ) -> Result<CreateGlobalReplicationGroupResult, RusotoError<CreateGlobalReplicationGroupError>>;

    /// <p><p>Creates a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group.</p> <p>This API can be used to create a standalone regional replication group or a secondary replication group associated with a Global Datastore.</p> <p>A Redis (cluster mode disabled) replication group is a collection of clusters, where one of the clusters is a read/write primary and the others are read-only replicas. Writes to the primary are asynchronously propagated to the replicas.</p> <p>A Redis (cluster mode enabled) replication group is a collection of 1 to 90 node groups (shards). Each node group (shard) has one read/write primary node and up to 5 read-only replica nodes. Writes to the primary are asynchronously propagated to the replicas. Redis (cluster mode enabled) replication groups partition the data across node groups (shards).</p> <p>When a Redis (cluster mode disabled) replication group has been successfully created, you can add one or more read replicas to it, up to a total of 5 read replicas. If you need to increase or decrease the number of node groups (console: shards), you can avail yourself of ElastiCache for Redis&#39; scaling. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Scaling.html">Scaling ElastiCache for Redis Clusters</a> in the <i>ElastiCache User Guide</i>.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn create_replication_group(
        &self,
        input: CreateReplicationGroupMessage,
    ) -> Result<CreateReplicationGroupResult, RusotoError<CreateReplicationGroupError>>;

    /// <p><p>Creates a copy of an entire cluster or replication group at a specific moment in time.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotMessage,
    ) -> Result<CreateSnapshotResult, RusotoError<CreateSnapshotError>>;

    /// <p>For Redis engine version 6.x onwards: Creates a Redis user. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>.</p>
    async fn create_user(
        &self,
        input: CreateUserMessage,
    ) -> Result<User, RusotoError<CreateUserError>>;

    /// <p>For Redis engine version 6.x onwards: Creates a Redis user group. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a> </p>
    async fn create_user_group(
        &self,
        input: CreateUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<CreateUserGroupError>>;

    /// <p>Decreases the number of node groups in a Global Datastore</p>
    async fn decrease_node_groups_in_global_replication_group(
        &self,
        input: DecreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) -> Result<
        DecreaseNodeGroupsInGlobalReplicationGroupResult,
        RusotoError<DecreaseNodeGroupsInGlobalReplicationGroupError>,
    >;

    /// <p>Dynamically decreases the number of replicas in a Redis (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Redis (cluster mode enabled) replication group. This operation is performed with no cluster down time.</p>
    async fn decrease_replica_count(
        &self,
        input: DecreaseReplicaCountMessage,
    ) -> Result<DecreaseReplicaCountResult, RusotoError<DecreaseReplicaCountError>>;

    /// <p><p>Deletes a previously provisioned cluster. <code>DeleteCacheCluster</code> deletes all associated cache nodes, node endpoints and the cluster itself. When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the cluster; you cannot cancel or revert this operation.</p> <p>This operation is not valid for:</p> <ul> <li> <p>Redis (cluster mode enabled) clusters</p> </li> <li> <p>A cluster that is the last read replica of a replication group</p> </li> <li> <p>A node group (shard) that has Multi-AZ mode enabled</p> </li> <li> <p>A cluster from a Redis (cluster mode enabled) replication group</p> </li> <li> <p>A cluster that is not in the <code>available</code> state</p> </li> </ul></p>
    async fn delete_cache_cluster(
        &self,
        input: DeleteCacheClusterMessage,
    ) -> Result<DeleteCacheClusterResult, RusotoError<DeleteCacheClusterError>>;

    /// <p>Deletes the specified cache parameter group. You cannot delete a cache parameter group if it is associated with any cache clusters.</p>
    async fn delete_cache_parameter_group(
        &self,
        input: DeleteCacheParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheParameterGroupError>>;

    /// <p><p>Deletes a cache security group.</p> <note> <p>You cannot delete a cache security group if it is associated with any clusters.</p> </note></p>
    async fn delete_cache_security_group(
        &self,
        input: DeleteCacheSecurityGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheSecurityGroupError>>;

    /// <p><p>Deletes a cache subnet group.</p> <note> <p>You cannot delete a cache subnet group if it is associated with any clusters.</p> </note></p>
    async fn delete_cache_subnet_group(
        &self,
        input: DeleteCacheSubnetGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheSubnetGroupError>>;

    /// <p>Deleting a Global Datastore is a two-step process: </p> <ul> <li> <p>First, you must <a>DisassociateGlobalReplicationGroup</a> to remove the secondary clusters in the Global Datastore.</p> </li> <li> <p>Once the Global Datastore contains only the primary cluster, you can use DeleteGlobalReplicationGroup API to delete the Global Datastore while retainining the primary cluster using Retain…= true.</p> </li> </ul> <p>Since the Global Datastore has only a primary cluster, you can delete the Global Datastore while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p>
    async fn delete_global_replication_group(
        &self,
        input: DeleteGlobalReplicationGroupMessage,
    ) -> Result<DeleteGlobalReplicationGroupResult, RusotoError<DeleteGlobalReplicationGroupError>>;

    /// <p><p>Deletes an existing replication group. By default, this operation deletes the entire replication group, including the primary/primaries and all of the read replicas. If the replication group has only one primary, you can optionally delete only the read replicas, while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn delete_replication_group(
        &self,
        input: DeleteReplicationGroupMessage,
    ) -> Result<DeleteReplicationGroupResult, RusotoError<DeleteReplicationGroupError>>;

    /// <p><p>Deletes an existing snapshot. When you receive a successful response from this operation, ElastiCache immediately begins deleting the snapshot; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn delete_snapshot(
        &self,
        input: DeleteSnapshotMessage,
    ) -> Result<DeleteSnapshotResult, RusotoError<DeleteSnapshotError>>;

    /// <p>For Redis engine version 6.x onwards: Deletes a user. The user will be removed from all user groups and in turn removed from all replication groups. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>. </p>
    async fn delete_user(
        &self,
        input: DeleteUserMessage,
    ) -> Result<User, RusotoError<DeleteUserError>>;

    /// <p>For Redis engine version 6.x onwards: Deletes a ser group. The user group must first be disassociated from the replcation group before it can be deleted. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>. </p>
    async fn delete_user_group(
        &self,
        input: DeleteUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<DeleteUserGroupError>>;

    /// <p>Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cache cluster if a cluster identifier is supplied.</p> <p>By default, abbreviated information about the clusters is returned. You can use the optional <i>ShowCacheNodeInfo</i> flag to retrieve detailed information about the cache nodes associated with the clusters. These details include the DNS address and port for the cache node endpoint.</p> <p>If the cluster is in the <i>creating</i> state, only cluster-level information is displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the <i>deleting</i> state, only cluster-level information is displayed.</p> <p>If cache nodes are currently being added to the cluster, node endpoint information and creation time for the additional nodes are not displayed until they are completely provisioned. When the cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If cache nodes are currently being removed from the cluster, no endpoint information for the removed nodes is displayed.</p>
    async fn describe_cache_clusters(
        &self,
        input: DescribeCacheClustersMessage,
    ) -> Result<CacheClusterMessage, RusotoError<DescribeCacheClustersError>>;

    /// Auto-paginating version of `describe_cache_clusters`
    fn describe_cache_clusters_pages(
        &self,
        input: DescribeCacheClustersMessage,
    ) -> RusotoStream<CacheCluster, DescribeCacheClustersError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_clusters(state.clone())
        })
    }

    /// <p>Returns a list of the available cache engines and their versions.</p>
    async fn describe_cache_engine_versions(
        &self,
        input: DescribeCacheEngineVersionsMessage,
    ) -> Result<CacheEngineVersionMessage, RusotoError<DescribeCacheEngineVersionsError>>;

    /// Auto-paginating version of `describe_cache_engine_versions`
    fn describe_cache_engine_versions_pages(
        &self,
        input: DescribeCacheEngineVersionsMessage,
    ) -> RusotoStream<CacheEngineVersion, DescribeCacheEngineVersionsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_engine_versions(state.clone())
        })
    }

    /// <p>Returns a list of cache parameter group descriptions. If a cache parameter group name is specified, the list contains only the descriptions for that group.</p>
    async fn describe_cache_parameter_groups(
        &self,
        input: DescribeCacheParameterGroupsMessage,
    ) -> Result<CacheParameterGroupsMessage, RusotoError<DescribeCacheParameterGroupsError>>;

    /// Auto-paginating version of `describe_cache_parameter_groups`
    fn describe_cache_parameter_groups_pages(
        &self,
        input: DescribeCacheParameterGroupsMessage,
    ) -> RusotoStream<CacheParameterGroup, DescribeCacheParameterGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_parameter_groups(state.clone())
        })
    }

    /// <p>Returns the detailed parameter list for a particular cache parameter group.</p>
    async fn describe_cache_parameters(
        &self,
        input: DescribeCacheParametersMessage,
    ) -> Result<CacheParameterGroupDetails, RusotoError<DescribeCacheParametersError>>;

    /// Auto-paginating version of `describe_cache_parameters`
    fn describe_cache_parameters_pages(
        &self,
        input: DescribeCacheParametersMessage,
    ) -> RusotoStream<Parameter, DescribeCacheParametersError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_parameters(state.clone())
        })
    }

    /// <p>Returns a list of cache security group descriptions. If a cache security group name is specified, the list contains only the description of that group. This applicable only when you have ElastiCache in Classic setup </p>
    async fn describe_cache_security_groups(
        &self,
        input: DescribeCacheSecurityGroupsMessage,
    ) -> Result<CacheSecurityGroupMessage, RusotoError<DescribeCacheSecurityGroupsError>>;

    /// Auto-paginating version of `describe_cache_security_groups`
    fn describe_cache_security_groups_pages(
        &self,
        input: DescribeCacheSecurityGroupsMessage,
    ) -> RusotoStream<CacheSecurityGroup, DescribeCacheSecurityGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_security_groups(state.clone())
        })
    }

    /// <p>Returns a list of cache subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group. This is applicable only when you have ElastiCache in VPC setup. All ElastiCache clusters now launch in VPC by default. </p>
    async fn describe_cache_subnet_groups(
        &self,
        input: DescribeCacheSubnetGroupsMessage,
    ) -> Result<CacheSubnetGroupMessage, RusotoError<DescribeCacheSubnetGroupsError>>;

    /// Auto-paginating version of `describe_cache_subnet_groups`
    fn describe_cache_subnet_groups_pages(
        &self,
        input: DescribeCacheSubnetGroupsMessage,
    ) -> RusotoStream<CacheSubnetGroup, DescribeCacheSubnetGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_cache_subnet_groups(state.clone())
        })
    }

    /// <p>Returns the default engine and system parameter information for the specified cache engine.</p>
    async fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> Result<
        DescribeEngineDefaultParametersResult,
        RusotoError<DescribeEngineDefaultParametersError>,
    >;

    /// Auto-paginating version of `describe_engine_default_parameters`
    fn describe_engine_default_parameters_pages(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> RusotoStream<Parameter, DescribeEngineDefaultParametersError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_engine_default_parameters(state.clone())
        })
    }

    /// <p>Returns events related to clusters, cache security groups, and cache parameter groups. You can obtain events specific to a particular cluster, cache security group, or cache parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<EventsMessage, RusotoError<DescribeEventsError>>;

    /// Auto-paginating version of `describe_events`
    fn describe_events_pages(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoStream<Event, DescribeEventsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_events(state.clone())
        })
    }

    /// <p>Returns information about a particular global replication group. If no identifier is specified, returns information about all Global Datastores. </p>
    async fn describe_global_replication_groups(
        &self,
        input: DescribeGlobalReplicationGroupsMessage,
    ) -> Result<
        DescribeGlobalReplicationGroupsResult,
        RusotoError<DescribeGlobalReplicationGroupsError>,
    >;

    /// Auto-paginating version of `describe_global_replication_groups`
    fn describe_global_replication_groups_pages(
        &self,
        input: DescribeGlobalReplicationGroupsMessage,
    ) -> RusotoStream<GlobalReplicationGroup, DescribeGlobalReplicationGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_global_replication_groups(state.clone())
        })
    }

    /// <p><p>Returns information about a particular replication group. If no identifier is specified, <code>DescribeReplicationGroups</code> returns information about all replication groups.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn describe_replication_groups(
        &self,
        input: DescribeReplicationGroupsMessage,
    ) -> Result<ReplicationGroupMessage, RusotoError<DescribeReplicationGroupsError>>;

    /// Auto-paginating version of `describe_replication_groups`
    fn describe_replication_groups_pages(
        &self,
        input: DescribeReplicationGroupsMessage,
    ) -> RusotoStream<ReplicationGroup, DescribeReplicationGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_replication_groups(state.clone())
        })
    }

    /// <p>Returns information about reserved cache nodes for this account, or about a specified reserved cache node.</p>
    async fn describe_reserved_cache_nodes(
        &self,
        input: DescribeReservedCacheNodesMessage,
    ) -> Result<ReservedCacheNodeMessage, RusotoError<DescribeReservedCacheNodesError>>;

    /// Auto-paginating version of `describe_reserved_cache_nodes`
    fn describe_reserved_cache_nodes_pages(
        &self,
        input: DescribeReservedCacheNodesMessage,
    ) -> RusotoStream<ReservedCacheNode, DescribeReservedCacheNodesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_reserved_cache_nodes(state.clone())
        })
    }

    /// <p>Lists available reserved cache node offerings.</p>
    async fn describe_reserved_cache_nodes_offerings(
        &self,
        input: DescribeReservedCacheNodesOfferingsMessage,
    ) -> Result<
        ReservedCacheNodesOfferingMessage,
        RusotoError<DescribeReservedCacheNodesOfferingsError>,
    >;

    /// Auto-paginating version of `describe_reserved_cache_nodes_offerings`
    fn describe_reserved_cache_nodes_offerings_pages(
        &self,
        input: DescribeReservedCacheNodesOfferingsMessage,
    ) -> RusotoStream<ReservedCacheNodesOffering, DescribeReservedCacheNodesOfferingsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_reserved_cache_nodes_offerings(state.clone())
        })
    }

    /// <p>Returns details of the service updates</p>
    async fn describe_service_updates(
        &self,
        input: DescribeServiceUpdatesMessage,
    ) -> Result<ServiceUpdatesMessage, RusotoError<DescribeServiceUpdatesError>>;

    /// Auto-paginating version of `describe_service_updates`
    fn describe_service_updates_pages(
        &self,
        input: DescribeServiceUpdatesMessage,
    ) -> RusotoStream<ServiceUpdate, DescribeServiceUpdatesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_service_updates(state.clone())
        })
    }

    /// <p><p>Returns information about cluster or replication group snapshots. By default, <code>DescribeSnapshots</code> lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a particular cache cluster.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn describe_snapshots(
        &self,
        input: DescribeSnapshotsMessage,
    ) -> Result<DescribeSnapshotsListMessage, RusotoError<DescribeSnapshotsError>>;

    /// Auto-paginating version of `describe_snapshots`
    fn describe_snapshots_pages(
        &self,
        input: DescribeSnapshotsMessage,
    ) -> RusotoStream<Snapshot, DescribeSnapshotsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_snapshots(state.clone())
        })
    }

    /// <p>Returns details of the update actions </p>
    async fn describe_update_actions(
        &self,
        input: DescribeUpdateActionsMessage,
    ) -> Result<UpdateActionsMessage, RusotoError<DescribeUpdateActionsError>>;

    /// Auto-paginating version of `describe_update_actions`
    fn describe_update_actions_pages(
        &self,
        input: DescribeUpdateActionsMessage,
    ) -> RusotoStream<UpdateAction, DescribeUpdateActionsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_update_actions(state.clone())
        })
    }

    /// <p>Returns a list of user groups.</p>
    async fn describe_user_groups(
        &self,
        input: DescribeUserGroupsMessage,
    ) -> Result<DescribeUserGroupsResult, RusotoError<DescribeUserGroupsError>>;

    /// Auto-paginating version of `describe_user_groups`
    fn describe_user_groups_pages(
        &self,
        input: DescribeUserGroupsMessage,
    ) -> RusotoStream<UserGroup, DescribeUserGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_user_groups(state.clone())
        })
    }

    /// <p>Returns a list of users.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersMessage,
    ) -> Result<DescribeUsersResult, RusotoError<DescribeUsersError>>;

    /// Auto-paginating version of `describe_users`
    fn describe_users_pages(
        &self,
        input: DescribeUsersMessage,
    ) -> RusotoStream<User, DescribeUsersError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_users(state.clone())
        })
    }

    /// <p>Remove a secondary cluster from the Global Datastore using the Global Datastore name. The secondary cluster will no longer receive updates from the primary cluster, but will remain as a standalone cluster in that AWS region.</p>
    async fn disassociate_global_replication_group(
        &self,
        input: DisassociateGlobalReplicationGroupMessage,
    ) -> Result<
        DisassociateGlobalReplicationGroupResult,
        RusotoError<DisassociateGlobalReplicationGroupError>,
    >;

    /// <p>Used to failover the primary region to a selected secondary region. The selected secondary region will become primary, and all other clusters will become secondary.</p>
    async fn failover_global_replication_group(
        &self,
        input: FailoverGlobalReplicationGroupMessage,
    ) -> Result<
        FailoverGlobalReplicationGroupResult,
        RusotoError<FailoverGlobalReplicationGroupError>,
    >;

    /// <p>Increase the number of node groups in the Global Datastore</p>
    async fn increase_node_groups_in_global_replication_group(
        &self,
        input: IncreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) -> Result<
        IncreaseNodeGroupsInGlobalReplicationGroupResult,
        RusotoError<IncreaseNodeGroupsInGlobalReplicationGroupError>,
    >;

    /// <p>Dynamically increases the number of replics in a Redis (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Redis (cluster mode enabled) replication group. This operation is performed with no cluster down time.</p>
    async fn increase_replica_count(
        &self,
        input: IncreaseReplicaCountMessage,
    ) -> Result<IncreaseReplicaCountResult, RusotoError<IncreaseReplicaCountError>>;

    /// <p>Lists all available node types that you can scale your Redis cluster's or replication group's current node type.</p> <p>When you use the <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code> operations to scale your cluster or replication group, the value of the <code>CacheNodeType</code> parameter must be one of the node types returned by this operation.</p>
    async fn list_allowed_node_type_modifications(
        &self,
        input: ListAllowedNodeTypeModificationsMessage,
    ) -> Result<
        AllowedNodeTypeModificationsMessage,
        RusotoError<ListAllowedNodeTypeModificationsError>,
    >;

    /// <p>Lists all cost allocation tags currently on the named resource. A <code>cost allocation tag</code> is a key-value pair where the key is case-sensitive and the value is optional. You can use cost allocation tags to categorize and track your AWS costs.</p> <p>If the cluster is not in the <i>available</i> state, <code>ListTagsForResource</code> returns an error.</p> <p>You can have a maximum of 50 cost allocation tags on an ElastiCache resource. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Tagging.html">Monitoring Costs with Tags</a>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<TagListMessage, RusotoError<ListTagsForResourceError>>;

    /// <p>Modifies the settings for a cluster. You can use this operation to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    async fn modify_cache_cluster(
        &self,
        input: ModifyCacheClusterMessage,
    ) -> Result<ModifyCacheClusterResult, RusotoError<ModifyCacheClusterError>>;

    /// <p>Modifies the parameters of a cache parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    async fn modify_cache_parameter_group(
        &self,
        input: ModifyCacheParameterGroupMessage,
    ) -> Result<CacheParameterGroupNameMessage, RusotoError<ModifyCacheParameterGroupError>>;

    /// <p>Modifies an existing cache subnet group.</p>
    async fn modify_cache_subnet_group(
        &self,
        input: ModifyCacheSubnetGroupMessage,
    ) -> Result<ModifyCacheSubnetGroupResult, RusotoError<ModifyCacheSubnetGroupError>>;

    /// <p>Modifies the settings for a Global Datastore.</p>
    async fn modify_global_replication_group(
        &self,
        input: ModifyGlobalReplicationGroupMessage,
    ) -> Result<ModifyGlobalReplicationGroupResult, RusotoError<ModifyGlobalReplicationGroupError>>;

    /// <p><p>Modifies the settings for a replication group.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/scaling-redis-cluster-mode-enabled.html">Scaling for Amazon ElastiCache for Redis (cluster mode enabled)</a> in the ElastiCache User Guide</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyReplicationGroupShardConfiguration.html">ModifyReplicationGroupShardConfiguration</a> in the ElastiCache API Reference</p> </li> </ul> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn modify_replication_group(
        &self,
        input: ModifyReplicationGroupMessage,
    ) -> Result<ModifyReplicationGroupResult, RusotoError<ModifyReplicationGroupError>>;

    /// <p>Modifies a replication group's shards (node groups) by allowing you to add shards, remove shards, or rebalance the keyspaces among exisiting shards.</p>
    async fn modify_replication_group_shard_configuration(
        &self,
        input: ModifyReplicationGroupShardConfigurationMessage,
    ) -> Result<
        ModifyReplicationGroupShardConfigurationResult,
        RusotoError<ModifyReplicationGroupShardConfigurationError>,
    >;

    /// <p>Changes user password(s) and/or access string.</p>
    async fn modify_user(
        &self,
        input: ModifyUserMessage,
    ) -> Result<User, RusotoError<ModifyUserError>>;

    /// <p>Changes the list of users that belong to the user group.</p>
    async fn modify_user_group(
        &self,
        input: ModifyUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<ModifyUserGroupError>>;

    /// <p>Allows you to purchase a reserved cache node offering.</p>
    async fn purchase_reserved_cache_nodes_offering(
        &self,
        input: PurchaseReservedCacheNodesOfferingMessage,
    ) -> Result<
        PurchaseReservedCacheNodesOfferingResult,
        RusotoError<PurchaseReservedCacheNodesOfferingError>,
    >;

    /// <p>Redistribute slots to ensure uniform distribution across existing shards in the cluster.</p>
    async fn rebalance_slots_in_global_replication_group(
        &self,
        input: RebalanceSlotsInGlobalReplicationGroupMessage,
    ) -> Result<
        RebalanceSlotsInGlobalReplicationGroupResult,
        RusotoError<RebalanceSlotsInGlobalReplicationGroupError>,
    >;

    /// <p>Reboots some, or all, of the cache nodes within a provisioned cluster. This operation applies any modified cache parameter groups to the cluster. The reboot operation takes place as soon as possible, and results in a momentary outage to the cluster. During the reboot, the cluster status is set to REBOOTING.</p> <p>The reboot causes the contents of the cache (for each cache node being rebooted) to be lost.</p> <p>When the reboot is complete, a cluster event is created.</p> <p>Rebooting a cluster is currently supported on Memcached and Redis (cluster mode disabled) clusters. Rebooting is not supported on Redis (cluster mode enabled) clusters.</p> <p>If you make changes to parameters that require a Redis (cluster mode enabled) cluster reboot for the changes to be applied, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Rebooting.html">Rebooting a Cluster</a> for an alternate process.</p>
    async fn reboot_cache_cluster(
        &self,
        input: RebootCacheClusterMessage,
    ) -> Result<RebootCacheClusterResult, RusotoError<RebootCacheClusterError>>;

    /// <p>Removes the tags identified by the <code>TagKeys</code> list from the named resource.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<TagListMessage, RusotoError<RemoveTagsFromResourceError>>;

    /// <p>Modifies the parameters of a cache parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire cache parameter group, specify the <code>ResetAllParameters</code> and <code>CacheParameterGroupName</code> parameters.</p>
    async fn reset_cache_parameter_group(
        &self,
        input: ResetCacheParameterGroupMessage,
    ) -> Result<CacheParameterGroupNameMessage, RusotoError<ResetCacheParameterGroupError>>;

    /// <p>Revokes ingress from a cache security group. Use this operation to disallow access from an Amazon EC2 security group that had been previously authorized.</p>
    async fn revoke_cache_security_group_ingress(
        &self,
        input: RevokeCacheSecurityGroupIngressMessage,
    ) -> Result<
        RevokeCacheSecurityGroupIngressResult,
        RusotoError<RevokeCacheSecurityGroupIngressError>,
    >;

    /// <p>Start the migration of data.</p>
    async fn start_migration(
        &self,
        input: StartMigrationMessage,
    ) -> Result<StartMigrationResponse, RusotoError<StartMigrationError>>;

    /// <p>Represents the input of a <code>TestFailover</code> operation which test automatic failover on a specified node group (called shard in the console) in a replication group (called cluster in the console).</p> <p class="title"> <b>Note the following</b> </p> <ul> <li> <p>A customer can use this operation to test automatic failover on up to 5 shards (called node groups in the ElastiCache API and AWS CLI) in any rolling 24-hour period.</p> </li> <li> <p>If calling this operation on shards in different clusters (called replication groups in the API and CLI), the calls can be made concurrently.</p> <p> </p> </li> <li> <p>If calling this operation multiple times on different shards in the same Redis (cluster mode enabled) replication group, the first node replacement must complete before a subsequent call can be made.</p> </li> <li> <p>To determine whether the node replacement is complete you can check Events using the Amazon ElastiCache console, the AWS CLI, or the ElastiCache API. Look for the following automatic failover related events, listed here in order of occurrance:</p> <ol> <li> <p>Replication group message: <code>Test Failover API called for node group &lt;node-group-id&gt;</code> </p> </li> <li> <p>Cache cluster message: <code>Failover from primary node &lt;primary-node-id&gt; to replica node &lt;node-id&gt; completed</code> </p> </li> <li> <p>Replication group message: <code>Failover from primary node &lt;primary-node-id&gt; to replica node &lt;node-id&gt; completed</code> </p> </li> <li> <p>Cache cluster message: <code>Recovering cache nodes &lt;node-id&gt;</code> </p> </li> <li> <p>Cache cluster message: <code>Finished recovery for cache nodes &lt;node-id&gt;</code> </p> </li> </ol> <p>For more information see:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ECEvents.Viewing.html">Viewing ElastiCache Events</a> in the <i>ElastiCache User Guide</i> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a> in the ElastiCache API Reference</p> </li> </ul> </li> </ul> <p>Also see, <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/AutoFailover.html#auto-failover-test">Testing Multi-AZ </a> in the <i>ElastiCache User Guide</i>.</p>
    async fn test_failover(
        &self,
        input: TestFailoverMessage,
    ) -> Result<TestFailoverResult, RusotoError<TestFailoverError>>;
}
/// A client for the Amazon ElastiCache API.
#[derive(Clone)]
pub struct ElastiCacheClient {
    client: Client,
    region: region::Region,
}

impl ElastiCacheClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ElastiCacheClient {
        ElastiCacheClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ElastiCacheClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        ElastiCacheClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> ElastiCacheClient {
        ElastiCacheClient { client, region }
    }
}

#[async_trait]
impl ElastiCache for ElastiCacheClient {
    /// <p>Adds up to 50 cost allocation tags to the named resource. A cost allocation tag is a key-value pair where the key and value are case-sensitive. You can use cost allocation tags to categorize and track your AWS costs.</p> <p> When you apply tags to your ElastiCache resources, AWS generates a cost allocation report as a comma-separated value (CSV) file with your usage and costs aggregated by your tags. You can apply tags that represent business categories (such as cost centers, application names, or owners) to organize your costs across multiple services. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Tagging.html">Using Cost Allocation Tags in Amazon ElastiCache</a> in the <i>ElastiCache User Guide</i>.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<TagListMessage, RusotoError<AddTagsToResourceError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("AddTagsToResource");
        let mut params = params;
        AddTagsToResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, AddTagsToResourceError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = TagListMessageDeserializer::deserialize("AddTagsToResourceResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Allows network ingress to a cache security group. Applications using ElastiCache must be running on Amazon EC2, and Amazon EC2 security groups are used as the authorization mechanism.</p> <note> <p>You cannot authorize ingress from an Amazon EC2 security group in one region to an ElastiCache cluster in another region.</p> </note></p>
    async fn authorize_cache_security_group_ingress(
        &self,
        input: AuthorizeCacheSecurityGroupIngressMessage,
    ) -> Result<
        AuthorizeCacheSecurityGroupIngressResult,
        RusotoError<AuthorizeCacheSecurityGroupIngressError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("AuthorizeCacheSecurityGroupIngress");
        let mut params = params;
        AuthorizeCacheSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                AuthorizeCacheSecurityGroupIngressError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AuthorizeCacheSecurityGroupIngressResultDeserializer::deserialize(
                "AuthorizeCacheSecurityGroupIngressResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Apply the service update. For more information on service updates and applying them, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/applying-updates.html">Applying Service Updates</a>.</p>
    async fn batch_apply_update_action(
        &self,
        input: BatchApplyUpdateActionMessage,
    ) -> Result<UpdateActionResultsMessage, RusotoError<BatchApplyUpdateActionError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("BatchApplyUpdateAction");
        let mut params = params;
        BatchApplyUpdateActionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, BatchApplyUpdateActionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UpdateActionResultsMessageDeserializer::deserialize(
                "BatchApplyUpdateActionResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Stop the service update. For more information on service updates and stopping them, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/stopping-self-service-updates.html">Stopping Service Updates</a>.</p>
    async fn batch_stop_update_action(
        &self,
        input: BatchStopUpdateActionMessage,
    ) -> Result<UpdateActionResultsMessage, RusotoError<BatchStopUpdateActionError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("BatchStopUpdateAction");
        let mut params = params;
        BatchStopUpdateActionMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, BatchStopUpdateActionError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UpdateActionResultsMessageDeserializer::deserialize(
                "BatchStopUpdateActionResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Complete the migration of data.</p>
    async fn complete_migration(
        &self,
        input: CompleteMigrationMessage,
    ) -> Result<CompleteMigrationResponse, RusotoError<CompleteMigrationError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CompleteMigration");
        let mut params = params;
        CompleteMigrationMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CompleteMigrationError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CompleteMigrationResponseDeserializer::deserialize(
                "CompleteMigrationResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Makes a copy of an existing snapshot.</p> <note> <p>This operation is valid for Redis only.</p> </note> <important> <p>Users or groups that have permissions to use the <code>CopySnapshot</code> operation can create their own Amazon S3 buckets and copy snapshots to it. To control access to your snapshots, use an IAM policy to control who has the ability to use the <code>CopySnapshot</code> operation. For more information about using IAM to control the use of ElastiCache operations, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html">Exporting Snapshots</a> and <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/IAM.html">Authentication &amp; Access Control</a>.</p> </important> <p>You could receive the following error messages.</p> <p class="title"> <b>Error Messages</b> </p> <ul> <li> <p> <b>Error Message:</b> The S3 bucket %s is outside of the region.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s does not exist.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s is not owned by the authenticated user.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-create-s3-bucket">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The authenticated user does not have sufficient permissions to perform the desired activity.</p> <p> <b>Solution:</b> Contact your system administrator to get the needed permissions.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s already contains an object with key %s.</p> <p> <b>Solution:</b> Give the <code>TargetSnapshotName</code> a new and unique value. If exporting a snapshot, you could alternatively create a new Amazon S3 bucket and use this same value for <code>TargetSnapshotName</code>.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add List and Read permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted WRITE permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add Upload/Delete permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ_ACP permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add View Permissions on the bucket. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/backups-exporting.html#backups-exporting-grant-access">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> </ul></p>
    async fn copy_snapshot(
        &self,
        input: CopySnapshotMessage,
    ) -> Result<CopySnapshotResult, RusotoError<CopySnapshotError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CopySnapshot");
        let mut params = params;
        CopySnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CopySnapshotError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CopySnapshotResultDeserializer::deserialize("CopySnapshotResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a cluster. All nodes in the cluster run the same protocol-compliant cache engine software, either Memcached or Redis.</p> <p>This operation is not supported for Redis (cluster mode enabled) clusters.</p>
    async fn create_cache_cluster(
        &self,
        input: CreateCacheClusterMessage,
    ) -> Result<CreateCacheClusterResult, RusotoError<CreateCacheClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateCacheCluster");
        let mut params = params;
        CreateCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateCacheClusterError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateCacheClusterResultDeserializer::deserialize(
                "CreateCacheClusterResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a new Amazon ElastiCache cache parameter group. An ElastiCache cache parameter group is a collection of parameters and their values that are applied to all of the nodes in any cluster or replication group using the CacheParameterGroup.</p> <p>A newly created CacheParameterGroup is an exact duplicate of the default parameter group for the CacheParameterGroupFamily. To customize the newly created CacheParameterGroup you can change the values of specific parameters. For more information, see:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyCacheParameterGroup.html">ModifyCacheParameterGroup</a> in the ElastiCache API Reference.</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ParameterGroups.html">Parameters and Parameter Groups</a> in the ElastiCache User Guide.</p> </li> </ul></p>
    async fn create_cache_parameter_group(
        &self,
        input: CreateCacheParameterGroupMessage,
    ) -> Result<CreateCacheParameterGroupResult, RusotoError<CreateCacheParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateCacheParameterGroup");
        let mut params = params;
        CreateCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateCacheParameterGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateCacheParameterGroupResultDeserializer::deserialize(
                "CreateCacheParameterGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new cache security group. Use a cache security group to control access to one or more clusters.</p> <p>Cache security groups are only used when you are creating a cluster outside of an Amazon Virtual Private Cloud (Amazon VPC). If you are creating a cluster inside of a VPC, use a cache subnet group instead. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateCacheSubnetGroup.html">CreateCacheSubnetGroup</a>.</p>
    async fn create_cache_security_group(
        &self,
        input: CreateCacheSecurityGroupMessage,
    ) -> Result<CreateCacheSecurityGroupResult, RusotoError<CreateCacheSecurityGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateCacheSecurityGroup");
        let mut params = params;
        CreateCacheSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateCacheSecurityGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateCacheSecurityGroupResultDeserializer::deserialize(
                "CreateCacheSecurityGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Creates a new cache subnet group.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>
    async fn create_cache_subnet_group(
        &self,
        input: CreateCacheSubnetGroupMessage,
    ) -> Result<CreateCacheSubnetGroupResult, RusotoError<CreateCacheSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateCacheSubnetGroup");
        let mut params = params;
        CreateCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateCacheSubnetGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateCacheSubnetGroupResultDeserializer::deserialize(
                "CreateCacheSubnetGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Global Datastore for Redis offers fully managed, fast, reliable and secure cross-region replication. Using Global Datastore for Redis, you can create cross-region read replica clusters for ElastiCache for Redis to enable low-latency reads and disaster recovery across regions. For more information, see <a href="/AmazonElastiCache/latest/red-ug/Redis-Global-Datastores.html">Replication Across Regions Using Global Datastore</a>. </p> <ul> <li> <p>The <b>GlobalReplicationGroupIdSuffix</b> is the name of the Global Datastore.</p> </li> <li> <p>The <b>PrimaryReplicationGroupId</b> represents the name of the primary cluster that accepts writes and will replicate updates to the secondary cluster.</p> </li> </ul></p>
    async fn create_global_replication_group(
        &self,
        input: CreateGlobalReplicationGroupMessage,
    ) -> Result<CreateGlobalReplicationGroupResult, RusotoError<CreateGlobalReplicationGroupError>>
    {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateGlobalReplicationGroup");
        let mut params = params;
        CreateGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateGlobalReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateGlobalReplicationGroupResultDeserializer::deserialize(
                "CreateGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group.</p> <p>This API can be used to create a standalone regional replication group or a secondary replication group associated with a Global Datastore.</p> <p>A Redis (cluster mode disabled) replication group is a collection of clusters, where one of the clusters is a read/write primary and the others are read-only replicas. Writes to the primary are asynchronously propagated to the replicas.</p> <p>A Redis (cluster mode enabled) replication group is a collection of 1 to 90 node groups (shards). Each node group (shard) has one read/write primary node and up to 5 read-only replica nodes. Writes to the primary are asynchronously propagated to the replicas. Redis (cluster mode enabled) replication groups partition the data across node groups (shards).</p> <p>When a Redis (cluster mode disabled) replication group has been successfully created, you can add one or more read replicas to it, up to a total of 5 read replicas. If you need to increase or decrease the number of node groups (console: shards), you can avail yourself of ElastiCache for Redis&#39; scaling. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Scaling.html">Scaling ElastiCache for Redis Clusters</a> in the <i>ElastiCache User Guide</i>.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn create_replication_group(
        &self,
        input: CreateReplicationGroupMessage,
    ) -> Result<CreateReplicationGroupResult, RusotoError<CreateReplicationGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateReplicationGroup");
        let mut params = params;
        CreateReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CreateReplicationGroupResultDeserializer::deserialize(
                "CreateReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Creates a copy of an entire cluster or replication group at a specific moment in time.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn create_snapshot(
        &self,
        input: CreateSnapshotMessage,
    ) -> Result<CreateSnapshotResult, RusotoError<CreateSnapshotError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateSnapshot");
        let mut params = params;
        CreateSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateSnapshotError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                CreateSnapshotResultDeserializer::deserialize("CreateSnapshotResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>For Redis engine version 6.x onwards: Creates a Redis user. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>.</p>
    async fn create_user(
        &self,
        input: CreateUserMessage,
    ) -> Result<User, RusotoError<CreateUserError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateUser");
        let mut params = params;
        CreateUserMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateUserError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserDeserializer::deserialize("CreateUserResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>For Redis engine version 6.x onwards: Creates a Redis user group. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a> </p>
    async fn create_user_group(
        &self,
        input: CreateUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<CreateUserGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("CreateUserGroup");
        let mut params = params;
        CreateUserGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, CreateUserGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserGroupDeserializer::deserialize("CreateUserGroupResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Decreases the number of node groups in a Global Datastore</p>
    async fn decrease_node_groups_in_global_replication_group(
        &self,
        input: DecreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) -> Result<
        DecreaseNodeGroupsInGlobalReplicationGroupResult,
        RusotoError<DecreaseNodeGroupsInGlobalReplicationGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DecreaseNodeGroupsInGlobalReplicationGroup");
        let mut params = params;
        DecreaseNodeGroupsInGlobalReplicationGroupMessageSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DecreaseNodeGroupsInGlobalReplicationGroupError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DecreaseNodeGroupsInGlobalReplicationGroupResultDeserializer::deserialize(
                "DecreaseNodeGroupsInGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Dynamically decreases the number of replicas in a Redis (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Redis (cluster mode enabled) replication group. This operation is performed with no cluster down time.</p>
    async fn decrease_replica_count(
        &self,
        input: DecreaseReplicaCountMessage,
    ) -> Result<DecreaseReplicaCountResult, RusotoError<DecreaseReplicaCountError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DecreaseReplicaCount");
        let mut params = params;
        DecreaseReplicaCountMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DecreaseReplicaCountError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DecreaseReplicaCountResultDeserializer::deserialize(
                "DecreaseReplicaCountResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes a previously provisioned cluster. <code>DeleteCacheCluster</code> deletes all associated cache nodes, node endpoints and the cluster itself. When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the cluster; you cannot cancel or revert this operation.</p> <p>This operation is not valid for:</p> <ul> <li> <p>Redis (cluster mode enabled) clusters</p> </li> <li> <p>A cluster that is the last read replica of a replication group</p> </li> <li> <p>A node group (shard) that has Multi-AZ mode enabled</p> </li> <li> <p>A cluster from a Redis (cluster mode enabled) replication group</p> </li> <li> <p>A cluster that is not in the <code>available</code> state</p> </li> </ul></p>
    async fn delete_cache_cluster(
        &self,
        input: DeleteCacheClusterMessage,
    ) -> Result<DeleteCacheClusterResult, RusotoError<DeleteCacheClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteCacheCluster");
        let mut params = params;
        DeleteCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteCacheClusterError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DeleteCacheClusterResultDeserializer::deserialize(
                "DeleteCacheClusterResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Deletes the specified cache parameter group. You cannot delete a cache parameter group if it is associated with any cache clusters.</p>
    async fn delete_cache_parameter_group(
        &self,
        input: DeleteCacheParameterGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteCacheParameterGroup");
        let mut params = params;
        DeleteCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteCacheParameterGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a cache security group.</p> <note> <p>You cannot delete a cache security group if it is associated with any clusters.</p> </note></p>
    async fn delete_cache_security_group(
        &self,
        input: DeleteCacheSecurityGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheSecurityGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteCacheSecurityGroup");
        let mut params = params;
        DeleteCacheSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteCacheSecurityGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p><p>Deletes a cache subnet group.</p> <note> <p>You cannot delete a cache subnet group if it is associated with any clusters.</p> </note></p>
    async fn delete_cache_subnet_group(
        &self,
        input: DeleteCacheSubnetGroupMessage,
    ) -> Result<(), RusotoError<DeleteCacheSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteCacheSubnetGroup");
        let mut params = params;
        DeleteCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteCacheSubnetGroupError::from_response)
            .await?;

        std::mem::drop(response);
        Ok(())
    }

    /// <p>Deleting a Global Datastore is a two-step process: </p> <ul> <li> <p>First, you must <a>DisassociateGlobalReplicationGroup</a> to remove the secondary clusters in the Global Datastore.</p> </li> <li> <p>Once the Global Datastore contains only the primary cluster, you can use DeleteGlobalReplicationGroup API to delete the Global Datastore while retainining the primary cluster using Retain…= true.</p> </li> </ul> <p>Since the Global Datastore has only a primary cluster, you can delete the Global Datastore while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p>
    async fn delete_global_replication_group(
        &self,
        input: DeleteGlobalReplicationGroupMessage,
    ) -> Result<DeleteGlobalReplicationGroupResult, RusotoError<DeleteGlobalReplicationGroupError>>
    {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteGlobalReplicationGroup");
        let mut params = params;
        DeleteGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteGlobalReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DeleteGlobalReplicationGroupResultDeserializer::deserialize(
                "DeleteGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes an existing replication group. By default, this operation deletes the entire replication group, including the primary/primaries and all of the read replicas. If the replication group has only one primary, you can optionally delete only the read replicas, while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn delete_replication_group(
        &self,
        input: DeleteReplicationGroupMessage,
    ) -> Result<DeleteReplicationGroupResult, RusotoError<DeleteReplicationGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteReplicationGroup");
        let mut params = params;
        DeleteReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DeleteReplicationGroupResultDeserializer::deserialize(
                "DeleteReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Deletes an existing snapshot. When you receive a successful response from this operation, ElastiCache immediately begins deleting the snapshot; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn delete_snapshot(
        &self,
        input: DeleteSnapshotMessage,
    ) -> Result<DeleteSnapshotResult, RusotoError<DeleteSnapshotError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteSnapshot");
        let mut params = params;
        DeleteSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteSnapshotError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                DeleteSnapshotResultDeserializer::deserialize("DeleteSnapshotResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>For Redis engine version 6.x onwards: Deletes a user. The user will be removed from all user groups and in turn removed from all replication groups. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>. </p>
    async fn delete_user(
        &self,
        input: DeleteUserMessage,
    ) -> Result<User, RusotoError<DeleteUserError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteUser");
        let mut params = params;
        DeleteUserMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteUserError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserDeserializer::deserialize("DeleteUserResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>For Redis engine version 6.x onwards: Deletes a ser group. The user group must first be disassociated from the replcation group before it can be deleted. For more information, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.RBAC.html">Using Role Based Access Control (RBAC)</a>. </p>
    async fn delete_user_group(
        &self,
        input: DeleteUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<DeleteUserGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DeleteUserGroup");
        let mut params = params;
        DeleteUserGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DeleteUserGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserGroupDeserializer::deserialize("DeleteUserGroupResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about all provisioned clusters if no cluster identifier is specified, or about a specific cache cluster if a cluster identifier is supplied.</p> <p>By default, abbreviated information about the clusters is returned. You can use the optional <i>ShowCacheNodeInfo</i> flag to retrieve detailed information about the cache nodes associated with the clusters. These details include the DNS address and port for the cache node endpoint.</p> <p>If the cluster is in the <i>creating</i> state, only cluster-level information is displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the <i>deleting</i> state, only cluster-level information is displayed.</p> <p>If cache nodes are currently being added to the cluster, node endpoint information and creation time for the additional nodes are not displayed until they are completely provisioned. When the cluster state is <i>available</i>, the cluster is ready for use.</p> <p>If cache nodes are currently being removed from the cluster, no endpoint information for the removed nodes is displayed.</p>
    async fn describe_cache_clusters(
        &self,
        input: DescribeCacheClustersMessage,
    ) -> Result<CacheClusterMessage, RusotoError<DescribeCacheClustersError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheClusters");
        let mut params = params;
        DescribeCacheClustersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheClustersError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                CacheClusterMessageDeserializer::deserialize("DescribeCacheClustersResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of the available cache engines and their versions.</p>
    async fn describe_cache_engine_versions(
        &self,
        input: DescribeCacheEngineVersionsMessage,
    ) -> Result<CacheEngineVersionMessage, RusotoError<DescribeCacheEngineVersionsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheEngineVersions");
        let mut params = params;
        DescribeCacheEngineVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheEngineVersionsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheEngineVersionMessageDeserializer::deserialize(
                "DescribeCacheEngineVersionsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of cache parameter group descriptions. If a cache parameter group name is specified, the list contains only the descriptions for that group.</p>
    async fn describe_cache_parameter_groups(
        &self,
        input: DescribeCacheParameterGroupsMessage,
    ) -> Result<CacheParameterGroupsMessage, RusotoError<DescribeCacheParameterGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheParameterGroups");
        let mut params = params;
        DescribeCacheParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheParameterGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheParameterGroupsMessageDeserializer::deserialize(
                "DescribeCacheParameterGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns the detailed parameter list for a particular cache parameter group.</p>
    async fn describe_cache_parameters(
        &self,
        input: DescribeCacheParametersMessage,
    ) -> Result<CacheParameterGroupDetails, RusotoError<DescribeCacheParametersError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheParameters");
        let mut params = params;
        DescribeCacheParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheParametersError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheParameterGroupDetailsDeserializer::deserialize(
                "DescribeCacheParametersResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of cache security group descriptions. If a cache security group name is specified, the list contains only the description of that group. This applicable only when you have ElastiCache in Classic setup </p>
    async fn describe_cache_security_groups(
        &self,
        input: DescribeCacheSecurityGroupsMessage,
    ) -> Result<CacheSecurityGroupMessage, RusotoError<DescribeCacheSecurityGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheSecurityGroups");
        let mut params = params;
        DescribeCacheSecurityGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheSecurityGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheSecurityGroupMessageDeserializer::deserialize(
                "DescribeCacheSecurityGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of cache subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group. This is applicable only when you have ElastiCache in VPC setup. All ElastiCache clusters now launch in VPC by default. </p>
    async fn describe_cache_subnet_groups(
        &self,
        input: DescribeCacheSubnetGroupsMessage,
    ) -> Result<CacheSubnetGroupMessage, RusotoError<DescribeCacheSubnetGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeCacheSubnetGroups");
        let mut params = params;
        DescribeCacheSubnetGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeCacheSubnetGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheSubnetGroupMessageDeserializer::deserialize(
                "DescribeCacheSubnetGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns the default engine and system parameter information for the specified cache engine.</p>
    async fn describe_engine_default_parameters(
        &self,
        input: DescribeEngineDefaultParametersMessage,
    ) -> Result<
        DescribeEngineDefaultParametersResult,
        RusotoError<DescribeEngineDefaultParametersError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeEngineDefaultParameters");
        let mut params = params;
        DescribeEngineDefaultParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeEngineDefaultParametersError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeEngineDefaultParametersResultDeserializer::deserialize(
                "DescribeEngineDefaultParametersResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns events related to clusters, cache security groups, and cache parameter groups. You can obtain events specific to a particular cluster, cache security group, or cache parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<EventsMessage, RusotoError<DescribeEventsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeEvents");
        let mut params = params;
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeEventsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = EventsMessageDeserializer::deserialize("DescribeEventsResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about a particular global replication group. If no identifier is specified, returns information about all Global Datastores. </p>
    async fn describe_global_replication_groups(
        &self,
        input: DescribeGlobalReplicationGroupsMessage,
    ) -> Result<
        DescribeGlobalReplicationGroupsResult,
        RusotoError<DescribeGlobalReplicationGroupsError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeGlobalReplicationGroups");
        let mut params = params;
        DescribeGlobalReplicationGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeGlobalReplicationGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeGlobalReplicationGroupsResultDeserializer::deserialize(
                "DescribeGlobalReplicationGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Returns information about a particular replication group. If no identifier is specified, <code>DescribeReplicationGroups</code> returns information about all replication groups.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn describe_replication_groups(
        &self,
        input: DescribeReplicationGroupsMessage,
    ) -> Result<ReplicationGroupMessage, RusotoError<DescribeReplicationGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeReplicationGroups");
        let mut params = params;
        DescribeReplicationGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeReplicationGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ReplicationGroupMessageDeserializer::deserialize(
                "DescribeReplicationGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns information about reserved cache nodes for this account, or about a specified reserved cache node.</p>
    async fn describe_reserved_cache_nodes(
        &self,
        input: DescribeReservedCacheNodesMessage,
    ) -> Result<ReservedCacheNodeMessage, RusotoError<DescribeReservedCacheNodesError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeReservedCacheNodes");
        let mut params = params;
        DescribeReservedCacheNodesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeReservedCacheNodesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ReservedCacheNodeMessageDeserializer::deserialize(
                "DescribeReservedCacheNodesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Lists available reserved cache node offerings.</p>
    async fn describe_reserved_cache_nodes_offerings(
        &self,
        input: DescribeReservedCacheNodesOfferingsMessage,
    ) -> Result<
        ReservedCacheNodesOfferingMessage,
        RusotoError<DescribeReservedCacheNodesOfferingsError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeReservedCacheNodesOfferings");
        let mut params = params;
        DescribeReservedCacheNodesOfferingsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DescribeReservedCacheNodesOfferingsError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ReservedCacheNodesOfferingMessageDeserializer::deserialize(
                "DescribeReservedCacheNodesOfferingsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns details of the service updates</p>
    async fn describe_service_updates(
        &self,
        input: DescribeServiceUpdatesMessage,
    ) -> Result<ServiceUpdatesMessage, RusotoError<DescribeServiceUpdatesError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeServiceUpdates");
        let mut params = params;
        DescribeServiceUpdatesMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeServiceUpdatesError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ServiceUpdatesMessageDeserializer::deserialize(
                "DescribeServiceUpdatesResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Returns information about cluster or replication group snapshots. By default, <code>DescribeSnapshots</code> lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a particular cache cluster.</p> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn describe_snapshots(
        &self,
        input: DescribeSnapshotsMessage,
    ) -> Result<DescribeSnapshotsListMessage, RusotoError<DescribeSnapshotsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeSnapshots");
        let mut params = params;
        DescribeSnapshotsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeSnapshotsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeSnapshotsListMessageDeserializer::deserialize(
                "DescribeSnapshotsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns details of the update actions </p>
    async fn describe_update_actions(
        &self,
        input: DescribeUpdateActionsMessage,
    ) -> Result<UpdateActionsMessage, RusotoError<DescribeUpdateActionsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeUpdateActions");
        let mut params = params;
        DescribeUpdateActionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeUpdateActionsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UpdateActionsMessageDeserializer::deserialize(
                "DescribeUpdateActionsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of user groups.</p>
    async fn describe_user_groups(
        &self,
        input: DescribeUserGroupsMessage,
    ) -> Result<DescribeUserGroupsResult, RusotoError<DescribeUserGroupsError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeUserGroups");
        let mut params = params;
        DescribeUserGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeUserGroupsError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DescribeUserGroupsResultDeserializer::deserialize(
                "DescribeUserGroupsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Returns a list of users.</p>
    async fn describe_users(
        &self,
        input: DescribeUsersMessage,
    ) -> Result<DescribeUsersResult, RusotoError<DescribeUsersError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DescribeUsers");
        let mut params = params;
        DescribeUsersMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, DescribeUsersError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                DescribeUsersResultDeserializer::deserialize("DescribeUsersResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Remove a secondary cluster from the Global Datastore using the Global Datastore name. The secondary cluster will no longer receive updates from the primary cluster, but will remain as a standalone cluster in that AWS region.</p>
    async fn disassociate_global_replication_group(
        &self,
        input: DisassociateGlobalReplicationGroupMessage,
    ) -> Result<
        DisassociateGlobalReplicationGroupResult,
        RusotoError<DisassociateGlobalReplicationGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("DisassociateGlobalReplicationGroup");
        let mut params = params;
        DisassociateGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                DisassociateGlobalReplicationGroupError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = DisassociateGlobalReplicationGroupResultDeserializer::deserialize(
                "DisassociateGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Used to failover the primary region to a selected secondary region. The selected secondary region will become primary, and all other clusters will become secondary.</p>
    async fn failover_global_replication_group(
        &self,
        input: FailoverGlobalReplicationGroupMessage,
    ) -> Result<
        FailoverGlobalReplicationGroupResult,
        RusotoError<FailoverGlobalReplicationGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("FailoverGlobalReplicationGroup");
        let mut params = params;
        FailoverGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, FailoverGlobalReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = FailoverGlobalReplicationGroupResultDeserializer::deserialize(
                "FailoverGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Increase the number of node groups in the Global Datastore</p>
    async fn increase_node_groups_in_global_replication_group(
        &self,
        input: IncreaseNodeGroupsInGlobalReplicationGroupMessage,
    ) -> Result<
        IncreaseNodeGroupsInGlobalReplicationGroupResult,
        RusotoError<IncreaseNodeGroupsInGlobalReplicationGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("IncreaseNodeGroupsInGlobalReplicationGroup");
        let mut params = params;
        IncreaseNodeGroupsInGlobalReplicationGroupMessageSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                IncreaseNodeGroupsInGlobalReplicationGroupError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = IncreaseNodeGroupsInGlobalReplicationGroupResultDeserializer::deserialize(
                "IncreaseNodeGroupsInGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Dynamically increases the number of replics in a Redis (cluster mode disabled) replication group or the number of replica nodes in one or more node groups (shards) of a Redis (cluster mode enabled) replication group. This operation is performed with no cluster down time.</p>
    async fn increase_replica_count(
        &self,
        input: IncreaseReplicaCountMessage,
    ) -> Result<IncreaseReplicaCountResult, RusotoError<IncreaseReplicaCountError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("IncreaseReplicaCount");
        let mut params = params;
        IncreaseReplicaCountMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, IncreaseReplicaCountError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = IncreaseReplicaCountResultDeserializer::deserialize(
                "IncreaseReplicaCountResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Lists all available node types that you can scale your Redis cluster's or replication group's current node type.</p> <p>When you use the <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code> operations to scale your cluster or replication group, the value of the <code>CacheNodeType</code> parameter must be one of the node types returned by this operation.</p>
    async fn list_allowed_node_type_modifications(
        &self,
        input: ListAllowedNodeTypeModificationsMessage,
    ) -> Result<
        AllowedNodeTypeModificationsMessage,
        RusotoError<ListAllowedNodeTypeModificationsError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ListAllowedNodeTypeModifications");
        let mut params = params;
        ListAllowedNodeTypeModificationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                ListAllowedNodeTypeModificationsError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = AllowedNodeTypeModificationsMessageDeserializer::deserialize(
                "ListAllowedNodeTypeModificationsResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Lists all cost allocation tags currently on the named resource. A <code>cost allocation tag</code> is a key-value pair where the key is case-sensitive and the value is optional. You can use cost allocation tags to categorize and track your AWS costs.</p> <p>If the cluster is not in the <i>available</i> state, <code>ListTagsForResource</code> returns an error.</p> <p>You can have a maximum of 50 cost allocation tags on an ElastiCache resource. For more information, see <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Tagging.html">Monitoring Costs with Tags</a>.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<TagListMessage, RusotoError<ListTagsForResourceError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ListTagsForResource");
        let mut params = params;
        ListTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                TagListMessageDeserializer::deserialize("ListTagsForResourceResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the settings for a cluster. You can use this operation to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>
    async fn modify_cache_cluster(
        &self,
        input: ModifyCacheClusterMessage,
    ) -> Result<ModifyCacheClusterResult, RusotoError<ModifyCacheClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyCacheCluster");
        let mut params = params;
        ModifyCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyCacheClusterError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyCacheClusterResultDeserializer::deserialize(
                "ModifyCacheClusterResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the parameters of a cache parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>
    async fn modify_cache_parameter_group(
        &self,
        input: ModifyCacheParameterGroupMessage,
    ) -> Result<CacheParameterGroupNameMessage, RusotoError<ModifyCacheParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyCacheParameterGroup");
        let mut params = params;
        ModifyCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyCacheParameterGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheParameterGroupNameMessageDeserializer::deserialize(
                "ModifyCacheParameterGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies an existing cache subnet group.</p>
    async fn modify_cache_subnet_group(
        &self,
        input: ModifyCacheSubnetGroupMessage,
    ) -> Result<ModifyCacheSubnetGroupResult, RusotoError<ModifyCacheSubnetGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyCacheSubnetGroup");
        let mut params = params;
        ModifyCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyCacheSubnetGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyCacheSubnetGroupResultDeserializer::deserialize(
                "ModifyCacheSubnetGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the settings for a Global Datastore.</p>
    async fn modify_global_replication_group(
        &self,
        input: ModifyGlobalReplicationGroupMessage,
    ) -> Result<ModifyGlobalReplicationGroupResult, RusotoError<ModifyGlobalReplicationGroupError>>
    {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyGlobalReplicationGroup");
        let mut params = params;
        ModifyGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyGlobalReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyGlobalReplicationGroupResultDeserializer::deserialize(
                "ModifyGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p><p>Modifies the settings for a replication group.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/scaling-redis-cluster-mode-enabled.html">Scaling for Amazon ElastiCache for Redis (cluster mode enabled)</a> in the ElastiCache User Guide</p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_ModifyReplicationGroupShardConfiguration.html">ModifyReplicationGroupShardConfiguration</a> in the ElastiCache API Reference</p> </li> </ul> <note> <p>This operation is valid for Redis only.</p> </note></p>
    async fn modify_replication_group(
        &self,
        input: ModifyReplicationGroupMessage,
    ) -> Result<ModifyReplicationGroupResult, RusotoError<ModifyReplicationGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyReplicationGroup");
        let mut params = params;
        ModifyReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyReplicationGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyReplicationGroupResultDeserializer::deserialize(
                "ModifyReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies a replication group's shards (node groups) by allowing you to add shards, remove shards, or rebalance the keyspaces among exisiting shards.</p>
    async fn modify_replication_group_shard_configuration(
        &self,
        input: ModifyReplicationGroupShardConfigurationMessage,
    ) -> Result<
        ModifyReplicationGroupShardConfigurationResult,
        RusotoError<ModifyReplicationGroupShardConfigurationError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyReplicationGroupShardConfiguration");
        let mut params = params;
        ModifyReplicationGroupShardConfigurationMessageSerializer::serialize(
            &mut params,
            "",
            &input,
        );
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                ModifyReplicationGroupShardConfigurationError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = ModifyReplicationGroupShardConfigurationResultDeserializer::deserialize(
                "ModifyReplicationGroupShardConfigurationResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Changes user password(s) and/or access string.</p>
    async fn modify_user(
        &self,
        input: ModifyUserMessage,
    ) -> Result<User, RusotoError<ModifyUserError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyUser");
        let mut params = params;
        ModifyUserMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyUserError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserDeserializer::deserialize("ModifyUserResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Changes the list of users that belong to the user group.</p>
    async fn modify_user_group(
        &self,
        input: ModifyUserGroupMessage,
    ) -> Result<UserGroup, RusotoError<ModifyUserGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ModifyUserGroup");
        let mut params = params;
        ModifyUserGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ModifyUserGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = UserGroupDeserializer::deserialize("ModifyUserGroupResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Allows you to purchase a reserved cache node offering.</p>
    async fn purchase_reserved_cache_nodes_offering(
        &self,
        input: PurchaseReservedCacheNodesOfferingMessage,
    ) -> Result<
        PurchaseReservedCacheNodesOfferingResult,
        RusotoError<PurchaseReservedCacheNodesOfferingError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("PurchaseReservedCacheNodesOffering");
        let mut params = params;
        PurchaseReservedCacheNodesOfferingMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                PurchaseReservedCacheNodesOfferingError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = PurchaseReservedCacheNodesOfferingResultDeserializer::deserialize(
                "PurchaseReservedCacheNodesOfferingResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Redistribute slots to ensure uniform distribution across existing shards in the cluster.</p>
    async fn rebalance_slots_in_global_replication_group(
        &self,
        input: RebalanceSlotsInGlobalReplicationGroupMessage,
    ) -> Result<
        RebalanceSlotsInGlobalReplicationGroupResult,
        RusotoError<RebalanceSlotsInGlobalReplicationGroupError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("RebalanceSlotsInGlobalReplicationGroup");
        let mut params = params;
        RebalanceSlotsInGlobalReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(
                request,
                RebalanceSlotsInGlobalReplicationGroupError::from_response,
            )
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = RebalanceSlotsInGlobalReplicationGroupResultDeserializer::deserialize(
                "RebalanceSlotsInGlobalReplicationGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Reboots some, or all, of the cache nodes within a provisioned cluster. This operation applies any modified cache parameter groups to the cluster. The reboot operation takes place as soon as possible, and results in a momentary outage to the cluster. During the reboot, the cluster status is set to REBOOTING.</p> <p>The reboot causes the contents of the cache (for each cache node being rebooted) to be lost.</p> <p>When the reboot is complete, a cluster event is created.</p> <p>Rebooting a cluster is currently supported on Memcached and Redis (cluster mode disabled) clusters. Rebooting is not supported on Redis (cluster mode enabled) clusters.</p> <p>If you make changes to parameters that require a Redis (cluster mode enabled) cluster reboot for the changes to be applied, see <a href="http://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/Clusters.Rebooting.html">Rebooting a Cluster</a> for an alternate process.</p>
    async fn reboot_cache_cluster(
        &self,
        input: RebootCacheClusterMessage,
    ) -> Result<RebootCacheClusterResult, RusotoError<RebootCacheClusterError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("RebootCacheCluster");
        let mut params = params;
        RebootCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, RebootCacheClusterError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = RebootCacheClusterResultDeserializer::deserialize(
                "RebootCacheClusterResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Removes the tags identified by the <code>TagKeys</code> list from the named resource.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<TagListMessage, RusotoError<RemoveTagsFromResourceError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("RemoveTagsFromResource");
        let mut params = params;
        RemoveTagsFromResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, RemoveTagsFromResourceError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                TagListMessageDeserializer::deserialize("RemoveTagsFromResourceResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Modifies the parameters of a cache parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire cache parameter group, specify the <code>ResetAllParameters</code> and <code>CacheParameterGroupName</code> parameters.</p>
    async fn reset_cache_parameter_group(
        &self,
        input: ResetCacheParameterGroupMessage,
    ) -> Result<CacheParameterGroupNameMessage, RusotoError<ResetCacheParameterGroupError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("ResetCacheParameterGroup");
        let mut params = params;
        ResetCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, ResetCacheParameterGroupError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = CacheParameterGroupNameMessageDeserializer::deserialize(
                "ResetCacheParameterGroupResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Revokes ingress from a cache security group. Use this operation to disallow access from an Amazon EC2 security group that had been previously authorized.</p>
    async fn revoke_cache_security_group_ingress(
        &self,
        input: RevokeCacheSecurityGroupIngressMessage,
    ) -> Result<
        RevokeCacheSecurityGroupIngressResult,
        RusotoError<RevokeCacheSecurityGroupIngressError>,
    > {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("RevokeCacheSecurityGroupIngress");
        let mut params = params;
        RevokeCacheSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, RevokeCacheSecurityGroupIngressError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = RevokeCacheSecurityGroupIngressResultDeserializer::deserialize(
                "RevokeCacheSecurityGroupIngressResult",
                stack,
            )?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Start the migration of data.</p>
    async fn start_migration(
        &self,
        input: StartMigrationMessage,
    ) -> Result<StartMigrationResponse, RusotoError<StartMigrationError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("StartMigration");
        let mut params = params;
        StartMigrationMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, StartMigrationError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result =
                StartMigrationResponseDeserializer::deserialize("StartMigrationResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }

    /// <p>Represents the input of a <code>TestFailover</code> operation which test automatic failover on a specified node group (called shard in the console) in a replication group (called cluster in the console).</p> <p class="title"> <b>Note the following</b> </p> <ul> <li> <p>A customer can use this operation to test automatic failover on up to 5 shards (called node groups in the ElastiCache API and AWS CLI) in any rolling 24-hour period.</p> </li> <li> <p>If calling this operation on shards in different clusters (called replication groups in the API and CLI), the calls can be made concurrently.</p> <p> </p> </li> <li> <p>If calling this operation multiple times on different shards in the same Redis (cluster mode enabled) replication group, the first node replacement must complete before a subsequent call can be made.</p> </li> <li> <p>To determine whether the node replacement is complete you can check Events using the Amazon ElastiCache console, the AWS CLI, or the ElastiCache API. Look for the following automatic failover related events, listed here in order of occurrance:</p> <ol> <li> <p>Replication group message: <code>Test Failover API called for node group &lt;node-group-id&gt;</code> </p> </li> <li> <p>Cache cluster message: <code>Failover from primary node &lt;primary-node-id&gt; to replica node &lt;node-id&gt; completed</code> </p> </li> <li> <p>Replication group message: <code>Failover from primary node &lt;primary-node-id&gt; to replica node &lt;node-id&gt; completed</code> </p> </li> <li> <p>Cache cluster message: <code>Recovering cache nodes &lt;node-id&gt;</code> </p> </li> <li> <p>Cache cluster message: <code>Finished recovery for cache nodes &lt;node-id&gt;</code> </p> </li> </ol> <p>For more information see:</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/ECEvents.Viewing.html">Viewing ElastiCache Events</a> in the <i>ElastiCache User Guide</i> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_DescribeEvents.html">DescribeEvents</a> in the ElastiCache API Reference</p> </li> </ul> </li> </ul> <p>Also see, <a href="https://docs.aws.amazon.com/AmazonElastiCache/latest/red-ug/AutoFailover.html#auto-failover-test">Testing Multi-AZ </a> in the <i>ElastiCache User Guide</i>.</p>
    async fn test_failover(
        &self,
        input: TestFailoverMessage,
    ) -> Result<TestFailoverResult, RusotoError<TestFailoverError>> {
        let mut request = SignedRequest::new("POST", "elasticache", &self.region, "/");
        let params = self.new_params("TestFailover");
        let mut params = params;
        TestFailoverMessageSerializer::serialize(&mut params, "", &input);
        request.set_payload(Some(serde_urlencoded::to_string(&params).unwrap()));
        request.set_content_type("application/x-www-form-urlencoded".to_owned());

        let response = self
            .sign_and_dispatch(request, TestFailoverError::from_response)
            .await?;

        let mut response = response;
        let result = xml_util::parse_response(&mut response, |actual_tag_name, stack| {
            xml_util::start_element(actual_tag_name, stack)?;
            let result = TestFailoverResultDeserializer::deserialize("TestFailoverResult", stack)?;
            skip_tree(stack);
            xml_util::end_element(actual_tag_name, stack)?;
            Ok(result)
        })
        .await?;

        drop(response); // parse non-payload
        Ok(result)
    }
}
