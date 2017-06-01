#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use std::str::FromStr;
use xml::EventReader;
use xml::reader::ParserConfig;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use xml::reader::XmlEvent;
use rusoto_core::xmlutil::{Next, Peek, XmlParseError, XmlResponse};
use rusoto_core::xmlutil::{characters, end_element, start_element, skip_tree, peek_at_name};
use rusoto_core::xmlerror::*;

enum DeserializerNext {
    Close,
    Skip,
    Element(String),
}
pub type AZMode = String;
#[doc="<p>Represents the input of an AddTagsToResource operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct AddTagsToResourceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource to which the tags are to be added, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
    pub resource_name: String,
    #[doc="<p>A list of cost allocation tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value.</p>"]
    pub tags: TagList,
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
        TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), &obj.tags);

    }
}

#[derive(Default,Debug,Clone)]
pub struct AllowedNodeTypeModificationsMessage {
    pub scale_up_modifications: Option<NodeTypeList>,
}

struct AllowedNodeTypeModificationsMessageDeserializer;
impl AllowedNodeTypeModificationsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<AllowedNodeTypeModificationsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AllowedNodeTypeModificationsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ScaleUpModifications" => {
                            obj.scale_up_modifications =
                                Some(try!(NodeTypeListDeserializer::deserialize("ScaleUpModifications",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of an AuthorizeCacheSecurityGroupIngress operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct AuthorizeCacheSecurityGroupIngressMessage {
    #[doc="<p>The cache security group that allows network ingress.</p>"]
    pub cache_security_group_name: String,
    #[doc="<p>The Amazon EC2 security group to be authorized for ingress to the cache security group.</p>"]
    pub ec2_security_group_name: String,
    #[doc="<p>The AWS account number of the Amazon EC2 security group owner. Note that this is not the same thing as an AWS access key ID - you must provide a valid AWS account number for this parameter.</p>"]
    pub ec2_security_group_owner_id: String,
}


/// Serialize `AuthorizeCacheSecurityGroupIngressMessage` contents to a `SignedRequest`.
struct AuthorizeCacheSecurityGroupIngressMessageSerializer;
impl AuthorizeCacheSecurityGroupIngressMessageSerializer {
    fn serialize(params: &mut Params,
                 name: &str,
                 obj: &AuthorizeCacheSecurityGroupIngressMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheSecurityGroupName"),
                   &obj.cache_security_group_name);
        params.put(&format!("{}{}", prefix, "EC2SecurityGroupName"),
                   &obj.ec2_security_group_name);
        params.put(&format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
                   &obj.ec2_security_group_owner_id);

    }
}

#[derive(Default,Debug,Clone)]
pub struct AuthorizeCacheSecurityGroupIngressResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

struct AuthorizeCacheSecurityGroupIngressResultDeserializer;
impl AuthorizeCacheSecurityGroupIngressResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<AuthorizeCacheSecurityGroupIngressResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = AuthorizeCacheSecurityGroupIngressResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroup" => {
                            obj.cache_security_group =
                                Some(try!(CacheSecurityGroupDeserializer::deserialize("CacheSecurityGroup",
                                                                                      stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type AutomaticFailoverStatus = String;
struct AutomaticFailoverStatusDeserializer;
impl AutomaticFailoverStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AutomaticFailoverStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Describes an Availability Zone in which the cache cluster is launched.</p>"]
#[derive(Default,Debug,Clone)]
pub struct AvailabilityZone {
    #[doc="<p>The name of the Availability Zone.</p>"]
    pub name: Option<String>,
}

struct AvailabilityZoneDeserializer;
impl AvailabilityZoneDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AvailabilityZone, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Name" => {
                            obj.name = Some(try!(StringDeserializer::deserialize("Name", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type AvailabilityZonesList = Vec<String>;
struct AvailabilityZonesListDeserializer;
impl AvailabilityZonesListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<AvailabilityZonesList, XmlParseError> {

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
                        obj.push(try!(StringDeserializer::deserialize("AvailabilityZone", stack)));
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

/// Serialize `AvailabilityZonesList` contents to a `SignedRequest`.
struct AvailabilityZonesListSerializer;
impl AvailabilityZonesListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &AvailabilityZonesList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type AwsQueryErrorMessage = String;
pub type Boolean = bool;
struct BooleanDeserializer;
impl BooleanDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Boolean, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = bool::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type BooleanOptional = bool;
#[doc="<p>Contains all of the attributes of a specific cache cluster.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheCluster {
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<Boolean>,
    #[doc="<p>The date and time when the cache cluster was created.</p>"]
    pub cache_cluster_create_time: Option<TStamp>,
    #[doc="<p>The user-supplied identifier of the cache cluster. This identifier is a unique key that identifies a cache cluster.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The current state of this cache cluster, one of the following values: <code>available</code>, <code>creating</code>, <code>deleted</code>, <code>deleting</code>, <code>incompatible-network</code>, <code>modifying</code>, <code>rebooting cache cluster nodes</code>, <code>restore-failed</code>, or <code>snapshotting</code>.</p>"]
    pub cache_cluster_status: Option<String>,
    #[doc="<p>The name of the compute and memory capacity node type for the cache cluster.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>A list of cache nodes that are members of the cache cluster.</p>"]
    pub cache_nodes: Option<CacheNodeList>,
    pub cache_parameter_group: Option<CacheParameterGroupStatus>,
    #[doc="<p>A list of cache security group elements, composed of name and status sub-elements.</p>"]
    pub cache_security_groups: Option<CacheSecurityGroupMembershipList>,
    #[doc="<p>The name of the cache subnet group associated with the cache cluster.</p>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>The URL of the web page where you can download the latest ElastiCache client library.</p>"]
    pub client_download_landing_page: Option<String>,
    pub configuration_endpoint: Option<Endpoint>,
    #[doc="<p>The name of the cache engine (<code>memcached</code> or <code>redis</code>) to be used for this cache cluster.</p>"]
    pub engine: Option<String>,
    #[doc="<p>The version of the cache engine that is used in this cache cluster.</p>"]
    pub engine_version: Option<String>,
    pub notification_configuration: Option<NotificationConfiguration>,
    #[doc="<p>The number of cache nodes in the cache cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>"]
    pub num_cache_nodes: Option<IntegerOptional>,
    pub pending_modified_values: Option<PendingModifiedValues>,
    #[doc="<p>The name of the Availability Zone in which the cache cluster is located or \"Multiple\" if the cache nodes are located in different Availability Zones.</p>"]
    pub preferred_availability_zone: Option<String>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>The replication group to which this cache cluster belongs. If this field is empty, the cache cluster is not associated with any replication group.</p>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>A list of VPC Security Groups associated with the cache cluster.</p>"]
    pub security_groups: Option<SecurityGroupMembershipList>,
    #[doc="<p>The number of days for which ElastiCache retains automatic cache cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <important> <p> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p> </important>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your cache cluster.</p> <p>Example: <code>05:00-09:00</code> </p>"]
    pub snapshot_window: Option<String>,
}

struct CacheClusterDeserializer;
impl CacheClusterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheCluster, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheCluster::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AutoMinorVersionUpgrade" => {
                            obj.auto_minor_version_upgrade =
                                Some(try!(BooleanDeserializer::deserialize("AutoMinorVersionUpgrade",
                                                                           stack)));
                        }
                        "CacheClusterCreateTime" => {
                            obj.cache_cluster_create_time =
                                Some(try!(TStampDeserializer::deserialize("CacheClusterCreateTime",
                                                                          stack)));
                        }
                        "CacheClusterId" => {
                            obj.cache_cluster_id =
                                Some(try!(StringDeserializer::deserialize("CacheClusterId",
                                                                          stack)));
                        }
                        "CacheClusterStatus" => {
                            obj.cache_cluster_status =
                                Some(try!(StringDeserializer::deserialize("CacheClusterStatus",
                                                                          stack)));
                        }
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "CacheNodes" => {
                            obj.cache_nodes =
                                Some(try!(CacheNodeListDeserializer::deserialize("CacheNodes",
                                                                                 stack)));
                        }
                        "CacheParameterGroup" => {
                            obj.cache_parameter_group = Some(try!(CacheParameterGroupStatusDeserializer::deserialize("CacheParameterGroup", stack)));
                        }
                        "CacheSecurityGroups" => {
                            obj.cache_security_groups = Some(try!(CacheSecurityGroupMembershipListDeserializer::deserialize("CacheSecurityGroups", stack)));
                        }
                        "CacheSubnetGroupName" => {
                            obj.cache_subnet_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheSubnetGroupName",
                                                                          stack)));
                        }
                        "ClientDownloadLandingPage" => {
                            obj.client_download_landing_page =
                                Some(try!(StringDeserializer::deserialize("ClientDownloadLandingPage",
                                                                          stack)));
                        }
                        "ConfigurationEndpoint" => {
                            obj.configuration_endpoint =
                                Some(try!(EndpointDeserializer::deserialize("ConfigurationEndpoint",
                                                                            stack)));
                        }
                        "Engine" => {
                            obj.engine = Some(try!(StringDeserializer::deserialize("Engine",
                                                                                   stack)));
                        }
                        "EngineVersion" => {
                            obj.engine_version = Some(try!(StringDeserializer::deserialize("EngineVersion",
                                                                                           stack)));
                        }
                        "NotificationConfiguration" => {
                            obj.notification_configuration = Some(try!(NotificationConfigurationDeserializer::deserialize("NotificationConfiguration", stack)));
                        }
                        "NumCacheNodes" => {
                            obj.num_cache_nodes =
                                Some(try!(IntegerOptionalDeserializer::deserialize("NumCacheNodes",
                                                                                   stack)));
                        }
                        "PendingModifiedValues" => {
                            obj.pending_modified_values =
                                Some(try!(PendingModifiedValuesDeserializer::deserialize("PendingModifiedValues",
                                                                                         stack)));
                        }
                        "PreferredAvailabilityZone" => {
                            obj.preferred_availability_zone =
                                Some(try!(StringDeserializer::deserialize("PreferredAvailabilityZone",
                                                                          stack)));
                        }
                        "PreferredMaintenanceWindow" => {
                            obj.preferred_maintenance_window =
                                Some(try!(StringDeserializer::deserialize("PreferredMaintenanceWindow",
                                                                          stack)));
                        }
                        "ReplicationGroupId" => {
                            obj.replication_group_id =
                                Some(try!(StringDeserializer::deserialize("ReplicationGroupId",
                                                                          stack)));
                        }
                        "SecurityGroups" => {
                            obj.security_groups = Some(try!(SecurityGroupMembershipListDeserializer::deserialize("SecurityGroups", stack)));
                        }
                        "SnapshotRetentionLimit" => {
                            obj.snapshot_retention_limit =
                                Some(try!(IntegerOptionalDeserializer::deserialize("SnapshotRetentionLimit",
                                                                                   stack)));
                        }
                        "SnapshotWindow" => {
                            obj.snapshot_window =
                                Some(try!(StringDeserializer::deserialize("SnapshotWindow",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheClusterList = Vec<CacheCluster>;
struct CacheClusterListDeserializer;
impl CacheClusterListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheClusterList, XmlParseError> {

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
                    if name == "CacheCluster" {
                        obj.push(try!(CacheClusterDeserializer::deserialize("CacheCluster",
                                                                            stack)));
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
#[doc="<p>Represents the output of a <code>DescribeCacheClusters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheClusterMessage {
    #[doc="<p>A list of cache clusters. Each item in the list contains detailed information about one cache cluster.</p>"]
    pub cache_clusters: Option<CacheClusterList>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct CacheClusterMessageDeserializer;
impl CacheClusterMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheClusterMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheClusterMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheClusters" => {
                            obj.cache_clusters =
                                Some(try!(CacheClusterListDeserializer::deserialize("CacheClusters",
                                                                                    stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Provides all of the details about a particular cache engine version.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheEngineVersion {
    #[doc="<p>The description of the cache engine.</p>"]
    pub cache_engine_description: Option<String>,
    #[doc="<p>The description of the cache engine version.</p>"]
    pub cache_engine_version_description: Option<String>,
    #[doc="<p>The name of the cache parameter group family associated with this cache engine.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p>"]
    pub cache_parameter_group_family: Option<String>,
    #[doc="<p>The name of the cache engine.</p>"]
    pub engine: Option<String>,
    #[doc="<p>The version number of the cache engine.</p>"]
    pub engine_version: Option<String>,
}

struct CacheEngineVersionDeserializer;
impl CacheEngineVersionDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheEngineVersion, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheEngineVersion::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheEngineDescription" => {
                            obj.cache_engine_description =
                                Some(try!(StringDeserializer::deserialize("CacheEngineDescription",
                                                                          stack)));
                        }
                        "CacheEngineVersionDescription" => {
                            obj.cache_engine_version_description =
                                Some(try!(StringDeserializer::deserialize("CacheEngineVersionDescription",
                                                                          stack)));
                        }
                        "CacheParameterGroupFamily" => {
                            obj.cache_parameter_group_family =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupFamily",
                                                                          stack)));
                        }
                        "Engine" => {
                            obj.engine = Some(try!(StringDeserializer::deserialize("Engine",
                                                                                   stack)));
                        }
                        "EngineVersion" => {
                            obj.engine_version = Some(try!(StringDeserializer::deserialize("EngineVersion",
                                                                                           stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheEngineVersionList = Vec<CacheEngineVersion>;
struct CacheEngineVersionListDeserializer;
impl CacheEngineVersionListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheEngineVersionList, XmlParseError> {

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
                    if name == "CacheEngineVersion" {
                        obj.push(try!(CacheEngineVersionDeserializer::deserialize("CacheEngineVersion", stack)));
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
#[doc="<p>Represents the output of a <a>DescribeCacheEngineVersions</a> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheEngineVersionMessage {
    #[doc="<p>A list of cache engine version details. Each element in the list contains detailed information about one cache engine version.</p>"]
    pub cache_engine_versions: Option<CacheEngineVersionList>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct CacheEngineVersionMessageDeserializer;
impl CacheEngineVersionMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheEngineVersionMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheEngineVersionMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheEngineVersions" => {
                            obj.cache_engine_versions =
                                Some(try!(CacheEngineVersionListDeserializer::deserialize("CacheEngineVersions",
                                                                                          stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents an individual cache node within a cache cluster. Each cache node runs its own instance of the cluster's protocol-compliant caching software - either Memcached or Redis.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheNode {
    #[doc="<p>The date and time when the cache node was created.</p>"]
    pub cache_node_create_time: Option<TStamp>,
    #[doc="<p>The cache node identifier. A node ID is a numeric identifier (0001, 0002, etc.). The combination of cluster ID and node ID uniquely identifies every cache node used in a customer's AWS account.</p>"]
    pub cache_node_id: Option<String>,
    #[doc="<p>The current state of this cache node.</p>"]
    pub cache_node_status: Option<String>,
    #[doc="<p>The Availability Zone where this node was created and now resides.</p>"]
    pub customer_availability_zone: Option<String>,
    #[doc="<p>The hostname for connecting to this cache node.</p>"]
    pub endpoint: Option<Endpoint>,
    #[doc="<p>The status of the parameter group applied to this cache node.</p>"]
    pub parameter_group_status: Option<String>,
    #[doc="<p>The ID of the primary node to which this read replica node is synchronized. If this field is empty, this node is not associated with a primary cache cluster.</p>"]
    pub source_cache_node_id: Option<String>,
}

struct CacheNodeDeserializer;
impl CacheNodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNode, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheNode::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeCreateTime" => {
                            obj.cache_node_create_time =
                                Some(try!(TStampDeserializer::deserialize("CacheNodeCreateTime",
                                                                          stack)));
                        }
                        "CacheNodeId" => {
                            obj.cache_node_id = Some(try!(StringDeserializer::deserialize("CacheNodeId",
                                                                                          stack)));
                        }
                        "CacheNodeStatus" => {
                            obj.cache_node_status =
                                Some(try!(StringDeserializer::deserialize("CacheNodeStatus",
                                                                          stack)));
                        }
                        "CustomerAvailabilityZone" => {
                            obj.customer_availability_zone =
                                Some(try!(StringDeserializer::deserialize("CustomerAvailabilityZone",
                                                                          stack)));
                        }
                        "Endpoint" => {
                            obj.endpoint = Some(try!(EndpointDeserializer::deserialize("Endpoint",
                                                                                       stack)));
                        }
                        "ParameterGroupStatus" => {
                            obj.parameter_group_status =
                                Some(try!(StringDeserializer::deserialize("ParameterGroupStatus",
                                                                          stack)));
                        }
                        "SourceCacheNodeId" => {
                            obj.source_cache_node_id =
                                Some(try!(StringDeserializer::deserialize("SourceCacheNodeId",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheNodeIdsList = Vec<String>;
struct CacheNodeIdsListDeserializer;
impl CacheNodeIdsListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNodeIdsList, XmlParseError> {

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
                    if name == "CacheNodeId" {
                        obj.push(try!(StringDeserializer::deserialize("CacheNodeId", stack)));
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

/// Serialize `CacheNodeIdsList` contents to a `SignedRequest`.
struct CacheNodeIdsListSerializer;
impl CacheNodeIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CacheNodeIdsList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type CacheNodeList = Vec<CacheNode>;
struct CacheNodeListDeserializer;
impl CacheNodeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNodeList, XmlParseError> {

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
                    if name == "CacheNode" {
                        obj.push(try!(CacheNodeDeserializer::deserialize("CacheNode", stack)));
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
#[doc="<p>A parameter that has a different value for each cache node type it is applied to. For example, in a Redis cache cluster, a <code>cache.m1.large</code> cache node type would have a larger <code>maxmemory</code> value than a <code>cache.m1.small</code> type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheNodeTypeSpecificParameter {
    #[doc="<p>The valid range of values for the parameter.</p>"]
    pub allowed_values: Option<String>,
    #[doc="<p>A list of cache node types and their corresponding values for this parameter.</p>"]
    pub cache_node_type_specific_values: Option<CacheNodeTypeSpecificValueList>,
    #[doc="<p>Indicates whether a change to the parameter is applied immediately or requires a reboot for the change to be applied. You can force a reboot or wait until the next maintenance window's reboot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Clusters.Rebooting.html\">Rebooting a Cluster</a>.</p>"]
    pub change_type: Option<ChangeType>,
    #[doc="<p>The valid data type for the parameter.</p>"]
    pub data_type: Option<String>,
    #[doc="<p>A description of the parameter.</p>"]
    pub description: Option<String>,
    #[doc="<p>Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed.</p>"]
    pub is_modifiable: Option<Boolean>,
    #[doc="<p>The earliest cache engine version to which the parameter can apply.</p>"]
    pub minimum_engine_version: Option<String>,
    #[doc="<p>The name of the parameter.</p>"]
    pub parameter_name: Option<String>,
    #[doc="<p>The source of the parameter value.</p>"]
    pub source: Option<String>,
}

struct CacheNodeTypeSpecificParameterDeserializer;
impl CacheNodeTypeSpecificParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNodeTypeSpecificParameter, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheNodeTypeSpecificParameter::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AllowedValues" => {
                            obj.allowed_values = Some(try!(StringDeserializer::deserialize("AllowedValues",
                                                                                           stack)));
                        }
                        "CacheNodeTypeSpecificValues" => {
                            obj.cache_node_type_specific_values = Some(try!(CacheNodeTypeSpecificValueListDeserializer::deserialize("CacheNodeTypeSpecificValues", stack)));
                        }
                        "ChangeType" => {
                            obj.change_type =
                                Some(try!(ChangeTypeDeserializer::deserialize("ChangeType",
                                                                              stack)));
                        }
                        "DataType" => {
                            obj.data_type = Some(try!(StringDeserializer::deserialize("DataType",
                                                                                      stack)));
                        }
                        "Description" => {
                            obj.description = Some(try!(StringDeserializer::deserialize("Description",
                                                                                        stack)));
                        }
                        "IsModifiable" => {
                            obj.is_modifiable = Some(try!(BooleanDeserializer::deserialize("IsModifiable",
                                                                                           stack)));
                        }
                        "MinimumEngineVersion" => {
                            obj.minimum_engine_version =
                                Some(try!(StringDeserializer::deserialize("MinimumEngineVersion",
                                                                          stack)));
                        }
                        "ParameterName" => {
                            obj.parameter_name = Some(try!(StringDeserializer::deserialize("ParameterName",
                                                                                           stack)));
                        }
                        "Source" => {
                            obj.source = Some(try!(StringDeserializer::deserialize("Source",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheNodeTypeSpecificParametersList = Vec<CacheNodeTypeSpecificParameter>;
struct CacheNodeTypeSpecificParametersListDeserializer;
impl CacheNodeTypeSpecificParametersListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<CacheNodeTypeSpecificParametersList, XmlParseError> {

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
                    if name == "CacheNodeTypeSpecificParameter" {
                        obj.push(try!(CacheNodeTypeSpecificParameterDeserializer::deserialize("CacheNodeTypeSpecificParameter", stack)));
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
#[doc="<p>A value that applies only to a certain cache node type.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheNodeTypeSpecificValue {
    #[doc="<p>The cache node type for which this value applies.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The value for the cache node type.</p>"]
    pub value: Option<String>,
}

struct CacheNodeTypeSpecificValueDeserializer;
impl CacheNodeTypeSpecificValueDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNodeTypeSpecificValue, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheNodeTypeSpecificValue::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(StringDeserializer::deserialize("Value", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheNodeTypeSpecificValueList = Vec<CacheNodeTypeSpecificValue>;
struct CacheNodeTypeSpecificValueListDeserializer;
impl CacheNodeTypeSpecificValueListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheNodeTypeSpecificValueList, XmlParseError> {

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
                    if name == "CacheNodeTypeSpecificValue" {
                        obj.push(try!(CacheNodeTypeSpecificValueDeserializer::deserialize("CacheNodeTypeSpecificValue", stack)));
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
#[doc="<p>Represents the output of a <code>CreateCacheParameterGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheParameterGroup {
    #[doc="<p>The name of the cache parameter group family that this cache parameter group is compatible with.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p>"]
    pub cache_parameter_group_family: Option<String>,
    #[doc="<p>The name of the cache parameter group.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>The description for this cache parameter group.</p>"]
    pub description: Option<String>,
}

struct CacheParameterGroupDeserializer;
impl CacheParameterGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheParameterGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheParameterGroupFamily" => {
                            obj.cache_parameter_group_family =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupFamily",
                                                                          stack)));
                        }
                        "CacheParameterGroupName" => {
                            obj.cache_parameter_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupName",
                                                                          stack)));
                        }
                        "Description" => {
                            obj.description = Some(try!(StringDeserializer::deserialize("Description",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of a <code>DescribeCacheParameters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheParameterGroupDetails {
    #[doc="<p>A list of parameters specific to a particular cache node type. Each element in the list contains detailed information about one parameter.</p>"]
    pub cache_node_type_specific_parameters: Option<CacheNodeTypeSpecificParametersList>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
    #[doc="<p>A list of <a>Parameter</a> instances.</p>"]
    pub parameters: Option<ParametersList>,
}

struct CacheParameterGroupDetailsDeserializer;
impl CacheParameterGroupDetailsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroupDetails, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheParameterGroupDetails::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeTypeSpecificParameters" => {
                            obj.cache_node_type_specific_parameters = Some(try!(CacheNodeTypeSpecificParametersListDeserializer::deserialize("CacheNodeTypeSpecificParameters", stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "Parameters" => {
                            obj.parameters =
                                Some(try!(ParametersListDeserializer::deserialize("Parameters",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheParameterGroupList = Vec<CacheParameterGroup>;
struct CacheParameterGroupListDeserializer;
impl CacheParameterGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroupList, XmlParseError> {

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
                    if name == "CacheParameterGroup" {
                        obj.push(try!(CacheParameterGroupDeserializer::deserialize("CacheParameterGroup", stack)));
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
#[doc="<p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>ModifyCacheParameterGroup</code> </p> </li> <li> <p> <code>ResetCacheParameterGroup</code> </p> </li> </ul>"]
#[derive(Default,Debug,Clone)]
pub struct CacheParameterGroupNameMessage {
    #[doc="<p>The name of the cache parameter group.</p>"]
    pub cache_parameter_group_name: Option<String>,
}

struct CacheParameterGroupNameMessageDeserializer;
impl CacheParameterGroupNameMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroupNameMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheParameterGroupNameMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheParameterGroupName" => {
                            obj.cache_parameter_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupName",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Status of the cache parameter group.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheParameterGroupStatus {
    #[doc="<p>A list of the cache node IDs which need to be rebooted for parameter changes to be applied. A node ID is a numeric identifier (0001, 0002, etc.).</p>"]
    pub cache_node_ids_to_reboot: Option<CacheNodeIdsList>,
    #[doc="<p>The name of the cache parameter group.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>The status of parameter updates.</p>"]
    pub parameter_apply_status: Option<String>,
}

struct CacheParameterGroupStatusDeserializer;
impl CacheParameterGroupStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroupStatus, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheParameterGroupStatus::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeIdsToReboot" => {
                            obj.cache_node_ids_to_reboot =
                                Some(try!(CacheNodeIdsListDeserializer::deserialize("CacheNodeIdsToReboot",
                                                                                    stack)));
                        }
                        "CacheParameterGroupName" => {
                            obj.cache_parameter_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupName",
                                                                          stack)));
                        }
                        "ParameterApplyStatus" => {
                            obj.parameter_apply_status =
                                Some(try!(StringDeserializer::deserialize("ParameterApplyStatus",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of a <code>DescribeCacheParameterGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheParameterGroupsMessage {
    #[doc="<p>A list of cache parameter groups. Each element in the list contains detailed information about one cache parameter group.</p>"]
    pub cache_parameter_groups: Option<CacheParameterGroupList>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct CacheParameterGroupsMessageDeserializer;
impl CacheParameterGroupsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheParameterGroupsMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheParameterGroupsMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheParameterGroups" => {
                            obj.cache_parameter_groups =
                                Some(try!(CacheParameterGroupListDeserializer::deserialize("CacheParameterGroups",
                                                                                           stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>AuthorizeCacheSecurityGroupIngress</code> </p> </li> <li> <p> <code>CreateCacheSecurityGroup</code> </p> </li> <li> <p> <code>RevokeCacheSecurityGroupIngress</code> </p> </li> </ul>"]
#[derive(Default,Debug,Clone)]
pub struct CacheSecurityGroup {
    #[doc="<p>The name of the cache security group.</p>"]
    pub cache_security_group_name: Option<String>,
    #[doc="<p>The description of the cache security group.</p>"]
    pub description: Option<String>,
    #[doc="<p>A list of Amazon EC2 security groups that are associated with this cache security group.</p>"]
    pub ec2_security_groups: Option<EC2SecurityGroupList>,
    #[doc="<p>The AWS account ID of the cache security group owner.</p>"]
    pub owner_id: Option<String>,
}

struct CacheSecurityGroupDeserializer;
impl CacheSecurityGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSecurityGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheSecurityGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroupName" => {
                            obj.cache_security_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheSecurityGroupName",
                                                                          stack)));
                        }
                        "Description" => {
                            obj.description = Some(try!(StringDeserializer::deserialize("Description",
                                                                                        stack)));
                        }
                        "EC2SecurityGroups" => {
                            obj.ec2_security_groups =
                                Some(try!(EC2SecurityGroupListDeserializer::deserialize("EC2SecurityGroups",
                                                                                        stack)));
                        }
                        "OwnerId" => {
                            obj.owner_id = Some(try!(StringDeserializer::deserialize("OwnerId",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents a cache cluster's status within a particular cache security group.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheSecurityGroupMembership {
    #[doc="<p>The name of the cache security group.</p>"]
    pub cache_security_group_name: Option<String>,
    #[doc="<p>The membership status in the cache security group. The status changes when a cache security group is modified, or when the cache security groups assigned to a cache cluster are modified.</p>"]
    pub status: Option<String>,
}

struct CacheSecurityGroupMembershipDeserializer;
impl CacheSecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheSecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroupName" => {
                            obj.cache_security_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheSecurityGroupName",
                                                                          stack)));
                        }
                        "Status" => {
                            obj.status = Some(try!(StringDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheSecurityGroupMembershipList = Vec<CacheSecurityGroupMembership>;
struct CacheSecurityGroupMembershipListDeserializer;
impl CacheSecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<CacheSecurityGroupMembershipList, XmlParseError> {

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
                    if name == "CacheSecurityGroup" {
                        obj.push(try!(CacheSecurityGroupMembershipDeserializer::deserialize("CacheSecurityGroup", stack)));
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
#[doc="<p>Represents the output of a <code>DescribeCacheSecurityGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheSecurityGroupMessage {
    #[doc="<p>A list of cache security groups. Each element in the list contains detailed information about one group.</p>"]
    pub cache_security_groups: Option<CacheSecurityGroups>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct CacheSecurityGroupMessageDeserializer;
impl CacheSecurityGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSecurityGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheSecurityGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroups" => {
                            obj.cache_security_groups =
                                Some(try!(CacheSecurityGroupsDeserializer::deserialize("CacheSecurityGroups",
                                                                                       stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheSecurityGroupNameList = Vec<String>;

/// Serialize `CacheSecurityGroupNameList` contents to a `SignedRequest`.
struct CacheSecurityGroupNameListSerializer;
impl CacheSecurityGroupNameListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CacheSecurityGroupNameList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type CacheSecurityGroups = Vec<CacheSecurityGroup>;
struct CacheSecurityGroupsDeserializer;
impl CacheSecurityGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSecurityGroups, XmlParseError> {

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
                    if name == "CacheSecurityGroup" {
                        obj.push(try!(CacheSecurityGroupDeserializer::deserialize("CacheSecurityGroup", stack)));
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
#[doc="<p>Represents the output of one of the following operations:</p> <ul> <li> <p> <code>CreateCacheSubnetGroup</code> </p> </li> <li> <p> <code>ModifyCacheSubnetGroup</code> </p> </li> </ul>"]
#[derive(Default,Debug,Clone)]
pub struct CacheSubnetGroup {
    #[doc="<p>The description of the cache subnet group.</p>"]
    pub cache_subnet_group_description: Option<String>,
    #[doc="<p>The name of the cache subnet group.</p>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>A list of subnets associated with the cache subnet group.</p>"]
    pub subnets: Option<SubnetList>,
    #[doc="<p>The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet group.</p>"]
    pub vpc_id: Option<String>,
}

struct CacheSubnetGroupDeserializer;
impl CacheSubnetGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSubnetGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheSubnetGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSubnetGroupDescription" => {
                            obj.cache_subnet_group_description =
                                Some(try!(StringDeserializer::deserialize("CacheSubnetGroupDescription",
                                                                          stack)));
                        }
                        "CacheSubnetGroupName" => {
                            obj.cache_subnet_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheSubnetGroupName",
                                                                          stack)));
                        }
                        "Subnets" => {
                            obj.subnets = Some(try!(SubnetListDeserializer::deserialize("Subnets",
                                                                                        stack)));
                        }
                        "VpcId" => {
                            obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of a <code>DescribeCacheSubnetGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CacheSubnetGroupMessage {
    #[doc="<p>A list of cache subnet groups. Each element in the list contains detailed information about one group.</p>"]
    pub cache_subnet_groups: Option<CacheSubnetGroups>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct CacheSubnetGroupMessageDeserializer;
impl CacheSubnetGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSubnetGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CacheSubnetGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSubnetGroups" => {
                            obj.cache_subnet_groups =
                                Some(try!(CacheSubnetGroupsDeserializer::deserialize("CacheSubnetGroups",
                                                                                     stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type CacheSubnetGroups = Vec<CacheSubnetGroup>;
struct CacheSubnetGroupsDeserializer;
impl CacheSubnetGroupsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CacheSubnetGroups, XmlParseError> {

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
                    if name == "CacheSubnetGroup" {
                        obj.push(try!(CacheSubnetGroupDeserializer::deserialize("CacheSubnetGroup",
                                                                                stack)));
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
pub type ChangeType = String;
struct ChangeTypeDeserializer;
impl ChangeTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ChangeType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type ClusterIdList = Vec<String>;
struct ClusterIdListDeserializer;
impl ClusterIdListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ClusterIdList, XmlParseError> {

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
                    if name == "ClusterId" {
                        obj.push(try!(StringDeserializer::deserialize("ClusterId", stack)));
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
#[doc="<p>Represents the input of a <code>CopySnapshotMessage</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CopySnapshotMessage {
    #[doc="<p>The name of an existing snapshot from which to make a copy.</p>"]
    pub source_snapshot_name: String,
    #[doc="<p>The Amazon S3 bucket to which the snapshot is exported. This parameter is used only when exporting a snapshot for external access.</p> <p>When using this parameter to export a snapshot, be sure Amazon ElastiCache has the needed permissions to this S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the <i>Amazon ElastiCache User Guide</i>.</p> <p>For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html\">Exporting a Snapshot</a> in the <i>Amazon ElastiCache User Guide</i>.</p>"]
    pub target_bucket: Option<String>,
    #[doc="<p>A name for the snapshot copy. ElastiCache does not permit overwriting a snapshot, therefore this name must be unique within its context - ElastiCache or an Amazon S3 bucket if exporting.</p>"]
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

        params.put(&format!("{}{}", prefix, "SourceSnapshotName"),
                   &obj.source_snapshot_name);
        if let Some(ref field_value) = obj.target_bucket {
            params.put(&format!("{}{}", prefix, "TargetBucket"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "TargetSnapshotName"),
                   &obj.target_snapshot_name);

    }
}

#[derive(Default,Debug,Clone)]
pub struct CopySnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct CopySnapshotResultDeserializer;
impl CopySnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CopySnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CopySnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Snapshot" => {
                            obj.snapshot = Some(try!(SnapshotDeserializer::deserialize("Snapshot",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a CreateCacheCluster operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateCacheClusterMessage {
    #[doc="<p>Specifies whether the nodes in this Memcached cluster are created in a single Availability Zone or created across multiple Availability Zones in the cluster's region.</p> <p>This parameter is only supported for Memcached cache clusters.</p> <p>If the <code>AZMode</code> and <code>PreferredAvailabilityZones</code> are not specified, ElastiCache assumes <code>single-az</code> mode.</p>"]
    pub az_mode: Option<AZMode>,
    #[doc="<p>The password used to access a password protected server.</p> <p>Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters.</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length.</p> </li> <li> <p>Cannot contain any of the following characters: '/', '\"', or \"@\". </p> </li> </ul> <p>For more information, see <a href=\"http://redis.io/commands/AUTH\">AUTH password</a> at Redis.</p>"]
    pub auth_token: Option<String>,
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>The node group (shard) identifier. This parameter is stored as a lowercase string.</p> <p> <b>Constraints:</b> </p> <ul> <li> <p>A name must contain from 1 to 20 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul>"]
    pub cache_cluster_id: String,
    #[doc="<p>The compute and memory capacity of the nodes in the node group (shard).</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The name of the parameter group to associate with this cache cluster. If this argument is omitted, the default parameter group for the specified engine is used. You cannot use any parameter group which has <code>cluster-enabled='yes'</code> when creating a cluster.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>A list of security group names to associate with this cache cluster.</p> <p>Use this parameter only when you are creating a cache cluster outside of an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[doc="<p>The name of the subnet group to be used for the cache cluster.</p> <p>Use this parameter only when you are creating a cache cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p> <important> <p>If you're going to launch your cluster in an Amazon VPC, you need to create a subnet group before you start creating a cluster. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SubnetGroups.html\">Subnets and Subnet Groups</a>.</p> </important>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>The name of the cache engine to be used for this cache cluster.</p> <p>Valid values for this parameter are: <code>memcached</code> | <code>redis</code> </p>"]
    pub engine: Option<String>,
    #[doc="<p>The version number of the cache engine to be used for this cache cluster. To view the supported cache engine versions, use the DescribeCacheEngineVersions operation.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SelectEngine.html#VersionManagement\">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cache cluster or replication group and create it anew with the earlier engine version. </p>"]
    pub engine_version: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be the same as the cache cluster owner.</p> </note>"]
    pub notification_topic_arn: Option<String>,
    #[doc="<p>The initial number of cache nodes that the cache cluster has.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p> <p>If you need more than 20 nodes for your Memcached cluster, please fill out the ElastiCache Limit Increase Request form at <a href=\"http://aws.amazon.com/contact-us/elasticache-node-limit-request/\">http://aws.amazon.com/contact-us/elasticache-node-limit-request/</a>.</p>"]
    pub num_cache_nodes: Option<IntegerOptional>,
    #[doc="<p>The port number on which each of the cache nodes accepts connections.</p>"]
    pub port: Option<IntegerOptional>,
    #[doc="<p>The EC2 Availability Zone in which the cache cluster is created.</p> <p>All nodes belonging to this Memcached cache cluster are placed in the preferred Availability Zone. If you want to create your nodes across multiple Availability Zones, use <code>PreferredAvailabilityZones</code>.</p> <p>Default: System chosen Availability Zone.</p>"]
    pub preferred_availability_zone: Option<String>,
    #[doc="<p>A list of the Availability Zones in which cache nodes are created. The order of the zones in the list is not important.</p> <p>This option is only supported on Memcached.</p> <note> <p>If you are creating your cache cluster in an Amazon VPC (recommended) you can only locate nodes in Availability Zones that are associated with the subnets in the selected subnet group.</p> <p>The number of Availability Zones listed must equal the value of <code>NumCacheNodes</code>.</p> </note> <p>If you want all the nodes in the same Availability Zone, use <code>PreferredAvailabilityZone</code> instead, or repeat the Availability Zone multiple times in the list.</p> <p>Default: System chosen Availability Zones.</p>"]
    pub preferred_availability_zones: Option<PreferredAvailabilityZoneList>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cache cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important> <p>The ID of the replication group to which this cache cluster should belong. If this parameter is specified, the cache cluster is added to the specified replication group as a read replica; otherwise, the cache cluster is a standalone primary that is not part of any replication group.</p> <p>If the specified replication group is Multi-AZ enabled and the Availability Zone is not specified, the cache cluster is created in Availability Zones that provide the best spread of read replicas across Availability Zones.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>One or more VPC security groups associated with the cache cluster.</p> <p>Use this parameter only when you are creating a cache cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[doc="<p>A single-element string list containing an Amazon Resource Name (ARN) that uniquely identifies a Redis RDB snapshot file stored in Amazon S3. The snapshot file is used to populate the node group (shard). The Amazon S3 object name in the ARN cannot contain any commas.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code> </p>"]
    pub snapshot_arns: Option<SnapshotArnsList>,
    #[doc="<p>The name of a Redis snapshot from which to restore data into the new node group (shard). The snapshot status changes to <code>restoring</code> while the new node group (shard) is being created.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note>"]
    pub snapshot_name: Option<String>,
    #[doc="<p>The number of days for which ElastiCache retains automatic snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot taken today is retained for 5 days before being deleted.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Default: 0 (i.e., automatic backups are disabled for this cache cluster).</p>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p> <p> <b>Note:</b> This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p>"]
    pub snapshot_window: Option<String>,
    #[doc="<p>A list of cost allocation tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value.</p>"]
    pub tags: Option<TagList>,
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
            params.put(&format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                       &field_value.to_string());
        }
        params.put(&format!("{}{}", prefix, "CacheClusterId"),
                   &obj.cache_cluster_id);
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(params,
                                                            &format!("{}{}",
                                                                    prefix,
                                                                    "CacheSecurityGroupNames"),
                                                            field_value);
        }
        if let Some(ref field_value) = obj.cache_subnet_group_name {
            params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(&format!("{}{}", prefix, "NotificationTopicArn"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.num_cache_nodes {
            params.put(&format!("{}{}", prefix, "NumCacheNodes"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.preferred_availability_zone {
            params.put(&format!("{}{}", prefix, "PreferredAvailabilityZone"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.preferred_availability_zones {
            PreferredAvailabilityZoneListSerializer::serialize(params,
                                                               &format!("{}{}",
                                                                       prefix,
                                                                       "PreferredAvailabilityZones"),
                                                               field_value);
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(&format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(params,
                                                      &format!("{}{}", prefix, "SecurityGroupIds"),
                                                      field_value);
        }
        if let Some(ref field_value) = obj.snapshot_arns {
            SnapshotArnsListSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "SnapshotArns"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(&format!("{}{}", prefix, "SnapshotRetentionLimit"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

struct CreateCacheClusterResultDeserializer;
impl CreateCacheClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateCacheClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateCacheClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheCluster" => {
                            obj.cache_cluster =
                                Some(try!(CacheClusterDeserializer::deserialize("CacheCluster",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>CreateCacheParameterGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateCacheParameterGroupMessage {
    #[doc="<p>The name of the cache parameter group family that the cache parameter group can be used with.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p>"]
    pub cache_parameter_group_family: String,
    #[doc="<p>A user-specified name for the cache parameter group.</p>"]
    pub cache_parameter_group_name: String,
    #[doc="<p>A user-specified description for the cache parameter group.</p>"]
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

        params.put(&format!("{}{}", prefix, "CacheParameterGroupFamily"),
                   &obj.cache_parameter_group_family);
        params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                   &obj.cache_parameter_group_name);
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateCacheParameterGroupResult {
    pub cache_parameter_group: Option<CacheParameterGroup>,
}

struct CreateCacheParameterGroupResultDeserializer;
impl CreateCacheParameterGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<CreateCacheParameterGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateCacheParameterGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheParameterGroup" => {
                            obj.cache_parameter_group =
                                Some(try!(CacheParameterGroupDeserializer::deserialize("CacheParameterGroup",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>CreateCacheSecurityGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateCacheSecurityGroupMessage {
    #[doc="<p>A name for the cache security group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Cannot be the word \"Default\".</p> <p>Example: <code>mysecuritygroup</code> </p>"]
    pub cache_security_group_name: String,
    #[doc="<p>A description for the cache security group.</p>"]
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

        params.put(&format!("{}{}", prefix, "CacheSecurityGroupName"),
                   &obj.cache_security_group_name);
        params.put(&format!("{}{}", prefix, "Description"), &obj.description);

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateCacheSecurityGroupResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

struct CreateCacheSecurityGroupResultDeserializer;
impl CreateCacheSecurityGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateCacheSecurityGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateCacheSecurityGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroup" => {
                            obj.cache_security_group =
                                Some(try!(CacheSecurityGroupDeserializer::deserialize("CacheSecurityGroup",
                                                                                      stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>CreateCacheSubnetGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateCacheSubnetGroupMessage {
    #[doc="<p>A description for the cache subnet group.</p>"]
    pub cache_subnet_group_description: String,
    #[doc="<p>A name for the cache subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p> <p>Example: <code>mysubnetgroup</code> </p>"]
    pub cache_subnet_group_name: String,
    #[doc="<p>A list of VPC subnet IDs for the cache subnet group.</p>"]
    pub subnet_ids: SubnetIdentifierList,
}


/// Serialize `CreateCacheSubnetGroupMessage` contents to a `SignedRequest`.
struct CreateCacheSubnetGroupMessageSerializer;
impl CreateCacheSubnetGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateCacheSubnetGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheSubnetGroupDescription"),
                   &obj.cache_subnet_group_description);
        params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                   &obj.cache_subnet_group_name);
        SubnetIdentifierListSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "SubnetIds"),
                                                  &obj.subnet_ids);

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateCacheSubnetGroupResult {
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

struct CreateCacheSubnetGroupResultDeserializer;
impl CreateCacheSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateCacheSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateCacheSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSubnetGroup" => {
                            obj.cache_subnet_group =
                                Some(try!(CacheSubnetGroupDeserializer::deserialize("CacheSubnetGroup",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>CreateReplicationGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateReplicationGroupMessage {
    #[doc="<p>The password used to access a password protected server.</p> <p>Password constraints:</p> <ul> <li> <p>Must be only printable ASCII characters.</p> </li> <li> <p>Must be at least 16 characters and no more than 128 characters in length.</p> </li> <li> <p>Cannot contain any of the following characters: '/', '\"', or \"@\". </p> </li> </ul> <p>For more information, see <a href=\"http://redis.io/commands/AUTH\">AUTH password</a> at Redis.</p>"]
    pub auth_token: Option<String>,
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>Specifies whether a read-only replica is automatically promoted to read/write primary if the existing primary fails.</p> <p>If <code>true</code>, Multi-AZ is enabled for this replication group. If <code>false</code>, Multi-AZ is disabled for this replication group.</p> <p> <code>AutomaticFailoverEnabled</code> must be enabled for Redis (cluster mode enabled) replication groups.</p> <p>Default: false</p> <note> <p>ElastiCache Multi-AZ replication groups is not supported on:</p> <ul> <li> <p>Redis versions earlier than 2.8.6.</p> </li> <li> <p>Redis (cluster mode disabled): T1 and T2 node types.</p> <p>Redis (cluster mode enabled): T2 node types.</p> </li> </ul> </note>"]
    pub automatic_failover_enabled: Option<BooleanOptional>,
    #[doc="<p>The compute and memory capacity of the nodes in the node group (shard).</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The name of the parameter group to associate with this replication group. If this argument is omitted, the default cache parameter group for the specified engine is used.</p> <p>If you are running Redis version 3.2.4 or later, only one node group (shard), and want to use a default parameter group, we recommend that you specify the parameter group by name. </p> <ul> <li> <p>To create a Redis (cluster mode disabled) replication group, use <code>CacheParameterGroupName=default.redis3.2</code>.</p> </li> <li> <p>To create a Redis (cluster mode enabled) replication group, use <code>CacheParameterGroupName=default.redis3.2.cluster.on</code>.</p> </li> </ul>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>A list of cache security group names to associate with this replication group.</p>"]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[doc="<p>The name of the cache subnet group to be used for the replication group.</p> <important> <p>If you're going to launch your cluster in an Amazon VPC, you need to create a subnet group before you start creating a cluster. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SubnetGroups.html\">Subnets and Subnet Groups</a>.</p> </important>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>The name of the cache engine to be used for the cache clusters in this replication group.</p>"]
    pub engine: Option<String>,
    #[doc="<p>The version number of the cache engine to be used for the cache clusters in this replication group. To view the supported cache engine versions, use the <code>DescribeCacheEngineVersions</code> operation.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SelectEngine.html#VersionManagement\">Selecting a Cache Engine and Version</a>) in the <i>ElastiCache User Guide</i>, but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cache cluster or replication group and create it anew with the earlier engine version. </p>"]
    pub engine_version: Option<String>,
    #[doc="<p>A list of node group (shard) configuration options. Each node group (shard) configuration has the following: Slots, PrimaryAvailabilityZone, ReplicaAvailabilityZones, ReplicaCount.</p> <p>If you're creating a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group, you can use this parameter to configure one node group (shard) or you can omit this parameter.</p>"]
    pub node_group_configuration: Option<NodeGroupConfigurationList>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon Simple Notification Service (SNS) topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be the same as the cache cluster owner.</p> </note>"]
    pub notification_topic_arn: Option<String>,
    #[doc="<p>The number of clusters this replication group initially has.</p> <p>This parameter is not used if there is more than one node group (shard). You should use <code>ReplicasPerNodeGroup</code> instead.</p> <p>If <code>Multi-AZ</code> is <code>enabled</code>, the value of this parameter must be at least 2.</p> <p>The maximum permitted value for <code>NumCacheClusters</code> is 6 (primary plus 5 replicas).</p>"]
    pub num_cache_clusters: Option<IntegerOptional>,
    #[doc="<p>An optional parameter that specifies the number of node groups (shards) for this Redis (cluster mode enabled) replication group. For Redis (cluster mode disabled) either omit this parameter or set it to 1.</p> <p>Default: 1</p>"]
    pub num_node_groups: Option<IntegerOptional>,
    #[doc="<p>The port number on which each member of the replication group accepts connections.</p>"]
    pub port: Option<IntegerOptional>,
    #[doc="<p>A list of EC2 Availability Zones in which the replication group's cache clusters are created. The order of the Availability Zones in the list is the order in which clusters are allocated. The primary cluster is created in the first AZ in the list.</p> <p>This parameter is not used if there is more than one node group (shard). You should use <code>NodeGroupConfiguration</code> instead.</p> <note> <p>If you are creating your replication group in an Amazon VPC (recommended), you can only locate cache clusters in Availability Zones associated with the subnets in the selected subnet group.</p> <p>The number of Availability Zones listed must equal the value of <code>NumCacheClusters</code>.</p> </note> <p>Default: system chosen Availability Zones.</p>"]
    pub preferred_cache_cluster_a_zs: Option<AvailabilityZonesList>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cache cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period. Valid values for <code>ddd</code> are:</p> <p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>The identifier of the cache cluster that serves as the primary for this replication group. This cache cluster must already exist and have a status of <code>available</code>.</p> <p>This parameter is not required if <code>NumCacheClusters</code>, <code>NumNodeGroups</code>, or <code>ReplicasPerNodeGroup</code> is specified.</p>"]
    pub primary_cluster_id: Option<String>,
    #[doc="<p>An optional parameter that specifies the number of replica nodes in each node group (shard). Valid values are 0 to 5.</p>"]
    pub replicas_per_node_group: Option<IntegerOptional>,
    #[doc="<p>A user-created description for the replication group.</p>"]
    pub replication_group_description: String,
    #[doc="<p>The replication group identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>A name must contain from 1 to 20 alphanumeric characters or hyphens.</p> </li> <li> <p>The first character must be a letter.</p> </li> <li> <p>A name cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul>"]
    pub replication_group_id: String,
    #[doc="<p>One or more Amazon VPC security groups associated with this replication group.</p> <p>Use this parameter only when you are creating a replication group in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[doc="<p>A list of Amazon Resource Names (ARN) that uniquely identify the Redis RDB snapshot files stored in Amazon S3. The snapshot files are used to populate the replication group. The Amazon S3 object name in the ARN cannot contain any commas. The list must match the number of node groups (shards) in the replication group, which means you cannot repartition.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Example of an Amazon S3 ARN: <code>arn:aws:s3:::my_bucket/snapshot1.rdb</code> </p>"]
    pub snapshot_arns: Option<SnapshotArnsList>,
    #[doc="<p>The name of a snapshot from which to restore data into the new replication group. The snapshot status changes to <code>restoring</code> while the new replication group is being created.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note>"]
    pub snapshot_name: Option<String>,
    #[doc="<p>The number of days for which ElastiCache retains automatic snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note> <p>Default: 0 (i.e., automatic backups are disabled for this cache cluster).</p>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p> <note> <p>This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p> </note>"]
    pub snapshot_window: Option<String>,
    #[doc="<p>A list of cost allocation tags to be added to this resource. A tag is a key-value pair. A tag key must be accompanied by a tag value.</p>"]
    pub tags: Option<TagList>,
}


/// Serialize `CreateReplicationGroupMessage` contents to a `SignedRequest`.
struct CreateReplicationGroupMessageSerializer;
impl CreateReplicationGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &CreateReplicationGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.auth_token {
            params.put(&format!("{}{}", prefix, "AuthToken"), &field_value);
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(&format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.automatic_failover_enabled {
            params.put(&format!("{}{}", prefix, "AutomaticFailoverEnabled"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(params,
                                                            &format!("{}{}",
                                                                    prefix,
                                                                    "CacheSecurityGroupNames"),
                                                            field_value);
        }
        if let Some(ref field_value) = obj.cache_subnet_group_name {
            params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.engine {
            params.put(&format!("{}{}", prefix, "Engine"), &field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.node_group_configuration {
            NodeGroupConfigurationListSerializer::serialize(params,
                                                            &format!("{}{}",
                                                                    prefix,
                                                                    "NodeGroupConfiguration"),
                                                            field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(&format!("{}{}", prefix, "NotificationTopicArn"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.num_cache_clusters {
            params.put(&format!("{}{}", prefix, "NumCacheClusters"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.num_node_groups {
            params.put(&format!("{}{}", prefix, "NumNodeGroups"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.port {
            params.put(&format!("{}{}", prefix, "Port"), &field_value.to_string());
        }
        if let Some(ref field_value) = obj.preferred_cache_cluster_a_zs {
            AvailabilityZonesListSerializer::serialize(params,
                                                       &format!("{}{}",
                                                               prefix,
                                                               "PreferredCacheClusterAZs"),
                                                       field_value);
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(&format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.primary_cluster_id {
            params.put(&format!("{}{}", prefix, "PrimaryClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.replicas_per_node_group {
            params.put(&format!("{}{}", prefix, "ReplicasPerNodeGroup"),
                       &field_value.to_string());
        }
        params.put(&format!("{}{}", prefix, "ReplicationGroupDescription"),
                   &obj.replication_group_description);
        params.put(&format!("{}{}", prefix, "ReplicationGroupId"),
                   &obj.replication_group_id);
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(params,
                                                      &format!("{}{}", prefix, "SecurityGroupIds"),
                                                      field_value);
        }
        if let Some(ref field_value) = obj.snapshot_arns {
            SnapshotArnsListSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "SnapshotArns"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(&format!("{}{}", prefix, "SnapshotRetentionLimit"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.tags {
            TagListSerializer::serialize(params, &format!("{}{}", prefix, "Tags"), field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

struct CreateReplicationGroupResultDeserializer;
impl CreateReplicationGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateReplicationGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateReplicationGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ReplicationGroup" => {
                            obj.replication_group =
                                Some(try!(ReplicationGroupDeserializer::deserialize("ReplicationGroup",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>CreateSnapshot</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct CreateSnapshotMessage {
    #[doc="<p>The identifier of an existing cache cluster. The snapshot is created from this cache cluster.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The identifier of an existing replication group. The snapshot is created from this replication group.</p>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>A name for the snapshot being created.</p>"]
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
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        params.put(&format!("{}{}", prefix, "SnapshotName"), &obj.snapshot_name);

    }
}

#[derive(Default,Debug,Clone)]
pub struct CreateSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct CreateSnapshotResultDeserializer;
impl CreateSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<CreateSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = CreateSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Snapshot" => {
                            obj.snapshot = Some(try!(SnapshotDeserializer::deserialize("Snapshot",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DeleteCacheCluster</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteCacheClusterMessage {
    #[doc="<p>The cache cluster identifier for the cluster to be deleted. This parameter is not case sensitive.</p>"]
    pub cache_cluster_id: String,
    #[doc="<p>The user-supplied name of a final cache cluster snapshot. This is the unique name that identifies the snapshot. ElastiCache creates the snapshot, and then deletes the cache cluster immediately afterward.</p>"]
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

        params.put(&format!("{}{}", prefix, "CacheClusterId"),
                   &obj.cache_cluster_id);
        if let Some(ref field_value) = obj.final_snapshot_identifier {
            params.put(&format!("{}{}", prefix, "FinalSnapshotIdentifier"),
                       &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DeleteCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

struct DeleteCacheClusterResultDeserializer;
impl DeleteCacheClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteCacheClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteCacheClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheCluster" => {
                            obj.cache_cluster =
                                Some(try!(CacheClusterDeserializer::deserialize("CacheCluster",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DeleteCacheParameterGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteCacheParameterGroupMessage {
    #[doc="<p>The name of the cache parameter group to delete.</p> <note> <p>The specified cache security group must not be associated with any cache clusters.</p> </note>"]
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

        params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                   &obj.cache_parameter_group_name);

    }
}

#[doc="<p>Represents the input of a <code>DeleteCacheSecurityGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteCacheSecurityGroupMessage {
    #[doc="<p>The name of the cache security group to delete.</p> <note> <p>You cannot delete the default security group.</p> </note>"]
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

        params.put(&format!("{}{}", prefix, "CacheSecurityGroupName"),
                   &obj.cache_security_group_name);

    }
}

#[doc="<p>Represents the input of a <code>DeleteCacheSubnetGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteCacheSubnetGroupMessage {
    #[doc="<p>The name of the cache subnet group to delete.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p>"]
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

        params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                   &obj.cache_subnet_group_name);

    }
}

#[doc="<p>Represents the input of a <code>DeleteReplicationGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteReplicationGroupMessage {
    #[doc="<p>The name of a final node group (shard) snapshot. ElastiCache creates the snapshot from the primary node in the cluster, rather than one of the replicas; this is to ensure that it captures the freshest data. After the final snapshot is taken, the replication group is immediately deleted.</p>"]
    pub final_snapshot_identifier: Option<String>,
    #[doc="<p>The identifier for the cluster to be deleted. This parameter is not case sensitive.</p>"]
    pub replication_group_id: String,
    #[doc="<p>If set to <code>true</code>, all of the read replicas are deleted, but the primary node is retained.</p>"]
    pub retain_primary_cluster: Option<BooleanOptional>,
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
            params.put(&format!("{}{}", prefix, "FinalSnapshotIdentifier"),
                       &field_value);
        }
        params.put(&format!("{}{}", prefix, "ReplicationGroupId"),
                   &obj.replication_group_id);
        if let Some(ref field_value) = obj.retain_primary_cluster {
            params.put(&format!("{}{}", prefix, "RetainPrimaryCluster"),
                       &field_value.to_string());
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DeleteReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

struct DeleteReplicationGroupResultDeserializer;
impl DeleteReplicationGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteReplicationGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteReplicationGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ReplicationGroup" => {
                            obj.replication_group =
                                Some(try!(ReplicationGroupDeserializer::deserialize("ReplicationGroup",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DeleteSnapshot</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DeleteSnapshotMessage {
    #[doc="<p>The name of the snapshot to be deleted.</p>"]
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

#[derive(Default,Debug,Clone)]
pub struct DeleteSnapshotResult {
    pub snapshot: Option<Snapshot>,
}

struct DeleteSnapshotResultDeserializer;
impl DeleteSnapshotResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DeleteSnapshotResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DeleteSnapshotResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Snapshot" => {
                            obj.snapshot = Some(try!(SnapshotDeserializer::deserialize("Snapshot",
                                                                                       stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DescribeCacheClusters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheClustersMessage {
    #[doc="<p>The user-supplied cluster identifier. If this parameter is specified, only information about that specific cache cluster is returned. This parameter isn't case sensitive.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>An optional flag that can be included in the DescribeCacheCluster request to retrieve information about the individual cache nodes.</p>"]
    pub show_cache_node_info: Option<BooleanOptional>,
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.show_cache_node_info {
            params.put(&format!("{}{}", prefix, "ShowCacheNodeInfo"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeCacheEngineVersions</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheEngineVersionsMessage {
    #[doc="<p>The name of a specific cache parameter group family to return details for.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p> <p>Constraints:</p> <ul> <li> <p>Must be 1 to 255 alphanumeric characters</p> </li> <li> <p>First character must be a letter</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens</p> </li> </ul>"]
    pub cache_parameter_group_family: Option<String>,
    #[doc="<p>If <code>true</code>, specifies that only the default version of the specified engine or engine and major version combination is to be returned.</p>"]
    pub default_only: Option<Boolean>,
    #[doc="<p>The cache engine to return. Valid values: <code>memcached</code> | <code>redis</code> </p>"]
    pub engine: Option<String>,
    #[doc="<p>The cache engine version to return.</p> <p>Example: <code>1.4.14</code> </p>"]
    pub engine_version: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
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
            params.put(&format!("{}{}", prefix, "CacheParameterGroupFamily"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.default_only {
            params.put(&format!("{}{}", prefix, "DefaultOnly"),
                       &field_value.to_string());
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeCacheParameterGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheParameterGroupsMessage {
    #[doc="<p>The name of a specific cache parameter group to return details for.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
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
            params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeCacheParameters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheParametersMessage {
    #[doc="<p>The name of a specific cache parameter group to return details for.</p>"]
    pub cache_parameter_group_name: String,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The parameter types to return.</p> <p>Valid values: <code>user</code> | <code>system</code> | <code>engine-default</code> </p>"]
    pub source: Option<String>,
}


/// Serialize `DescribeCacheParametersMessage` contents to a `SignedRequest`.
struct DescribeCacheParametersMessageSerializer;
impl DescribeCacheParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeCacheParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                   &obj.cache_parameter_group_name);
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.source {
            params.put(&format!("{}{}", prefix, "Source"), &field_value);
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeCacheSecurityGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheSecurityGroupsMessage {
    #[doc="<p>The name of the cache security group to return details for.</p>"]
    pub cache_security_group_name: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
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
            params.put(&format!("{}{}", prefix, "CacheSecurityGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeCacheSubnetGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeCacheSubnetGroupsMessage {
    #[doc="<p>The name of the cache subnet group to return details for.</p>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
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
            params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeEngineDefaultParameters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeEngineDefaultParametersMessage {
    #[doc="<p>The name of the cache parameter group family.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p>"]
    pub cache_parameter_group_family: String,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
}


/// Serialize `DescribeEngineDefaultParametersMessage` contents to a `SignedRequest`.
struct DescribeEngineDefaultParametersMessageSerializer;
impl DescribeEngineDefaultParametersMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &DescribeEngineDefaultParametersMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheParameterGroupFamily"),
                   &obj.cache_parameter_group_family);
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct DescribeEngineDefaultParametersResult {
    pub engine_defaults: Option<EngineDefaults>,
}

struct DescribeEngineDefaultParametersResultDeserializer;
impl DescribeEngineDefaultParametersResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<DescribeEngineDefaultParametersResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeEngineDefaultParametersResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "EngineDefaults" => {
                            obj.engine_defaults =
                                Some(try!(EngineDefaultsDeserializer::deserialize("EngineDefaults",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DescribeEvents</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeEventsMessage {
    #[doc="<p>The number of minutes' worth of events to retrieve.</p>"]
    pub duration: Option<IntegerOptional>,
    #[doc="<p>The end of the time interval for which to retrieve events, specified in ISO 8601 format.</p>"]
    pub end_time: Option<TStamp>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The identifier of the event source for which events are returned. If not specified, all sources are included in the response.</p>"]
    pub source_identifier: Option<String>,
    #[doc="<p>The event source to retrieve events for. If no value is specified, all events are returned.</p>"]
    pub source_type: Option<SourceType>,
    #[doc="<p>The beginning of the time interval to retrieve events for, specified in ISO 8601 format.</p>"]
    pub start_time: Option<TStamp>,
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
            params.put(&format!("{}{}", prefix, "Duration"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.end_time {
            params.put(&format!("{}{}", prefix, "EndTime"), &field_value);
        }
        if let Some(ref field_value) = obj.marker {
            params.put(&format!("{}{}", prefix, "Marker"), &field_value);
        }
        if let Some(ref field_value) = obj.max_records {
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
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

#[doc="<p>Represents the input of a <code>DescribeReplicationGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeReplicationGroupsMessage {
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The identifier for the replication group to be described. This parameter is not case sensitive.</p> <p>If you do not specify this parameter, information about all replication groups is returned.</p>"]
    pub replication_group_id: Option<String>,
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeReservedCacheNodes</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeReservedCacheNodesMessage {
    #[doc="<p>The cache node type filter value. Use this parameter to show only those reservations matching the specified cache node type.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The duration filter value, specified in years or seconds. Use this parameter to show only reservations for this duration.</p> <p>Valid Values: <code>1 | 3 | 31536000 | 94608000</code> </p>"]
    pub duration: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The offering type filter value. Use this parameter to show only the available offerings matching the specified offering type.</p> <p>Valid values: <code>\"Light Utilization\"|\"Medium Utilization\"|\"Heavy Utilization\"</code> </p>"]
    pub offering_type: Option<String>,
    #[doc="<p>The product description filter value. Use this parameter to show only those reservations matching the specified product description.</p>"]
    pub product_description: Option<String>,
    #[doc="<p>The reserved cache node identifier filter value. Use this parameter to show only the reservation that matches the specified reservation ID.</p>"]
    pub reserved_cache_node_id: Option<String>,
    #[doc="<p>The offering identifier filter value. Use this parameter to show only purchased reservations matching the specified offering identifier.</p>"]
    pub reserved_cache_nodes_offering_id: Option<String>,
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.offering_type {
            params.put(&format!("{}{}", prefix, "OfferingType"), &field_value);
        }
        if let Some(ref field_value) = obj.product_description {
            params.put(&format!("{}{}", prefix, "ProductDescription"), &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_node_id {
            params.put(&format!("{}{}", prefix, "ReservedCacheNodeId"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_nodes_offering_id {
            params.put(&format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
                       &field_value);
        }

    }
}

#[doc="<p>Represents the input of a <code>DescribeReservedCacheNodesOfferings</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeReservedCacheNodesOfferingsMessage {
    #[doc="<p>The cache node type filter value. Use this parameter to show only the available offerings matching the specified cache node type.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>Duration filter value, specified in years or seconds. Use this parameter to show only reservations for a given duration.</p> <p>Valid Values: <code>1 | 3 | 31536000 | 94608000</code> </p>"]
    pub duration: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 100</p> <p>Constraints: minimum 20; maximum 100.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The offering type filter value. Use this parameter to show only the available offerings matching the specified offering type.</p> <p>Valid Values: <code>\"Light Utilization\"|\"Medium Utilization\"|\"Heavy Utilization\"</code> </p>"]
    pub offering_type: Option<String>,
    #[doc="<p>The product description filter value. Use this parameter to show only the available offerings matching the specified product description.</p>"]
    pub product_description: Option<String>,
    #[doc="<p>The offering identifier filter value. Use this parameter to show only the available offering that matches the specified reservation identifier.</p> <p>Example: <code>438012d3-4052-4cc7-b2e3-8d3372e0e706</code> </p>"]
    pub reserved_cache_nodes_offering_id: Option<String>,
}


/// Serialize `DescribeReservedCacheNodesOfferingsMessage` contents to a `SignedRequest`.
struct DescribeReservedCacheNodesOfferingsMessageSerializer;
impl DescribeReservedCacheNodesOfferingsMessageSerializer {
    fn serialize(params: &mut Params,
                 name: &str,
                 obj: &DescribeReservedCacheNodesOfferingsMessage) {
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.offering_type {
            params.put(&format!("{}{}", prefix, "OfferingType"), &field_value);
        }
        if let Some(ref field_value) = obj.product_description {
            params.put(&format!("{}{}", prefix, "ProductDescription"), &field_value);
        }
        if let Some(ref field_value) = obj.reserved_cache_nodes_offering_id {
            params.put(&format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
                       &field_value);
        }

    }
}

#[doc="<p>Represents the output of a <code>DescribeSnapshots</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeSnapshotsListMessage {
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>A list of snapshots. Each item in the list contains detailed information about one snapshot.</p>"]
    pub snapshots: Option<SnapshotList>,
}

struct DescribeSnapshotsListMessageDeserializer;
impl DescribeSnapshotsListMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<DescribeSnapshotsListMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = DescribeSnapshotsListMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "Snapshots" => {
                            obj.snapshots =
                                Some(try!(SnapshotListDeserializer::deserialize("Snapshots",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>DescribeSnapshotsMessage</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct DescribeSnapshotsMessage {
    #[doc="<p>A user-supplied cluster identifier. If this parameter is specified, only snapshots associated with that specific cache cluster are described.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>An optional marker returned from a prior request. Use this marker for pagination of results from this operation. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>"]
    pub marker: Option<String>,
    #[doc="<p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a marker is included in the response so that the remaining results can be retrieved.</p> <p>Default: 50</p> <p>Constraints: minimum 20; maximum 50.</p>"]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>A user-supplied replication group identifier. If this parameter is specified, only snapshots associated with that specific replication group are described.</p>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>A Boolean value which if true, the node group (shard) configuration is included in the snapshot description.</p>"]
    pub show_node_group_config: Option<BooleanOptional>,
    #[doc="<p>A user-supplied name of the snapshot. If this parameter is specified, only this snapshot are described.</p>"]
    pub snapshot_name: Option<String>,
    #[doc="<p>If set to <code>system</code>, the output shows snapshots that were automatically created by ElastiCache. If set to <code>user</code> the output shows snapshots that were manually created. If omitted, the output shows both automatically and manually created snapshots.</p>"]
    pub snapshot_source: Option<String>,
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
            params.put(&format!("{}{}", prefix, "MaxRecords"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.replication_group_id {
            params.put(&format!("{}{}", prefix, "ReplicationGroupId"), &field_value);
        }
        if let Some(ref field_value) = obj.show_node_group_config {
            params.put(&format!("{}{}", prefix, "ShowNodeGroupConfig"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.snapshot_name {
            params.put(&format!("{}{}", prefix, "SnapshotName"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshot_source {
            params.put(&format!("{}{}", prefix, "SnapshotSource"), &field_value);
        }

    }
}

pub type Double = f64;
struct DoubleDeserializer;
impl DoubleDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Double, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = f64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Provides ownership and status information for an Amazon EC2 security group.</p>"]
#[derive(Default,Debug,Clone)]
pub struct EC2SecurityGroup {
    #[doc="<p>The name of the Amazon EC2 security group.</p>"]
    pub ec2_security_group_name: Option<String>,
    #[doc="<p>The AWS account ID of the Amazon EC2 security group owner.</p>"]
    pub ec2_security_group_owner_id: Option<String>,
    #[doc="<p>The status of the Amazon EC2 security group.</p>"]
    pub status: Option<String>,
}

struct EC2SecurityGroupDeserializer;
impl EC2SecurityGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EC2SecurityGroup, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "EC2SecurityGroupName" => {
                            obj.ec2_security_group_name =
                                Some(try!(StringDeserializer::deserialize("EC2SecurityGroupName",
                                                                          stack)));
                        }
                        "EC2SecurityGroupOwnerId" => {
                            obj.ec2_security_group_owner_id =
                                Some(try!(StringDeserializer::deserialize("EC2SecurityGroupOwnerId",
                                                                          stack)));
                        }
                        "Status" => {
                            obj.status = Some(try!(StringDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type EC2SecurityGroupList = Vec<EC2SecurityGroup>;
struct EC2SecurityGroupListDeserializer;
impl EC2SecurityGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EC2SecurityGroupList, XmlParseError> {

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
                        obj.push(try!(EC2SecurityGroupDeserializer::deserialize("EC2SecurityGroup",
                                                                                stack)));
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
#[doc="<p>Represents the information required for client programs to connect to a cache node.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Endpoint {
    #[doc="<p>The DNS hostname of the cache node.</p>"]
    pub address: Option<String>,
    #[doc="<p>The port number that the cache engine is listening on.</p>"]
    pub port: Option<Integer>,
}

struct EndpointDeserializer;
impl EndpointDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Endpoint, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Address" => {
                            obj.address = Some(try!(StringDeserializer::deserialize("Address",
                                                                                    stack)));
                        }
                        "Port" => {
                            obj.port = Some(try!(IntegerDeserializer::deserialize("Port", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of a <code>DescribeEngineDefaultParameters</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct EngineDefaults {
    #[doc="<p>A list of parameters specific to a particular cache node type. Each element in the list contains detailed information about one parameter.</p>"]
    pub cache_node_type_specific_parameters: Option<CacheNodeTypeSpecificParametersList>,
    #[doc="<p>Specifies the name of the cache parameter group family to which the engine default parameters apply.</p> <p>Valid values are: <code>memcached1.4</code> | <code>redis2.6</code> | <code>redis2.8</code> | <code>redis3.2</code> </p>"]
    pub cache_parameter_group_family: Option<String>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
    #[doc="<p>Contains a list of engine default parameters.</p>"]
    pub parameters: Option<ParametersList>,
}

struct EngineDefaultsDeserializer;
impl EngineDefaultsDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EngineDefaults, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = EngineDefaults::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeTypeSpecificParameters" => {
                            obj.cache_node_type_specific_parameters = Some(try!(CacheNodeTypeSpecificParametersListDeserializer::deserialize("CacheNodeTypeSpecificParameters", stack)));
                        }
                        "CacheParameterGroupFamily" => {
                            obj.cache_parameter_group_family =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupFamily",
                                                                          stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "Parameters" => {
                            obj.parameters =
                                Some(try!(ParametersListDeserializer::deserialize("Parameters",
                                                                                  stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents a single occurrence of something interesting within the system. Some examples of events are creating a cache cluster, adding or removing a cache node, or rebooting a node.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Event {
    #[doc="<p>The date and time when the event occurred.</p>"]
    pub date: Option<TStamp>,
    #[doc="<p>The text of the event.</p>"]
    pub message: Option<String>,
    #[doc="<p>The identifier for the source of the event. For example, if the event occurred at the cache cluster level, the identifier would be the name of the cache cluster.</p>"]
    pub source_identifier: Option<String>,
    #[doc="<p>Specifies the origin of this event - a cache cluster, a parameter group, a security group, etc.</p>"]
    pub source_type: Option<SourceType>,
}

struct EventDeserializer;
impl EventDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Event, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Date" => {
                            obj.date = Some(try!(TStampDeserializer::deserialize("Date", stack)));
                        }
                        "Message" => {
                            obj.message = Some(try!(StringDeserializer::deserialize("Message",
                                                                                    stack)));
                        }
                        "SourceIdentifier" => {
                            obj.source_identifier =
                                Some(try!(StringDeserializer::deserialize("SourceIdentifier",
                                                                          stack)));
                        }
                        "SourceType" => {
                            obj.source_type =
                                Some(try!(SourceTypeDeserializer::deserialize("SourceType",
                                                                              stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type EventList = Vec<Event>;
struct EventListDeserializer;
impl EventListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EventList, XmlParseError> {

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
#[doc="<p>Represents the output of a <code>DescribeEvents</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct EventsMessage {
    #[doc="<p>A list of events. Each element in the list contains detailed information about one event.</p>"]
    pub events: Option<EventList>,
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
}

struct EventsMessageDeserializer;
impl EventsMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<EventsMessage, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Events" => {
                            obj.events = Some(try!(EventListDeserializer::deserialize("Events",
                                                                                      stack)));
                        }
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type Integer = i64;
struct IntegerDeserializer;
impl IntegerDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Integer, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type IntegerOptional = i64;
struct IntegerOptionalDeserializer;
impl IntegerOptionalDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<IntegerOptional, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = i64::from_str(try!(characters(stack)).as_ref()).unwrap();
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
pub type KeyList = Vec<String>;

/// Serialize `KeyList` contents to a `SignedRequest`.
struct KeyListSerializer;
impl KeyListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &KeyList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[doc="<p>The input parameters for the <code>ListAllowedNodeTypeModifications</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListAllowedNodeTypeModificationsMessage {
    #[doc="<p>The name of the cache cluster you want to scale up to a larger node instanced type. ElastiCache uses the cluster id to identify the current node type of this cluster and from that to create a list of node types you can scale up to.</p> <important> <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p> </important>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The name of the replication group want to scale up to a larger node type. ElastiCache uses the replication group id to identify the current node type being used by this replication group, and from that to create a list of node types you can scale up to.</p> <important> <p>You must provide a value for either the <code>CacheClusterId</code> or the <code>ReplicationGroupId</code>.</p> </important>"]
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

#[doc="<p>The input parameters for the <code>ListTagsForResource</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ListTagsForResourceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource for which you want the list of tags, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
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

#[doc="<p>Represents the input of a <code>ModifyCacheCluster</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ModifyCacheClusterMessage {
    #[doc="<p>Specifies whether the new nodes in this Memcached cache cluster are all created in a single Availability Zone or created across multiple Availability Zones.</p> <p>Valid values: <code>single-az</code> | <code>cross-az</code>.</p> <p>This option is only supported for Memcached cache clusters.</p> <note> <p>You cannot specify <code>single-az</code> if the Memcached cache cluster already has cache nodes in different Availability Zones. If <code>cross-az</code> is specified, existing Memcached nodes remain in their current Availability Zone.</p> <p>Only newly created nodes are located in different Availability Zones. For instructions on how to move existing Memcached nodes to different Availability Zones, see the <b>Availability Zone Considerations</b> section of <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheNode.Memcached.html\">Cache Node Considerations for Memcached</a>.</p> </note>"]
    pub az_mode: Option<AZMode>,
    #[doc="<p>If <code>true</code>, this parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the cache cluster.</p> <p>If <code>false</code>, changes to the cache cluster are applied on the next maintenance reboot, or the next failure reboot, whichever occurs first.</p> <important> <p>If you perform a <code>ModifyCacheCluster</code> before a pending modification is applied, the pending modification is replaced by the newer modification.</p> </important> <p>Valid values: <code>true</code> | <code>false</code> </p> <p>Default: <code>false</code> </p>"]
    pub apply_immediately: Option<Boolean>,
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>The cache cluster identifier. This value is stored as a lowercase string.</p>"]
    pub cache_cluster_id: String,
    #[doc="<p>A list of cache node IDs to be removed. A node ID is a numeric identifier (0001, 0002, etc.). This parameter is only valid when <code>NumCacheNodes</code> is less than the existing number of cache nodes. The number of cache node IDs supplied in this parameter must match the difference between the existing number of cache nodes in the cluster or pending cache nodes, whichever is greater, and the value of <code>NumCacheNodes</code> in the request.</p> <p>For example: If you have 3 active cache nodes, 7 pending cache nodes, and the number of cache nodes in this <code>ModifyCacheCluser</code> call is 5, you must list 2 (7 - 5) cache node IDs to remove.</p>"]
    pub cache_node_ids_to_remove: Option<CacheNodeIdsList>,
    #[doc="<p>A valid cache node type that you want to scale this cache cluster up to.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The name of the cache parameter group to apply to this cache cluster. This change is asynchronously applied as soon as possible for parameters when the <code>ApplyImmediately</code> parameter is specified as <code>true</code> for this request.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>A list of cache security group names to authorize on this cache cluster. This change is asynchronously applied as soon as possible.</p> <p>You can use this parameter only with clusters that are created outside of an Amazon Virtual Private Cloud (Amazon VPC).</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Must not be \"Default\".</p>"]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[doc="<p>The upgraded version of the cache engine to be run on the cache nodes.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SelectEngine.html#VersionManagement\">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing cache cluster and create it anew with the earlier engine version. </p>"]
    pub engine_version: Option<String>,
    #[doc="<p>The list of Availability Zones where the new Memcached cache nodes are created.</p> <p>This parameter is only valid when <code>NumCacheNodes</code> in the request is greater than the sum of the number of active cache nodes and the number of cache nodes pending creation (which may be zero). The number of Availability Zones supplied in this list must match the cache nodes being added in this request.</p> <p>This option is only supported on Memcached clusters.</p> <p>Scenarios:</p> <ul> <li> <p> <b>Scenario 1:</b> You have 3 active nodes and wish to add 2 nodes. Specify <code>NumCacheNodes=5</code> (3 + 2) and optionally specify two Availability Zones for the two new nodes.</p> </li> <li> <p> <b>Scenario 2:</b> You have 3 active nodes and 2 nodes pending creation (from the scenario 1 call) and want to add 1 more node. Specify <code>NumCacheNodes=6</code> ((3 + 2) + 1) and optionally specify an Availability Zone for the new node.</p> </li> <li> <p> <b>Scenario 3:</b> You want to cancel all pending operations. Specify <code>NumCacheNodes=3</code> to cancel all pending operations.</p> </li> </ul> <p>The Availability Zone placement of nodes pending creation cannot be modified. If you wish to cancel any nodes pending creation, add 0 nodes by setting <code>NumCacheNodes</code> to the number of current nodes.</p> <p>If <code>cross-az</code> is specified, existing Memcached nodes remain in their current Availability Zone. Only newly created nodes can be located in different Availability Zones. For guidance on how to move existing Memcached nodes to different Availability Zones, see the <b>Availability Zone Considerations</b> section of <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheNode.Memcached.html\">Cache Node Considerations for Memcached</a>.</p> <p> <b>Impact of new add/remove requests upon pending requests</b> </p> <ul> <li> <p>Scenario-1</p> <ul> <li> <p>Pending Action: Delete</p> </li> <li> <p>New Request: Delete</p> </li> <li> <p>Result: The new delete, pending or immediate, replaces the pending delete.</p> </li> </ul> </li> <li> <p>Scenario-2</p> <ul> <li> <p>Pending Action: Delete</p> </li> <li> <p>New Request: Create</p> </li> <li> <p>Result: The new create, pending or immediate, replaces the pending delete.</p> </li> </ul> </li> <li> <p>Scenario-3</p> <ul> <li> <p>Pending Action: Create</p> </li> <li> <p>New Request: Delete</p> </li> <li> <p>Result: The new delete, pending or immediate, replaces the pending create.</p> </li> </ul> </li> <li> <p>Scenario-4</p> <ul> <li> <p>Pending Action: Create</p> </li> <li> <p>New Request: Create</p> </li> <li> <p>Result: The new create is added to the pending create.</p> <important> <p> <b>Important:</b> If the new create request is <b>Apply Immediately - Yes</b>, all creates are performed immediately. If the new create request is <b>Apply Immediately - No</b>, all creates are pending.</p> </important> </li> </ul> </li> </ul>"]
    pub new_availability_zones: Option<PreferredAvailabilityZoneList>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be same as the cache cluster owner.</p> </note>"]
    pub notification_topic_arn: Option<String>,
    #[doc="<p>The status of the Amazon SNS notification topic. Notifications are sent only if the status is <code>active</code>.</p> <p>Valid values: <code>active</code> | <code>inactive</code> </p>"]
    pub notification_topic_status: Option<String>,
    #[doc="<p>The number of cache nodes that the cache cluster should have. If the value for <code>NumCacheNodes</code> is greater than the sum of the number of current cache nodes and the number of cache nodes pending creation (which may be zero), more nodes are added. If the value is less than the number of existing cache nodes, nodes are removed. If the value is equal to the number of current cache nodes, any pending add or remove requests are canceled.</p> <p>If you are removing cache nodes, you must use the <code>CacheNodeIdsToRemove</code> parameter to provide the IDs of the specific cache nodes to remove.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p> <note> <p>Adding or removing Memcached cache nodes can be applied immediately or as a pending operation (see <code>ApplyImmediately</code>).</p> <p>A pending operation to modify the number of cache nodes in a cluster during its maintenance window, whether by adding or removing nodes in accordance with the scale out architecture, is not queued. The customer's latest request to add or remove nodes to the cluster overrides any previous pending operations to modify the number of cache nodes in the cluster. For example, a request to remove 2 nodes would override a previous pending operation to remove 3 nodes. Similarly, a request to add 2 nodes would override a previous pending operation to remove 3 nodes and vice versa. As Memcached cache nodes may now be provisioned in different Availability Zones with flexible cache node placement, a request to add nodes does not automatically override a previous pending operation to add nodes. The customer can modify the previous pending operation to add more nodes or explicitly cancel the pending request and retry the new request. To cancel pending operations to modify the number of cache nodes in a cluster, use the <code>ModifyCacheCluster</code> request and set <code>NumCacheNodes</code> equal to the number of cache nodes currently in the cache cluster.</p> </note>"]
    pub num_cache_nodes: Option<IntegerOptional>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>Specifies the VPC Security Groups associated with the cache cluster.</p> <p>This parameter can be used only with clusters that are created in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[doc="<p>The number of days for which ElastiCache retains automatic cache cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <note> <p>If the value of <code>SnapshotRetentionLimit</code> is set to zero (0), backups are turned off.</p> </note>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your cache cluster. </p>"]
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
            params.put(&format!("{}{}", prefix, "ApplyImmediately"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(&format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                       &field_value.to_string());
        }
        params.put(&format!("{}{}", prefix, "CacheClusterId"),
                   &obj.cache_cluster_id);
        if let Some(ref field_value) = obj.cache_node_ids_to_remove {
            CacheNodeIdsListSerializer::serialize(params,
                                                  &format!("{}{}", prefix, "CacheNodeIdsToRemove"),
                                                  field_value);
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(params,
                                                            &format!("{}{}",
                                                                    prefix,
                                                                    "CacheSecurityGroupNames"),
                                                            field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.new_availability_zones {
            PreferredAvailabilityZoneListSerializer::serialize(params,
                                                               &format!("{}{}",
                                                                       prefix,
                                                                       "NewAvailabilityZones"),
                                                               field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(&format!("{}{}", prefix, "NotificationTopicArn"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_status {
            params.put(&format!("{}{}", prefix, "NotificationTopicStatus"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.num_cache_nodes {
            params.put(&format!("{}{}", prefix, "NumCacheNodes"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(&format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(params,
                                                      &format!("{}{}", prefix, "SecurityGroupIds"),
                                                      field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(&format!("{}{}", prefix, "SnapshotRetentionLimit"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ModifyCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

struct ModifyCacheClusterResultDeserializer;
impl ModifyCacheClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ModifyCacheClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyCacheClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheCluster" => {
                            obj.cache_cluster =
                                Some(try!(CacheClusterDeserializer::deserialize("CacheCluster",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>ModifyCacheParameterGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ModifyCacheParameterGroupMessage {
    #[doc="<p>The name of the cache parameter group to modify.</p>"]
    pub cache_parameter_group_name: String,
    #[doc="<p>An array of parameter names and values for the parameter update. You must supply at least one parameter name and value; subsequent arguments are optional. A maximum of 20 parameters may be modified per request.</p>"]
    pub parameter_name_values: ParameterNameValueList,
}


/// Serialize `ModifyCacheParameterGroupMessage` contents to a `SignedRequest`.
struct ModifyCacheParameterGroupMessageSerializer;
impl ModifyCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ModifyCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                   &obj.cache_parameter_group_name);
        ParameterNameValueListSerializer::serialize(params,
                                                    &format!("{}{}",
                                                            prefix,
                                                            "ParameterNameValues"),
                                                    &obj.parameter_name_values);

    }
}

#[doc="<p>Represents the input of a <code>ModifyCacheSubnetGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ModifyCacheSubnetGroupMessage {
    #[doc="<p>A description of the cache subnet group.</p>"]
    pub cache_subnet_group_description: Option<String>,
    #[doc="<p>The name for the cache subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters or hyphens.</p> <p>Example: <code>mysubnetgroup</code> </p>"]
    pub cache_subnet_group_name: String,
    #[doc="<p>The EC2 subnet IDs for the cache subnet group.</p>"]
    pub subnet_ids: Option<SubnetIdentifierList>,
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
            params.put(&format!("{}{}", prefix, "CacheSubnetGroupDescription"),
                       &field_value);
        }
        params.put(&format!("{}{}", prefix, "CacheSubnetGroupName"),
                   &obj.cache_subnet_group_name);
        if let Some(ref field_value) = obj.subnet_ids {
            SubnetIdentifierListSerializer::serialize(params,
                                                      &format!("{}{}", prefix, "SubnetIds"),
                                                      field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ModifyCacheSubnetGroupResult {
    pub cache_subnet_group: Option<CacheSubnetGroup>,
}

struct ModifyCacheSubnetGroupResultDeserializer;
impl ModifyCacheSubnetGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ModifyCacheSubnetGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyCacheSubnetGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSubnetGroup" => {
                            obj.cache_subnet_group =
                                Some(try!(CacheSubnetGroupDeserializer::deserialize("CacheSubnetGroup",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>ModifyReplicationGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ModifyReplicationGroupMessage {
    #[doc="<p>If <code>true</code>, this parameter causes the modifications in this request and any pending modifications to be applied, asynchronously and as soon as possible, regardless of the <code>PreferredMaintenanceWindow</code> setting for the replication group.</p> <p>If <code>false</code>, changes to the nodes in the replication group are applied on the next maintenance reboot, or the next failure reboot, whichever occurs first.</p> <p>Valid values: <code>true</code> | <code>false</code> </p> <p>Default: <code>false</code> </p>"]
    pub apply_immediately: Option<Boolean>,
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>Determines whether a read replica is automatically promoted to read/write primary if the existing primary encounters a failure.</p> <p>Valid values: <code>true</code> | <code>false</code> </p> <note> <p>ElastiCache Multi-AZ replication groups are not supported on:</p> <ul> <li> <p>Redis versions earlier than 2.8.6.</p> </li> <li> <p>Redis (cluster mode disabled):T1 and T2 cache node types.</p> <p>Redis (cluster mode enabled): T1 node types.</p> </li> </ul> </note>"]
    pub automatic_failover_enabled: Option<BooleanOptional>,
    #[doc="<p>A valid cache node type that you want to scale this replication group to.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The name of the cache parameter group to apply to all of the clusters in this replication group. This change is asynchronously applied as soon as possible for parameters when the <code>ApplyImmediately</code> parameter is specified as <code>true</code> for this request.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>A list of cache security group names to authorize for the clusters in this replication group. This change is asynchronously applied as soon as possible.</p> <p>This parameter can be used only with replication group containing cache clusters running outside of an Amazon Virtual Private Cloud (Amazon VPC).</p> <p>Constraints: Must contain no more than 255 alphanumeric characters. Must not be <code>Default</code>.</p>"]
    pub cache_security_group_names: Option<CacheSecurityGroupNameList>,
    #[doc="<p>The upgraded version of the cache engine to be run on the cache clusters in the replication group.</p> <p> <b>Important:</b> You can upgrade to a newer engine version (see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/SelectEngine.html#VersionManagement\">Selecting a Cache Engine and Version</a>), but you cannot downgrade to an earlier engine version. If you want to use an earlier engine version, you must delete the existing replication group and create it anew with the earlier engine version. </p>"]
    pub engine_version: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the Amazon SNS topic to which notifications are sent.</p> <note> <p>The Amazon SNS topic owner must be same as the replication group owner. </p> </note>"]
    pub notification_topic_arn: Option<String>,
    #[doc="<p>The status of the Amazon SNS notification topic for the replication group. Notifications are sent only if the status is <code>active</code>.</p> <p>Valid values: <code>active</code> | <code>inactive</code> </p>"]
    pub notification_topic_status: Option<String>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>For replication groups with a single primary, if this parameter is specified, ElastiCache promotes the specified cluster in the specified replication group to the primary role. The nodes of all other clusters in the replication group are read replicas.</p>"]
    pub primary_cluster_id: Option<String>,
    #[doc="<p>A description for the replication group. Maximum length is 255 characters.</p>"]
    pub replication_group_description: Option<String>,
    #[doc="<p>The identifier of the replication group to modify.</p>"]
    pub replication_group_id: String,
    #[doc="<p>Specifies the VPC Security Groups associated with the cache clusters in the replication group.</p> <p>This parameter can be used only with replication group containing cache clusters running in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    pub security_group_ids: Option<SecurityGroupIdsList>,
    #[doc="<p>The number of days for which ElastiCache retains automatic node group (shard) snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <p> <b>Important</b> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of the node group (shard) specified by <code>SnapshottingClusterId</code>.</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p>"]
    pub snapshot_window: Option<String>,
    #[doc="<p>The cache cluster ID that is used as the daily snapshot source for the replication group. This parameter cannot be set for Redis (cluster mode enabled) replication groups.</p>"]
    pub snapshotting_cluster_id: Option<String>,
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
            params.put(&format!("{}{}", prefix, "ApplyImmediately"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.auto_minor_version_upgrade {
            params.put(&format!("{}{}", prefix, "AutoMinorVersionUpgrade"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.automatic_failover_enabled {
            params.put(&format!("{}{}", prefix, "AutomaticFailoverEnabled"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.cache_node_type {
            params.put(&format!("{}{}", prefix, "CacheNodeType"), &field_value);
        }
        if let Some(ref field_value) = obj.cache_parameter_group_name {
            params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.cache_security_group_names {
            CacheSecurityGroupNameListSerializer::serialize(params,
                                                            &format!("{}{}",
                                                                    prefix,
                                                                    "CacheSecurityGroupNames"),
                                                            field_value);
        }
        if let Some(ref field_value) = obj.engine_version {
            params.put(&format!("{}{}", prefix, "EngineVersion"), &field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_arn {
            params.put(&format!("{}{}", prefix, "NotificationTopicArn"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.notification_topic_status {
            params.put(&format!("{}{}", prefix, "NotificationTopicStatus"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.preferred_maintenance_window {
            params.put(&format!("{}{}", prefix, "PreferredMaintenanceWindow"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.primary_cluster_id {
            params.put(&format!("{}{}", prefix, "PrimaryClusterId"), &field_value);
        }
        if let Some(ref field_value) = obj.replication_group_description {
            params.put(&format!("{}{}", prefix, "ReplicationGroupDescription"),
                       &field_value);
        }
        params.put(&format!("{}{}", prefix, "ReplicationGroupId"),
                   &obj.replication_group_id);
        if let Some(ref field_value) = obj.security_group_ids {
            SecurityGroupIdsListSerializer::serialize(params,
                                                      &format!("{}{}", prefix, "SecurityGroupIds"),
                                                      field_value);
        }
        if let Some(ref field_value) = obj.snapshot_retention_limit {
            params.put(&format!("{}{}", prefix, "SnapshotRetentionLimit"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.snapshot_window {
            params.put(&format!("{}{}", prefix, "SnapshotWindow"), &field_value);
        }
        if let Some(ref field_value) = obj.snapshotting_cluster_id {
            params.put(&format!("{}{}", prefix, "SnapshottingClusterId"),
                       &field_value);
        }

    }
}

#[derive(Default,Debug,Clone)]
pub struct ModifyReplicationGroupResult {
    pub replication_group: Option<ReplicationGroup>,
}

struct ModifyReplicationGroupResultDeserializer;
impl ModifyReplicationGroupResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ModifyReplicationGroupResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ModifyReplicationGroupResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ReplicationGroup" => {
                            obj.replication_group =
                                Some(try!(ReplicationGroupDeserializer::deserialize("ReplicationGroup",
                                                                                    stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents a collection of cache nodes in a replication group. One node in the node group is the read/write primary node. All the other nodes are read-only Replica nodes.</p>"]
#[derive(Default,Debug,Clone)]
pub struct NodeGroup {
    #[doc="<p>The identifier for the node group (shard). A Redis (cluster mode disabled) replication group contains only 1 node group; therefore, the node group ID is 0001. A Redis (cluster mode enabled) replication group contains 1 to 15 node groups numbered 0001 to 0015. </p>"]
    pub node_group_id: Option<String>,
    #[doc="<p>A list containing information about individual nodes within the node group (shard).</p>"]
    pub node_group_members: Option<NodeGroupMemberList>,
    #[doc="<p>The endpoint of the primary node in this node group (shard).</p>"]
    pub primary_endpoint: Option<Endpoint>,
    #[doc="<p>The keyspace for this node group (shard).</p>"]
    pub slots: Option<String>,
    #[doc="<p>The current state of this replication group - <code>creating</code>, <code>available</code>, etc.</p>"]
    pub status: Option<String>,
}

struct NodeGroupDeserializer;
impl NodeGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NodeGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "NodeGroupId" => {
                            obj.node_group_id = Some(try!(StringDeserializer::deserialize("NodeGroupId",
                                                                                          stack)));
                        }
                        "NodeGroupMembers" => {
                            obj.node_group_members =
                                Some(try!(NodeGroupMemberListDeserializer::deserialize("NodeGroupMembers",
                                                                                       stack)));
                        }
                        "PrimaryEndpoint" => {
                            obj.primary_endpoint =
                                Some(try!(EndpointDeserializer::deserialize("PrimaryEndpoint",
                                                                            stack)));
                        }
                        "Slots" => {
                            obj.slots = Some(try!(StringDeserializer::deserialize("Slots", stack)));
                        }
                        "Status" => {
                            obj.status = Some(try!(StringDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>node group (shard) configuration options. Each node group (shard) configuration has the following: <code>Slots</code>, <code>PrimaryAvailabilityZone</code>, <code>ReplicaAvailabilityZones</code>, <code>ReplicaCount</code>.</p>"]
#[derive(Default,Debug,Clone)]
pub struct NodeGroupConfiguration {
    #[doc="<p>The Availability Zone where the primary node of this node group (shard) is launched.</p>"]
    pub primary_availability_zone: Option<String>,
    #[doc="<p>A list of Availability Zones to be used for the read replicas. The number of Availability Zones in this list must match the value of <code>ReplicaCount</code> or <code>ReplicasPerNodeGroup</code> if not specified.</p>"]
    pub replica_availability_zones: Option<AvailabilityZonesList>,
    #[doc="<p>The number of read replica nodes in this node group (shard).</p>"]
    pub replica_count: Option<IntegerOptional>,
    #[doc="<p>A string that specifies the keyspaces as a series of comma separated values. Keyspaces are 0 to 16,383. The string is in the format <code>startkey-endkey</code>.</p> <p>Example: <code>\"0-3999\"</code> </p>"]
    pub slots: Option<String>,
}

struct NodeGroupConfigurationDeserializer;
impl NodeGroupConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeGroupConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NodeGroupConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "PrimaryAvailabilityZone" => {
                            obj.primary_availability_zone =
                                Some(try!(StringDeserializer::deserialize("PrimaryAvailabilityZone",
                                                                          stack)));
                        }
                        "ReplicaAvailabilityZones" => {
                            obj.replica_availability_zones =
                                Some(try!(AvailabilityZonesListDeserializer::deserialize("ReplicaAvailabilityZones",
                                                                                         stack)));
                        }
                        "ReplicaCount" => {
                            obj.replica_count =
                                Some(try!(IntegerOptionalDeserializer::deserialize("ReplicaCount",
                                                                                   stack)));
                        }
                        "Slots" => {
                            obj.slots = Some(try!(StringDeserializer::deserialize("Slots", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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

/// Serialize `NodeGroupConfiguration` contents to a `SignedRequest`.
struct NodeGroupConfigurationSerializer;
impl NodeGroupConfigurationSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &NodeGroupConfiguration) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.primary_availability_zone {
            params.put(&format!("{}{}", prefix, "PrimaryAvailabilityZone"),
                       &field_value);
        }
        if let Some(ref field_value) = obj.replica_availability_zones {
            AvailabilityZonesListSerializer::serialize(params,
                                                       &format!("{}{}",
                                                               prefix,
                                                               "ReplicaAvailabilityZones"),
                                                       field_value);
        }
        if let Some(ref field_value) = obj.replica_count {
            params.put(&format!("{}{}", prefix, "ReplicaCount"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.slots {
            params.put(&format!("{}{}", prefix, "Slots"), &field_value);
        }

    }
}

pub type NodeGroupConfigurationList = Vec<NodeGroupConfiguration>;

/// Serialize `NodeGroupConfigurationList` contents to a `SignedRequest`.
struct NodeGroupConfigurationListSerializer;
impl NodeGroupConfigurationListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &NodeGroupConfigurationList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            NodeGroupConfigurationSerializer::serialize(params, &key, obj);
        }
    }
}

pub type NodeGroupList = Vec<NodeGroup>;
struct NodeGroupListDeserializer;
impl NodeGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeGroupList, XmlParseError> {

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
                    if name == "NodeGroup" {
                        obj.push(try!(NodeGroupDeserializer::deserialize("NodeGroup", stack)));
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
#[doc="<p>Represents a single node within a node group (shard).</p>"]
#[derive(Default,Debug,Clone)]
pub struct NodeGroupMember {
    #[doc="<p>The ID of the cache cluster to which the node belongs.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The ID of the node within its cache cluster. A node ID is a numeric identifier (0001, 0002, etc.).</p>"]
    pub cache_node_id: Option<String>,
    #[doc="<p>The role that is currently assigned to the node - <code>primary</code> or <code>replica</code>.</p>"]
    pub current_role: Option<String>,
    #[doc="<p>The name of the Availability Zone in which the node is located.</p>"]
    pub preferred_availability_zone: Option<String>,
    pub read_endpoint: Option<Endpoint>,
}

struct NodeGroupMemberDeserializer;
impl NodeGroupMemberDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeGroupMember, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NodeGroupMember::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheClusterId" => {
                            obj.cache_cluster_id =
                                Some(try!(StringDeserializer::deserialize("CacheClusterId",
                                                                          stack)));
                        }
                        "CacheNodeId" => {
                            obj.cache_node_id = Some(try!(StringDeserializer::deserialize("CacheNodeId",
                                                                                          stack)));
                        }
                        "CurrentRole" => {
                            obj.current_role = Some(try!(StringDeserializer::deserialize("CurrentRole",
                                                                                         stack)));
                        }
                        "PreferredAvailabilityZone" => {
                            obj.preferred_availability_zone =
                                Some(try!(StringDeserializer::deserialize("PreferredAvailabilityZone",
                                                                          stack)));
                        }
                        "ReadEndpoint" => {
                            obj.read_endpoint =
                                Some(try!(EndpointDeserializer::deserialize("ReadEndpoint",
                                                                            stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type NodeGroupMemberList = Vec<NodeGroupMember>;
struct NodeGroupMemberListDeserializer;
impl NodeGroupMemberListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeGroupMemberList, XmlParseError> {

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
                    if name == "NodeGroupMember" {
                        obj.push(try!(NodeGroupMemberDeserializer::deserialize("NodeGroupMember",
                                                                               stack)));
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
#[doc="<p>Represents an individual cache node in a snapshot of a cache cluster.</p>"]
#[derive(Default,Debug,Clone)]
pub struct NodeSnapshot {
    #[doc="<p>A unique identifier for the source cache cluster.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The date and time when the cache node was created in the source cache cluster.</p>"]
    pub cache_node_create_time: Option<TStamp>,
    #[doc="<p>The cache node identifier for the node in the source cache cluster.</p>"]
    pub cache_node_id: Option<String>,
    #[doc="<p>The size of the cache on the source cache node.</p>"]
    pub cache_size: Option<String>,
    #[doc="<p>The configuration for the source node group (shard).</p>"]
    pub node_group_configuration: Option<NodeGroupConfiguration>,
    #[doc="<p>A unique identifier for the source node group (shard).</p>"]
    pub node_group_id: Option<String>,
    #[doc="<p>The date and time when the source node's metadata and cache data set was obtained for the snapshot.</p>"]
    pub snapshot_create_time: Option<TStamp>,
}

struct NodeSnapshotDeserializer;
impl NodeSnapshotDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeSnapshot, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NodeSnapshot::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheClusterId" => {
                            obj.cache_cluster_id =
                                Some(try!(StringDeserializer::deserialize("CacheClusterId",
                                                                          stack)));
                        }
                        "CacheNodeCreateTime" => {
                            obj.cache_node_create_time =
                                Some(try!(TStampDeserializer::deserialize("CacheNodeCreateTime",
                                                                          stack)));
                        }
                        "CacheNodeId" => {
                            obj.cache_node_id = Some(try!(StringDeserializer::deserialize("CacheNodeId",
                                                                                          stack)));
                        }
                        "CacheSize" => {
                            obj.cache_size = Some(try!(StringDeserializer::deserialize("CacheSize",
                                                                                       stack)));
                        }
                        "NodeGroupConfiguration" => {
                            obj.node_group_configuration =
                                Some(try!(NodeGroupConfigurationDeserializer::deserialize("NodeGroupConfiguration",
                                                                                          stack)));
                        }
                        "NodeGroupId" => {
                            obj.node_group_id = Some(try!(StringDeserializer::deserialize("NodeGroupId",
                                                                                          stack)));
                        }
                        "SnapshotCreateTime" => {
                            obj.snapshot_create_time =
                                Some(try!(TStampDeserializer::deserialize("SnapshotCreateTime",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type NodeSnapshotList = Vec<NodeSnapshot>;
struct NodeSnapshotListDeserializer;
impl NodeSnapshotListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeSnapshotList, XmlParseError> {

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
                    if name == "NodeSnapshot" {
                        obj.push(try!(NodeSnapshotDeserializer::deserialize("NodeSnapshot",
                                                                            stack)));
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
pub type NodeTypeList = Vec<String>;
struct NodeTypeListDeserializer;
impl NodeTypeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NodeTypeList, XmlParseError> {

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
#[doc="<p>Describes a notification topic and its status. Notification topics are used for publishing ElastiCache events to subscribers using Amazon Simple Notification Service (SNS).</p>"]
#[derive(Default,Debug,Clone)]
pub struct NotificationConfiguration {
    #[doc="<p>The Amazon Resource Name (ARN) that identifies the topic.</p>"]
    pub topic_arn: Option<String>,
    #[doc="<p>The current state of the topic.</p>"]
    pub topic_status: Option<String>,
}

struct NotificationConfigurationDeserializer;
impl NotificationConfigurationDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<NotificationConfiguration, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = NotificationConfiguration::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TopicArn" => {
                            obj.topic_arn = Some(try!(StringDeserializer::deserialize("TopicArn",
                                                                                      stack)));
                        }
                        "TopicStatus" => {
                            obj.topic_status = Some(try!(StringDeserializer::deserialize("TopicStatus",
                                                                                         stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Describes an individual setting that controls some aspect of ElastiCache behavior.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Parameter {
    #[doc="<p>The valid range of values for the parameter.</p>"]
    pub allowed_values: Option<String>,
    #[doc="<p>Indicates whether a change to the parameter is applied immediately or requires a reboot for the change to be applied. You can force a reboot or wait until the next maintenance window's reboot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Clusters.Rebooting.html\">Rebooting a Cluster</a>.</p>"]
    pub change_type: Option<ChangeType>,
    #[doc="<p>The valid data type for the parameter.</p>"]
    pub data_type: Option<String>,
    #[doc="<p>A description of the parameter.</p>"]
    pub description: Option<String>,
    #[doc="<p>Indicates whether (<code>true</code>) or not (<code>false</code>) the parameter can be modified. Some parameters have security or operational implications that prevent them from being changed.</p>"]
    pub is_modifiable: Option<Boolean>,
    #[doc="<p>The earliest cache engine version to which the parameter can apply.</p>"]
    pub minimum_engine_version: Option<String>,
    #[doc="<p>The name of the parameter.</p>"]
    pub parameter_name: Option<String>,
    #[doc="<p>The value of the parameter.</p>"]
    pub parameter_value: Option<String>,
    #[doc="<p>The source of the parameter.</p>"]
    pub source: Option<String>,
}

struct ParameterDeserializer;
impl ParameterDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Parameter, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AllowedValues" => {
                            obj.allowed_values = Some(try!(StringDeserializer::deserialize("AllowedValues",
                                                                                           stack)));
                        }
                        "ChangeType" => {
                            obj.change_type =
                                Some(try!(ChangeTypeDeserializer::deserialize("ChangeType",
                                                                              stack)));
                        }
                        "DataType" => {
                            obj.data_type = Some(try!(StringDeserializer::deserialize("DataType",
                                                                                      stack)));
                        }
                        "Description" => {
                            obj.description = Some(try!(StringDeserializer::deserialize("Description",
                                                                                        stack)));
                        }
                        "IsModifiable" => {
                            obj.is_modifiable = Some(try!(BooleanDeserializer::deserialize("IsModifiable",
                                                                                           stack)));
                        }
                        "MinimumEngineVersion" => {
                            obj.minimum_engine_version =
                                Some(try!(StringDeserializer::deserialize("MinimumEngineVersion",
                                                                          stack)));
                        }
                        "ParameterName" => {
                            obj.parameter_name = Some(try!(StringDeserializer::deserialize("ParameterName",
                                                                                           stack)));
                        }
                        "ParameterValue" => {
                            obj.parameter_value =
                                Some(try!(StringDeserializer::deserialize("ParameterValue",
                                                                          stack)));
                        }
                        "Source" => {
                            obj.source = Some(try!(StringDeserializer::deserialize("Source",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Describes a name-value pair that is used to update the value of a parameter.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ParameterNameValue {
    #[doc="<p>The name of the parameter.</p>"]
    pub parameter_name: Option<String>,
    #[doc="<p>The value of the parameter.</p>"]
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

pub type ParameterNameValueList = Vec<ParameterNameValue>;

/// Serialize `ParameterNameValueList` contents to a `SignedRequest`.
struct ParameterNameValueListSerializer;
impl ParameterNameValueListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ParameterNameValueList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            ParameterNameValueSerializer::serialize(params, &key, obj);
        }
    }
}

pub type ParametersList = Vec<Parameter>;
struct ParametersListDeserializer;
impl ParametersListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ParametersList, XmlParseError> {

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
pub type PendingAutomaticFailoverStatus = String;
struct PendingAutomaticFailoverStatusDeserializer;
impl PendingAutomaticFailoverStatusDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<PendingAutomaticFailoverStatus, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>A group of settings that are applied to the cache cluster in the future, or that are currently being applied.</p>"]
#[derive(Default,Debug,Clone)]
pub struct PendingModifiedValues {
    #[doc="<p>A list of cache node IDs that are being removed (or will be removed) from the cache cluster. A node ID is a numeric identifier (0001, 0002, etc.).</p>"]
    pub cache_node_ids_to_remove: Option<CacheNodeIdsList>,
    #[doc="<p>The cache node type that this cache cluster or replication group is scaled to.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The new cache engine version that the cache cluster runs.</p>"]
    pub engine_version: Option<String>,
    #[doc="<p>The new number of cache nodes for the cache cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>"]
    pub num_cache_nodes: Option<IntegerOptional>,
}

struct PendingModifiedValuesDeserializer;
impl PendingModifiedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<PendingModifiedValues, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeIdsToRemove" => {
                            obj.cache_node_ids_to_remove =
                                Some(try!(CacheNodeIdsListDeserializer::deserialize("CacheNodeIdsToRemove",
                                                                                    stack)));
                        }
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "EngineVersion" => {
                            obj.engine_version = Some(try!(StringDeserializer::deserialize("EngineVersion",
                                                                                           stack)));
                        }
                        "NumCacheNodes" => {
                            obj.num_cache_nodes =
                                Some(try!(IntegerOptionalDeserializer::deserialize("NumCacheNodes",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type PreferredAvailabilityZoneList = Vec<String>;

/// Serialize `PreferredAvailabilityZoneList` contents to a `SignedRequest`.
struct PreferredAvailabilityZoneListSerializer;
impl PreferredAvailabilityZoneListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &PreferredAvailabilityZoneList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[doc="<p>Represents the input of a <code>PurchaseReservedCacheNodesOffering</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct PurchaseReservedCacheNodesOfferingMessage {
    #[doc="<p>The number of cache node instances to reserve.</p> <p>Default: <code>1</code> </p>"]
    pub cache_node_count: Option<IntegerOptional>,
    #[doc="<p>A customer-specified identifier to track this reservation.</p> <note> <p>The Reserved Cache Node ID is an unique customer-specified identifier to track this reservation. If this parameter is not specified, ElastiCache automatically generates an identifier for the reservation.</p> </note> <p>Example: myreservationID</p>"]
    pub reserved_cache_node_id: Option<String>,
    #[doc="<p>The ID of the reserved cache node offering to purchase.</p> <p>Example: <code>438012d3-4052-4cc7-b2e3-8d3372e0e706</code> </p>"]
    pub reserved_cache_nodes_offering_id: String,
}


/// Serialize `PurchaseReservedCacheNodesOfferingMessage` contents to a `SignedRequest`.
struct PurchaseReservedCacheNodesOfferingMessageSerializer;
impl PurchaseReservedCacheNodesOfferingMessageSerializer {
    fn serialize(params: &mut Params,
                 name: &str,
                 obj: &PurchaseReservedCacheNodesOfferingMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        if let Some(ref field_value) = obj.cache_node_count {
            params.put(&format!("{}{}", prefix, "CacheNodeCount"),
                       &field_value.to_string());
        }
        if let Some(ref field_value) = obj.reserved_cache_node_id {
            params.put(&format!("{}{}", prefix, "ReservedCacheNodeId"),
                       &field_value);
        }
        params.put(&format!("{}{}", prefix, "ReservedCacheNodesOfferingId"),
                   &obj.reserved_cache_nodes_offering_id);

    }
}

#[derive(Default,Debug,Clone)]
pub struct PurchaseReservedCacheNodesOfferingResult {
    pub reserved_cache_node: Option<ReservedCacheNode>,
}

struct PurchaseReservedCacheNodesOfferingResultDeserializer;
impl PurchaseReservedCacheNodesOfferingResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<PurchaseReservedCacheNodesOfferingResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = PurchaseReservedCacheNodesOfferingResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "ReservedCacheNode" => {
                            obj.reserved_cache_node =
                                Some(try!(ReservedCacheNodeDeserializer::deserialize("ReservedCacheNode",
                                                                                     stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>RebootCacheCluster</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct RebootCacheClusterMessage {
    #[doc="<p>The cache cluster identifier. This parameter is stored as a lowercase string.</p>"]
    pub cache_cluster_id: String,
    #[doc="<p>A list of cache node IDs to reboot. A node ID is a numeric identifier (0001, 0002, etc.). To reboot an entire cache cluster, specify all of the cache node IDs.</p>"]
    pub cache_node_ids_to_reboot: CacheNodeIdsList,
}


/// Serialize `RebootCacheClusterMessage` contents to a `SignedRequest`.
struct RebootCacheClusterMessageSerializer;
impl RebootCacheClusterMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &RebootCacheClusterMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheClusterId"),
                   &obj.cache_cluster_id);
        CacheNodeIdsListSerializer::serialize(params,
                                              &format!("{}{}", prefix, "CacheNodeIdsToReboot"),
                                              &obj.cache_node_ids_to_reboot);

    }
}

#[derive(Default,Debug,Clone)]
pub struct RebootCacheClusterResult {
    pub cache_cluster: Option<CacheCluster>,
}

struct RebootCacheClusterResultDeserializer;
impl RebootCacheClusterResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RebootCacheClusterResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RebootCacheClusterResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheCluster" => {
                            obj.cache_cluster =
                                Some(try!(CacheClusterDeserializer::deserialize("CacheCluster",
                                                                                stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Contains the specific price and frequency of a recurring charges for a reserved cache node, or for a reserved cache node offering.</p>"]
#[derive(Default,Debug,Clone)]
pub struct RecurringCharge {
    #[doc="<p>The monetary amount of the recurring charge.</p>"]
    pub recurring_charge_amount: Option<Double>,
    #[doc="<p>The frequency of the recurring charge.</p>"]
    pub recurring_charge_frequency: Option<String>,
}

struct RecurringChargeDeserializer;
impl RecurringChargeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RecurringCharge, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "RecurringChargeAmount" => {
                            obj.recurring_charge_amount =
                                Some(try!(DoubleDeserializer::deserialize("RecurringChargeAmount",
                                                                          stack)));
                        }
                        "RecurringChargeFrequency" => {
                            obj.recurring_charge_frequency =
                                Some(try!(StringDeserializer::deserialize("RecurringChargeFrequency",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type RecurringChargeList = Vec<RecurringCharge>;
struct RecurringChargeListDeserializer;
impl RecurringChargeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<RecurringChargeList, XmlParseError> {

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
                        obj.push(try!(RecurringChargeDeserializer::deserialize("RecurringCharge",
                                                                               stack)));
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
#[doc="<p>Represents the input of a <code>RemoveTagsFromResource</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct RemoveTagsFromResourceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the resource from which you want the tags removed, for example <code>arn:aws:elasticache:us-west-2:0123456789:cluster:myCluster</code> or <code>arn:aws:elasticache:us-west-2:0123456789:snapshot:mySnapshot</code>.</p> <p>For more information about ARNs, see <a href=\"http://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html\">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>"]
    pub resource_name: String,
    #[doc="<p>A list of <code>TagKeys</code> identifying the tags you want removed from the named resource.</p>"]
    pub tag_keys: KeyList,
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

#[doc="<p>Contains all of the attributes of a specific Redis replication group.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReplicationGroup {
    #[doc="<p>Indicates the status of Multi-AZ for this replication group.</p> <note> <p>ElastiCache Multi-AZ replication groups are not supported on:</p> <ul> <li> <p>Redis versions earlier than 2.8.6.</p> </li> <li> <p>Redis (cluster mode disabled):T1 and T2 cache node types.</p> <p>Redis (cluster mode enabled): T1 node types.</p> </li> </ul> </note>"]
    pub automatic_failover: Option<AutomaticFailoverStatus>,
    #[doc="<p>The configuration endpoint for this replicaiton group. Use the configuration endpoint to connect to this replication group.</p>"]
    pub configuration_endpoint: Option<Endpoint>,
    #[doc="<p>The description of the replication group.</p>"]
    pub description: Option<String>,
    #[doc="<p>The names of all the cache clusters that are part of this replication group.</p>"]
    pub member_clusters: Option<ClusterIdList>,
    #[doc="<p>A single element list with information about the nodes in the replication group.</p>"]
    pub node_groups: Option<NodeGroupList>,
    #[doc="<p>A group of settings to be applied to the replication group, either immediately or during the next maintenance window.</p>"]
    pub pending_modified_values: Option<ReplicationGroupPendingModifiedValues>,
    #[doc="<p>The identifier for the replication group.</p>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>The number of days for which ElastiCache retains automatic cache cluster snapshots before deleting them. For example, if you set <code>SnapshotRetentionLimit</code> to 5, a snapshot that was taken today is retained for 5 days before being deleted.</p> <important> <p> If the value of <code>SnapshotRetentionLimit</code> is set to zero (0), backups are turned off.</p> </important>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>The daily time range (in UTC) during which ElastiCache begins taking a daily snapshot of your node group (shard).</p> <p>Example: <code>05:00-09:00</code> </p> <p>If you do not specify this parameter, ElastiCache automatically chooses an appropriate time range.</p> <p> <b>Note:</b> This parameter is only valid if the <code>Engine</code> parameter is <code>redis</code>.</p>"]
    pub snapshot_window: Option<String>,
    #[doc="<p>The cache cluster ID that is used as the daily snapshot source for the replication group.</p>"]
    pub snapshotting_cluster_id: Option<String>,
    #[doc="<p>The current state of this replication group - <code>creating</code>, <code>available</code>, <code>modifying</code>, <code>deleting</code>, <code>create-failed</code>, <code>snapshotting</code>.</p>"]
    pub status: Option<String>,
}

struct ReplicationGroupDeserializer;
impl ReplicationGroupDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReplicationGroup, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationGroup::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AutomaticFailover" => {
                            obj.automatic_failover =
                                Some(try!(AutomaticFailoverStatusDeserializer::deserialize("AutomaticFailover",
                                                                                           stack)));
                        }
                        "ConfigurationEndpoint" => {
                            obj.configuration_endpoint =
                                Some(try!(EndpointDeserializer::deserialize("ConfigurationEndpoint",
                                                                            stack)));
                        }
                        "Description" => {
                            obj.description = Some(try!(StringDeserializer::deserialize("Description",
                                                                                        stack)));
                        }
                        "MemberClusters" => {
                            obj.member_clusters =
                                Some(try!(ClusterIdListDeserializer::deserialize("MemberClusters",
                                                                                 stack)));
                        }
                        "NodeGroups" => {
                            obj.node_groups =
                                Some(try!(NodeGroupListDeserializer::deserialize("NodeGroups",
                                                                                 stack)));
                        }
                        "PendingModifiedValues" => {
                            obj.pending_modified_values = Some(try!(ReplicationGroupPendingModifiedValuesDeserializer::deserialize("PendingModifiedValues", stack)));
                        }
                        "ReplicationGroupId" => {
                            obj.replication_group_id =
                                Some(try!(StringDeserializer::deserialize("ReplicationGroupId",
                                                                          stack)));
                        }
                        "SnapshotRetentionLimit" => {
                            obj.snapshot_retention_limit =
                                Some(try!(IntegerOptionalDeserializer::deserialize("SnapshotRetentionLimit",
                                                                                   stack)));
                        }
                        "SnapshotWindow" => {
                            obj.snapshot_window =
                                Some(try!(StringDeserializer::deserialize("SnapshotWindow",
                                                                          stack)));
                        }
                        "SnapshottingClusterId" => {
                            obj.snapshotting_cluster_id =
                                Some(try!(StringDeserializer::deserialize("SnapshottingClusterId",
                                                                          stack)));
                        }
                        "Status" => {
                            obj.status = Some(try!(StringDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type ReplicationGroupList = Vec<ReplicationGroup>;
struct ReplicationGroupListDeserializer;
impl ReplicationGroupListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReplicationGroupList, XmlParseError> {

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
                    if name == "ReplicationGroup" {
                        obj.push(try!(ReplicationGroupDeserializer::deserialize("ReplicationGroup",
                                                                                stack)));
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
#[doc="<p>Represents the output of a <code>DescribeReplicationGroups</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReplicationGroupMessage {
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
    #[doc="<p>A list of replication groups. Each item in the list contains detailed information about one replication group.</p>"]
    pub replication_groups: Option<ReplicationGroupList>,
}

struct ReplicationGroupMessageDeserializer;
impl ReplicationGroupMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReplicationGroupMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationGroupMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "ReplicationGroups" => {
                            obj.replication_groups =
                                Some(try!(ReplicationGroupListDeserializer::deserialize("ReplicationGroups",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>The settings to be applied to the Redis replication group, either immediately or during the next maintenance window.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReplicationGroupPendingModifiedValues {
    #[doc="<p>Indicates the status of Multi-AZ for this Redis replication group.</p> <note> <p>ElastiCache Multi-AZ replication groups are not supported on:</p> <ul> <li> <p>Redis versions earlier than 2.8.6.</p> </li> <li> <p>Redis (cluster mode disabled):T1 and T2 cache node types.</p> <p>Redis (cluster mode enabled): T1 node types.</p> </li> </ul> </note>"]
    pub automatic_failover_status: Option<PendingAutomaticFailoverStatus>,
    #[doc="<p>The primary cluster ID that is applied immediately (if <code>--apply-immediately</code> was specified), or during the next maintenance window.</p>"]
    pub primary_cluster_id: Option<String>,
}

struct ReplicationGroupPendingModifiedValuesDeserializer;
impl ReplicationGroupPendingModifiedValuesDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<ReplicationGroupPendingModifiedValues, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReplicationGroupPendingModifiedValues::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AutomaticFailoverStatus" => {
                            obj.automatic_failover_status = Some(try!(PendingAutomaticFailoverStatusDeserializer::deserialize("AutomaticFailoverStatus", stack)));
                        }
                        "PrimaryClusterId" => {
                            obj.primary_cluster_id =
                                Some(try!(StringDeserializer::deserialize("PrimaryClusterId",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the output of a <code>PurchaseReservedCacheNodesOffering</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReservedCacheNode {
    #[doc="<p>The number of cache nodes that have been reserved.</p>"]
    pub cache_node_count: Option<Integer>,
    #[doc="<p>The cache node type for the reserved cache nodes.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The duration of the reservation in seconds.</p>"]
    pub duration: Option<Integer>,
    #[doc="<p>The fixed price charged for this reserved cache node.</p>"]
    pub fixed_price: Option<Double>,
    #[doc="<p>The offering type of this reserved cache node.</p>"]
    pub offering_type: Option<String>,
    #[doc="<p>The description of the reserved cache node.</p>"]
    pub product_description: Option<String>,
    #[doc="<p>The recurring price charged to run this reserved cache node.</p>"]
    pub recurring_charges: Option<RecurringChargeList>,
    #[doc="<p>The unique identifier for the reservation.</p>"]
    pub reserved_cache_node_id: Option<String>,
    #[doc="<p>The offering identifier.</p>"]
    pub reserved_cache_nodes_offering_id: Option<String>,
    #[doc="<p>The time the reservation started.</p>"]
    pub start_time: Option<TStamp>,
    #[doc="<p>The state of the reserved cache node.</p>"]
    pub state: Option<String>,
    #[doc="<p>The hourly price charged for this reserved cache node.</p>"]
    pub usage_price: Option<Double>,
}

struct ReservedCacheNodeDeserializer;
impl ReservedCacheNodeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReservedCacheNode, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedCacheNode::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeCount" => {
                            obj.cache_node_count =
                                Some(try!(IntegerDeserializer::deserialize("CacheNodeCount",
                                                                           stack)));
                        }
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "Duration" => {
                            obj.duration = Some(try!(IntegerDeserializer::deserialize("Duration",
                                                                                      stack)));
                        }
                        "FixedPrice" => {
                            obj.fixed_price = Some(try!(DoubleDeserializer::deserialize("FixedPrice",
                                                                                        stack)));
                        }
                        "OfferingType" => {
                            obj.offering_type = Some(try!(StringDeserializer::deserialize("OfferingType",
                                                                                          stack)));
                        }
                        "ProductDescription" => {
                            obj.product_description =
                                Some(try!(StringDeserializer::deserialize("ProductDescription",
                                                                          stack)));
                        }
                        "RecurringCharges" => {
                            obj.recurring_charges =
                                Some(try!(RecurringChargeListDeserializer::deserialize("RecurringCharges",
                                                                                       stack)));
                        }
                        "ReservedCacheNodeId" => {
                            obj.reserved_cache_node_id =
                                Some(try!(StringDeserializer::deserialize("ReservedCacheNodeId",
                                                                          stack)));
                        }
                        "ReservedCacheNodesOfferingId" => {
                            obj.reserved_cache_nodes_offering_id =
                                Some(try!(StringDeserializer::deserialize("ReservedCacheNodesOfferingId",
                                                                          stack)));
                        }
                        "StartTime" => {
                            obj.start_time = Some(try!(TStampDeserializer::deserialize("StartTime",
                                                                                       stack)));
                        }
                        "State" => {
                            obj.state = Some(try!(StringDeserializer::deserialize("State", stack)));
                        }
                        "UsagePrice" => {
                            obj.usage_price = Some(try!(DoubleDeserializer::deserialize("UsagePrice",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type ReservedCacheNodeList = Vec<ReservedCacheNode>;
struct ReservedCacheNodeListDeserializer;
impl ReservedCacheNodeListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReservedCacheNodeList, XmlParseError> {

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
                    if name == "ReservedCacheNode" {
                        obj.push(try!(ReservedCacheNodeDeserializer::deserialize("ReservedCacheNode", stack)));
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
#[doc="<p>Represents the output of a <code>DescribeReservedCacheNodes</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReservedCacheNodeMessage {
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
    #[doc="<p>A list of reserved cache nodes. Each element in the list contains detailed information about one node.</p>"]
    pub reserved_cache_nodes: Option<ReservedCacheNodeList>,
}

struct ReservedCacheNodeMessageDeserializer;
impl ReservedCacheNodeMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReservedCacheNodeMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedCacheNodeMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "ReservedCacheNodes" => {
                            obj.reserved_cache_nodes =
                                Some(try!(ReservedCacheNodeListDeserializer::deserialize("ReservedCacheNodes",
                                                                                         stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Describes all of the attributes of a reserved cache node offering.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReservedCacheNodesOffering {
    #[doc="<p>The cache node type for the reserved cache node.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The duration of the offering. in seconds.</p>"]
    pub duration: Option<Integer>,
    #[doc="<p>The fixed price charged for this offering.</p>"]
    pub fixed_price: Option<Double>,
    #[doc="<p>The offering type.</p>"]
    pub offering_type: Option<String>,
    #[doc="<p>The cache engine used by the offering.</p>"]
    pub product_description: Option<String>,
    #[doc="<p>The recurring price charged to run this reserved cache node.</p>"]
    pub recurring_charges: Option<RecurringChargeList>,
    #[doc="<p>A unique identifier for the reserved cache node offering.</p>"]
    pub reserved_cache_nodes_offering_id: Option<String>,
    #[doc="<p>The hourly price charged for this offering.</p>"]
    pub usage_price: Option<Double>,
}

struct ReservedCacheNodesOfferingDeserializer;
impl ReservedCacheNodesOfferingDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReservedCacheNodesOffering, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedCacheNodesOffering::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "Duration" => {
                            obj.duration = Some(try!(IntegerDeserializer::deserialize("Duration",
                                                                                      stack)));
                        }
                        "FixedPrice" => {
                            obj.fixed_price = Some(try!(DoubleDeserializer::deserialize("FixedPrice",
                                                                                        stack)));
                        }
                        "OfferingType" => {
                            obj.offering_type = Some(try!(StringDeserializer::deserialize("OfferingType",
                                                                                          stack)));
                        }
                        "ProductDescription" => {
                            obj.product_description =
                                Some(try!(StringDeserializer::deserialize("ProductDescription",
                                                                          stack)));
                        }
                        "RecurringCharges" => {
                            obj.recurring_charges =
                                Some(try!(RecurringChargeListDeserializer::deserialize("RecurringCharges",
                                                                                       stack)));
                        }
                        "ReservedCacheNodesOfferingId" => {
                            obj.reserved_cache_nodes_offering_id =
                                Some(try!(StringDeserializer::deserialize("ReservedCacheNodesOfferingId",
                                                                          stack)));
                        }
                        "UsagePrice" => {
                            obj.usage_price = Some(try!(DoubleDeserializer::deserialize("UsagePrice",
                                                                                        stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type ReservedCacheNodesOfferingList = Vec<ReservedCacheNodesOffering>;
struct ReservedCacheNodesOfferingListDeserializer;
impl ReservedCacheNodesOfferingListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<ReservedCacheNodesOfferingList, XmlParseError> {

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
                    if name == "ReservedCacheNodesOffering" {
                        obj.push(try!(ReservedCacheNodesOfferingDeserializer::deserialize("ReservedCacheNodesOffering", stack)));
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
#[doc="<p>Represents the output of a <code>DescribeReservedCacheNodesOfferings</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ReservedCacheNodesOfferingMessage {
    #[doc="<p>Provides an identifier to allow retrieval of paginated results.</p>"]
    pub marker: Option<String>,
    #[doc="<p>A list of reserved cache node offerings. Each element in the list contains detailed information about one offering.</p>"]
    pub reserved_cache_nodes_offerings: Option<ReservedCacheNodesOfferingList>,
}

struct ReservedCacheNodesOfferingMessageDeserializer;
impl ReservedCacheNodesOfferingMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<ReservedCacheNodesOfferingMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = ReservedCacheNodesOfferingMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Marker" => {
                            obj.marker = Some(try!(StringDeserializer::deserialize("Marker",
                                                                                   stack)));
                        }
                        "ReservedCacheNodesOfferings" => {
                            obj.reserved_cache_nodes_offerings = Some(try!(ReservedCacheNodesOfferingListDeserializer::deserialize("ReservedCacheNodesOfferings", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
#[doc="<p>Represents the input of a <code>ResetCacheParameterGroup</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct ResetCacheParameterGroupMessage {
    #[doc="<p>The name of the cache parameter group to reset.</p>"]
    pub cache_parameter_group_name: String,
    #[doc="<p>An array of parameter names to reset to their default values. If <code>ResetAllParameters</code> is <code>true</code>, do not use <code>ParameterNameValues</code>. If <code>ResetAllParameters</code> is <code>false</code>, you must specify the name of at least one parameter to reset.</p>"]
    pub parameter_name_values: Option<ParameterNameValueList>,
    #[doc="<p>If <code>true</code>, all parameters in the cache parameter group are reset to their default values. If <code>false</code>, only the parameters listed by <code>ParameterNameValues</code> are reset to their default values.</p> <p>Valid values: <code>true</code> | <code>false</code> </p>"]
    pub reset_all_parameters: Option<Boolean>,
}


/// Serialize `ResetCacheParameterGroupMessage` contents to a `SignedRequest`.
struct ResetCacheParameterGroupMessageSerializer;
impl ResetCacheParameterGroupMessageSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &ResetCacheParameterGroupMessage) {
        let mut prefix = name.to_string();
        if prefix != "" {
            prefix.push_str(".");
        }

        params.put(&format!("{}{}", prefix, "CacheParameterGroupName"),
                   &obj.cache_parameter_group_name);
        if let Some(ref field_value) = obj.parameter_name_values {
            ParameterNameValueListSerializer::serialize(params,
                                                        &format!("{}{}",
                                                                prefix,
                                                                "ParameterNameValues"),
                                                        field_value);
        }
        if let Some(ref field_value) = obj.reset_all_parameters {
            params.put(&format!("{}{}", prefix, "ResetAllParameters"),
                       &field_value.to_string());
        }

    }
}

#[doc="<p>Represents the input of a <code>RevokeCacheSecurityGroupIngress</code> operation.</p>"]
#[derive(Default,Debug,Clone)]
pub struct RevokeCacheSecurityGroupIngressMessage {
    #[doc="<p>The name of the cache security group to revoke ingress from.</p>"]
    pub cache_security_group_name: String,
    #[doc="<p>The name of the Amazon EC2 security group to revoke access from.</p>"]
    pub ec2_security_group_name: String,
    #[doc="<p>The AWS account number of the Amazon EC2 security group owner. Note that this is not the same thing as an AWS access key ID - you must provide a valid AWS account number for this parameter.</p>"]
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

        params.put(&format!("{}{}", prefix, "CacheSecurityGroupName"),
                   &obj.cache_security_group_name);
        params.put(&format!("{}{}", prefix, "EC2SecurityGroupName"),
                   &obj.ec2_security_group_name);
        params.put(&format!("{}{}", prefix, "EC2SecurityGroupOwnerId"),
                   &obj.ec2_security_group_owner_id);

    }
}

#[derive(Default,Debug,Clone)]
pub struct RevokeCacheSecurityGroupIngressResult {
    pub cache_security_group: Option<CacheSecurityGroup>,
}

struct RevokeCacheSecurityGroupIngressResultDeserializer;
impl RevokeCacheSecurityGroupIngressResultDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>
        (tag_name: &str,
         stack: &mut T)
         -> Result<RevokeCacheSecurityGroupIngressResult, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = RevokeCacheSecurityGroupIngressResult::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "CacheSecurityGroup" => {
                            obj.cache_security_group =
                                Some(try!(CacheSecurityGroupDeserializer::deserialize("CacheSecurityGroup",
                                                                                      stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type SecurityGroupIdsList = Vec<String>;

/// Serialize `SecurityGroupIdsList` contents to a `SignedRequest`.
struct SecurityGroupIdsListSerializer;
impl SecurityGroupIdsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SecurityGroupIdsList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

#[doc="<p>Represents a single cache security group and its status.</p>"]
#[derive(Default,Debug,Clone)]
pub struct SecurityGroupMembership {
    #[doc="<p>The identifier of the cache security group.</p>"]
    pub security_group_id: Option<String>,
    #[doc="<p>The status of the cache security group membership. The status changes whenever a cache security group is modified, or when the cache security groups assigned to a cache cluster are modified.</p>"]
    pub status: Option<String>,
}

struct SecurityGroupMembershipDeserializer;
impl SecurityGroupMembershipDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<SecurityGroupMembership, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = SecurityGroupMembership::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "SecurityGroupId" => {
                            obj.security_group_id =
                                Some(try!(StringDeserializer::deserialize("SecurityGroupId",
                                                                          stack)));
                        }
                        "Status" => {
                            obj.status = Some(try!(StringDeserializer::deserialize("Status",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type SecurityGroupMembershipList = Vec<SecurityGroupMembership>;
struct SecurityGroupMembershipListDeserializer;
impl SecurityGroupMembershipListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<SecurityGroupMembershipList, XmlParseError> {

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
                        obj.push(try!(SecurityGroupMembershipDeserializer::deserialize("member",
                                                                                       stack)));
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
#[doc="<p>Represents a copy of an entire Redis cache cluster as of the time when the snapshot was taken.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Snapshot {
    #[doc="<p>This parameter is currently disabled.</p>"]
    pub auto_minor_version_upgrade: Option<Boolean>,
    #[doc="<p>Indicates the status of Multi-AZ for the source replication group.</p> <note> <p>ElastiCache Multi-AZ replication groups are not supported on:</p> <ul> <li> <p>Redis versions earlier than 2.8.6.</p> </li> <li> <p>Redis (cluster mode disabled):T1 and T2 cache node types.</p> <p>Redis (cluster mode enabled): T1 node types.</p> </li> </ul> </note>"]
    pub automatic_failover: Option<AutomaticFailoverStatus>,
    #[doc="<p>The date and time when the source cache cluster was created.</p>"]
    pub cache_cluster_create_time: Option<TStamp>,
    #[doc="<p>The user-supplied identifier of the source cache cluster.</p>"]
    pub cache_cluster_id: Option<String>,
    #[doc="<p>The name of the compute and memory capacity node type for the source cache cluster.</p> <p>Valid node types are as follows:</p> <ul> <li> <p>General purpose:</p> <ul> <li> <p>Current generation: <code>cache.t2.micro</code>, <code>cache.t2.small</code>, <code>cache.t2.medium</code>, <code>cache.m3.medium</code>, <code>cache.m3.large</code>, <code>cache.m3.xlarge</code>, <code>cache.m3.2xlarge</code>, <code>cache.m4.large</code>, <code>cache.m4.xlarge</code>, <code>cache.m4.2xlarge</code>, <code>cache.m4.4xlarge</code>, <code>cache.m4.10xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.t1.micro</code>, <code>cache.m1.small</code>, <code>cache.m1.medium</code>, <code>cache.m1.large</code>, <code>cache.m1.xlarge</code> </p> </li> </ul> </li> <li> <p>Compute optimized: <code>cache.c1.xlarge</code> </p> </li> <li> <p>Memory optimized:</p> <ul> <li> <p>Current generation: <code>cache.r3.large</code>, <code>cache.r3.xlarge</code>, <code>cache.r3.2xlarge</code>, <code>cache.r3.4xlarge</code>, <code>cache.r3.8xlarge</code> </p> </li> <li> <p>Previous generation: <code>cache.m2.xlarge</code>, <code>cache.m2.2xlarge</code>, <code>cache.m2.4xlarge</code> </p> </li> </ul> </li> </ul> <p> <b>Notes:</b> </p> <ul> <li> <p>All T2 instances are created in an Amazon Virtual Private Cloud (Amazon VPC).</p> </li> <li> <p>Redis backup/restore is not supported for Redis (cluster mode disabled) T1 and T2 instances. Backup/restore is supported on Redis (cluster mode enabled) T2 instances.</p> </li> <li> <p>Redis Append-only files (AOF) functionality is not supported for T1 or T2 instances.</p> </li> </ul> <p>For a complete listing of node types and specifications, see <a href=\"http://aws.amazon.com/elasticache/details\">Amazon ElastiCache Product Features and Details</a> and either <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Memcached.html#ParameterGroups.Memcached.NodeSpecific\">Cache Node Type-Specific Parameters for Memcached</a> or <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/CacheParameterGroups.Redis.html#ParameterGroups.Redis.NodeSpecific\">Cache Node Type-Specific Parameters for Redis</a>.</p>"]
    pub cache_node_type: Option<String>,
    #[doc="<p>The cache parameter group that is associated with the source cache cluster.</p>"]
    pub cache_parameter_group_name: Option<String>,
    #[doc="<p>The name of the cache subnet group associated with the source cache cluster.</p>"]
    pub cache_subnet_group_name: Option<String>,
    #[doc="<p>The name of the cache engine (<code>memcached</code> or <code>redis</code>) used by the source cache cluster.</p>"]
    pub engine: Option<String>,
    #[doc="<p>The version of the cache engine version that is used by the source cache cluster.</p>"]
    pub engine_version: Option<String>,
    #[doc="<p>A list of the cache nodes in the source cache cluster.</p>"]
    pub node_snapshots: Option<NodeSnapshotList>,
    #[doc="<p>The number of cache nodes in the source cache cluster.</p> <p>For clusters running Redis, this value must be 1. For clusters running Memcached, this value must be between 1 and 20.</p>"]
    pub num_cache_nodes: Option<IntegerOptional>,
    #[doc="<p>The number of node groups (shards) in this snapshot. When restoring from a snapshot, the number of node groups (shards) in the snapshot and in the restored replication group must be the same.</p>"]
    pub num_node_groups: Option<IntegerOptional>,
    #[doc="<p>The port number used by each cache nodes in the source cache cluster.</p>"]
    pub port: Option<IntegerOptional>,
    #[doc="<p>The name of the Availability Zone in which the source cache cluster is located.</p>"]
    pub preferred_availability_zone: Option<String>,
    #[doc="<p>Specifies the weekly time range during which maintenance on the cluster is performed. It is specified as a range in the format ddd:hh24:mi-ddd:hh24:mi (24H Clock UTC). The minimum maintenance window is a 60 minute period.</p> <p>Valid values for <code>ddd</code> are:</p> <ul> <li> <p> <code>sun</code> </p> </li> <li> <p> <code>mon</code> </p> </li> <li> <p> <code>tue</code> </p> </li> <li> <p> <code>wed</code> </p> </li> <li> <p> <code>thu</code> </p> </li> <li> <p> <code>fri</code> </p> </li> <li> <p> <code>sat</code> </p> </li> </ul> <p>Example: <code>sun:23:00-mon:01:30</code> </p>"]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>A description of the source replication group.</p>"]
    pub replication_group_description: Option<String>,
    #[doc="<p>The unique identifier of the source replication group.</p>"]
    pub replication_group_id: Option<String>,
    #[doc="<p>The name of a snapshot. For an automatic snapshot, the name is system-generated. For a manual snapshot, this is the user-provided name.</p>"]
    pub snapshot_name: Option<String>,
    #[doc="<p>For an automatic snapshot, the number of days for which ElastiCache retains the snapshot before deleting it.</p> <p>For manual snapshots, this field reflects the <code>SnapshotRetentionLimit</code> for the source cache cluster when the snapshot was created. This field is otherwise ignored: Manual snapshots do not expire, and can only be deleted using the <code>DeleteSnapshot</code> operation. </p> <p> <b>Important</b> If the value of SnapshotRetentionLimit is set to zero (0), backups are turned off.</p>"]
    pub snapshot_retention_limit: Option<IntegerOptional>,
    #[doc="<p>Indicates whether the snapshot is from an automatic backup (<code>automated</code>) or was created manually (<code>manual</code>).</p>"]
    pub snapshot_source: Option<String>,
    #[doc="<p>The status of the snapshot. Valid values: <code>creating</code> | <code>available</code> | <code>restoring</code> | <code>copying</code> | <code>deleting</code>.</p>"]
    pub snapshot_status: Option<String>,
    #[doc="<p>The daily time range during which ElastiCache takes daily snapshots of the source cache cluster.</p>"]
    pub snapshot_window: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) for the topic used by the source cache cluster for publishing notifications.</p>"]
    pub topic_arn: Option<String>,
    #[doc="<p>The Amazon Virtual Private Cloud identifier (VPC ID) of the cache subnet group for the source cache cluster.</p>"]
    pub vpc_id: Option<String>,
}

struct SnapshotDeserializer;
impl SnapshotDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Snapshot, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "AutoMinorVersionUpgrade" => {
                            obj.auto_minor_version_upgrade =
                                Some(try!(BooleanDeserializer::deserialize("AutoMinorVersionUpgrade",
                                                                           stack)));
                        }
                        "AutomaticFailover" => {
                            obj.automatic_failover =
                                Some(try!(AutomaticFailoverStatusDeserializer::deserialize("AutomaticFailover",
                                                                                           stack)));
                        }
                        "CacheClusterCreateTime" => {
                            obj.cache_cluster_create_time =
                                Some(try!(TStampDeserializer::deserialize("CacheClusterCreateTime",
                                                                          stack)));
                        }
                        "CacheClusterId" => {
                            obj.cache_cluster_id =
                                Some(try!(StringDeserializer::deserialize("CacheClusterId",
                                                                          stack)));
                        }
                        "CacheNodeType" => {
                            obj.cache_node_type =
                                Some(try!(StringDeserializer::deserialize("CacheNodeType", stack)));
                        }
                        "CacheParameterGroupName" => {
                            obj.cache_parameter_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheParameterGroupName",
                                                                          stack)));
                        }
                        "CacheSubnetGroupName" => {
                            obj.cache_subnet_group_name =
                                Some(try!(StringDeserializer::deserialize("CacheSubnetGroupName",
                                                                          stack)));
                        }
                        "Engine" => {
                            obj.engine = Some(try!(StringDeserializer::deserialize("Engine",
                                                                                   stack)));
                        }
                        "EngineVersion" => {
                            obj.engine_version = Some(try!(StringDeserializer::deserialize("EngineVersion",
                                                                                           stack)));
                        }
                        "NodeSnapshots" => {
                            obj.node_snapshots =
                                Some(try!(NodeSnapshotListDeserializer::deserialize("NodeSnapshots",
                                                                                    stack)));
                        }
                        "NumCacheNodes" => {
                            obj.num_cache_nodes =
                                Some(try!(IntegerOptionalDeserializer::deserialize("NumCacheNodes",
                                                                                   stack)));
                        }
                        "NumNodeGroups" => {
                            obj.num_node_groups =
                                Some(try!(IntegerOptionalDeserializer::deserialize("NumNodeGroups",
                                                                                   stack)));
                        }
                        "Port" => {
                            obj.port = Some(try!(IntegerOptionalDeserializer::deserialize("Port",
                                                                                          stack)));
                        }
                        "PreferredAvailabilityZone" => {
                            obj.preferred_availability_zone =
                                Some(try!(StringDeserializer::deserialize("PreferredAvailabilityZone",
                                                                          stack)));
                        }
                        "PreferredMaintenanceWindow" => {
                            obj.preferred_maintenance_window =
                                Some(try!(StringDeserializer::deserialize("PreferredMaintenanceWindow",
                                                                          stack)));
                        }
                        "ReplicationGroupDescription" => {
                            obj.replication_group_description =
                                Some(try!(StringDeserializer::deserialize("ReplicationGroupDescription",
                                                                          stack)));
                        }
                        "ReplicationGroupId" => {
                            obj.replication_group_id =
                                Some(try!(StringDeserializer::deserialize("ReplicationGroupId",
                                                                          stack)));
                        }
                        "SnapshotName" => {
                            obj.snapshot_name = Some(try!(StringDeserializer::deserialize("SnapshotName",
                                                                                          stack)));
                        }
                        "SnapshotRetentionLimit" => {
                            obj.snapshot_retention_limit =
                                Some(try!(IntegerOptionalDeserializer::deserialize("SnapshotRetentionLimit",
                                                                                   stack)));
                        }
                        "SnapshotSource" => {
                            obj.snapshot_source =
                                Some(try!(StringDeserializer::deserialize("SnapshotSource",
                                                                          stack)));
                        }
                        "SnapshotStatus" => {
                            obj.snapshot_status =
                                Some(try!(StringDeserializer::deserialize("SnapshotStatus",
                                                                          stack)));
                        }
                        "SnapshotWindow" => {
                            obj.snapshot_window =
                                Some(try!(StringDeserializer::deserialize("SnapshotWindow",
                                                                          stack)));
                        }
                        "TopicArn" => {
                            obj.topic_arn = Some(try!(StringDeserializer::deserialize("TopicArn",
                                                                                      stack)));
                        }
                        "VpcId" => {
                            obj.vpc_id = Some(try!(StringDeserializer::deserialize("VpcId",
                                                                                   stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type SnapshotArnsList = Vec<String>;

/// Serialize `SnapshotArnsList` contents to a `SignedRequest`.
struct SnapshotArnsListSerializer;
impl SnapshotArnsListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SnapshotArnsList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type SnapshotList = Vec<Snapshot>;
struct SnapshotListDeserializer;
impl SnapshotListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<SnapshotList, XmlParseError> {

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
pub type SourceType = String;
struct SourceTypeDeserializer;
impl SourceTypeDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<SourceType, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
struct StringDeserializer;
impl StringDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<String, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>Represents the subnet associated with a cache cluster. This parameter refers to subnets defined in Amazon Virtual Private Cloud (Amazon VPC) and used with ElastiCache.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Subnet {
    #[doc="<p>The Availability Zone associated with the subnet.</p>"]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    #[doc="<p>The unique identifier for the subnet.</p>"]
    pub subnet_identifier: Option<String>,
}

struct SubnetDeserializer;
impl SubnetDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Subnet, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "SubnetAvailabilityZone" => {
                            obj.subnet_availability_zone =
                                Some(try!(AvailabilityZoneDeserializer::deserialize("SubnetAvailabilityZone",
                                                                                    stack)));
                        }
                        "SubnetIdentifier" => {
                            obj.subnet_identifier =
                                Some(try!(StringDeserializer::deserialize("SubnetIdentifier",
                                                                          stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
pub type SubnetIdentifierList = Vec<String>;

/// Serialize `SubnetIdentifierList` contents to a `SignedRequest`.
struct SubnetIdentifierListSerializer;
impl SubnetIdentifierListSerializer {
    fn serialize(params: &mut Params, name: &str, obj: &SubnetIdentifierList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            params.put(&key, &obj);
        }
    }
}

pub type SubnetList = Vec<Subnet>;
struct SubnetListDeserializer;
impl SubnetListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<SubnetList, XmlParseError> {

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
pub type TStamp = String;
struct TStampDeserializer;
impl TStampDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TStamp, XmlParseError> {
        try!(start_element(tag_name, stack));
        let obj = try!(characters(stack));
        try!(end_element(tag_name, stack));

        Ok(obj)

    }
}
#[doc="<p>A cost allocation Tag that can be added to an ElastiCache cluster or replication group. Tags are composed of a Key/Value pair. A tag with a null Value is permitted.</p>"]
#[derive(Default,Debug,Clone)]
pub struct Tag {
    #[doc="<p>The key for the tag.</p>"]
    pub key: Option<String>,
    #[doc="<p>The tag's value. May not be null.</p>"]
    pub value: Option<String>,
}

struct TagDeserializer;
impl TagDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<Tag, XmlParseError> {
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
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "Key" => {
                            obj.key = Some(try!(StringDeserializer::deserialize("Key", stack)));
                        }
                        "Value" => {
                            obj.value = Some(try!(StringDeserializer::deserialize("Value", stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
            params.put(&format!("{}{}", prefix, "Key"), &field_value);
        }
        if let Some(ref field_value) = obj.value {
            params.put(&format!("{}{}", prefix, "Value"), &field_value);
        }

    }
}

pub type TagList = Vec<Tag>;
struct TagListDeserializer;
impl TagListDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TagList, XmlParseError> {

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
    fn serialize(params: &mut Params, name: &str, obj: &TagList) {
        for (index, obj) in obj.iter().enumerate() {
            let key = format!("{}.member.{}", name, index + 1);
            TagSerializer::serialize(params, &key, obj);
        }
    }
}

#[doc="<p>Represents the output from the <code>AddTagsToResource</code>, <code>ListTagsOnResource</code>, and <code>RemoveTagsFromResource</code> operations.</p>"]
#[derive(Default,Debug,Clone)]
pub struct TagListMessage {
    #[doc="<p>A list of cost allocation tags as key-value pairs.</p>"]
    pub tag_list: Option<TagList>,
}

struct TagListMessageDeserializer;
impl TagListMessageDeserializer {
    #[allow(unused_variables)]
    fn deserialize<'a, T: Peek + Next>(tag_name: &str,
                                       stack: &mut T)
                                       -> Result<TagListMessage, XmlParseError> {
        try!(start_element(tag_name, stack));

        let mut obj = TagListMessage::default();

        loop {
            let next_event = match stack.peek() {
                Some(&Ok(XmlEvent::EndElement { ref name, .. })) => DeserializerNext::Close,
                Some(&Ok(XmlEvent::StartElement { ref name, .. })) => {
                    DeserializerNext::Element(name.local_name.to_owned())
                }
                _ => DeserializerNext::Skip,
            };

            match next_event {
                DeserializerNext::Element(name) => {
                    match &name[..] {
                        "TagList" => {
                            obj.tag_list = Some(try!(TagListDeserializer::deserialize("TagList",
                                                                                      stack)));
                        }
                        _ => skip_tree(stack),
                    }
                }
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
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    ///<p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 10.</p>
    TagQuotaPerResourceExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AddTagsToResourceError {
    pub fn from_body(body: &str) -> AddTagsToResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => AddTagsToResourceError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidARNFault" => {
                        AddTagsToResourceError::InvalidARNFault(String::from(parsed_error.message))
                    }
                    "SnapshotNotFoundFault" => AddTagsToResourceError::SnapshotNotFoundFault(String::from(parsed_error.message)),
                    "TagQuotaPerResourceExceeded" => AddTagsToResourceError::TagQuotaPerResourceExceeded(String::from(parsed_error.message)),
                    _ => AddTagsToResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToResourceError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for AddTagsToResourceError {
    fn from(err: XmlParseError) -> AddTagsToResourceError {
        let XmlParseError(message) = err;
        AddTagsToResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AddTagsToResourceError {
    fn from(err: CredentialsError) -> AddTagsToResourceError {
        AddTagsToResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddTagsToResourceError {
    fn from(err: HttpDispatchError) -> AddTagsToResourceError {
        AddTagsToResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for AddTagsToResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddTagsToResourceError {
    fn description(&self) -> &str {
        match *self {
            AddTagsToResourceError::CacheClusterNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::InvalidARNFault(ref cause) => cause,
            AddTagsToResourceError::SnapshotNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::TagQuotaPerResourceExceeded(ref cause) => cause,
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AuthorizeCacheSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum AuthorizeCacheSecurityGroupIngressError {
    ///<p>The specified Amazon EC2 security group is already authorized for the specified cache security group.</p>
    AuthorizationAlreadyExistsFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl AuthorizeCacheSecurityGroupIngressError {
    pub fn from_body(body: &str) -> AuthorizeCacheSecurityGroupIngressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "AuthorizationAlreadyExistsFault" => AuthorizeCacheSecurityGroupIngressError::AuthorizationAlreadyExistsFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => AuthorizeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheSecurityGroupStateFault" => AuthorizeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => AuthorizeCacheSecurityGroupIngressError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => AuthorizeCacheSecurityGroupIngressError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => AuthorizeCacheSecurityGroupIngressError::Unknown(String::from(body)),
                }
            }
            Err(_) => AuthorizeCacheSecurityGroupIngressError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for AuthorizeCacheSecurityGroupIngressError {
    fn from(err: XmlParseError) -> AuthorizeCacheSecurityGroupIngressError {
        let XmlParseError(message) = err;
        AuthorizeCacheSecurityGroupIngressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for AuthorizeCacheSecurityGroupIngressError {
    fn from(err: CredentialsError) -> AuthorizeCacheSecurityGroupIngressError {
        AuthorizeCacheSecurityGroupIngressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AuthorizeCacheSecurityGroupIngressError {
    fn from(err: HttpDispatchError) -> AuthorizeCacheSecurityGroupIngressError {
        AuthorizeCacheSecurityGroupIngressError::HttpDispatch(err)
    }
}
impl fmt::Display for AuthorizeCacheSecurityGroupIngressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AuthorizeCacheSecurityGroupIngressError {
    fn description(&self) -> &str {
        match *self {
            AuthorizeCacheSecurityGroupIngressError::AuthorizationAlreadyExistsFault(ref cause) => {
                cause
            }
            AuthorizeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(ref cause) => {
                cause
            }
            AuthorizeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(ref cause) => cause,
            AuthorizeCacheSecurityGroupIngressError::InvalidParameterCombination(ref cause) => {
                cause
            }
            AuthorizeCacheSecurityGroupIngressError::InvalidParameterValue(ref cause) => cause,
            AuthorizeCacheSecurityGroupIngressError::Validation(ref cause) => cause,
            AuthorizeCacheSecurityGroupIngressError::Credentials(ref err) => err.description(),
            AuthorizeCacheSecurityGroupIngressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AuthorizeCacheSecurityGroupIngressError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CopySnapshot
#[derive(Debug, PartialEq)]
pub enum CopySnapshotError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The current state of the snapshot does not allow the requested operation to occur.</p>
    InvalidSnapshotStateFault(String),
    ///<p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CopySnapshotError {
    pub fn from_body(body: &str) -> CopySnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => CopySnapshotError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => {
                        CopySnapshotError::InvalidParameterValue(String::from(parsed_error.message))
                    }
                    "InvalidSnapshotStateFault" => CopySnapshotError::InvalidSnapshotStateFault(String::from(parsed_error.message)),
                    "SnapshotAlreadyExistsFault" => CopySnapshotError::SnapshotAlreadyExistsFault(String::from(parsed_error.message)),
                    "SnapshotNotFoundFault" => {
                        CopySnapshotError::SnapshotNotFoundFault(String::from(parsed_error.message))
                    }
                    "SnapshotQuotaExceededFault" => CopySnapshotError::SnapshotQuotaExceededFault(String::from(parsed_error.message)),
                    _ => CopySnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CopySnapshotError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CopySnapshotError {
    fn from(err: XmlParseError) -> CopySnapshotError {
        let XmlParseError(message) = err;
        CopySnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CopySnapshotError {
    fn from(err: CredentialsError) -> CopySnapshotError {
        CopySnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CopySnapshotError {
    fn from(err: HttpDispatchError) -> CopySnapshotError {
        CopySnapshotError::HttpDispatch(err)
    }
}
impl fmt::Display for CopySnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CopySnapshotError {
    fn description(&self) -> &str {
        match *self {
            CopySnapshotError::InvalidParameterCombination(ref cause) => cause,
            CopySnapshotError::InvalidParameterValue(ref cause) => cause,
            CopySnapshotError::InvalidSnapshotStateFault(ref cause) => cause,
            CopySnapshotError::SnapshotAlreadyExistsFault(ref cause) => cause,
            CopySnapshotError::SnapshotNotFoundFault(ref cause) => cause,
            CopySnapshotError::SnapshotQuotaExceededFault(ref cause) => cause,
            CopySnapshotError::Validation(ref cause) => cause,
            CopySnapshotError::Credentials(ref err) => err.description(),
            CopySnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CopySnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCacheCluster
#[derive(Debug, PartialEq)]
pub enum CreateCacheClusterError {
    ///<p>You already have a cache cluster with the given identifier.</p>
    CacheClusterAlreadyExistsFault(String),
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    ///<p>The requested cache node type is not available in the specified Availability Zone.</p>
    InsufficientCacheClusterCapacityFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    ///<p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cache cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    ///<p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 10.</p>
    TagQuotaPerResourceExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCacheClusterError {
    pub fn from_body(body: &str) -> CreateCacheClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterAlreadyExistsFault" => CreateCacheClusterError::CacheClusterAlreadyExistsFault(String::from(parsed_error.message)),
                    "CacheParameterGroupNotFoundFault" => CreateCacheClusterError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => CreateCacheClusterError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSubnetGroupNotFoundFault" => CreateCacheClusterError::CacheSubnetGroupNotFoundFault(String::from(parsed_error.message)),
                    "ClusterQuotaForCustomerExceededFault" => CreateCacheClusterError::ClusterQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    "InsufficientCacheClusterCapacityFault" => CreateCacheClusterError::InsufficientCacheClusterCapacityFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => CreateCacheClusterError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => CreateCacheClusterError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidReplicationGroupStateFault" => CreateCacheClusterError::InvalidReplicationGroupStateFault(String::from(parsed_error.message)),
                    "InvalidVPCNetworkStateFault" => CreateCacheClusterError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),
                    "NodeQuotaForClusterExceededFault" => CreateCacheClusterError::NodeQuotaForClusterExceededFault(String::from(parsed_error.message)),
                    "NodeQuotaForCustomerExceededFault" => CreateCacheClusterError::NodeQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => CreateCacheClusterError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    "TagQuotaPerResourceExceeded" => CreateCacheClusterError::TagQuotaPerResourceExceeded(String::from(parsed_error.message)),
                    _ => CreateCacheClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCacheClusterError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateCacheClusterError {
    fn from(err: XmlParseError) -> CreateCacheClusterError {
        let XmlParseError(message) = err;
        CreateCacheClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCacheClusterError {
    fn from(err: CredentialsError) -> CreateCacheClusterError {
        CreateCacheClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCacheClusterError {
    fn from(err: HttpDispatchError) -> CreateCacheClusterError {
        CreateCacheClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCacheClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCacheClusterError {
    fn description(&self) -> &str {
        match *self {
            CreateCacheClusterError::CacheClusterAlreadyExistsFault(ref cause) => cause,
            CreateCacheClusterError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            CreateCacheClusterError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            CreateCacheClusterError::CacheSubnetGroupNotFoundFault(ref cause) => cause,
            CreateCacheClusterError::ClusterQuotaForCustomerExceededFault(ref cause) => cause,
            CreateCacheClusterError::InsufficientCacheClusterCapacityFault(ref cause) => cause,
            CreateCacheClusterError::InvalidParameterCombination(ref cause) => cause,
            CreateCacheClusterError::InvalidParameterValue(ref cause) => cause,
            CreateCacheClusterError::InvalidReplicationGroupStateFault(ref cause) => cause,
            CreateCacheClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateCacheClusterError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            CreateCacheClusterError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            CreateCacheClusterError::ReplicationGroupNotFoundFault(ref cause) => cause,
            CreateCacheClusterError::TagQuotaPerResourceExceeded(ref cause) => cause,
            CreateCacheClusterError::Validation(ref cause) => cause,
            CreateCacheClusterError::Credentials(ref err) => err.description(),
            CreateCacheClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCacheClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheParameterGroupError {
    ///<p>A cache parameter group with the requested name already exists.</p>
    CacheParameterGroupAlreadyExistsFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum number of cache security groups.</p>
    CacheParameterGroupQuotaExceededFault(String),
    ///<p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCacheParameterGroupError {
    pub fn from_body(body: &str) -> CreateCacheParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupAlreadyExistsFault" => CreateCacheParameterGroupError::CacheParameterGroupAlreadyExistsFault(String::from(parsed_error.message)),
                    "CacheParameterGroupQuotaExceededFault" => CreateCacheParameterGroupError::CacheParameterGroupQuotaExceededFault(String::from(parsed_error.message)),
                    "InvalidCacheParameterGroupStateFault" => CreateCacheParameterGroupError::InvalidCacheParameterGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => CreateCacheParameterGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => CreateCacheParameterGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => CreateCacheParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCacheParameterGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateCacheParameterGroupError {
    fn from(err: XmlParseError) -> CreateCacheParameterGroupError {
        let XmlParseError(message) = err;
        CreateCacheParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCacheParameterGroupError {
    fn from(err: CredentialsError) -> CreateCacheParameterGroupError {
        CreateCacheParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCacheParameterGroupError {
    fn from(err: HttpDispatchError) -> CreateCacheParameterGroupError {
        CreateCacheParameterGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCacheParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCacheParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateCacheParameterGroupError::CacheParameterGroupAlreadyExistsFault(ref cause) => {
                cause
            }
            CreateCacheParameterGroupError::CacheParameterGroupQuotaExceededFault(ref cause) => {
                cause
            }
            CreateCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                cause
            }
            CreateCacheParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            CreateCacheParameterGroupError::InvalidParameterValue(ref cause) => cause,
            CreateCacheParameterGroupError::Validation(ref cause) => cause,
            CreateCacheParameterGroupError::Credentials(ref err) => err.description(),
            CreateCacheParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCacheParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCacheSecurityGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheSecurityGroupError {
    ///<p>A cache security group with the specified name already exists.</p>
    CacheSecurityGroupAlreadyExistsFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache security groups.</p>
    CacheSecurityGroupQuotaExceededFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCacheSecurityGroupError {
    pub fn from_body(body: &str) -> CreateCacheSecurityGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupAlreadyExistsFault" => CreateCacheSecurityGroupError::CacheSecurityGroupAlreadyExistsFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupQuotaExceededFault" => CreateCacheSecurityGroupError::CacheSecurityGroupQuotaExceededFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => CreateCacheSecurityGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => CreateCacheSecurityGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => CreateCacheSecurityGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCacheSecurityGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateCacheSecurityGroupError {
    fn from(err: XmlParseError) -> CreateCacheSecurityGroupError {
        let XmlParseError(message) = err;
        CreateCacheSecurityGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCacheSecurityGroupError {
    fn from(err: CredentialsError) -> CreateCacheSecurityGroupError {
        CreateCacheSecurityGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCacheSecurityGroupError {
    fn from(err: HttpDispatchError) -> CreateCacheSecurityGroupError {
        CreateCacheSecurityGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCacheSecurityGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCacheSecurityGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateCacheSecurityGroupError::CacheSecurityGroupAlreadyExistsFault(ref cause) => cause,
            CreateCacheSecurityGroupError::CacheSecurityGroupQuotaExceededFault(ref cause) => cause,
            CreateCacheSecurityGroupError::InvalidParameterCombination(ref cause) => cause,
            CreateCacheSecurityGroupError::InvalidParameterValue(ref cause) => cause,
            CreateCacheSecurityGroupError::Validation(ref cause) => cause,
            CreateCacheSecurityGroupError::Credentials(ref err) => err.description(),
            CreateCacheSecurityGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCacheSecurityGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateCacheSubnetGroupError {
    ///<p>The requested cache subnet group name is already in use by an existing cache subnet group.</p>
    CacheSubnetGroupAlreadyExistsFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache subnet groups.</p>
    CacheSubnetGroupQuotaExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of subnets in a cache subnet group.</p>
    CacheSubnetQuotaExceededFault(String),
    ///<p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateCacheSubnetGroupError {
    pub fn from_body(body: &str) -> CreateCacheSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupAlreadyExistsFault" => CreateCacheSubnetGroupError::CacheSubnetGroupAlreadyExistsFault(String::from(parsed_error.message)),
                    "CacheSubnetGroupQuotaExceededFault" => CreateCacheSubnetGroupError::CacheSubnetGroupQuotaExceededFault(String::from(parsed_error.message)),
                    "CacheSubnetQuotaExceededFault" => CreateCacheSubnetGroupError::CacheSubnetQuotaExceededFault(String::from(parsed_error.message)),
                    "InvalidSubnet" => CreateCacheSubnetGroupError::InvalidSubnet(String::from(parsed_error.message)),
                    _ => CreateCacheSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateCacheSubnetGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateCacheSubnetGroupError {
    fn from(err: XmlParseError) -> CreateCacheSubnetGroupError {
        let XmlParseError(message) = err;
        CreateCacheSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateCacheSubnetGroupError {
    fn from(err: CredentialsError) -> CreateCacheSubnetGroupError {
        CreateCacheSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateCacheSubnetGroupError {
    fn from(err: HttpDispatchError) -> CreateCacheSubnetGroupError {
        CreateCacheSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateCacheSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateCacheSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateCacheSubnetGroupError::CacheSubnetGroupAlreadyExistsFault(ref cause) => cause,
            CreateCacheSubnetGroupError::CacheSubnetGroupQuotaExceededFault(ref cause) => cause,
            CreateCacheSubnetGroupError::CacheSubnetQuotaExceededFault(ref cause) => cause,
            CreateCacheSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateCacheSubnetGroupError::Validation(ref cause) => cause,
            CreateCacheSubnetGroupError::Credentials(ref err) => err.description(),
            CreateCacheSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateCacheSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationGroup
#[derive(Debug, PartialEq)]
pub enum CreateReplicationGroupError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache clusters per customer.</p>
    ClusterQuotaForCustomerExceededFault(String),
    ///<p>The requested cache node type is not available in the specified Availability Zone.</p>
    InsufficientCacheClusterCapacityFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum of 15 node groups (shards) in a single replication group.</p>
    NodeGroupsPerReplicationGroupQuotaExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cache cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    ///<p>The specified replication group already exists.</p>
    ReplicationGroupAlreadyExistsFault(String),
    ///<p>The request cannot be processed because it would cause the resource to have more than the allowed number of tags. The maximum number of tags permitted on a resource is 10.</p>
    TagQuotaPerResourceExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateReplicationGroupError {
    pub fn from_body(body: &str) -> CreateReplicationGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => CreateReplicationGroupError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "CacheParameterGroupNotFoundFault" => CreateReplicationGroupError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => CreateReplicationGroupError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSubnetGroupNotFoundFault" => CreateReplicationGroupError::CacheSubnetGroupNotFoundFault(String::from(parsed_error.message)),
                    "ClusterQuotaForCustomerExceededFault" => CreateReplicationGroupError::ClusterQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    "InsufficientCacheClusterCapacityFault" => CreateReplicationGroupError::InsufficientCacheClusterCapacityFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => CreateReplicationGroupError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => CreateReplicationGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => CreateReplicationGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidVPCNetworkStateFault" => CreateReplicationGroupError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),
                    "NodeGroupsPerReplicationGroupQuotaExceededFault" => CreateReplicationGroupError::NodeGroupsPerReplicationGroupQuotaExceededFault(String::from(parsed_error.message)),
                    "NodeQuotaForClusterExceededFault" => CreateReplicationGroupError::NodeQuotaForClusterExceededFault(String::from(parsed_error.message)),
                    "NodeQuotaForCustomerExceededFault" => CreateReplicationGroupError::NodeQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    "ReplicationGroupAlreadyExistsFault" => CreateReplicationGroupError::ReplicationGroupAlreadyExistsFault(String::from(parsed_error.message)),
                    "TagQuotaPerResourceExceeded" => CreateReplicationGroupError::TagQuotaPerResourceExceeded(String::from(parsed_error.message)),
                    _ => CreateReplicationGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateReplicationGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateReplicationGroupError {
    fn from(err: XmlParseError) -> CreateReplicationGroupError {
        let XmlParseError(message) = err;
        CreateReplicationGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateReplicationGroupError {
    fn from(err: CredentialsError) -> CreateReplicationGroupError {
        CreateReplicationGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationGroupError {
    fn from(err: HttpDispatchError) -> CreateReplicationGroupError {
        CreateReplicationGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateReplicationGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationGroupError::CacheClusterNotFoundFault(ref cause) => cause,
            CreateReplicationGroupError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            CreateReplicationGroupError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            CreateReplicationGroupError::CacheSubnetGroupNotFoundFault(ref cause) => cause,
            CreateReplicationGroupError::ClusterQuotaForCustomerExceededFault(ref cause) => cause,
            CreateReplicationGroupError::InsufficientCacheClusterCapacityFault(ref cause) => cause,
            CreateReplicationGroupError::InvalidCacheClusterStateFault(ref cause) => cause,
            CreateReplicationGroupError::InvalidParameterCombination(ref cause) => cause,
            CreateReplicationGroupError::InvalidParameterValue(ref cause) => cause,
            CreateReplicationGroupError::InvalidVPCNetworkStateFault(ref cause) => cause,
            CreateReplicationGroupError::NodeGroupsPerReplicationGroupQuotaExceededFault(ref cause) => cause,
            CreateReplicationGroupError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            CreateReplicationGroupError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            CreateReplicationGroupError::ReplicationGroupAlreadyExistsFault(ref cause) => cause,
            CreateReplicationGroupError::TagQuotaPerResourceExceeded(ref cause) => cause,
            CreateReplicationGroupError::Validation(ref cause) => cause,
            CreateReplicationGroupError::Credentials(ref err) => err.description(),
            CreateReplicationGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateSnapshot
#[derive(Debug, PartialEq)]
pub enum CreateSnapshotError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    ///<p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    ///<p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cache cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cache cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateSnapshotError {
    pub fn from_body(body: &str) -> CreateSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => CreateSnapshotError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => CreateSnapshotError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => CreateSnapshotError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => CreateSnapshotError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidReplicationGroupStateFault" => CreateSnapshotError::InvalidReplicationGroupStateFault(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => CreateSnapshotError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    "SnapshotAlreadyExistsFault" => CreateSnapshotError::SnapshotAlreadyExistsFault(String::from(parsed_error.message)),
                    "SnapshotFeatureNotSupportedFault" => CreateSnapshotError::SnapshotFeatureNotSupportedFault(String::from(parsed_error.message)),
                    "SnapshotQuotaExceededFault" => CreateSnapshotError::SnapshotQuotaExceededFault(String::from(parsed_error.message)),
                    _ => CreateSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateSnapshotError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for CreateSnapshotError {
    fn from(err: XmlParseError) -> CreateSnapshotError {
        let XmlParseError(message) = err;
        CreateSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for CreateSnapshotError {
    fn from(err: CredentialsError) -> CreateSnapshotError {
        CreateSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateSnapshotError {
    fn from(err: HttpDispatchError) -> CreateSnapshotError {
        CreateSnapshotError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateSnapshotError {
    fn description(&self) -> &str {
        match *self {
            CreateSnapshotError::CacheClusterNotFoundFault(ref cause) => cause,
            CreateSnapshotError::InvalidCacheClusterStateFault(ref cause) => cause,
            CreateSnapshotError::InvalidParameterCombination(ref cause) => cause,
            CreateSnapshotError::InvalidParameterValue(ref cause) => cause,
            CreateSnapshotError::InvalidReplicationGroupStateFault(ref cause) => cause,
            CreateSnapshotError::ReplicationGroupNotFoundFault(ref cause) => cause,
            CreateSnapshotError::SnapshotAlreadyExistsFault(ref cause) => cause,
            CreateSnapshotError::SnapshotFeatureNotSupportedFault(ref cause) => cause,
            CreateSnapshotError::SnapshotQuotaExceededFault(ref cause) => cause,
            CreateSnapshotError::Validation(ref cause) => cause,
            CreateSnapshotError::Credentials(ref err) => err.description(),
            CreateSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCacheCluster
#[derive(Debug, PartialEq)]
pub enum DeleteCacheClusterError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    ///<p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cache cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cache cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCacheClusterError {
    pub fn from_body(body: &str) -> DeleteCacheClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => DeleteCacheClusterError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => DeleteCacheClusterError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DeleteCacheClusterError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DeleteCacheClusterError::InvalidParameterValue(String::from(parsed_error.message)),
                    "SnapshotAlreadyExistsFault" => DeleteCacheClusterError::SnapshotAlreadyExistsFault(String::from(parsed_error.message)),
                    "SnapshotFeatureNotSupportedFault" => DeleteCacheClusterError::SnapshotFeatureNotSupportedFault(String::from(parsed_error.message)),
                    "SnapshotQuotaExceededFault" => DeleteCacheClusterError::SnapshotQuotaExceededFault(String::from(parsed_error.message)),
                    _ => DeleteCacheClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCacheClusterError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteCacheClusterError {
    fn from(err: XmlParseError) -> DeleteCacheClusterError {
        let XmlParseError(message) = err;
        DeleteCacheClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCacheClusterError {
    fn from(err: CredentialsError) -> DeleteCacheClusterError {
        DeleteCacheClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCacheClusterError {
    fn from(err: HttpDispatchError) -> DeleteCacheClusterError {
        DeleteCacheClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCacheClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCacheClusterError {
    fn description(&self) -> &str {
        match *self {
            DeleteCacheClusterError::CacheClusterNotFoundFault(ref cause) => cause,
            DeleteCacheClusterError::InvalidCacheClusterStateFault(ref cause) => cause,
            DeleteCacheClusterError::InvalidParameterCombination(ref cause) => cause,
            DeleteCacheClusterError::InvalidParameterValue(ref cause) => cause,
            DeleteCacheClusterError::SnapshotAlreadyExistsFault(ref cause) => cause,
            DeleteCacheClusterError::SnapshotFeatureNotSupportedFault(ref cause) => cause,
            DeleteCacheClusterError::SnapshotQuotaExceededFault(ref cause) => cause,
            DeleteCacheClusterError::Validation(ref cause) => cause,
            DeleteCacheClusterError::Credentials(ref err) => err.description(),
            DeleteCacheClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCacheClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheParameterGroupError {
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCacheParameterGroupError {
    pub fn from_body(body: &str) -> DeleteCacheParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFoundFault" => DeleteCacheParameterGroupError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheParameterGroupStateFault" => DeleteCacheParameterGroupError::InvalidCacheParameterGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DeleteCacheParameterGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DeleteCacheParameterGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DeleteCacheParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCacheParameterGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteCacheParameterGroupError {
    fn from(err: XmlParseError) -> DeleteCacheParameterGroupError {
        let XmlParseError(message) = err;
        DeleteCacheParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCacheParameterGroupError {
    fn from(err: CredentialsError) -> DeleteCacheParameterGroupError {
        DeleteCacheParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCacheParameterGroupError {
    fn from(err: HttpDispatchError) -> DeleteCacheParameterGroupError {
        DeleteCacheParameterGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCacheParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCacheParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            DeleteCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                cause
            }
            DeleteCacheParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            DeleteCacheParameterGroupError::InvalidParameterValue(ref cause) => cause,
            DeleteCacheParameterGroupError::Validation(ref cause) => cause,
            DeleteCacheParameterGroupError::Credentials(ref err) => err.description(),
            DeleteCacheParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCacheParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCacheSecurityGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheSecurityGroupError {
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCacheSecurityGroupError {
    pub fn from_body(body: &str) -> DeleteCacheSecurityGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupNotFoundFault" => DeleteCacheSecurityGroupError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheSecurityGroupStateFault" => DeleteCacheSecurityGroupError::InvalidCacheSecurityGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DeleteCacheSecurityGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DeleteCacheSecurityGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DeleteCacheSecurityGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCacheSecurityGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteCacheSecurityGroupError {
    fn from(err: XmlParseError) -> DeleteCacheSecurityGroupError {
        let XmlParseError(message) = err;
        DeleteCacheSecurityGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCacheSecurityGroupError {
    fn from(err: CredentialsError) -> DeleteCacheSecurityGroupError {
        DeleteCacheSecurityGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCacheSecurityGroupError {
    fn from(err: HttpDispatchError) -> DeleteCacheSecurityGroupError {
        DeleteCacheSecurityGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCacheSecurityGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCacheSecurityGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteCacheSecurityGroupError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            DeleteCacheSecurityGroupError::InvalidCacheSecurityGroupStateFault(ref cause) => cause,
            DeleteCacheSecurityGroupError::InvalidParameterCombination(ref cause) => cause,
            DeleteCacheSecurityGroupError::InvalidParameterValue(ref cause) => cause,
            DeleteCacheSecurityGroupError::Validation(ref cause) => cause,
            DeleteCacheSecurityGroupError::Credentials(ref err) => err.description(),
            DeleteCacheSecurityGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCacheSecurityGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteCacheSubnetGroupError {
    ///<p>The requested cache subnet group is currently in use.</p>
    CacheSubnetGroupInUse(String),
    ///<p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteCacheSubnetGroupError {
    pub fn from_body(body: &str) -> DeleteCacheSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupInUse" => DeleteCacheSubnetGroupError::CacheSubnetGroupInUse(String::from(parsed_error.message)),
                    "CacheSubnetGroupNotFoundFault" => DeleteCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(String::from(parsed_error.message)),
                    _ => DeleteCacheSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCacheSubnetGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteCacheSubnetGroupError {
    fn from(err: XmlParseError) -> DeleteCacheSubnetGroupError {
        let XmlParseError(message) = err;
        DeleteCacheSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteCacheSubnetGroupError {
    fn from(err: CredentialsError) -> DeleteCacheSubnetGroupError {
        DeleteCacheSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCacheSubnetGroupError {
    fn from(err: HttpDispatchError) -> DeleteCacheSubnetGroupError {
        DeleteCacheSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCacheSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCacheSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteCacheSubnetGroupError::CacheSubnetGroupInUse(ref cause) => cause,
            DeleteCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(ref cause) => cause,
            DeleteCacheSubnetGroupError::Validation(ref cause) => cause,
            DeleteCacheSubnetGroupError::Credentials(ref err) => err.description(),
            DeleteCacheSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCacheSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationGroupError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    ///<p>You already have a snapshot with the given name.</p>
    SnapshotAlreadyExistsFault(String),
    ///<p>You attempted one of the following operations:</p> <ul> <li> <p>Creating a snapshot of a Redis cache cluster running on a <code>cache.t1.micro</code> cache node.</p> </li> <li> <p>Creating a snapshot of a cache cluster that is running Memcached rather than Redis.</p> </li> </ul> <p>Neither of these are supported by ElastiCache.</p>
    SnapshotFeatureNotSupportedFault(String),
    ///<p>The request cannot be processed because it would exceed the maximum number of snapshots.</p>
    SnapshotQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteReplicationGroupError {
    pub fn from_body(body: &str) -> DeleteReplicationGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DeleteReplicationGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DeleteReplicationGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidReplicationGroupStateFault" => DeleteReplicationGroupError::InvalidReplicationGroupStateFault(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => DeleteReplicationGroupError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    "SnapshotAlreadyExistsFault" => DeleteReplicationGroupError::SnapshotAlreadyExistsFault(String::from(parsed_error.message)),
                    "SnapshotFeatureNotSupportedFault" => DeleteReplicationGroupError::SnapshotFeatureNotSupportedFault(String::from(parsed_error.message)),
                    "SnapshotQuotaExceededFault" => DeleteReplicationGroupError::SnapshotQuotaExceededFault(String::from(parsed_error.message)),
                    _ => DeleteReplicationGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteReplicationGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteReplicationGroupError {
    fn from(err: XmlParseError) -> DeleteReplicationGroupError {
        let XmlParseError(message) = err;
        DeleteReplicationGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationGroupError {
    fn from(err: CredentialsError) -> DeleteReplicationGroupError {
        DeleteReplicationGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationGroupError {
    fn from(err: HttpDispatchError) -> DeleteReplicationGroupError {
        DeleteReplicationGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteReplicationGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationGroupError::InvalidParameterCombination(ref cause) => cause,
            DeleteReplicationGroupError::InvalidParameterValue(ref cause) => cause,
            DeleteReplicationGroupError::InvalidReplicationGroupStateFault(ref cause) => cause,
            DeleteReplicationGroupError::ReplicationGroupNotFoundFault(ref cause) => cause,
            DeleteReplicationGroupError::SnapshotAlreadyExistsFault(ref cause) => cause,
            DeleteReplicationGroupError::SnapshotFeatureNotSupportedFault(ref cause) => cause,
            DeleteReplicationGroupError::SnapshotQuotaExceededFault(ref cause) => cause,
            DeleteReplicationGroupError::Validation(ref cause) => cause,
            DeleteReplicationGroupError::Credentials(ref err) => err.description(),
            DeleteReplicationGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteSnapshot
#[derive(Debug, PartialEq)]
pub enum DeleteSnapshotError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The current state of the snapshot does not allow the requested operation to occur.</p>
    InvalidSnapshotStateFault(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteSnapshotError {
    pub fn from_body(body: &str) -> DeleteSnapshotError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DeleteSnapshotError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DeleteSnapshotError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidSnapshotStateFault" => DeleteSnapshotError::InvalidSnapshotStateFault(String::from(parsed_error.message)),
                    "SnapshotNotFoundFault" => DeleteSnapshotError::SnapshotNotFoundFault(String::from(parsed_error.message)),
                    _ => DeleteSnapshotError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteSnapshotError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DeleteSnapshotError {
    fn from(err: XmlParseError) -> DeleteSnapshotError {
        let XmlParseError(message) = err;
        DeleteSnapshotError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DeleteSnapshotError {
    fn from(err: CredentialsError) -> DeleteSnapshotError {
        DeleteSnapshotError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteSnapshotError {
    fn from(err: HttpDispatchError) -> DeleteSnapshotError {
        DeleteSnapshotError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteSnapshotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteSnapshotError {
    fn description(&self) -> &str {
        match *self {
            DeleteSnapshotError::InvalidParameterCombination(ref cause) => cause,
            DeleteSnapshotError::InvalidParameterValue(ref cause) => cause,
            DeleteSnapshotError::InvalidSnapshotStateFault(ref cause) => cause,
            DeleteSnapshotError::SnapshotNotFoundFault(ref cause) => cause,
            DeleteSnapshotError::Validation(ref cause) => cause,
            DeleteSnapshotError::Credentials(ref err) => err.description(),
            DeleteSnapshotError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteSnapshotError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheClusters
#[derive(Debug, PartialEq)]
pub enum DescribeCacheClustersError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheClustersError {
    pub fn from_body(body: &str) -> DescribeCacheClustersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => DescribeCacheClustersError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DescribeCacheClustersError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeCacheClustersError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeCacheClustersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheClustersError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheClustersError {
    fn from(err: XmlParseError) -> DescribeCacheClustersError {
        let XmlParseError(message) = err;
        DescribeCacheClustersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheClustersError {
    fn from(err: CredentialsError) -> DescribeCacheClustersError {
        DescribeCacheClustersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheClustersError {
    fn from(err: HttpDispatchError) -> DescribeCacheClustersError {
        DescribeCacheClustersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheClustersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheClustersError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheClustersError::CacheClusterNotFoundFault(ref cause) => cause,
            DescribeCacheClustersError::InvalidParameterCombination(ref cause) => cause,
            DescribeCacheClustersError::InvalidParameterValue(ref cause) => cause,
            DescribeCacheClustersError::Validation(ref cause) => cause,
            DescribeCacheClustersError::Credentials(ref err) => err.description(),
            DescribeCacheClustersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheClustersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheEngineVersions
#[derive(Debug, PartialEq)]
pub enum DescribeCacheEngineVersionsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheEngineVersionsError {
    pub fn from_body(body: &str) -> DescribeCacheEngineVersionsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    _ => DescribeCacheEngineVersionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheEngineVersionsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheEngineVersionsError {
    fn from(err: XmlParseError) -> DescribeCacheEngineVersionsError {
        let XmlParseError(message) = err;
        DescribeCacheEngineVersionsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheEngineVersionsError {
    fn from(err: CredentialsError) -> DescribeCacheEngineVersionsError {
        DescribeCacheEngineVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheEngineVersionsError {
    fn from(err: HttpDispatchError) -> DescribeCacheEngineVersionsError {
        DescribeCacheEngineVersionsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheEngineVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheEngineVersionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheEngineVersionsError::Validation(ref cause) => cause,
            DescribeCacheEngineVersionsError::Credentials(ref err) => err.description(),
            DescribeCacheEngineVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheEngineVersionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheParameterGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheParameterGroupsError {
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheParameterGroupsError {
    pub fn from_body(body: &str) -> DescribeCacheParameterGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFoundFault" => DescribeCacheParameterGroupsError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DescribeCacheParameterGroupsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeCacheParameterGroupsError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeCacheParameterGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheParameterGroupsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheParameterGroupsError {
    fn from(err: XmlParseError) -> DescribeCacheParameterGroupsError {
        let XmlParseError(message) = err;
        DescribeCacheParameterGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheParameterGroupsError {
    fn from(err: CredentialsError) -> DescribeCacheParameterGroupsError {
        DescribeCacheParameterGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheParameterGroupsError {
    fn from(err: HttpDispatchError) -> DescribeCacheParameterGroupsError {
        DescribeCacheParameterGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheParameterGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheParameterGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheParameterGroupsError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            DescribeCacheParameterGroupsError::InvalidParameterCombination(ref cause) => cause,
            DescribeCacheParameterGroupsError::InvalidParameterValue(ref cause) => cause,
            DescribeCacheParameterGroupsError::Validation(ref cause) => cause,
            DescribeCacheParameterGroupsError::Credentials(ref err) => err.description(),
            DescribeCacheParameterGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheParameterGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheParameters
#[derive(Debug, PartialEq)]
pub enum DescribeCacheParametersError {
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheParametersError {
    pub fn from_body(body: &str) -> DescribeCacheParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFoundFault" => DescribeCacheParametersError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DescribeCacheParametersError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeCacheParametersError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeCacheParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheParametersError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheParametersError {
    fn from(err: XmlParseError) -> DescribeCacheParametersError {
        let XmlParseError(message) = err;
        DescribeCacheParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheParametersError {
    fn from(err: CredentialsError) -> DescribeCacheParametersError {
        DescribeCacheParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheParametersError {
    fn from(err: HttpDispatchError) -> DescribeCacheParametersError {
        DescribeCacheParametersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheParametersError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            DescribeCacheParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeCacheParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeCacheParametersError::Validation(ref cause) => cause,
            DescribeCacheParametersError::Credentials(ref err) => err.description(),
            DescribeCacheParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheSecurityGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheSecurityGroupsError {
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheSecurityGroupsError {
    pub fn from_body(body: &str) -> DescribeCacheSecurityGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSecurityGroupNotFoundFault" => DescribeCacheSecurityGroupsError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DescribeCacheSecurityGroupsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeCacheSecurityGroupsError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeCacheSecurityGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheSecurityGroupsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheSecurityGroupsError {
    fn from(err: XmlParseError) -> DescribeCacheSecurityGroupsError {
        let XmlParseError(message) = err;
        DescribeCacheSecurityGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheSecurityGroupsError {
    fn from(err: CredentialsError) -> DescribeCacheSecurityGroupsError {
        DescribeCacheSecurityGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheSecurityGroupsError {
    fn from(err: HttpDispatchError) -> DescribeCacheSecurityGroupsError {
        DescribeCacheSecurityGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheSecurityGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheSecurityGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheSecurityGroupsError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            DescribeCacheSecurityGroupsError::InvalidParameterCombination(ref cause) => cause,
            DescribeCacheSecurityGroupsError::InvalidParameterValue(ref cause) => cause,
            DescribeCacheSecurityGroupsError::Validation(ref cause) => cause,
            DescribeCacheSecurityGroupsError::Credentials(ref err) => err.description(),
            DescribeCacheSecurityGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheSecurityGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCacheSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeCacheSubnetGroupsError {
    ///<p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeCacheSubnetGroupsError {
    pub fn from_body(body: &str) -> DescribeCacheSubnetGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupNotFoundFault" => DescribeCacheSubnetGroupsError::CacheSubnetGroupNotFoundFault(String::from(parsed_error.message)),
                    _ => DescribeCacheSubnetGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCacheSubnetGroupsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeCacheSubnetGroupsError {
    fn from(err: XmlParseError) -> DescribeCacheSubnetGroupsError {
        let XmlParseError(message) = err;
        DescribeCacheSubnetGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeCacheSubnetGroupsError {
    fn from(err: CredentialsError) -> DescribeCacheSubnetGroupsError {
        DescribeCacheSubnetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCacheSubnetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeCacheSubnetGroupsError {
        DescribeCacheSubnetGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCacheSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCacheSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeCacheSubnetGroupsError::CacheSubnetGroupNotFoundFault(ref cause) => cause,
            DescribeCacheSubnetGroupsError::Validation(ref cause) => cause,
            DescribeCacheSubnetGroupsError::Credentials(ref err) => err.description(),
            DescribeCacheSubnetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCacheSubnetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEngineDefaultParameters
#[derive(Debug, PartialEq)]
pub enum DescribeEngineDefaultParametersError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEngineDefaultParametersError {
    pub fn from_body(body: &str) -> DescribeEngineDefaultParametersError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DescribeEngineDefaultParametersError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeEngineDefaultParametersError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeEngineDefaultParametersError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEngineDefaultParametersError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeEngineDefaultParametersError {
    fn from(err: XmlParseError) -> DescribeEngineDefaultParametersError {
        let XmlParseError(message) = err;
        DescribeEngineDefaultParametersError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeEngineDefaultParametersError {
    fn from(err: CredentialsError) -> DescribeEngineDefaultParametersError {
        DescribeEngineDefaultParametersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEngineDefaultParametersError {
    fn from(err: HttpDispatchError) -> DescribeEngineDefaultParametersError {
        DescribeEngineDefaultParametersError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEngineDefaultParametersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEngineDefaultParametersError {
    fn description(&self) -> &str {
        match *self {
            DescribeEngineDefaultParametersError::InvalidParameterCombination(ref cause) => cause,
            DescribeEngineDefaultParametersError::InvalidParameterValue(ref cause) => cause,
            DescribeEngineDefaultParametersError::Validation(ref cause) => cause,
            DescribeEngineDefaultParametersError::Credentials(ref err) => err.description(),
            DescribeEngineDefaultParametersError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEngineDefaultParametersError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEvents
#[derive(Debug, PartialEq)]
pub enum DescribeEventsError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
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
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DescribeEventsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeEventsError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => DescribeEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventsError::Unknown(body.to_string()),
        }
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
impl fmt::Display for DescribeEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventsError::InvalidParameterCombination(ref cause) => cause,
            DescribeEventsError::InvalidParameterValue(ref cause) => cause,
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationGroups
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationGroupsError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeReplicationGroupsError {
    pub fn from_body(body: &str) -> DescribeReplicationGroupsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DescribeReplicationGroupsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeReplicationGroupsError::InvalidParameterValue(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => DescribeReplicationGroupsError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    _ => DescribeReplicationGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReplicationGroupsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeReplicationGroupsError {
    fn from(err: XmlParseError) -> DescribeReplicationGroupsError {
        let XmlParseError(message) = err;
        DescribeReplicationGroupsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReplicationGroupsError {
    fn from(err: CredentialsError) -> DescribeReplicationGroupsError {
        DescribeReplicationGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReplicationGroupsError {
    fn from(err: HttpDispatchError) -> DescribeReplicationGroupsError {
        DescribeReplicationGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReplicationGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationGroupsError::InvalidParameterCombination(ref cause) => cause,
            DescribeReplicationGroupsError::InvalidParameterValue(ref cause) => cause,
            DescribeReplicationGroupsError::ReplicationGroupNotFoundFault(ref cause) => cause,
            DescribeReplicationGroupsError::Validation(ref cause) => cause,
            DescribeReplicationGroupsError::Credentials(ref err) => err.description(),
            DescribeReplicationGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReplicationGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReservedCacheNodes
#[derive(Debug, PartialEq)]
pub enum DescribeReservedCacheNodesError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested reserved cache node was not found.</p>
    ReservedCacheNodeNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeReservedCacheNodesError {
    pub fn from_body(body: &str) -> DescribeReservedCacheNodesError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DescribeReservedCacheNodesError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeReservedCacheNodesError::InvalidParameterValue(String::from(parsed_error.message)),
                    "ReservedCacheNodeNotFoundFault" => DescribeReservedCacheNodesError::ReservedCacheNodeNotFoundFault(String::from(parsed_error.message)),
                    _ => DescribeReservedCacheNodesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReservedCacheNodesError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeReservedCacheNodesError {
    fn from(err: XmlParseError) -> DescribeReservedCacheNodesError {
        let XmlParseError(message) = err;
        DescribeReservedCacheNodesError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReservedCacheNodesError {
    fn from(err: CredentialsError) -> DescribeReservedCacheNodesError {
        DescribeReservedCacheNodesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReservedCacheNodesError {
    fn from(err: HttpDispatchError) -> DescribeReservedCacheNodesError {
        DescribeReservedCacheNodesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReservedCacheNodesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReservedCacheNodesError {
    fn description(&self) -> &str {
        match *self {
            DescribeReservedCacheNodesError::InvalidParameterCombination(ref cause) => cause,
            DescribeReservedCacheNodesError::InvalidParameterValue(ref cause) => cause,
            DescribeReservedCacheNodesError::ReservedCacheNodeNotFoundFault(ref cause) => cause,
            DescribeReservedCacheNodesError::Validation(ref cause) => cause,
            DescribeReservedCacheNodesError::Credentials(ref err) => err.description(),
            DescribeReservedCacheNodesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReservedCacheNodesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReservedCacheNodesOfferings
#[derive(Debug, PartialEq)]
pub enum DescribeReservedCacheNodesOfferingsError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested cache node offering does not exist.</p>
    ReservedCacheNodesOfferingNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeReservedCacheNodesOfferingsError {
    pub fn from_body(body: &str) -> DescribeReservedCacheNodesOfferingsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => DescribeReservedCacheNodesOfferingsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeReservedCacheNodesOfferingsError::InvalidParameterValue(String::from(parsed_error.message)),
                    "ReservedCacheNodesOfferingNotFoundFault" => DescribeReservedCacheNodesOfferingsError::ReservedCacheNodesOfferingNotFoundFault(String::from(parsed_error.message)),
                    _ => DescribeReservedCacheNodesOfferingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReservedCacheNodesOfferingsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeReservedCacheNodesOfferingsError {
    fn from(err: XmlParseError) -> DescribeReservedCacheNodesOfferingsError {
        let XmlParseError(message) = err;
        DescribeReservedCacheNodesOfferingsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeReservedCacheNodesOfferingsError {
    fn from(err: CredentialsError) -> DescribeReservedCacheNodesOfferingsError {
        DescribeReservedCacheNodesOfferingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReservedCacheNodesOfferingsError {
    fn from(err: HttpDispatchError) -> DescribeReservedCacheNodesOfferingsError {
        DescribeReservedCacheNodesOfferingsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReservedCacheNodesOfferingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReservedCacheNodesOfferingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReservedCacheNodesOfferingsError::InvalidParameterCombination(ref cause) => {
                cause
            }
            DescribeReservedCacheNodesOfferingsError::InvalidParameterValue(ref cause) => cause,
            DescribeReservedCacheNodesOfferingsError::ReservedCacheNodesOfferingNotFoundFault(ref cause) => cause,
            DescribeReservedCacheNodesOfferingsError::Validation(ref cause) => cause,
            DescribeReservedCacheNodesOfferingsError::Credentials(ref err) => err.description(),
            DescribeReservedCacheNodesOfferingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReservedCacheNodesOfferingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSnapshots
#[derive(Debug, PartialEq)]
pub enum DescribeSnapshotsError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeSnapshotsError {
    pub fn from_body(body: &str) -> DescribeSnapshotsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => DescribeSnapshotsError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => DescribeSnapshotsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => DescribeSnapshotsError::InvalidParameterValue(String::from(parsed_error.message)),
                    "SnapshotNotFoundFault" => DescribeSnapshotsError::SnapshotNotFoundFault(String::from(parsed_error.message)),
                    _ => DescribeSnapshotsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSnapshotsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for DescribeSnapshotsError {
    fn from(err: XmlParseError) -> DescribeSnapshotsError {
        let XmlParseError(message) = err;
        DescribeSnapshotsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for DescribeSnapshotsError {
    fn from(err: CredentialsError) -> DescribeSnapshotsError {
        DescribeSnapshotsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSnapshotsError {
    fn from(err: HttpDispatchError) -> DescribeSnapshotsError {
        DescribeSnapshotsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeSnapshotsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSnapshotsError {
    fn description(&self) -> &str {
        match *self {
            DescribeSnapshotsError::CacheClusterNotFoundFault(ref cause) => cause,
            DescribeSnapshotsError::InvalidParameterCombination(ref cause) => cause,
            DescribeSnapshotsError::InvalidParameterValue(ref cause) => cause,
            DescribeSnapshotsError::SnapshotNotFoundFault(ref cause) => cause,
            DescribeSnapshotsError::Validation(ref cause) => cause,
            DescribeSnapshotsError::Credentials(ref err) => err.description(),
            DescribeSnapshotsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeSnapshotsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAllowedNodeTypeModifications
#[derive(Debug, PartialEq)]
pub enum ListAllowedNodeTypeModificationsError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListAllowedNodeTypeModificationsError {
    pub fn from_body(body: &str) -> ListAllowedNodeTypeModificationsError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => ListAllowedNodeTypeModificationsError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => ListAllowedNodeTypeModificationsError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => ListAllowedNodeTypeModificationsError::InvalidParameterValue(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => ListAllowedNodeTypeModificationsError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    _ => ListAllowedNodeTypeModificationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAllowedNodeTypeModificationsError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListAllowedNodeTypeModificationsError {
    fn from(err: XmlParseError) -> ListAllowedNodeTypeModificationsError {
        let XmlParseError(message) = err;
        ListAllowedNodeTypeModificationsError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListAllowedNodeTypeModificationsError {
    fn from(err: CredentialsError) -> ListAllowedNodeTypeModificationsError {
        ListAllowedNodeTypeModificationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAllowedNodeTypeModificationsError {
    fn from(err: HttpDispatchError) -> ListAllowedNodeTypeModificationsError {
        ListAllowedNodeTypeModificationsError::HttpDispatch(err)
    }
}
impl fmt::Display for ListAllowedNodeTypeModificationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAllowedNodeTypeModificationsError {
    fn description(&self) -> &str {
        match *self {
            ListAllowedNodeTypeModificationsError::CacheClusterNotFoundFault(ref cause) => cause,
            ListAllowedNodeTypeModificationsError::InvalidParameterCombination(ref cause) => cause,
            ListAllowedNodeTypeModificationsError::InvalidParameterValue(ref cause) => cause,
            ListAllowedNodeTypeModificationsError::ReplicationGroupNotFoundFault(ref cause) => {
                cause
            }
            ListAllowedNodeTypeModificationsError::Validation(ref cause) => cause,
            ListAllowedNodeTypeModificationsError::Credentials(ref err) => err.description(),
            ListAllowedNodeTypeModificationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListAllowedNodeTypeModificationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => ListTagsForResourceError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidARNFault" => ListTagsForResourceError::InvalidARNFault(String::from(parsed_error.message)),
                    "SnapshotNotFoundFault" => ListTagsForResourceError::SnapshotNotFoundFault(String::from(parsed_error.message)),
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ListTagsForResourceError {
    fn from(err: XmlParseError) -> ListTagsForResourceError {
        let XmlParseError(message) = err;
        ListTagsForResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
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
            ListTagsForResourceError::CacheClusterNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::InvalidARNFault(ref cause) => cause,
            ListTagsForResourceError::SnapshotNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyCacheCluster
#[derive(Debug, PartialEq)]
pub enum ModifyCacheClusterError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The requested cache node type is not available in the specified Availability Zone.</p>
    InsufficientCacheClusterCapacityFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    ///<p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cache cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyCacheClusterError {
    pub fn from_body(body: &str) -> ModifyCacheClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => ModifyCacheClusterError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "CacheParameterGroupNotFoundFault" => ModifyCacheClusterError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => ModifyCacheClusterError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InsufficientCacheClusterCapacityFault" => ModifyCacheClusterError::InsufficientCacheClusterCapacityFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => ModifyCacheClusterError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    "InvalidCacheSecurityGroupStateFault" => ModifyCacheClusterError::InvalidCacheSecurityGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => ModifyCacheClusterError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => ModifyCacheClusterError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidVPCNetworkStateFault" => ModifyCacheClusterError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),
                    "NodeQuotaForClusterExceededFault" => ModifyCacheClusterError::NodeQuotaForClusterExceededFault(String::from(parsed_error.message)),
                    "NodeQuotaForCustomerExceededFault" => ModifyCacheClusterError::NodeQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    _ => ModifyCacheClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyCacheClusterError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ModifyCacheClusterError {
    fn from(err: XmlParseError) -> ModifyCacheClusterError {
        let XmlParseError(message) = err;
        ModifyCacheClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyCacheClusterError {
    fn from(err: CredentialsError) -> ModifyCacheClusterError {
        ModifyCacheClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyCacheClusterError {
    fn from(err: HttpDispatchError) -> ModifyCacheClusterError {
        ModifyCacheClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyCacheClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyCacheClusterError {
    fn description(&self) -> &str {
        match *self {
            ModifyCacheClusterError::CacheClusterNotFoundFault(ref cause) => cause,
            ModifyCacheClusterError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            ModifyCacheClusterError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            ModifyCacheClusterError::InsufficientCacheClusterCapacityFault(ref cause) => cause,
            ModifyCacheClusterError::InvalidCacheClusterStateFault(ref cause) => cause,
            ModifyCacheClusterError::InvalidCacheSecurityGroupStateFault(ref cause) => cause,
            ModifyCacheClusterError::InvalidParameterCombination(ref cause) => cause,
            ModifyCacheClusterError::InvalidParameterValue(ref cause) => cause,
            ModifyCacheClusterError::InvalidVPCNetworkStateFault(ref cause) => cause,
            ModifyCacheClusterError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            ModifyCacheClusterError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            ModifyCacheClusterError::Validation(ref cause) => cause,
            ModifyCacheClusterError::Credentials(ref err) => err.description(),
            ModifyCacheClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyCacheClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum ModifyCacheParameterGroupError {
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyCacheParameterGroupError {
    pub fn from_body(body: &str) -> ModifyCacheParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFoundFault" => ModifyCacheParameterGroupError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheParameterGroupStateFault" => ModifyCacheParameterGroupError::InvalidCacheParameterGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => ModifyCacheParameterGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => ModifyCacheParameterGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => ModifyCacheParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyCacheParameterGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ModifyCacheParameterGroupError {
    fn from(err: XmlParseError) -> ModifyCacheParameterGroupError {
        let XmlParseError(message) = err;
        ModifyCacheParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyCacheParameterGroupError {
    fn from(err: CredentialsError) -> ModifyCacheParameterGroupError {
        ModifyCacheParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyCacheParameterGroupError {
    fn from(err: HttpDispatchError) -> ModifyCacheParameterGroupError {
        ModifyCacheParameterGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyCacheParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyCacheParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            ModifyCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => {
                cause
            }
            ModifyCacheParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            ModifyCacheParameterGroupError::InvalidParameterValue(ref cause) => cause,
            ModifyCacheParameterGroupError::Validation(ref cause) => cause,
            ModifyCacheParameterGroupError::Credentials(ref err) => err.description(),
            ModifyCacheParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyCacheParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyCacheSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyCacheSubnetGroupError {
    ///<p>The requested cache subnet group name does not refer to an existing cache subnet group.</p>
    CacheSubnetGroupNotFoundFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of subnets in a cache subnet group.</p>
    CacheSubnetQuotaExceededFault(String),
    ///<p>An invalid subnet identifier was specified.</p>
    InvalidSubnet(String),
    ///<p>The requested subnet is being used by another cache subnet group.</p>
    SubnetInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyCacheSubnetGroupError {
    pub fn from_body(body: &str) -> ModifyCacheSubnetGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheSubnetGroupNotFoundFault" => ModifyCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSubnetQuotaExceededFault" => ModifyCacheSubnetGroupError::CacheSubnetQuotaExceededFault(String::from(parsed_error.message)),
                    "InvalidSubnet" => ModifyCacheSubnetGroupError::InvalidSubnet(String::from(parsed_error.message)),
                    "SubnetInUse" => {
                        ModifyCacheSubnetGroupError::SubnetInUse(String::from(parsed_error.message))
                    }
                    _ => ModifyCacheSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyCacheSubnetGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ModifyCacheSubnetGroupError {
    fn from(err: XmlParseError) -> ModifyCacheSubnetGroupError {
        let XmlParseError(message) = err;
        ModifyCacheSubnetGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyCacheSubnetGroupError {
    fn from(err: CredentialsError) -> ModifyCacheSubnetGroupError {
        ModifyCacheSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyCacheSubnetGroupError {
    fn from(err: HttpDispatchError) -> ModifyCacheSubnetGroupError {
        ModifyCacheSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyCacheSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyCacheSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyCacheSubnetGroupError::CacheSubnetGroupNotFoundFault(ref cause) => cause,
            ModifyCacheSubnetGroupError::CacheSubnetQuotaExceededFault(ref cause) => cause,
            ModifyCacheSubnetGroupError::InvalidSubnet(ref cause) => cause,
            ModifyCacheSubnetGroupError::SubnetInUse(ref cause) => cause,
            ModifyCacheSubnetGroupError::Validation(ref cause) => cause,
            ModifyCacheSubnetGroupError::Credentials(ref err) => err.description(),
            ModifyCacheSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyCacheSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationGroup
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationGroupError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The requested cache node type is not available in the specified Availability Zone.</p>
    InsufficientCacheClusterCapacityFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    ///<p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>The requested replication group is not in the <code>available</code> state.</p>
    InvalidReplicationGroupStateFault(String),
    ///<p>The VPC network is in an invalid state.</p>
    InvalidVPCNetworkStateFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes in a single cache cluster.</p>
    NodeQuotaForClusterExceededFault(String),
    ///<p>The request cannot be processed because it would exceed the allowed number of cache nodes per customer.</p>
    NodeQuotaForCustomerExceededFault(String),
    ///<p>The specified replication group does not exist.</p>
    ReplicationGroupNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyReplicationGroupError {
    pub fn from_body(body: &str) -> ModifyReplicationGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => ModifyReplicationGroupError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "CacheParameterGroupNotFoundFault" => ModifyReplicationGroupError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => ModifyReplicationGroupError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InsufficientCacheClusterCapacityFault" => ModifyReplicationGroupError::InsufficientCacheClusterCapacityFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => ModifyReplicationGroupError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    "InvalidCacheSecurityGroupStateFault" => ModifyReplicationGroupError::InvalidCacheSecurityGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => ModifyReplicationGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => ModifyReplicationGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    "InvalidReplicationGroupStateFault" => ModifyReplicationGroupError::InvalidReplicationGroupStateFault(String::from(parsed_error.message)),
                    "InvalidVPCNetworkStateFault" => ModifyReplicationGroupError::InvalidVPCNetworkStateFault(String::from(parsed_error.message)),
                    "NodeQuotaForClusterExceededFault" => ModifyReplicationGroupError::NodeQuotaForClusterExceededFault(String::from(parsed_error.message)),
                    "NodeQuotaForCustomerExceededFault" => ModifyReplicationGroupError::NodeQuotaForCustomerExceededFault(String::from(parsed_error.message)),
                    "ReplicationGroupNotFoundFault" => ModifyReplicationGroupError::ReplicationGroupNotFoundFault(String::from(parsed_error.message)),
                    _ => ModifyReplicationGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyReplicationGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ModifyReplicationGroupError {
    fn from(err: XmlParseError) -> ModifyReplicationGroupError {
        let XmlParseError(message) = err;
        ModifyReplicationGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ModifyReplicationGroupError {
    fn from(err: CredentialsError) -> ModifyReplicationGroupError {
        ModifyReplicationGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyReplicationGroupError {
    fn from(err: HttpDispatchError) -> ModifyReplicationGroupError {
        ModifyReplicationGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyReplicationGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationGroupError::CacheClusterNotFoundFault(ref cause) => cause,
            ModifyReplicationGroupError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            ModifyReplicationGroupError::CacheSecurityGroupNotFoundFault(ref cause) => cause,
            ModifyReplicationGroupError::InsufficientCacheClusterCapacityFault(ref cause) => cause,
            ModifyReplicationGroupError::InvalidCacheClusterStateFault(ref cause) => cause,
            ModifyReplicationGroupError::InvalidCacheSecurityGroupStateFault(ref cause) => cause,
            ModifyReplicationGroupError::InvalidParameterCombination(ref cause) => cause,
            ModifyReplicationGroupError::InvalidParameterValue(ref cause) => cause,
            ModifyReplicationGroupError::InvalidReplicationGroupStateFault(ref cause) => cause,
            ModifyReplicationGroupError::InvalidVPCNetworkStateFault(ref cause) => cause,
            ModifyReplicationGroupError::NodeQuotaForClusterExceededFault(ref cause) => cause,
            ModifyReplicationGroupError::NodeQuotaForCustomerExceededFault(ref cause) => cause,
            ModifyReplicationGroupError::ReplicationGroupNotFoundFault(ref cause) => cause,
            ModifyReplicationGroupError::Validation(ref cause) => cause,
            ModifyReplicationGroupError::Credentials(ref err) => err.description(),
            ModifyReplicationGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyReplicationGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PurchaseReservedCacheNodesOffering
#[derive(Debug, PartialEq)]
pub enum PurchaseReservedCacheNodesOfferingError {
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    ///<p>You already have a reservation with the given identifier.</p>
    ReservedCacheNodeAlreadyExistsFault(String),
    ///<p>The request cannot be processed because it would exceed the user's cache node quota.</p>
    ReservedCacheNodeQuotaExceededFault(String),
    ///<p>The requested cache node offering does not exist.</p>
    ReservedCacheNodesOfferingNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PurchaseReservedCacheNodesOfferingError {
    pub fn from_body(body: &str) -> PurchaseReservedCacheNodesOfferingError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "InvalidParameterCombinationException" => PurchaseReservedCacheNodesOfferingError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => PurchaseReservedCacheNodesOfferingError::InvalidParameterValue(String::from(parsed_error.message)),
                    "ReservedCacheNodeAlreadyExistsFault" => PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeAlreadyExistsFault(String::from(parsed_error.message)),
                    "ReservedCacheNodeQuotaExceededFault" => PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeQuotaExceededFault(String::from(parsed_error.message)),
                    "ReservedCacheNodesOfferingNotFoundFault" => PurchaseReservedCacheNodesOfferingError::ReservedCacheNodesOfferingNotFoundFault(String::from(parsed_error.message)),
                    _ => PurchaseReservedCacheNodesOfferingError::Unknown(String::from(body)),
                }
            }
            Err(_) => PurchaseReservedCacheNodesOfferingError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for PurchaseReservedCacheNodesOfferingError {
    fn from(err: XmlParseError) -> PurchaseReservedCacheNodesOfferingError {
        let XmlParseError(message) = err;
        PurchaseReservedCacheNodesOfferingError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for PurchaseReservedCacheNodesOfferingError {
    fn from(err: CredentialsError) -> PurchaseReservedCacheNodesOfferingError {
        PurchaseReservedCacheNodesOfferingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PurchaseReservedCacheNodesOfferingError {
    fn from(err: HttpDispatchError) -> PurchaseReservedCacheNodesOfferingError {
        PurchaseReservedCacheNodesOfferingError::HttpDispatch(err)
    }
}
impl fmt::Display for PurchaseReservedCacheNodesOfferingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PurchaseReservedCacheNodesOfferingError {
    fn description(&self) -> &str {
        match *self {
            PurchaseReservedCacheNodesOfferingError::InvalidParameterCombination(ref cause) => {
                cause
            }
            PurchaseReservedCacheNodesOfferingError::InvalidParameterValue(ref cause) => cause,
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeAlreadyExistsFault(ref cause) => cause,
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodeQuotaExceededFault(ref cause) => cause,
            PurchaseReservedCacheNodesOfferingError::ReservedCacheNodesOfferingNotFoundFault(ref cause) => cause,
            PurchaseReservedCacheNodesOfferingError::Validation(ref cause) => cause,
            PurchaseReservedCacheNodesOfferingError::Credentials(ref err) => err.description(),
            PurchaseReservedCacheNodesOfferingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PurchaseReservedCacheNodesOfferingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RebootCacheCluster
#[derive(Debug, PartialEq)]
pub enum RebootCacheClusterError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested cache cluster is not in the <code>available</code> state.</p>
    InvalidCacheClusterStateFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RebootCacheClusterError {
    pub fn from_body(body: &str) -> RebootCacheClusterError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => RebootCacheClusterError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheClusterStateFault" => RebootCacheClusterError::InvalidCacheClusterStateFault(String::from(parsed_error.message)),
                    _ => RebootCacheClusterError::Unknown(String::from(body)),
                }
            }
            Err(_) => RebootCacheClusterError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for RebootCacheClusterError {
    fn from(err: XmlParseError) -> RebootCacheClusterError {
        let XmlParseError(message) = err;
        RebootCacheClusterError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RebootCacheClusterError {
    fn from(err: CredentialsError) -> RebootCacheClusterError {
        RebootCacheClusterError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RebootCacheClusterError {
    fn from(err: HttpDispatchError) -> RebootCacheClusterError {
        RebootCacheClusterError::HttpDispatch(err)
    }
}
impl fmt::Display for RebootCacheClusterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RebootCacheClusterError {
    fn description(&self) -> &str {
        match *self {
            RebootCacheClusterError::CacheClusterNotFoundFault(ref cause) => cause,
            RebootCacheClusterError::InvalidCacheClusterStateFault(ref cause) => cause,
            RebootCacheClusterError::Validation(ref cause) => cause,
            RebootCacheClusterError::Credentials(ref err) => err.description(),
            RebootCacheClusterError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RebootCacheClusterError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    ///<p>The requested cache cluster ID does not refer to an existing cache cluster.</p>
    CacheClusterNotFoundFault(String),
    ///<p>The requested Amazon Resource Name (ARN) does not refer to an existing resource.</p>
    InvalidARNFault(String),
    ///<p>The requested snapshot name does not refer to an existing snapshot.</p>
    SnapshotNotFoundFault(String),
    ///<p>The requested tag was not found on this resource.</p>
    TagNotFoundFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RemoveTagsFromResourceError {
    pub fn from_body(body: &str) -> RemoveTagsFromResourceError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheClusterNotFoundFault" => RemoveTagsFromResourceError::CacheClusterNotFoundFault(String::from(parsed_error.message)),
                    "InvalidARNFault" => RemoveTagsFromResourceError::InvalidARNFault(String::from(parsed_error.message)),
                    "SnapshotNotFoundFault" => RemoveTagsFromResourceError::SnapshotNotFoundFault(String::from(parsed_error.message)),
                    "TagNotFoundFault" => RemoveTagsFromResourceError::TagNotFoundFault(String::from(parsed_error.message)),
                    _ => RemoveTagsFromResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromResourceError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for RemoveTagsFromResourceError {
    fn from(err: XmlParseError) -> RemoveTagsFromResourceError {
        let XmlParseError(message) = err;
        RemoveTagsFromResourceError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RemoveTagsFromResourceError {
    fn from(err: CredentialsError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveTagsFromResourceError {
    fn from(err: HttpDispatchError) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::HttpDispatch(err)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveTagsFromResourceError {
    fn description(&self) -> &str {
        match *self {
            RemoveTagsFromResourceError::CacheClusterNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::InvalidARNFault(ref cause) => cause,
            RemoveTagsFromResourceError::SnapshotNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::TagNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ResetCacheParameterGroup
#[derive(Debug, PartialEq)]
pub enum ResetCacheParameterGroupError {
    ///<p>The requested cache parameter group name does not refer to an existing cache parameter group.</p>
    CacheParameterGroupNotFoundFault(String),
    ///<p>The current state of the cache parameter group does not allow the requested operation to occur.</p>
    InvalidCacheParameterGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ResetCacheParameterGroupError {
    pub fn from_body(body: &str) -> ResetCacheParameterGroupError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "CacheParameterGroupNotFoundFault" => ResetCacheParameterGroupError::CacheParameterGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheParameterGroupStateFault" => ResetCacheParameterGroupError::InvalidCacheParameterGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => ResetCacheParameterGroupError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => ResetCacheParameterGroupError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => ResetCacheParameterGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ResetCacheParameterGroupError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for ResetCacheParameterGroupError {
    fn from(err: XmlParseError) -> ResetCacheParameterGroupError {
        let XmlParseError(message) = err;
        ResetCacheParameterGroupError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for ResetCacheParameterGroupError {
    fn from(err: CredentialsError) -> ResetCacheParameterGroupError {
        ResetCacheParameterGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ResetCacheParameterGroupError {
    fn from(err: HttpDispatchError) -> ResetCacheParameterGroupError {
        ResetCacheParameterGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ResetCacheParameterGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ResetCacheParameterGroupError {
    fn description(&self) -> &str {
        match *self {
            ResetCacheParameterGroupError::CacheParameterGroupNotFoundFault(ref cause) => cause,
            ResetCacheParameterGroupError::InvalidCacheParameterGroupStateFault(ref cause) => cause,
            ResetCacheParameterGroupError::InvalidParameterCombination(ref cause) => cause,
            ResetCacheParameterGroupError::InvalidParameterValue(ref cause) => cause,
            ResetCacheParameterGroupError::Validation(ref cause) => cause,
            ResetCacheParameterGroupError::Credentials(ref err) => err.description(),
            ResetCacheParameterGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ResetCacheParameterGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RevokeCacheSecurityGroupIngress
#[derive(Debug, PartialEq)]
pub enum RevokeCacheSecurityGroupIngressError {
    ///<p>The specified Amazon EC2 security group is not authorized for the specified cache security group.</p>
    AuthorizationNotFoundFault(String),
    ///<p>The requested cache security group name does not refer to an existing cache security group.</p>
    CacheSecurityGroupNotFoundFault(String),
    ///<p>The current state of the cache security group does not allow deletion.</p>
    InvalidCacheSecurityGroupStateFault(String),
    ///<p>Two or more incompatible parameters were specified.</p>
    InvalidParameterCombination(String),
    ///<p>The value for a parameter is invalid.</p>
    InvalidParameterValue(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RevokeCacheSecurityGroupIngressError {
    pub fn from_body(body: &str) -> RevokeCacheSecurityGroupIngressError {
        let reader = EventReader::new(body.as_bytes());
        let mut stack = XmlResponse::new(reader.into_iter().peekable());
        let _start_document = stack.next();
        let _response_envelope = stack.next();
        match XmlErrorDeserializer::deserialize("Error", &mut stack) {
            Ok(parsed_error) => {
                match &parsed_error.code[..] {
                    "AuthorizationNotFoundFault" => RevokeCacheSecurityGroupIngressError::AuthorizationNotFoundFault(String::from(parsed_error.message)),
                    "CacheSecurityGroupNotFoundFault" => RevokeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(String::from(parsed_error.message)),
                    "InvalidCacheSecurityGroupStateFault" => RevokeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(String::from(parsed_error.message)),
                    "InvalidParameterCombinationException" => RevokeCacheSecurityGroupIngressError::InvalidParameterCombination(String::from(parsed_error.message)),
                    "InvalidParameterValueException" => RevokeCacheSecurityGroupIngressError::InvalidParameterValue(String::from(parsed_error.message)),
                    _ => RevokeCacheSecurityGroupIngressError::Unknown(String::from(body)),
                }
            }
            Err(_) => RevokeCacheSecurityGroupIngressError::Unknown(body.to_string()),
        }
    }
}

impl From<XmlParseError> for RevokeCacheSecurityGroupIngressError {
    fn from(err: XmlParseError) -> RevokeCacheSecurityGroupIngressError {
        let XmlParseError(message) = err;
        RevokeCacheSecurityGroupIngressError::Unknown(message.to_string())
    }
}
impl From<CredentialsError> for RevokeCacheSecurityGroupIngressError {
    fn from(err: CredentialsError) -> RevokeCacheSecurityGroupIngressError {
        RevokeCacheSecurityGroupIngressError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RevokeCacheSecurityGroupIngressError {
    fn from(err: HttpDispatchError) -> RevokeCacheSecurityGroupIngressError {
        RevokeCacheSecurityGroupIngressError::HttpDispatch(err)
    }
}
impl fmt::Display for RevokeCacheSecurityGroupIngressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RevokeCacheSecurityGroupIngressError {
    fn description(&self) -> &str {
        match *self {
            RevokeCacheSecurityGroupIngressError::AuthorizationNotFoundFault(ref cause) => cause,
            RevokeCacheSecurityGroupIngressError::CacheSecurityGroupNotFoundFault(ref cause) => {
                cause
            }
            RevokeCacheSecurityGroupIngressError::InvalidCacheSecurityGroupStateFault(ref cause) => cause,
            RevokeCacheSecurityGroupIngressError::InvalidParameterCombination(ref cause) => cause,
            RevokeCacheSecurityGroupIngressError::InvalidParameterValue(ref cause) => cause,
            RevokeCacheSecurityGroupIngressError::Validation(ref cause) => cause,
            RevokeCacheSecurityGroupIngressError::Credentials(ref err) => err.description(),
            RevokeCacheSecurityGroupIngressError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RevokeCacheSecurityGroupIngressError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon ElastiCache API. Amazon ElastiCache clients implement this trait.
pub trait ElastiCache {
    #[doc="<p>Adds up to 10 cost allocation tags to the named resource. A cost allocation tag is a key-value pair where the key and value are case-sensitive. You can use cost allocation tags to categorize and track your AWS costs.</p> <p> When you apply tags to your ElastiCache resources, AWS generates a cost allocation report as a comma-separated value (CSV) file with your usage and costs aggregated by your tags. You can apply tags that represent business categories (such as cost centers, application names, or owners) to organize your costs across multiple services. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Tagging.html\">Using Cost Allocation Tags in Amazon ElastiCache</a> in the <i>ElastiCache User Guide</i>.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceMessage)
                            -> Result<TagListMessage, AddTagsToResourceError>;


    #[doc="<p>Allows network ingress to a cache security group. Applications using ElastiCache must be running on Amazon EC2, and Amazon EC2 security groups are used as the authorization mechanism.</p> <note> <p>You cannot authorize ingress from an Amazon EC2 security group in one region to an ElastiCache cluster in another region.</p> </note>"]
    fn authorize_cache_security_group_ingress
        (&self,
         input: &AuthorizeCacheSecurityGroupIngressMessage)
         -> Result<AuthorizeCacheSecurityGroupIngressResult,
                   AuthorizeCacheSecurityGroupIngressError>;


    #[doc="<p>Makes a copy of an existing snapshot.</p> <note> <p>This operation is valid for Redis only.</p> </note> <important> <p>Users or groups that have permissions to use the <code>CopySnapshot</code> operation can create their own Amazon S3 buckets and copy snapshots to it. To control access to your snapshots, use an IAM policy to control who has the ability to use the <code>CopySnapshot</code> operation. For more information about using IAM to control the use of ElastiCache operations, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html\">Exporting Snapshots</a> and <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/IAM.html\">Authentication &amp; Access Control</a>.</p> </important> <p>You could receive the following error messages.</p> <p class=\"title\"> <b>Error Messages</b> </p> <ul> <li> <p> <b>Error Message:</b> The S3 bucket %s is outside of the region.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s does not exist.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s is not owned by the authenticated user.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The authenticated user does not have sufficient permissions to perform the desired activity.</p> <p> <b>Solution:</b> Contact your system administrator to get the needed permissions.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s already contains an object with key %s.</p> <p> <b>Solution:</b> Give the <code>TargetSnapshotName</code> a new and unique value. If exporting a snapshot, you could alternatively create a new Amazon S3 bucket and use this same value for <code>TargetSnapshotName</code>.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add List and Read permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted WRITE permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add Upload/Delete permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ_ACP permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add View Permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> </ul>"]
    fn copy_snapshot(&self,
                     input: &CopySnapshotMessage)
                     -> Result<CopySnapshotResult, CopySnapshotError>;


    #[doc="<p>Creates a cache cluster. All nodes in the cache cluster run the same protocol-compliant cache engine software, either Memcached or Redis.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important>"]
    fn create_cache_cluster(&self,
                            input: &CreateCacheClusterMessage)
                            -> Result<CreateCacheClusterResult, CreateCacheClusterError>;


    #[doc="<p>Creates a new cache parameter group. A cache parameter group is a collection of parameters that you apply to all of the nodes in a cache cluster.</p>"]
    fn create_cache_parameter_group
        (&self,
         input: &CreateCacheParameterGroupMessage)
         -> Result<CreateCacheParameterGroupResult, CreateCacheParameterGroupError>;


    #[doc="<p>Creates a new cache security group. Use a cache security group to control access to one or more cache clusters.</p> <p>Cache security groups are only used when you are creating a cache cluster outside of an Amazon Virtual Private Cloud (Amazon VPC). If you are creating a cache cluster inside of a VPC, use a cache subnet group instead. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateCacheSubnetGroup.html\">CreateCacheSubnetGroup</a>.</p>"]
    fn create_cache_security_group
        (&self,
         input: &CreateCacheSecurityGroupMessage)
         -> Result<CreateCacheSecurityGroupResult, CreateCacheSecurityGroupError>;


    #[doc="<p>Creates a new cache subnet group.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    fn create_cache_subnet_group
        (&self,
         input: &CreateCacheSubnetGroupMessage)
         -> Result<CreateCacheSubnetGroupResult, CreateCacheSubnetGroupError>;


    #[doc="<p>Creates a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group.</p> <p>A Redis (cluster mode disabled) replication group is a collection of cache clusters, where one of the cache clusters is a read/write primary and the others are read-only replicas. Writes to the primary are asynchronously propagated to the replicas.</p> <p>A Redis (cluster mode enabled) replication group is a collection of 1 to 15 node groups (shards). Each node group (shard) has one read/write primary node and up to 5 read-only replica nodes. Writes to the primary are asynchronously propagated to the replicas. Redis (cluster mode enabled) replication groups partition the data across node groups (shards).</p> <p>When a Redis (cluster mode disabled) replication group has been successfully created, you can add one or more read replicas to it, up to a total of 5 read replicas. You cannot alter a Redis (cluster mode enabled) replication group after it has been created.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn create_replication_group
        (&self,
         input: &CreateReplicationGroupMessage)
         -> Result<CreateReplicationGroupResult, CreateReplicationGroupError>;


    #[doc="<p>Creates a copy of an entire cache cluster or replication group at a specific moment in time.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn create_snapshot(&self,
                       input: &CreateSnapshotMessage)
                       -> Result<CreateSnapshotResult, CreateSnapshotError>;


    #[doc="<p>Deletes a previously provisioned cache cluster. <code>DeleteCacheCluster</code> deletes all associated cache nodes, node endpoints and the cache cluster itself. When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the cache cluster; you cannot cancel or revert this operation.</p> <p>This operation cannot be used to delete a cache cluster that is the last read replica of a replication group or node group (shard) that has Multi-AZ mode enabled or a cache cluster from a Redis (cluster mode enabled) replication group.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important>"]
    fn delete_cache_cluster(&self,
                            input: &DeleteCacheClusterMessage)
                            -> Result<DeleteCacheClusterResult, DeleteCacheClusterError>;


    #[doc="<p>Deletes the specified cache parameter group. You cannot delete a cache parameter group if it is associated with any cache clusters.</p>"]
    fn delete_cache_parameter_group(&self,
                                    input: &DeleteCacheParameterGroupMessage)
                                    -> Result<(), DeleteCacheParameterGroupError>;


    #[doc="<p>Deletes a cache security group.</p> <note> <p>You cannot delete a cache security group if it is associated with any cache clusters.</p> </note>"]
    fn delete_cache_security_group(&self,
                                   input: &DeleteCacheSecurityGroupMessage)
                                   -> Result<(), DeleteCacheSecurityGroupError>;


    #[doc="<p>Deletes a cache subnet group.</p> <note> <p>You cannot delete a cache subnet group if it is associated with any cache clusters.</p> </note>"]
    fn delete_cache_subnet_group(&self,
                                 input: &DeleteCacheSubnetGroupMessage)
                                 -> Result<(), DeleteCacheSubnetGroupError>;


    #[doc="<p>Deletes an existing replication group. By default, this operation deletes the entire replication group, including the primary/primaries and all of the read replicas. If the replication group has only one primary, you can optionally delete only the read replicas, while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn delete_replication_group
        (&self,
         input: &DeleteReplicationGroupMessage)
         -> Result<DeleteReplicationGroupResult, DeleteReplicationGroupError>;


    #[doc="<p>Deletes an existing snapshot. When you receive a successful response from this operation, ElastiCache immediately begins deleting the snapshot; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn delete_snapshot(&self,
                       input: &DeleteSnapshotMessage)
                       -> Result<DeleteSnapshotResult, DeleteSnapshotError>;


    #[doc="<p>Returns information about all provisioned cache clusters if no cache cluster identifier is specified, or about a specific cache cluster if a cache cluster identifier is supplied.</p> <p>By default, abbreviated information about the cache clusters are returned. You can use the optional <code>ShowDetails</code> flag to retrieve detailed information about the cache nodes associated with the cache clusters. These details include the DNS address and port for the cache node endpoint.</p> <p>If the cluster is in the CREATING state, only cluster-level information is displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster-level information is displayed.</p> <p>If cache nodes are currently being added to the cache cluster, node endpoint information and creation time for the additional nodes are not displayed until they are completely provisioned. When the cache cluster state is <code>available</code>, the cluster is ready for use.</p> <p>If cache nodes are currently being removed from the cache cluster, no endpoint information for the removed nodes is displayed.</p>"]
    fn describe_cache_clusters(&self,
                               input: &DescribeCacheClustersMessage)
                               -> Result<CacheClusterMessage, DescribeCacheClustersError>;


    #[doc="<p>Returns a list of the available cache engines and their versions.</p>"]
    fn describe_cache_engine_versions
        (&self,
         input: &DescribeCacheEngineVersionsMessage)
         -> Result<CacheEngineVersionMessage, DescribeCacheEngineVersionsError>;


    #[doc="<p>Returns a list of cache parameter group descriptions. If a cache parameter group name is specified, the list contains only the descriptions for that group.</p>"]
    fn describe_cache_parameter_groups
        (&self,
         input: &DescribeCacheParameterGroupsMessage)
         -> Result<CacheParameterGroupsMessage, DescribeCacheParameterGroupsError>;


    #[doc="<p>Returns the detailed parameter list for a particular cache parameter group.</p>"]
    fn describe_cache_parameters
        (&self,
         input: &DescribeCacheParametersMessage)
         -> Result<CacheParameterGroupDetails, DescribeCacheParametersError>;


    #[doc="<p>Returns a list of cache security group descriptions. If a cache security group name is specified, the list contains only the description of that group.</p>"]
    fn describe_cache_security_groups
        (&self,
         input: &DescribeCacheSecurityGroupsMessage)
         -> Result<CacheSecurityGroupMessage, DescribeCacheSecurityGroupsError>;


    #[doc="<p>Returns a list of cache subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group.</p>"]
    fn describe_cache_subnet_groups
        (&self,
         input: &DescribeCacheSubnetGroupsMessage)
         -> Result<CacheSubnetGroupMessage, DescribeCacheSubnetGroupsError>;


    #[doc="<p>Returns the default engine and system parameter information for the specified cache engine.</p>"]
    fn describe_engine_default_parameters
        (&self,
         input: &DescribeEngineDefaultParametersMessage)
         -> Result<DescribeEngineDefaultParametersResult, DescribeEngineDefaultParametersError>;


    #[doc="<p>Returns events related to cache clusters, cache security groups, and cache parameter groups. You can obtain events specific to a particular cache cluster, cache security group, or cache parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>"]
    fn describe_events(&self,
                       input: &DescribeEventsMessage)
                       -> Result<EventsMessage, DescribeEventsError>;


    #[doc="<p>Returns information about a particular replication group. If no identifier is specified, <code>DescribeReplicationGroups</code> returns information about all replication groups.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn describe_replication_groups
        (&self,
         input: &DescribeReplicationGroupsMessage)
         -> Result<ReplicationGroupMessage, DescribeReplicationGroupsError>;


    #[doc="<p>Returns information about reserved cache nodes for this account, or about a specified reserved cache node.</p>"]
    fn describe_reserved_cache_nodes
        (&self,
         input: &DescribeReservedCacheNodesMessage)
         -> Result<ReservedCacheNodeMessage, DescribeReservedCacheNodesError>;


    #[doc="<p>Lists available reserved cache node offerings.</p>"]
    fn describe_reserved_cache_nodes_offerings
        (&self,
         input: &DescribeReservedCacheNodesOfferingsMessage)
         -> Result<ReservedCacheNodesOfferingMessage, DescribeReservedCacheNodesOfferingsError>;


    #[doc="<p>Returns information about cache cluster or replication group snapshots. By default, <code>DescribeSnapshots</code> lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a particular cache cluster.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn describe_snapshots(&self,
                          input: &DescribeSnapshotsMessage)
                          -> Result<DescribeSnapshotsListMessage, DescribeSnapshotsError>;


    #[doc="<p>Lists all available node types that you can scale your Redis cluster's or replication group's current node type up to.</p> <p>When you use the <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code> operations to scale up your cluster or replication group, the value of the <code>CacheNodeType</code> parameter must be one of the node types returned by this operation.</p>"]
    fn list_allowed_node_type_modifications
        (&self,
         input: &ListAllowedNodeTypeModificationsMessage)
         -> Result<AllowedNodeTypeModificationsMessage, ListAllowedNodeTypeModificationsError>;


    #[doc="<p>Lists all cost allocation tags currently on the named resource. A <code>cost allocation tag</code> is a key-value pair where the key is case-sensitive and the value is optional. You can use cost allocation tags to categorize and track your AWS costs.</p> <p>You can have a maximum of 10 cost allocation tags on an ElastiCache resource. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/BestPractices.html\">Using Cost Allocation Tags in Amazon ElastiCache</a>.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceMessage)
                              -> Result<TagListMessage, ListTagsForResourceError>;


    #[doc="<p>Modifies the settings for a cache cluster. You can use this operation to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>"]
    fn modify_cache_cluster(&self,
                            input: &ModifyCacheClusterMessage)
                            -> Result<ModifyCacheClusterResult, ModifyCacheClusterError>;


    #[doc="<p>Modifies the parameters of a cache parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>"]
    fn modify_cache_parameter_group
        (&self,
         input: &ModifyCacheParameterGroupMessage)
         -> Result<CacheParameterGroupNameMessage, ModifyCacheParameterGroupError>;


    #[doc="<p>Modifies an existing cache subnet group.</p>"]
    fn modify_cache_subnet_group
        (&self,
         input: &ModifyCacheSubnetGroupMessage)
         -> Result<ModifyCacheSubnetGroupResult, ModifyCacheSubnetGroupError>;


    #[doc="<p>Modifies the settings for a replication group.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn modify_replication_group
        (&self,
         input: &ModifyReplicationGroupMessage)
         -> Result<ModifyReplicationGroupResult, ModifyReplicationGroupError>;


    #[doc="<p>Allows you to purchase a reserved cache node offering.</p>"]
    fn purchase_reserved_cache_nodes_offering
        (&self,
         input: &PurchaseReservedCacheNodesOfferingMessage)
         -> Result<PurchaseReservedCacheNodesOfferingResult,
                   PurchaseReservedCacheNodesOfferingError>;


    #[doc="<p>Reboots some, or all, of the cache nodes within a provisioned cache cluster. This operation applies any modified cache parameter groups to the cache cluster. The reboot operation takes place as soon as possible, and results in a momentary outage to the cache cluster. During the reboot, the cache cluster status is set to REBOOTING.</p> <p>The reboot causes the contents of the cache (for each cache node being rebooted) to be lost.</p> <p>When the reboot is complete, a cache cluster event is created.</p>"]
    fn reboot_cache_cluster(&self,
                            input: &RebootCacheClusterMessage)
                            -> Result<RebootCacheClusterResult, RebootCacheClusterError>;


    #[doc="<p>Removes the tags identified by the <code>TagKeys</code> list from the named resource.</p>"]
    fn remove_tags_from_resource(&self,
                                 input: &RemoveTagsFromResourceMessage)
                                 -> Result<TagListMessage, RemoveTagsFromResourceError>;


    #[doc="<p>Modifies the parameters of a cache parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire cache parameter group, specify the <code>ResetAllParameters</code> and <code>CacheParameterGroupName</code> parameters.</p>"]
    fn reset_cache_parameter_group
        (&self,
         input: &ResetCacheParameterGroupMessage)
         -> Result<CacheParameterGroupNameMessage, ResetCacheParameterGroupError>;


    #[doc="<p>Revokes ingress from a cache security group. Use this operation to disallow access from an Amazon EC2 security group that had been previously authorized.</p>"]
    fn revoke_cache_security_group_ingress
        (&self,
         input: &RevokeCacheSecurityGroupIngressMessage)
         -> Result<RevokeCacheSecurityGroupIngressResult, RevokeCacheSecurityGroupIngressError>;
}
/// A client for the Amazon ElastiCache API.
pub struct ElastiCacheClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> ElastiCacheClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        ElastiCacheClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> ElastiCache for ElastiCacheClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds up to 10 cost allocation tags to the named resource. A cost allocation tag is a key-value pair where the key and value are case-sensitive. You can use cost allocation tags to categorize and track your AWS costs.</p> <p> When you apply tags to your ElastiCache resources, AWS generates a cost allocation report as a comma-separated value (CSV) file with your usage and costs aggregated by your tags. You can apply tags that represent business categories (such as cost centers, application names, or owners) to organize your costs across multiple services. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Tagging.html\">Using Cost Allocation Tags in Amazon ElastiCache</a> in the <i>ElastiCache User Guide</i>.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceMessage)
                            -> Result<TagListMessage, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AddTagsToResource");
        params.put("Version", "2015-02-02");
        AddTagsToResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = TagListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TagListMessageDeserializer::deserialize("AddTagsToResourceResult",
                                                                          &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(AddTagsToResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Allows network ingress to a cache security group. Applications using ElastiCache must be running on Amazon EC2, and Amazon EC2 security groups are used as the authorization mechanism.</p> <note> <p>You cannot authorize ingress from an Amazon EC2 security group in one region to an ElastiCache cluster in another region.</p> </note>"]
    fn authorize_cache_security_group_ingress
        (&self,
         input: &AuthorizeCacheSecurityGroupIngressMessage)
         -> Result<AuthorizeCacheSecurityGroupIngressResult,
                   AuthorizeCacheSecurityGroupIngressError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "AuthorizeCacheSecurityGroupIngress");
        params.put("Version", "2015-02-02");
        AuthorizeCacheSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = AuthorizeCacheSecurityGroupIngressResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AuthorizeCacheSecurityGroupIngressResultDeserializer::deserialize("AuthorizeCacheSecurityGroupIngressResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(AuthorizeCacheSecurityGroupIngressError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Makes a copy of an existing snapshot.</p> <note> <p>This operation is valid for Redis only.</p> </note> <important> <p>Users or groups that have permissions to use the <code>CopySnapshot</code> operation can create their own Amazon S3 buckets and copy snapshots to it. To control access to your snapshots, use an IAM policy to control who has the ability to use the <code>CopySnapshot</code> operation. For more information about using IAM to control the use of ElastiCache operations, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html\">Exporting Snapshots</a> and <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/IAM.html\">Authentication &amp; Access Control</a>.</p> </important> <p>You could receive the following error messages.</p> <p class=\"title\"> <b>Error Messages</b> </p> <ul> <li> <p> <b>Error Message:</b> The S3 bucket %s is outside of the region.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s does not exist.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s is not owned by the authenticated user.</p> <p> <b>Solution:</b> Create an Amazon S3 bucket in the same region as your snapshot. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.CreateBucket\">Step 1: Create an Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message:</b> The authenticated user does not have sufficient permissions to perform the desired activity.</p> <p> <b>Solution:</b> Contact your system administrator to get the needed permissions.</p> </li> <li> <p> <b>Error Message:</b> The S3 bucket %s already contains an object with key %s.</p> <p> <b>Solution:</b> Give the <code>TargetSnapshotName</code> a new and unique value. If exporting a snapshot, you could alternatively create a new Amazon S3 bucket and use this same value for <code>TargetSnapshotName</code>.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add List and Read permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted WRITE permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add Upload/Delete permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> <li> <p> <b>Error Message: </b> ElastiCache has not been granted READ_ACP permissions %s on the S3 Bucket.</p> <p> <b>Solution:</b> Add View Permissions on the bucket. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/Snapshots.Exporting.html#Snapshots.Exporting.GrantAccess\">Step 2: Grant ElastiCache Access to Your Amazon S3 Bucket</a> in the ElastiCache User Guide.</p> </li> </ul>"]
    fn copy_snapshot(&self,
                     input: &CopySnapshotMessage)
                     -> Result<CopySnapshotResult, CopySnapshotError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CopySnapshot");
        params.put("Version", "2015-02-02");
        CopySnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CopySnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CopySnapshotResultDeserializer::deserialize("CopySnapshotResult",
                                                                              &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(CopySnapshotError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Creates a cache cluster. All nodes in the cache cluster run the same protocol-compliant cache engine software, either Memcached or Redis.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important>"]
    fn create_cache_cluster(&self,
                            input: &CreateCacheClusterMessage)
                            -> Result<CreateCacheClusterResult, CreateCacheClusterError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCacheCluster");
        params.put("Version", "2015-02-02");
        CreateCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateCacheClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateCacheClusterResultDeserializer::deserialize("CreateCacheClusterResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(CreateCacheClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a new cache parameter group. A cache parameter group is a collection of parameters that you apply to all of the nodes in a cache cluster.</p>"]
    fn create_cache_parameter_group
        (&self,
         input: &CreateCacheParameterGroupMessage)
         -> Result<CreateCacheParameterGroupResult, CreateCacheParameterGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCacheParameterGroup");
        params.put("Version", "2015-02-02");
        CreateCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateCacheParameterGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CreateCacheParameterGroupResultDeserializer::deserialize("CreateCacheParameterGroupResult",
                                                                                      &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(CreateCacheParameterGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Creates a new cache security group. Use a cache security group to control access to one or more cache clusters.</p> <p>Cache security groups are only used when you are creating a cache cluster outside of an Amazon Virtual Private Cloud (Amazon VPC). If you are creating a cache cluster inside of a VPC, use a cache subnet group instead. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/APIReference/API_CreateCacheSubnetGroup.html\">CreateCacheSubnetGroup</a>.</p>"]
    fn create_cache_security_group
        (&self,
         input: &CreateCacheSecurityGroupMessage)
         -> Result<CreateCacheSecurityGroupResult, CreateCacheSecurityGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCacheSecurityGroup");
        params.put("Version", "2015-02-02");
        CreateCacheSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateCacheSecurityGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CreateCacheSecurityGroupResultDeserializer::deserialize("CreateCacheSecurityGroupResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(CreateCacheSecurityGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Creates a new cache subnet group.</p> <p>Use this parameter only when you are creating a cluster in an Amazon Virtual Private Cloud (Amazon VPC).</p>"]
    fn create_cache_subnet_group
        (&self,
         input: &CreateCacheSubnetGroupMessage)
         -> Result<CreateCacheSubnetGroupResult, CreateCacheSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateCacheSubnetGroup");
        params.put("Version", "2015-02-02");
        CreateCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateCacheSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CreateCacheSubnetGroupResultDeserializer::deserialize("CreateCacheSubnetGroupResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(CreateCacheSubnetGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a Redis (cluster mode disabled) or a Redis (cluster mode enabled) replication group.</p> <p>A Redis (cluster mode disabled) replication group is a collection of cache clusters, where one of the cache clusters is a read/write primary and the others are read-only replicas. Writes to the primary are asynchronously propagated to the replicas.</p> <p>A Redis (cluster mode enabled) replication group is a collection of 1 to 15 node groups (shards). Each node group (shard) has one read/write primary node and up to 5 read-only replica nodes. Writes to the primary are asynchronously propagated to the replicas. Redis (cluster mode enabled) replication groups partition the data across node groups (shards).</p> <p>When a Redis (cluster mode disabled) replication group has been successfully created, you can add one or more read replicas to it, up to a total of 5 read replicas. You cannot alter a Redis (cluster mode enabled) replication group after it has been created.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn create_replication_group
        (&self,
         input: &CreateReplicationGroupMessage)
         -> Result<CreateReplicationGroupResult, CreateReplicationGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateReplicationGroup");
        params.put("Version", "2015-02-02");
        CreateReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateReplicationGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CreateReplicationGroupResultDeserializer::deserialize("CreateReplicationGroupResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(CreateReplicationGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Creates a copy of an entire cache cluster or replication group at a specific moment in time.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn create_snapshot(&self,
                       input: &CreateSnapshotMessage)
                       -> Result<CreateSnapshotResult, CreateSnapshotError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "CreateSnapshot");
        params.put("Version", "2015-02-02");
        CreateSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CreateSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CreateSnapshotResultDeserializer::deserialize("CreateSnapshotResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(CreateSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a previously provisioned cache cluster. <code>DeleteCacheCluster</code> deletes all associated cache nodes, node endpoints and the cache cluster itself. When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the cache cluster; you cannot cancel or revert this operation.</p> <p>This operation cannot be used to delete a cache cluster that is the last read replica of a replication group or node group (shard) that has Multi-AZ mode enabled or a cache cluster from a Redis (cluster mode enabled) replication group.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important>"]
    fn delete_cache_cluster(&self,
                            input: &DeleteCacheClusterMessage)
                            -> Result<DeleteCacheClusterResult, DeleteCacheClusterError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCacheCluster");
        params.put("Version", "2015-02-02");
        DeleteCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DeleteCacheClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteCacheClusterResultDeserializer::deserialize("DeleteCacheClusterResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DeleteCacheClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified cache parameter group. You cannot delete a cache parameter group if it is associated with any cache clusters.</p>"]
    fn delete_cache_parameter_group(&self,
                                    input: &DeleteCacheParameterGroupMessage)
                                    -> Result<(), DeleteCacheParameterGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCacheParameterGroup");
        params.put("Version", "2015-02-02");
        DeleteCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                            Err(DeleteCacheParameterGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Deletes a cache security group.</p> <note> <p>You cannot delete a cache security group if it is associated with any cache clusters.</p> </note>"]
    fn delete_cache_security_group(&self,
                                   input: &DeleteCacheSecurityGroupMessage)
                                   -> Result<(), DeleteCacheSecurityGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCacheSecurityGroup");
        params.put("Version", "2015-02-02");
        DeleteCacheSecurityGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                            Err(DeleteCacheSecurityGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Deletes a cache subnet group.</p> <note> <p>You cannot delete a cache subnet group if it is associated with any cache clusters.</p> </note>"]
    fn delete_cache_subnet_group(&self,
                                 input: &DeleteCacheSubnetGroupMessage)
                                 -> Result<(), DeleteCacheSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteCacheSubnetGroup");
        params.put("Version", "2015-02-02");
        DeleteCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {
                let result = ();
                Ok(result)
            }
            _ => {
                Err(DeleteCacheSubnetGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an existing replication group. By default, this operation deletes the entire replication group, including the primary/primaries and all of the read replicas. If the replication group has only one primary, you can optionally delete only the read replicas, while retaining the primary by setting <code>RetainPrimaryCluster=true</code>.</p> <p>When you receive a successful response from this operation, Amazon ElastiCache immediately begins deleting the selected resources; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn delete_replication_group
        (&self,
         input: &DeleteReplicationGroupMessage)
         -> Result<DeleteReplicationGroupResult, DeleteReplicationGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteReplicationGroup");
        params.put("Version", "2015-02-02");
        DeleteReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DeleteReplicationGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(DeleteReplicationGroupResultDeserializer::deserialize("DeleteReplicationGroupResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DeleteReplicationGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes an existing snapshot. When you receive a successful response from this operation, ElastiCache immediately begins deleting the snapshot; you cannot cancel or revert this operation.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn delete_snapshot(&self,
                       input: &DeleteSnapshotMessage)
                       -> Result<DeleteSnapshotResult, DeleteSnapshotError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DeleteSnapshot");
        params.put("Version", "2015-02-02");
        DeleteSnapshotMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DeleteSnapshotResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DeleteSnapshotResultDeserializer::deserialize("DeleteSnapshotResult",
                                                                                &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DeleteSnapshotError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about all provisioned cache clusters if no cache cluster identifier is specified, or about a specific cache cluster if a cache cluster identifier is supplied.</p> <p>By default, abbreviated information about the cache clusters are returned. You can use the optional <code>ShowDetails</code> flag to retrieve detailed information about the cache nodes associated with the cache clusters. These details include the DNS address and port for the cache node endpoint.</p> <p>If the cluster is in the CREATING state, only cluster-level information is displayed until all of the nodes are successfully provisioned.</p> <p>If the cluster is in the DELETING state, only cluster-level information is displayed.</p> <p>If cache nodes are currently being added to the cache cluster, node endpoint information and creation time for the additional nodes are not displayed until they are completely provisioned. When the cache cluster state is <code>available</code>, the cluster is ready for use.</p> <p>If cache nodes are currently being removed from the cache cluster, no endpoint information for the removed nodes is displayed.</p>"]
    fn describe_cache_clusters(&self,
                               input: &DescribeCacheClustersMessage)
                               -> Result<CacheClusterMessage, DescribeCacheClustersError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheClusters");
        params.put("Version", "2015-02-02");
        DescribeCacheClustersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheClusterMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheClusterMessageDeserializer::deserialize("DescribeCacheClustersResult",
                                                                               &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DescribeCacheClustersError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Returns a list of the available cache engines and their versions.</p>"]
    fn describe_cache_engine_versions
        (&self,
         input: &DescribeCacheEngineVersionsMessage)
         -> Result<CacheEngineVersionMessage, DescribeCacheEngineVersionsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheEngineVersions");
        params.put("Version", "2015-02-02");
        DescribeCacheEngineVersionsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheEngineVersionMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheEngineVersionMessageDeserializer::deserialize("DescribeCacheEngineVersionsResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeCacheEngineVersionsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns a list of cache parameter group descriptions. If a cache parameter group name is specified, the list contains only the descriptions for that group.</p>"]
    fn describe_cache_parameter_groups
        (&self,
         input: &DescribeCacheParameterGroupsMessage)
         -> Result<CacheParameterGroupsMessage, DescribeCacheParameterGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheParameterGroups");
        params.put("Version", "2015-02-02");
        DescribeCacheParameterGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheParameterGroupsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheParameterGroupsMessageDeserializer::deserialize("DescribeCacheParameterGroupsResult",
                                                                                       &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeCacheParameterGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns the detailed parameter list for a particular cache parameter group.</p>"]
    fn describe_cache_parameters
        (&self,
         input: &DescribeCacheParametersMessage)
         -> Result<CacheParameterGroupDetails, DescribeCacheParametersError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheParameters");
        params.put("Version", "2015-02-02");
        DescribeCacheParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheParameterGroupDetails::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheParameterGroupDetailsDeserializer::deserialize("DescribeCacheParametersResult",
                                                                                      &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeCacheParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns a list of cache security group descriptions. If a cache security group name is specified, the list contains only the description of that group.</p>"]
    fn describe_cache_security_groups
        (&self,
         input: &DescribeCacheSecurityGroupsMessage)
         -> Result<CacheSecurityGroupMessage, DescribeCacheSecurityGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheSecurityGroups");
        params.put("Version", "2015-02-02");
        DescribeCacheSecurityGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheSecurityGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheSecurityGroupMessageDeserializer::deserialize("DescribeCacheSecurityGroupsResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeCacheSecurityGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns a list of cache subnet group descriptions. If a subnet group name is specified, the list contains only the description of that group.</p>"]
    fn describe_cache_subnet_groups
        (&self,
         input: &DescribeCacheSubnetGroupsMessage)
         -> Result<CacheSubnetGroupMessage, DescribeCacheSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeCacheSubnetGroups");
        params.put("Version", "2015-02-02");
        DescribeCacheSubnetGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheSubnetGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(CacheSubnetGroupMessageDeserializer::deserialize("DescribeCacheSubnetGroupsResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeCacheSubnetGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns the default engine and system parameter information for the specified cache engine.</p>"]
    fn describe_engine_default_parameters
        (&self,
         input: &DescribeEngineDefaultParametersMessage)
         -> Result<DescribeEngineDefaultParametersResult, DescribeEngineDefaultParametersError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEngineDefaultParameters");
        params.put("Version", "2015-02-02");
        DescribeEngineDefaultParametersMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DescribeEngineDefaultParametersResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(DescribeEngineDefaultParametersResultDeserializer::deserialize("DescribeEngineDefaultParametersResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeEngineDefaultParametersError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns events related to cache clusters, cache security groups, and cache parameter groups. You can obtain events specific to a particular cache cluster, cache security group, or cache parameter group by providing the name as a parameter.</p> <p>By default, only the events occurring within the last hour are returned; however, you can retrieve up to 14 days' worth of events if necessary.</p>"]
    fn describe_events(&self,
                       input: &DescribeEventsMessage)
                       -> Result<EventsMessage, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeEvents");
        params.put("Version", "2015-02-02");
        DescribeEventsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = EventsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(EventsMessageDeserializer::deserialize("DescribeEventsResult",
                                                                         &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DescribeEventsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about a particular replication group. If no identifier is specified, <code>DescribeReplicationGroups</code> returns information about all replication groups.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn describe_replication_groups
        (&self,
         input: &DescribeReplicationGroupsMessage)
         -> Result<ReplicationGroupMessage, DescribeReplicationGroupsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReplicationGroups");
        params.put("Version", "2015-02-02");
        DescribeReplicationGroupsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ReplicationGroupMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReplicationGroupMessageDeserializer::deserialize("DescribeReplicationGroupsResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeReplicationGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns information about reserved cache nodes for this account, or about a specified reserved cache node.</p>"]
    fn describe_reserved_cache_nodes
        (&self,
         input: &DescribeReservedCacheNodesMessage)
         -> Result<ReservedCacheNodeMessage, DescribeReservedCacheNodesError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReservedCacheNodes");
        params.put("Version", "2015-02-02");
        DescribeReservedCacheNodesMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ReservedCacheNodeMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReservedCacheNodeMessageDeserializer::deserialize("DescribeReservedCacheNodesResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeReservedCacheNodesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Lists available reserved cache node offerings.</p>"]
    fn describe_reserved_cache_nodes_offerings
        (&self,
         input: &DescribeReservedCacheNodesOfferingsMessage)
         -> Result<ReservedCacheNodesOfferingMessage, DescribeReservedCacheNodesOfferingsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeReservedCacheNodesOfferings");
        params.put("Version", "2015-02-02");
        DescribeReservedCacheNodesOfferingsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ReservedCacheNodesOfferingMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ReservedCacheNodesOfferingMessageDeserializer::deserialize("DescribeReservedCacheNodesOfferingsResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(DescribeReservedCacheNodesOfferingsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Returns information about cache cluster or replication group snapshots. By default, <code>DescribeSnapshots</code> lists all of your snapshots; it can optionally describe a single snapshot, or just the snapshots associated with a particular cache cluster.</p> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn describe_snapshots(&self,
                          input: &DescribeSnapshotsMessage)
                          -> Result<DescribeSnapshotsListMessage, DescribeSnapshotsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "DescribeSnapshots");
        params.put("Version", "2015-02-02");
        DescribeSnapshotsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = DescribeSnapshotsListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(DescribeSnapshotsListMessageDeserializer::deserialize("DescribeSnapshotsResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(DescribeSnapshotsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all available node types that you can scale your Redis cluster's or replication group's current node type up to.</p> <p>When you use the <code>ModifyCacheCluster</code> or <code>ModifyReplicationGroup</code> operations to scale up your cluster or replication group, the value of the <code>CacheNodeType</code> parameter must be one of the node types returned by this operation.</p>"]
    fn list_allowed_node_type_modifications
        (&self,
         input: &ListAllowedNodeTypeModificationsMessage)
         -> Result<AllowedNodeTypeModificationsMessage, ListAllowedNodeTypeModificationsError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListAllowedNodeTypeModifications");
        params.put("Version", "2015-02-02");
        ListAllowedNodeTypeModificationsMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = AllowedNodeTypeModificationsMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(AllowedNodeTypeModificationsMessageDeserializer::deserialize("ListAllowedNodeTypeModificationsResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(ListAllowedNodeTypeModificationsError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Lists all cost allocation tags currently on the named resource. A <code>cost allocation tag</code> is a key-value pair where the key is case-sensitive and the value is optional. You can use cost allocation tags to categorize and track your AWS costs.</p> <p>You can have a maximum of 10 cost allocation tags on an ElastiCache resource. For more information, see <a href=\"http://docs.aws.amazon.com/AmazonElastiCache/latest/UserGuide/BestPractices.html\">Using Cost Allocation Tags in Amazon ElastiCache</a>.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceMessage)
                              -> Result<TagListMessage, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ListTagsForResource");
        params.put("Version", "2015-02-02");
        ListTagsForResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = TagListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TagListMessageDeserializer::deserialize("ListTagsForResourceResult",
                                                                          &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the settings for a cache cluster. You can use this operation to change one or more cluster configuration parameters by specifying the parameters and the new values.</p>"]
    fn modify_cache_cluster(&self,
                            input: &ModifyCacheClusterMessage)
                            -> Result<ModifyCacheClusterResult, ModifyCacheClusterError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyCacheCluster");
        params.put("Version", "2015-02-02");
        ModifyCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ModifyCacheClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(ModifyCacheClusterResultDeserializer::deserialize("ModifyCacheClusterResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(ModifyCacheClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the parameters of a cache parameter group. You can modify up to 20 parameters in a single request by submitting a list parameter name and value pairs.</p>"]
    fn modify_cache_parameter_group
        (&self,
         input: &ModifyCacheParameterGroupMessage)
         -> Result<CacheParameterGroupNameMessage, ModifyCacheParameterGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyCacheParameterGroup");
        params.put("Version", "2015-02-02");
        ModifyCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CacheParameterGroupNameMessageDeserializer::deserialize("ModifyCacheParameterGroupResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(ModifyCacheParameterGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Modifies an existing cache subnet group.</p>"]
    fn modify_cache_subnet_group
        (&self,
         input: &ModifyCacheSubnetGroupMessage)
         -> Result<ModifyCacheSubnetGroupResult, ModifyCacheSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyCacheSubnetGroup");
        params.put("Version", "2015-02-02");
        ModifyCacheSubnetGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ModifyCacheSubnetGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(ModifyCacheSubnetGroupResultDeserializer::deserialize("ModifyCacheSubnetGroupResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(ModifyCacheSubnetGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the settings for a replication group.</p> <important> <p>Due to current limitations on Redis (cluster mode disabled), this operation or parameter is not supported on Redis (cluster mode enabled) replication groups.</p> </important> <note> <p>This operation is valid for Redis only.</p> </note>"]
    fn modify_replication_group
        (&self,
         input: &ModifyReplicationGroupMessage)
         -> Result<ModifyReplicationGroupResult, ModifyReplicationGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ModifyReplicationGroup");
        params.put("Version", "2015-02-02");
        ModifyReplicationGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = ModifyReplicationGroupResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(ModifyReplicationGroupResultDeserializer::deserialize("ModifyReplicationGroupResult",
                                                                                   &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(ModifyReplicationGroupError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Allows you to purchase a reserved cache node offering.</p>"]
    fn purchase_reserved_cache_nodes_offering
        (&self,
         input: &PurchaseReservedCacheNodesOfferingMessage)
         -> Result<PurchaseReservedCacheNodesOfferingResult,
                   PurchaseReservedCacheNodesOfferingError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "PurchaseReservedCacheNodesOffering");
        params.put("Version", "2015-02-02");
        PurchaseReservedCacheNodesOfferingMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = PurchaseReservedCacheNodesOfferingResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(PurchaseReservedCacheNodesOfferingResultDeserializer::deserialize("PurchaseReservedCacheNodesOfferingResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(PurchaseReservedCacheNodesOfferingError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Reboots some, or all, of the cache nodes within a provisioned cache cluster. This operation applies any modified cache parameter groups to the cache cluster. The reboot operation takes place as soon as possible, and results in a momentary outage to the cache cluster. During the reboot, the cache cluster status is set to REBOOTING.</p> <p>The reboot causes the contents of the cache (for each cache node being rebooted) to be lost.</p> <p>When the reboot is complete, a cache cluster event is created.</p>"]
    fn reboot_cache_cluster(&self,
                            input: &RebootCacheClusterMessage)
                            -> Result<RebootCacheClusterResult, RebootCacheClusterError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RebootCacheCluster");
        params.put("Version", "2015-02-02");
        RebootCacheClusterMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = RebootCacheClusterResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RebootCacheClusterResultDeserializer::deserialize("RebootCacheClusterResult",
                                                                                    &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(RebootCacheClusterError::from_body(String::from_utf8_lossy(&response.body)
                                                           .as_ref()))
            }
        }
    }


    #[doc="<p>Removes the tags identified by the <code>TagKeys</code> list from the named resource.</p>"]
    fn remove_tags_from_resource(&self,
                                 input: &RemoveTagsFromResourceMessage)
                                 -> Result<TagListMessage, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RemoveTagsFromResource");
        params.put("Version", "2015-02-02");
        RemoveTagsFromResourceMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = TagListMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(TagListMessageDeserializer::deserialize("RemoveTagsFromResourceResult",
                                                                          &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                Err(RemoveTagsFromResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the parameters of a cache parameter group to the engine or system default value. You can reset specific parameters by submitting a list of parameter names. To reset the entire cache parameter group, specify the <code>ResetAllParameters</code> and <code>CacheParameterGroupName</code> parameters.</p>"]
    fn reset_cache_parameter_group
        (&self,
         input: &ResetCacheParameterGroupMessage)
         -> Result<CacheParameterGroupNameMessage, ResetCacheParameterGroupError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "ResetCacheParameterGroup");
        params.put("Version", "2015-02-02");
        ResetCacheParameterGroupMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = CacheParameterGroupNameMessage::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result =
                        try!(CacheParameterGroupNameMessageDeserializer::deserialize("ResetCacheParameterGroupResult",
                                                                                     &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(ResetCacheParameterGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }


    #[doc="<p>Revokes ingress from a cache security group. Use this operation to disallow access from an Amazon EC2 security group that had been previously authorized.</p>"]
    fn revoke_cache_security_group_ingress
        (&self,
         input: &RevokeCacheSecurityGroupIngressMessage)
         -> Result<RevokeCacheSecurityGroupIngressResult, RevokeCacheSecurityGroupIngressError> {
        let mut request = SignedRequest::new("POST", "elasticache", self.region, "/");
        let mut params = Params::new();

        params.put("Action", "RevokeCacheSecurityGroupIngress");
        params.put("Version", "2015-02-02");
        RevokeCacheSecurityGroupIngressMessageSerializer::serialize(&mut params, "", &input);
        request.set_params(params);

        request.sign(&try!(self.credentials_provider.credentials()));
        let response = try!(self.dispatcher.dispatch(&request));
        match response.status {
            StatusCode::Ok => {

                let result;

                if response.body.is_empty() {
                    result = RevokeCacheSecurityGroupIngressResult::default();
                } else {
                    let reader = EventReader::new_with_config(response.body.as_slice(),
                                                              ParserConfig::new()
                                                                  .trim_whitespace(true));
                    let mut stack = XmlResponse::new(reader.into_iter().peekable());
                    let _start_document = stack.next();
                    let actual_tag_name = try!(peek_at_name(&mut stack));
                    try!(start_element(&actual_tag_name, &mut stack));
                    result = try!(RevokeCacheSecurityGroupIngressResultDeserializer::deserialize("RevokeCacheSecurityGroupIngressResult", &mut stack));
                    skip_tree(&mut stack);
                    try!(end_element(&actual_tag_name, &mut stack));
                }
                Ok(result)
            }
            _ => {
                            Err(RevokeCacheSecurityGroupIngressError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
                        }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
