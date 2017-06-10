#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Describes a quota for an AWS account, for example, the number of replication instances allowed.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AccountQuota {
    #[doc="<p>The name of the AWS DMS quota for this AWS account.</p>"]
    #[serde(rename="AccountQuotaName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_quota_name: Option<String>,
    #[doc="<p>The maximum allowed value for the quota.</p>"]
    #[serde(rename="Max")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max: Option<Long>,
    #[doc="<p>The amount currently used toward the quota maximum.</p>"]
    #[serde(rename="Used")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub used: Option<Long>,
}

pub type AccountQuotaList = Vec<AccountQuota>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct AddTagsToResourceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the AWS DMS resource the tag is to be added to. AWS DMS resources include a replication instance, endpoint, and a replication task.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>The tag to be assigned to the DMS resource.</p>"]
    #[serde(rename="Tags")]
    pub tags: TagList,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AddTagsToResourceResponse;

pub type AuthMechanismValue = String;
pub type AuthTypeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct AvailabilityZone {
    #[doc="<p>The name of the availability zone.</p>"]
    #[serde(rename="Name")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub name: Option<String>,
}

pub type Boolean = bool;
pub type BooleanOptional = bool;
#[doc="<p>The SSL certificate that can be used to encrypt connections between the endpoints and the replication instance.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Certificate {
    #[doc="<p>The Amazon Resource Name (ARN) for the certificate.</p>"]
    #[serde(rename="CertificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The date that the certificate was created.</p>"]
    #[serde(rename="CertificateCreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_creation_date: Option<TStamp>,
    #[doc="<p>The customer-assigned name of the certificate. Valid characters are A-z and 0-9.</p>"]
    #[serde(rename="CertificateIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_identifier: Option<String>,
    #[doc="<p>The owner of the certificate.</p>"]
    #[serde(rename="CertificateOwner")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_owner: Option<String>,
    #[doc="<p>The contents of the .pem X.509 certificate file for the certificate.</p>"]
    #[serde(rename="CertificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
    #[doc="<p>The location of the imported Oracle Wallet certificate for use with SSL.</p>"]
    #[serde(rename="CertificateWallet")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub certificate_wallet: Option<CertificateWallet>,
    #[doc="<p>The key length of the cryptographic algorithm being used.</p>"]
    #[serde(rename="KeyLength")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key_length: Option<IntegerOptional>,
    #[doc="<p>The signing algorithm for the certificate.</p>"]
    #[serde(rename="SigningAlgorithm")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub signing_algorithm: Option<String>,
    #[doc="<p>The beginning date that the certificate is valid.</p>"]
    #[serde(rename="ValidFromDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub valid_from_date: Option<TStamp>,
    #[doc="<p>The final date that the certificate is valid.</p>"]
    #[serde(rename="ValidToDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub valid_to_date: Option<TStamp>,
}

pub type CertificateList = Vec<Certificate>;
pub type CertificateWallet = Vec<u8>;
pub type CompressionTypeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Connection {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[doc="<p>The identifier of the endpoint. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>"]
    #[serde(rename="EndpointIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[doc="<p>The error message when the connection last failed.</p>"]
    #[serde(rename="LastFailureMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_failure_message: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[doc="<p>The replication instance identifier. This parameter is stored as a lowercase string.</p>"]
    #[serde(rename="ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[doc="<p>The connection status.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
}

pub type ConnectionList = Vec<Connection>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateEndpointMessage {
    #[doc="<p>The Amazon Resource Number (ARN) for the certificate.</p>"]
    #[serde(rename="CertificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The name of the endpoint database.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>Settings in JSON format for the target Amazon DynamoDB endpoint. For more information about the available settings, see the <b>Using Object Mapping to Migrate Data to DynamoDB</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html\"> Using an Amazon DynamoDB Database as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="DynamoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[doc="<p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>"]
    #[serde(rename="EndpointIdentifier")]
    pub endpoint_identifier: String,
    #[doc="<p>The type of endpoint.</p>"]
    #[serde(rename="EndpointType")]
    pub endpoint_type: ReplicationEndpointTypeValue,
    #[doc="<p>The type of engine for the endpoint. Valid values, depending on the EndPointType, include MYSQL, ORACLE, POSTGRES, MARIADB, AURORA, REDSHIFT, S3, SYBASE, DYNAMODB, MONGODB, and SQLSERVER.</p>"]
    #[serde(rename="EngineName")]
    pub engine_name: String,
    #[doc="<p>Additional attributes associated with the connection.</p>"]
    #[serde(rename="ExtraConnectionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[doc="<p>The KMS key identifier that will be used to encrypt the connection parameters. If you do not specify a value for the KmsKeyId parameter, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.</p>"]
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kms_key_id: Option<String>,
    #[doc="<p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see the <b>Configuration Properties When Using MongoDB as a Source for AWS Database Migration Service</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html\"> Using Amazon S3 as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="MongoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[doc="<p>The password to be used to login to the endpoint database.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<SecretString>,
    #[doc="<p>The port used by the endpoint database.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<IntegerOptional>,
    #[doc="<p>Settings in JSON format for the target S3 endpoint. For more information about the available settings, see the <b>Extra Connection Attributes</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html\"> Using Amazon S3 as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="S3Settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[doc="<p>The name of the server where the endpoint database resides.</p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p>The SSL mode to use for the SSL connection.</p> <p>SSL mode can be one of four values: none, require, verify-ca, verify-full. </p> <p>The default value is none.</p>"]
    #[serde(rename="SslMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_mode: Option<DmsSslModeValue>,
    #[doc="<p>Tags to be added to the endpoint.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagList>,
    #[doc="<p>The user name to be used to login to the endpoint database.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateEndpointResponse {
    #[doc="<p>The endpoint that was created.</p>"]
    #[serde(rename="Endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateEventSubscriptionMessage {
    #[doc="<p> A Boolean value; set to <b>true</b> to activate the subscription, or set to <b>false</b> to create the subscription but not activate it. </p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<BooleanOptional>,
    #[doc="<p> A list of event categories for a source type that you want to subscribe to. You can see a list of the categories for a given source type by calling the <b>DescribeEventCategories</b> action or in the topic <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications</a> in the AWS Database Migration Service User Guide. </p>"]
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[doc="<p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>"]
    #[serde(rename="SnsTopicArn")]
    pub sns_topic_arn: String,
    #[doc="<p> The list of identifiers of the event sources for which events will be returned. If not specified, then all sources are included in the response. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it cannot end with a hyphen or contain two consecutive hyphens. </p>"]
    #[serde(rename="SourceIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_ids: Option<SourceIdsList>,
    #[doc="<p> The type of AWS DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value is not specified, all events are returned. </p> <p>Valid values: replication-instance | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
    #[doc="<p>The name of the DMS event notification subscription. </p> <p>Constraints: The name must be less than 255 characters. </p>"]
    #[serde(rename="SubscriptionName")]
    pub subscription_name: String,
    #[doc="<p>A tag to be attached to the event subscription.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateEventSubscriptionResponse {
    #[doc="<p>The event subscription that was created.</p>"]
    #[serde(rename="EventSubscription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateReplicationInstanceMessage {
    #[doc="<p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>"]
    #[serde(rename="AllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_storage: Option<IntegerOptional>,
    #[doc="<p>Indicates that minor engine upgrades will be applied automatically to the replication instance during the maintenance window.</p> <p>Default: <code>true</code> </p>"]
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>The EC2 Availability Zone that the replication instance will be created in.</p> <p>Default: A random, system-chosen Availability Zone in the endpoint's region.</p> <p> Example: <code>us-east-1d</code> </p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The engine version number of the replication instance.</p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p>The KMS key identifier that will be used to encrypt the content on the replication instance. If you do not specify a value for the KmsKeyId parameter, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.</p>"]
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kms_key_id: Option<String>,
    #[doc="<p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>"]
    #[serde(rename="MultiAZ")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_az: Option<BooleanOptional>,
    #[doc="<p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>Default: A 30-minute window selected at random from an 8-hour block of time per region, occurring on a random day of the week.</p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>"]
    #[serde(rename="PubliclyAccessible")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub publicly_accessible: Option<BooleanOptional>,
    #[doc="<p>The compute and memory capacity of the replication instance as specified by the replication instance class.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>"]
    #[serde(rename="ReplicationInstanceClass")]
    pub replication_instance_class: String,
    #[doc="<p>The replication instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>"]
    #[serde(rename="ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: String,
    #[doc="<p>A subnet group to associate with the replication instance.</p>"]
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    #[doc="<p>Tags to be associated with the replication instance.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagList>,
    #[doc="<p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>"]
    #[serde(rename="VpcSecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateReplicationInstanceResponse {
    #[doc="<p>The replication instance that was created.</p>"]
    #[serde(rename="ReplicationInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateReplicationSubnetGroupMessage {
    #[doc="<p>The description for the subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    #[doc="<p>The name for the replication subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens. Must not be \"default\".</p> <p>Example: <code>mySubnetgroup</code> </p>"]
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    #[doc="<p>The EC2 subnet IDs for the subnet group.</p>"]
    #[serde(rename="SubnetIds")]
    pub subnet_ids: SubnetIdentifierList,
    #[doc="<p>The tag to be assigned to the subnet group.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateReplicationSubnetGroupResponse {
    #[doc="<p>The replication subnet group that was created.</p>"]
    #[serde(rename="ReplicationSubnetGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateReplicationTaskMessage {
    #[doc="<p>The start time for the Change Data Capture (CDC) operation.</p>"]
    #[serde(rename="CdcStartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cdc_start_time: Option<TStamp>,
    #[doc="<p>The migration type.</p>"]
    #[serde(rename="MigrationType")]
    pub migration_type: MigrationTypeValue,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    #[doc="<p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul>"]
    #[serde(rename="ReplicationTaskIdentifier")]
    pub replication_task_identifier: String,
    #[doc="<p>Settings for the task, such as target metadata settings. For a complete list of task settings, see <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html\">Task Settings for AWS Database Migration Service Tasks</a>.</p>"]
    #[serde(rename="ReplicationTaskSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="SourceEndpointArn")]
    pub source_endpoint_arn: String,
    #[doc="<p>When using the AWS CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with \"file://\". When working with the DMS API, provide the JSON as the parameter value.</p> <p>For example, --table-mappings file://mappingfile.json</p>"]
    #[serde(rename="TableMappings")]
    pub table_mappings: String,
    #[doc="<p>Tags to be added to the replication instance.</p>"]
    #[serde(rename="Tags")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tags: Option<TagList>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="TargetEndpointArn")]
    pub target_endpoint_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateReplicationTaskResponse {
    #[doc="<p>The replication task that was created.</p>"]
    #[serde(rename="ReplicationTask")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteCertificateMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the deleted certificate.</p>"]
    #[serde(rename="CertificateArn")]
    pub certificate_arn: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteCertificateResponse {
    #[doc="<p>The Secure Sockets Layer (SSL) certificate.</p>"]
    #[serde(rename="Certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<Certificate>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteEndpointMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteEndpointResponse {
    #[doc="<p>The endpoint that was deleted.</p>"]
    #[serde(rename="Endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteEventSubscriptionMessage {
    #[doc="<p>The name of the DMS event notification subscription to be deleted.</p>"]
    #[serde(rename="SubscriptionName")]
    pub subscription_name: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteEventSubscriptionResponse {
    #[doc="<p>The event subscription that was deleted.</p>"]
    #[serde(rename="EventSubscription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteReplicationInstanceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance to be deleted.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteReplicationInstanceResponse {
    #[doc="<p>The replication instance that was deleted.</p>"]
    #[serde(rename="ReplicationInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteReplicationSubnetGroupMessage {
    #[doc="<p>The subnet group name of the replication instance.</p>"]
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteReplicationSubnetGroupResponse;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteReplicationTaskMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteReplicationTaskResponse {
    #[doc="<p>The deleted replication task.</p>"]
    #[serde(rename="ReplicationTask")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeAccountAttributesMessage;

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeAccountAttributesResponse {
    #[doc="<p>Account quota information.</p>"]
    #[serde(rename="AccountQuotas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub account_quotas: Option<AccountQuotaList>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeCertificatesMessage {
    #[doc="<p>Filters applied to the certificate described in the form of key-value pairs.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 10</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeCertificatesResponse {
    #[doc="<p>The Secure Sockets Layer (SSL) certificates associated with the replication instance.</p>"]
    #[serde(rename="Certificates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificates: Option<CertificateList>,
    #[doc="<p>The pagination token.</p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeConnectionsMessage {
    #[doc="<p>The filters applied to the connection.</p> <p>Valid filter names: endpoint-arn | replication-instance-arn</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeConnectionsResponse {
    #[doc="<p>A description of the connections.</p>"]
    #[serde(rename="Connections")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connections: Option<ConnectionList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEndpointTypesMessage {
    #[doc="<p>Filters applied to the describe action.</p> <p>Valid filter names: engine-name | endpoint-type</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEndpointTypesResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The type of endpoints that are supported.</p>"]
    #[serde(rename="SupportedEndpointTypes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supported_endpoint_types: Option<SupportedEndpointTypeList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEndpointsMessage {
    #[doc="<p>Filters applied to the describe action.</p> <p>Valid filter names: endpoint-arn | endpoint-type | endpoint-id | engine-name</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEndpointsResponse {
    #[doc="<p>Endpoint description.</p>"]
    #[serde(rename="Endpoints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoints: Option<EndpointList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventCategoriesMessage {
    #[doc="<p>Filters applied to the action.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventCategoriesResponse {
    #[doc="<p>A list of event categories.</p>"]
    #[serde(rename="EventCategoryGroupList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_category_group_list: Option<EventCategoryGroupList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventSubscriptionsMessage {
    #[doc="<p>Filters applied to the action.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The name of the AWS DMS event subscription to be described.</p>"]
    #[serde(rename="SubscriptionName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_name: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventSubscriptionsResponse {
    #[doc="<p>A list of event subscriptions.</p>"]
    #[serde(rename="EventSubscriptionsList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_subscriptions_list: Option<EventSubscriptionsList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeEventsMessage {
    #[doc="<p>The duration of the events to be listed.</p>"]
    #[serde(rename="Duration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration: Option<IntegerOptional>,
    #[doc="<p>The end time for the events to be listed.</p>"]
    #[serde(rename="EndTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub end_time: Option<TStamp>,
    #[doc="<p>A list of event categories for a source type that you want to subscribe to.</p>"]
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[doc="<p>Filters applied to the action.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p> The identifier of the event source. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens. It cannot end with a hyphen or contain two consecutive hyphens. </p>"]
    #[serde(rename="SourceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_identifier: Option<String>,
    #[doc="<p>The type of AWS DMS resource that generates events.</p> <p>Valid values: replication-instance | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<SourceType>,
    #[doc="<p>The start time for the events to be listed.</p>"]
    #[serde(rename="StartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub start_time: Option<TStamp>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeEventsResponse {
    #[doc="<p>The events described.</p>"]
    #[serde(rename="Events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<EventList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeOrderableReplicationInstancesMessage {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeOrderableReplicationInstancesResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The order-able replication instances available.</p>"]
    #[serde(rename="OrderableReplicationInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub orderable_replication_instances: Option<OrderableReplicationInstanceList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeRefreshSchemasStatusMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeRefreshSchemasStatusResponse {
    #[doc="<p>The status of the schema.</p>"]
    #[serde(rename="RefreshSchemasStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeReplicationInstancesMessage {
    #[doc="<p>Filters applied to the describe action.</p> <p>Valid filter names: replication-instance-arn | replication-instance-id | replication-instance-class | engine-version</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeReplicationInstancesResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The replication instances described.</p>"]
    #[serde(rename="ReplicationInstances")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instances: Option<ReplicationInstanceList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeReplicationSubnetGroupsMessage {
    #[doc="<p>Filters applied to the describe action.</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeReplicationSubnetGroupsResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>A description of the replication subnet groups.</p>"]
    #[serde(rename="ReplicationSubnetGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_groups: Option<ReplicationSubnetGroups>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeReplicationTasksMessage {
    #[doc="<p>Filters applied to the describe action.</p> <p>Valid filter names: replication-task-arn | replication-task-id | migration-type | endpoint-arn | replication-instance-arn</p>"]
    #[serde(rename="Filters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub filters: Option<FilterList>,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeReplicationTasksResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>A description of the replication tasks.</p>"]
    #[serde(rename="ReplicationTasks")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_tasks: Option<ReplicationTaskList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeSchemasMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeSchemasResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The described schema.</p>"]
    #[serde(rename="Schemas")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schemas: Option<SchemaList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeTableStatisticsMessage {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>"]
    #[serde(rename="MaxRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_records: Option<IntegerOptional>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeTableStatisticsResponse {
    #[doc="<p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>"]
    #[serde(rename="Marker")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub marker: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[doc="<p>The table statistics.</p>"]
    #[serde(rename="TableStatistics")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_statistics: Option<TableStatisticsList>,
}

pub type DmsSslModeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct DynamoDbSettings {
    #[doc="<p> The Amazon Resource Name (ARN) used by the service access IAM role. </p>"]
    #[serde(rename="ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Endpoint {
    #[doc="<p>The Amazon Resource Name (ARN) used for SSL connection to the endpoint.</p>"]
    #[serde(rename="CertificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The name of the database at the endpoint.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>The settings for the target DynamoDB database. For more information, see the <code>DynamoDBSettings</code> structure.</p>"]
    #[serde(rename="DynamoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[doc="<p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>"]
    #[serde(rename="EndpointIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[doc="<p>The type of endpoint.</p>"]
    #[serde(rename="EndpointType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_type: Option<ReplicationEndpointTypeValue>,
    #[doc="<p>The database engine name. Valid values, depending on the EndPointType, include MYSQL, ORACLE, POSTGRES, MARIADB, AURORA, REDSHIFT, S3, SYBASE, DYNAMODB, MONGODB, and SQLSERVER.</p>"]
    #[serde(rename="EngineName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_name: Option<String>,
    #[doc="<p> Value returned by a call to CreateEndpoint that can be used for cross-account validation. Use it on a subsequent call to CreateEndpoint to create the endpoint with a cross-account. </p>"]
    #[serde(rename="ExternalId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_id: Option<String>,
    #[doc="<p>Additional connection attributes used to connect to the endpoint.</p>"]
    #[serde(rename="ExtraConnectionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[doc="<p>The KMS key identifier that will be used to encrypt the connection parameters. If you do not specify a value for the KmsKeyId parameter, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.</p>"]
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kms_key_id: Option<String>,
    #[doc="<p>The settings for the MongoDB source endpoint. For more information, see the <code>MongoDbSettings</code> structure.</p>"]
    #[serde(rename="MongoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[doc="<p>The port value used to access the endpoint.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<IntegerOptional>,
    #[doc="<p>The settings for the S3 target endpoint. For more information, see the <code>S3Settings</code> structure.</p>"]
    #[serde(rename="S3Settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[doc="<p>The name of the server at the endpoint.</p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p>The SSL mode used to connect to the endpoint.</p> <p>SSL mode can be one of four values: none, require, verify-ca, verify-full. </p> <p>The default value is none.</p>"]
    #[serde(rename="SslMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_mode: Option<DmsSslModeValue>,
    #[doc="<p>The status of the endpoint.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The user name used to connect to the endpoint.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

pub type EndpointList = Vec<Endpoint>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Event {
    #[doc="<p>The date of the event.</p>"]
    #[serde(rename="Date")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub date: Option<TStamp>,
    #[doc="<p>The event categories available for the specified source type.</p>"]
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[doc="<p>The event message.</p>"]
    #[serde(rename="Message")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub message: Option<String>,
    #[doc="<p> The identifier of the event source. An identifier must begin with a letter and must contain only ASCII letters, digits, and hyphens; it cannot end with a hyphen or contain two consecutive hyphens. </p> <p>Constraints:replication instance, endpoint, migration task</p>"]
    #[serde(rename="SourceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_identifier: Option<String>,
    #[doc="<p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | endpoint | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<SourceType>,
}

pub type EventCategoriesList = Vec<String>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventCategoryGroup {
    #[doc="<p> A list of event categories for a <code>SourceType</code> that you want to subscribe to. </p>"]
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[doc="<p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
}

pub type EventCategoryGroupList = Vec<EventCategoryGroup>;
pub type EventList = Vec<Event>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct EventSubscription {
    #[doc="<p>The AWS DMS event notification subscription Id.</p>"]
    #[serde(rename="CustSubscriptionId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cust_subscription_id: Option<String>,
    #[doc="<p>The AWS customer account associated with the AWS DMS event notification subscription.</p>"]
    #[serde(rename="CustomerAwsId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub customer_aws_id: Option<String>,
    #[doc="<p>Boolean value that indicates if the event subscription is enabled.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<Boolean>,
    #[doc="<p>A lists of event categories.</p>"]
    #[serde(rename="EventCategoriesList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories_list: Option<EventCategoriesList>,
    #[doc="<p>The topic ARN of the AWS DMS event notification subscription.</p>"]
    #[serde(rename="SnsTopicArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[doc="<p>A list of source Ids for the event subscription.</p>"]
    #[serde(rename="SourceIdsList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_ids_list: Option<SourceIdsList>,
    #[doc="<p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
    #[doc="<p>The status of the AWS DMS event notification subscription.</p> <p>Constraints:</p> <p>Can be one of the following: creating | modifying | deleting | active | no-permission | topic-not-exist</p> <p>The status \"no-permission\" indicates that AWS DMS no longer has permission to post to the SNS topic. The status \"topic-not-exist\" indicates that the topic was deleted after the subscription was created.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The time the RDS event notification subscription was created.</p>"]
    #[serde(rename="SubscriptionCreationTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

pub type EventSubscriptionsList = Vec<EventSubscription>;
pub type ExceptionMessage = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Filter {
    #[doc="<p>The name of the filter.</p>"]
    #[serde(rename="Name")]
    pub name: String,
    #[doc="<p>The filter value.</p>"]
    #[serde(rename="Values")]
    pub values: FilterValueList,
}

pub type FilterList = Vec<Filter>;
pub type FilterValueList = Vec<String>;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ImportCertificateMessage {
    #[doc="<p>The customer-assigned name of the certificate. Valid characters are A-z and 0-9.</p>"]
    #[serde(rename="CertificateIdentifier")]
    pub certificate_identifier: String,
    #[doc="<p>The contents of the .pem X.509 certificate file for the certificate.</p>"]
    #[serde(rename="CertificatePem")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_pem: Option<String>,
    #[doc="<p>The location of the imported Oracle Wallet certificate for use with SSL.</p>"]
    #[serde(rename="CertificateWallet")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub certificate_wallet: Option<CertificateWallet>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ImportCertificateResponse {
    #[doc="<p>The certificate to be uploaded.</p>"]
    #[serde(rename="Certificate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate: Option<Certificate>,
}

pub type Integer = i64;
pub type IntegerOptional = i64;
pub type KeyList = Vec<String>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListTagsForResourceMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the AWS DMS resource.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListTagsForResourceResponse {
    #[doc="<p>A list of tags for the resource.</p>"]
    #[serde(rename="TagList")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tag_list: Option<TagList>,
}

pub type Long = i64;
pub type MigrationTypeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyEndpointMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the certificate used for SSL connection.</p>"]
    #[serde(rename="CertificateArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub certificate_arn: Option<String>,
    #[doc="<p>The name of the endpoint database.</p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p>Settings in JSON format for the target Amazon DynamoDB endpoint. For more information about the available settings, see the <b>Using Object Mapping to Migrate Data to DynamoDB</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html\"> Using an Amazon DynamoDB Database as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="DynamoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
    #[doc="<p>The database endpoint identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens.</p>"]
    #[serde(rename="EndpointIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_identifier: Option<String>,
    #[doc="<p>The type of endpoint.</p>"]
    #[serde(rename="EndpointType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_type: Option<ReplicationEndpointTypeValue>,
    #[doc="<p>The type of engine for the endpoint. Valid values, depending on the EndPointType, include MYSQL, ORACLE, POSTGRES, MARIADB, AURORA, REDSHIFT, S3, DYNAMODB, MONGODB, SYBASE, and SQLSERVER.</p>"]
    #[serde(rename="EngineName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_name: Option<String>,
    #[doc="<p>Additional attributes associated with the connection.</p>"]
    #[serde(rename="ExtraConnectionAttributes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    #[doc="<p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see the <b>Configuration Properties When Using MongoDB as a Source for AWS Database Migration Service</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html\"> Using Amazon S3 as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="MongoDbSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    #[doc="<p>The password to be used to login to the endpoint database.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<SecretString>,
    #[doc="<p>The port used by the endpoint database.</p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<IntegerOptional>,
    #[doc="<p>Settings in JSON format for the target S3 endpoint. For more information about the available settings, see the <b>Extra Connection Attributes</b> section at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html\"> Using Amazon S3 as a Target for AWS Database Migration Service</a>. </p>"]
    #[serde(rename="S3Settings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    #[doc="<p>The name of the server where the endpoint database resides.</p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p>The SSL mode to be used.</p> <p>SSL mode can be one of four values: none, require, verify-ca, verify-full. </p> <p>The default value is none.</p>"]
    #[serde(rename="SslMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ssl_mode: Option<DmsSslModeValue>,
    #[doc="<p>The user name to be used to login to the endpoint database.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModifyEndpointResponse {
    #[doc="<p>The modified endpoint.</p>"]
    #[serde(rename="Endpoint")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyEventSubscriptionMessage {
    #[doc="<p> A Boolean value; set to <b>true</b> to activate the subscription. </p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<BooleanOptional>,
    #[doc="<p> A list of event categories for a source type that you want to subscribe to. Use the <code>DescribeEventCategories</code> action to see a list of event categories. </p>"]
    #[serde(rename="EventCategories")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[doc="<p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>"]
    #[serde(rename="SnsTopicArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[doc="<p> The type of AWS DMS resource that generates the events you want to subscribe to. </p> <p>Valid values: replication-instance | migration-task</p>"]
    #[serde(rename="SourceType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_type: Option<String>,
    #[doc="<p>The name of the AWS DMS event notification subscription to be modified.</p>"]
    #[serde(rename="SubscriptionName")]
    pub subscription_name: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModifyEventSubscriptionResponse {
    #[doc="<p>The modified event subscription.</p>"]
    #[serde(rename="EventSubscription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyReplicationInstanceMessage {
    #[doc="<p>The amount of storage (in gigabytes) to be allocated for the replication instance.</p>"]
    #[serde(rename="AllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_storage: Option<IntegerOptional>,
    #[doc="<p>Indicates that major version upgrades are allowed. Changing this parameter does not result in an outage and the change is asynchronously applied as soon as possible.</p> <p>Constraints: This parameter must be set to true when specifying a value for the <code>EngineVersion</code> parameter that is a different major version than the replication instance's current version.</p>"]
    #[serde(rename="AllowMajorVersionUpgrade")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allow_major_version_upgrade: Option<Boolean>,
    #[doc="<p>Indicates whether the changes should be applied immediately or during the next maintenance window.</p>"]
    #[serde(rename="ApplyImmediately")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub apply_immediately: Option<Boolean>,
    #[doc="<p> Indicates that minor version upgrades will be applied automatically to the replication instance during the maintenance window. Changing this parameter does not result in an outage except in the following case and the change is asynchronously applied as soon as possible. An outage will result if this parameter is set to <code>true</code> during the maintenance window, and a newer minor version is available, and AWS DMS has enabled auto patching for that engine version. </p>"]
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_minor_version_upgrade: Option<BooleanOptional>,
    #[doc="<p>The engine version number of the replication instance.</p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>"]
    #[serde(rename="MultiAZ")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_az: Option<BooleanOptional>,
    #[doc="<p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter does not result in an outage, except in the following situation, and the change is asynchronously applied as soon as possible. If moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure pending changes are applied.</p> <p>Default: Uses existing setting</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes</p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    #[doc="<p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>"]
    #[serde(rename="ReplicationInstanceClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[doc="<p>The replication instance identifier. This parameter is stored as a lowercase string.</p>"]
    #[serde(rename="ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[doc="<p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>"]
    #[serde(rename="VpcSecurityGroupIds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModifyReplicationInstanceResponse {
    #[doc="<p>The modified replication instance.</p>"]
    #[serde(rename="ReplicationInstance")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyReplicationSubnetGroupMessage {
    #[doc="<p>The description of the replication instance subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    #[doc="<p>The name of the replication instance subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    #[doc="<p>A list of subnet IDs.</p>"]
    #[serde(rename="SubnetIds")]
    pub subnet_ids: SubnetIdentifierList,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModifyReplicationSubnetGroupResponse {
    #[doc="<p>The modified replication subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ModifyReplicationTaskMessage {
    #[doc="<p>The start time for the Change Data Capture (CDC) operation.</p>"]
    #[serde(rename="CdcStartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cdc_start_time: Option<TStamp>,
    #[doc="<p>The migration type.</p> <p>Valid values: full-load | cdc | full-load-and-cdc</p>"]
    #[serde(rename="MigrationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub migration_type: Option<MigrationTypeValue>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
    #[doc="<p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul>"]
    #[serde(rename="ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[doc="<p>JSON file that contains settings for the task, such as target metadata settings.</p>"]
    #[serde(rename="ReplicationTaskSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[doc="<p>When using the AWS CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with \"file://\". When working with the DMS API, provide the JSON as the parameter value.</p> <p>For example, --table-mappings file://mappingfile.json</p>"]
    #[serde(rename="TableMappings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_mappings: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ModifyReplicationTaskResponse {
    #[doc="<p>The replication task that was modified.</p>"]
    #[serde(rename="ReplicationTask")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct MongoDbSettings {
    #[doc="<p> The authentication mechanism you use to access the MongoDB source endpoint.</p> <p>Valid values: DEFAULT, MONGODB_CR, SCRAM_SHA_1 </p> <p>DEFAULT – For MongoDB version 2.x, use MONGODB_CR. For MongoDB version 3.x, use SCRAM_SHA_1. This attribute is not used when authType=No.</p>"]
    #[serde(rename="AuthMechanism")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_mechanism: Option<AuthMechanismValue>,
    #[doc="<p> The MongoDB database name. This attribute is not used when <code>authType=NO</code>. </p> <p>The default is admin.</p>"]
    #[serde(rename="AuthSource")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_source: Option<String>,
    #[doc="<p> The authentication type you use to access the MongoDB source endpoint.</p> <p>Valid values: NO, PASSWORD </p> <p>When NO is selected, user name and password parameters are not used and can be empty. </p>"]
    #[serde(rename="AuthType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auth_type: Option<AuthTypeValue>,
    #[doc="<p> The database name on the MongoDB source endpoint. </p>"]
    #[serde(rename="DatabaseName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub database_name: Option<String>,
    #[doc="<p> Indicates the number of documents to preview to determine the document organization. Use this attribute when <code>NestingLevel</code> is set to ONE. </p> <p>Must be a positive value greater than 0. Default value is 1000.</p>"]
    #[serde(rename="DocsToInvestigate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub docs_to_investigate: Option<String>,
    #[doc="<p> Specifies the document ID. Use this attribute when <code>NestingLevel</code> is set to NONE. </p> <p>Default value is false. </p>"]
    #[serde(rename="ExtractDocId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extract_doc_id: Option<String>,
    #[doc="<p> Specifies either document or table mode. </p> <p>Valid values: NONE, ONE</p> <p>Default value is NONE. Specify NONE to use document mode. Specify ONE to use table mode.</p>"]
    #[serde(rename="NestingLevel")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub nesting_level: Option<NestingLevelValue>,
    #[doc="<p> The password for the user account you use to access the MongoDB source endpoint. </p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<SecretString>,
    #[doc="<p> The port value for the MongoDB source endpoint. </p>"]
    #[serde(rename="Port")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub port: Option<IntegerOptional>,
    #[doc="<p> The name of the server on the MongoDB source endpoint. </p>"]
    #[serde(rename="ServerName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub server_name: Option<String>,
    #[doc="<p>The user name you use to access the MongoDB source endpoint. </p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

pub type NestingLevelValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct OrderableReplicationInstance {
    #[doc="<p>The default amount of storage (in gigabytes) that is allocated for the replication instance.</p>"]
    #[serde(rename="DefaultAllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub default_allocated_storage: Option<Integer>,
    #[doc="<p>The version of the replication engine.</p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>"]
    #[serde(rename="IncludedAllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub included_allocated_storage: Option<Integer>,
    #[doc="<p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>"]
    #[serde(rename="MaxAllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_allocated_storage: Option<Integer>,
    #[doc="<p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>"]
    #[serde(rename="MinAllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub min_allocated_storage: Option<Integer>,
    #[doc="<p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>"]
    #[serde(rename="ReplicationInstanceClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[doc="<p>The type of storage used by the replication instance.</p>"]
    #[serde(rename="StorageType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub storage_type: Option<String>,
}

pub type OrderableReplicationInstanceList = Vec<OrderableReplicationInstance>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RefreshSchemasMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RefreshSchemasResponse {
    #[doc="<p>The status of the refreshed schema.</p>"]
    #[serde(rename="RefreshSchemasStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RefreshSchemasStatus {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_arn: Option<String>,
    #[doc="<p>The last failure message for the schema.</p>"]
    #[serde(rename="LastFailureMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_failure_message: Option<String>,
    #[doc="<p>The date the schema was last refreshed.</p>"]
    #[serde(rename="LastRefreshDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_refresh_date: Option<TStamp>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[doc="<p>The status of the schema.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<RefreshSchemasStatusTypeValue>,
}

pub type RefreshSchemasStatusTypeValue = String;
#[derive(Default,Debug,Clone,Serialize)]
pub struct ReloadTablesMessage {
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance. </p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
    #[doc="<p>The name and schema of the table to be reloaded. </p>"]
    #[serde(rename="TablesToReload")]
    pub tables_to_reload: TableListToReload,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReloadTablesResponse {
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task. </p>"]
    #[serde(rename="ReplicationTaskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_arn: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RemoveTagsFromResourceMessage {
    #[doc="<p>&gt;The Amazon Resource Name (ARN) of the AWS DMS resource the tag is to be removed from.</p>"]
    #[serde(rename="ResourceArn")]
    pub resource_arn: String,
    #[doc="<p>The tag key (name) of the tag to be removed.</p>"]
    #[serde(rename="TagKeys")]
    pub tag_keys: KeyList,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RemoveTagsFromResourceResponse;

pub type ReplicationEndpointTypeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReplicationInstance {
    #[doc="<p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>"]
    #[serde(rename="AllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_storage: Option<Integer>,
    #[doc="<p>Boolean value indicating if minor version upgrades will be automatically applied to the instance.</p>"]
    #[serde(rename="AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_minor_version_upgrade: Option<Boolean>,
    #[doc="<p>The Availability Zone for the instance.</p>"]
    #[serde(rename="AvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub availability_zone: Option<String>,
    #[doc="<p>The engine version number of the replication instance.</p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p>The time the replication instance was created.</p>"]
    #[serde(rename="InstanceCreateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub instance_create_time: Option<TStamp>,
    #[doc="<p>The KMS key identifier that is used to encrypt the content on the replication instance. If you do not specify a value for the KmsKeyId parameter, then AWS DMS will use your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS region.</p>"]
    #[serde(rename="KmsKeyId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kms_key_id: Option<String>,
    #[doc="<p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>"]
    #[serde(rename="MultiAZ")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_az: Option<Boolean>,
    #[doc="<p>The pending modification values.</p>"]
    #[serde(rename="PendingModifiedValues")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub pending_modified_values: Option<ReplicationPendingModifiedValues>,
    #[doc="<p>The maintenance window times for the replication instance.</p>"]
    #[serde(rename="PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[doc="<p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>"]
    #[serde(rename="PubliclyAccessible")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub publicly_accessible: Option<Boolean>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[doc="<p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>"]
    #[serde(rename="ReplicationInstanceClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_class: Option<String>,
    #[doc="<p>The replication instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>"]
    #[serde(rename="ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    #[doc="<p>The private IP address of the replication instance.</p>"]
    #[serde(rename="ReplicationInstancePrivateIpAddresses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_private_ip_addresses:
        Option<ReplicationInstancePrivateIpAddressList>,
    #[doc="<p>The public IP address of the replication instance.</p>"]
    #[serde(rename="ReplicationInstancePublicIpAddresses")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_public_ip_addresses:
        Option<ReplicationInstancePublicIpAddressList>,
    #[doc="<p>The status of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_status: Option<String>,
    #[doc="<p>The subnet group for the replication instance.</p>"]
    #[serde(rename="ReplicationSubnetGroup")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
    #[doc="<p>The availability zone of the standby replication instance in a Multi-AZ deployment.</p>"]
    #[serde(rename="SecondaryAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[doc="<p>The VPC security group for the instance.</p>"]
    #[serde(rename="VpcSecurityGroups")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

pub type ReplicationInstanceList = Vec<ReplicationInstance>;
pub type ReplicationInstancePrivateIpAddressList = Vec<String>;
pub type ReplicationInstancePublicIpAddressList = Vec<String>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReplicationPendingModifiedValues {
    #[doc="<p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>"]
    #[serde(rename="AllocatedStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub allocated_storage: Option<IntegerOptional>,
    #[doc="<p>The engine version number of the replication instance.</p>"]
    #[serde(rename="EngineVersion")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_version: Option<String>,
    #[doc="<p> Specifies if the replication instance is a Multi-AZ deployment. You cannot set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>"]
    #[serde(rename="MultiAZ")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub multi_az: Option<BooleanOptional>,
    #[doc="<p>The compute and memory capacity of the replication instance.</p> <p> Valid Values: <code>dms.t2.micro | dms.t2.small | dms.t2.medium | dms.t2.large | dms.c4.large | dms.c4.xlarge | dms.c4.2xlarge | dms.c4.4xlarge </code> </p>"]
    #[serde(rename="ReplicationInstanceClass")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_class: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReplicationSubnetGroup {
    #[doc="<p>The description of the replication subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    #[doc="<p>The identifier of the replication instance subnet group.</p>"]
    #[serde(rename="ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    #[doc="<p>The status of the subnet group.</p>"]
    #[serde(rename="SubnetGroupStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_group_status: Option<String>,
    #[doc="<p>The subnets that are in the subnet group.</p>"]
    #[serde(rename="Subnets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnets: Option<SubnetList>,
    #[doc="<p>The ID of the VPC.</p>"]
    #[serde(rename="VpcId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_id: Option<String>,
}

pub type ReplicationSubnetGroups = Vec<ReplicationSubnetGroup>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReplicationTask {
    #[doc="<p>The last error (failure) message generated for the replication instance.</p>"]
    #[serde(rename="LastFailureMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_failure_message: Option<String>,
    #[doc="<p>The type of migration.</p>"]
    #[serde(rename="MigrationType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub migration_type: Option<MigrationTypeValue>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_instance_arn: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication task.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_arn: Option<String>,
    #[doc="<p>The date the replication task was created.</p>"]
    #[serde(rename="ReplicationTaskCreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_creation_date: Option<TStamp>,
    #[doc="<p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain from 1 to 255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul>"]
    #[serde(rename="ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_identifier: Option<String>,
    #[doc="<p>The settings for the replication task.</p>"]
    #[serde(rename="ReplicationTaskSettings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_settings: Option<String>,
    #[doc="<p>The date the replication task is scheduled to start.</p>"]
    #[serde(rename="ReplicationTaskStartDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_start_date: Option<TStamp>,
    #[doc="<p>The statistics for the task, including elapsed time, tables loaded, and table errors.</p>"]
    #[serde(rename="ReplicationTaskStats")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task_stats: Option<ReplicationTaskStats>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="SourceEndpointArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    #[doc="<p>The status of the replication task.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The reason the replication task was stopped.</p>"]
    #[serde(rename="StopReason")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stop_reason: Option<String>,
    #[doc="<p>Table mappings specified in the task.</p>"]
    #[serde(rename="TableMappings")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_mappings: Option<String>,
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="TargetEndpointArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub target_endpoint_arn: Option<String>,
}

pub type ReplicationTaskList = Vec<ReplicationTask>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ReplicationTaskStats {
    #[doc="<p>The elapsed time of the task, in milliseconds.</p>"]
    #[serde(rename="ElapsedTimeMillis")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elapsed_time_millis: Option<Long>,
    #[doc="<p>The percent complete for the full load migration task.</p>"]
    #[serde(rename="FullLoadProgressPercent")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_load_progress_percent: Option<Integer>,
    #[doc="<p>The number of errors that have occurred during this task.</p>"]
    #[serde(rename="TablesErrored")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_errored: Option<Integer>,
    #[doc="<p>The number of tables loaded for this task.</p>"]
    #[serde(rename="TablesLoaded")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_loaded: Option<Integer>,
    #[doc="<p>The number of tables currently loading for this task.</p>"]
    #[serde(rename="TablesLoading")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_loading: Option<Integer>,
    #[doc="<p>The number of tables queued for this task.</p>"]
    #[serde(rename="TablesQueued")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub tables_queued: Option<Integer>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct S3Settings {
    #[doc="<p> An optional parameter to set a folder name in the S3 bucket. If provided, tables are created in the path &lt;bucketFolder&gt;/&lt;schema_name&gt;/&lt;table_name&gt;/. If this parameter is not specified, then the path used is &lt;schema_name&gt;/&lt;table_name&gt;/. </p>"]
    #[serde(rename="BucketFolder")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_folder: Option<String>,
    #[doc="<p> The name of the S3 bucket. </p>"]
    #[serde(rename="BucketName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_name: Option<String>,
    #[doc="<p> An optional parameter to use GZIP to compress the target files. Set to GZIP to compress the target files. Set to NONE (the default) or do not use to leave the files uncompressed. </p>"]
    #[serde(rename="CompressionType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compression_type: Option<CompressionTypeValue>,
    #[doc="<p> The delimiter used to separate columns in the source files. The default is a comma. </p>"]
    #[serde(rename="CsvDelimiter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csv_delimiter: Option<String>,
    #[doc="<p> The delimiter used to separate rows in the source files. The default is a carriage return (\\n). </p>"]
    #[serde(rename="CsvRowDelimiter")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub csv_row_delimiter: Option<String>,
    #[doc="<p> </p>"]
    #[serde(rename="ExternalTableDefinition")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub external_table_definition: Option<String>,
    #[doc="<p> The Amazon Resource Name (ARN) used by the service access IAM role. </p>"]
    #[serde(rename="ServiceAccessRoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

pub type SchemaList = Vec<String>;
pub type SecretString = String;
pub type SourceIdsList = Vec<String>;
pub type SourceType = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StartReplicationTaskMessage {
    #[doc="<p>The start time for the Change Data Capture (CDC) operation.</p>"]
    #[serde(rename="CdcStartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cdc_start_time: Option<TStamp>,
    #[doc="<p>The Amazon Resource Number (ARN) of the replication task to be started.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
    #[doc="<p>The type of replication task.</p>"]
    #[serde(rename="StartReplicationTaskType")]
    pub start_replication_task_type: StartReplicationTaskTypeValue,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StartReplicationTaskResponse {
    #[doc="<p>The replication task started.</p>"]
    #[serde(rename="ReplicationTask")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

pub type StartReplicationTaskTypeValue = String;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct StopReplicationTaskMessage {
    #[doc="<p>The Amazon Resource Number(ARN) of the replication task to be stopped.</p>"]
    #[serde(rename="ReplicationTaskArn")]
    pub replication_task_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct StopReplicationTaskResponse {
    #[doc="<p>The replication task stopped.</p>"]
    #[serde(rename="ReplicationTask")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Subnet {
    #[doc="<p>The Availability Zone of the subnet.</p>"]
    #[serde(rename="SubnetAvailabilityZone")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    #[doc="<p>The subnet identifier.</p>"]
    #[serde(rename="SubnetIdentifier")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_identifier: Option<String>,
    #[doc="<p>The status of the subnet.</p>"]
    #[serde(rename="SubnetStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub subnet_status: Option<String>,
}

pub type SubnetIdentifierList = Vec<String>;
pub type SubnetList = Vec<Subnet>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SupportedEndpointType {
    #[doc="<p>The type of endpoint.</p>"]
    #[serde(rename="EndpointType")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub endpoint_type: Option<ReplicationEndpointTypeValue>,
    #[doc="<p>The database engine name. Valid values, depending on the EndPointType, include MYSQL, ORACLE, POSTGRES, MARIADB, AURORA, REDSHIFT, S3, SYBASE, DYNAMODB, MONGODB, and SQLSERVER.</p>"]
    #[serde(rename="EngineName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub engine_name: Option<String>,
    #[doc="<p>Indicates if Change Data Capture (CDC) is supported.</p>"]
    #[serde(rename="SupportsCDC")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub supports_cdc: Option<Boolean>,
}

pub type SupportedEndpointTypeList = Vec<SupportedEndpointType>;
pub type TStamp = f64;
pub type TableListToReload = Vec<TableToReload>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TableStatistics {
    #[doc="<p>The Data Definition Language (DDL) used to build and modify the structure of your tables.</p>"]
    #[serde(rename="Ddls")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub ddls: Option<Long>,
    #[doc="<p>The number of delete actions performed on a table.</p>"]
    #[serde(rename="Deletes")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub deletes: Option<Long>,
    #[doc="<p>The number of rows added during the Full Load operation.</p>"]
    #[serde(rename="FullLoadRows")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub full_load_rows: Option<Long>,
    #[doc="<p>The number of insert actions performed on a table.</p>"]
    #[serde(rename="Inserts")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub inserts: Option<Long>,
    #[doc="<p>The last time the table was updated.</p>"]
    #[serde(rename="LastUpdateTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_update_time: Option<TStamp>,
    #[doc="<p>The schema name.</p>"]
    #[serde(rename="SchemaName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_name: Option<String>,
    #[doc="<p>The name of the table.</p>"]
    #[serde(rename="TableName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_name: Option<String>,
    #[doc="<p>The state of the table.</p>"]
    #[serde(rename="TableState")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_state: Option<String>,
    #[doc="<p>The number of update actions performed on a table.</p>"]
    #[serde(rename="Updates")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub updates: Option<Long>,
}

pub type TableStatisticsList = Vec<TableStatistics>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TableToReload {
    #[doc="<p>The schema name of the table to be reloaded.</p>"]
    #[serde(rename="SchemaName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub schema_name: Option<String>,
    #[doc="<p>The table name of the table to be reloaded.</p>"]
    #[serde(rename="TableName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub table_name: Option<String>,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Tag {
    #[doc="<p>A key is the required name of the tag. The string value can be from 1 to 128 Unicode characters in length and cannot be prefixed with \"aws:\" or \"dms:\". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: \"^([\\\\p{L}\\\\p{Z}\\\\p{N}_.:/=+\\\\-]*)$\").</p>"]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="<p>A value is the optional value of the tag. The string value can be from 1 to 256 Unicode characters in length and cannot be prefixed with \"aws:\" or \"dms:\". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regex: \"^([\\\\p{L}\\\\p{Z}\\\\p{N}_.:/=+\\\\-]*)$\").</p>"]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

pub type TagList = Vec<Tag>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct TestConnectionMessage {
    #[doc="<p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>"]
    #[serde(rename="EndpointArn")]
    pub endpoint_arn: String,
    #[doc="<p>The Amazon Resource Name (ARN) of the replication instance.</p>"]
    #[serde(rename="ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct TestConnectionResponse {
    #[doc="<p>The connection tested.</p>"]
    #[serde(rename="Connection")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub connection: Option<Connection>,
}

pub type VpcSecurityGroupIdList = Vec<String>;
#[doc="<p/>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct VpcSecurityGroupMembership {
    #[doc="<p>The status of the VPC security group.</p>"]
    #[serde(rename="Status")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub status: Option<String>,
    #[doc="<p>The VPC security group Id.</p>"]
    #[serde(rename="VpcSecurityGroupId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

pub type VpcSecurityGroupMembershipList = Vec<VpcSecurityGroupMembership>;
/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    ///<p>The resource could not be found.</p>
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


impl AddTagsToResourceError {
    pub fn from_body(body: &str) -> AddTagsToResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => {
                        AddTagsToResourceError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddTagsToResourceError::Validation(error_message.to_string())
                    }
                    _ => AddTagsToResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddTagsToResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddTagsToResourceError {
    fn from(err: serde_json::error::Error) -> AddTagsToResourceError {
        AddTagsToResourceError::Unknown(err.description().to_string())
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
            AddTagsToResourceError::ResourceNotFoundFault(ref cause) => cause,
            AddTagsToResourceError::Validation(ref cause) => cause,
            AddTagsToResourceError::Credentials(ref err) => err.description(),
            AddTagsToResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddTagsToResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    ///<p>AWS DMS was denied access to the endpoint.</p>
    AccessDeniedFault(String),
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateEndpointError {
    pub fn from_body(body: &str) -> CreateEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedFault" => {
                        CreateEndpointError::AccessDeniedFault(String::from(error_message))
                    }
                    "InvalidResourceStateFault" => {
                        CreateEndpointError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        CreateEndpointError::KMSKeyNotAccessibleFault(String::from(error_message))
                    }
                    "ResourceAlreadyExistsFault" => {
                        CreateEndpointError::ResourceAlreadyExistsFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        CreateEndpointError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ResourceQuotaExceededFault" => {
                        CreateEndpointError::ResourceQuotaExceededFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateEndpointError::Validation(error_message.to_string())
                    }
                    _ => CreateEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEndpointError {
    fn from(err: serde_json::error::Error) -> CreateEndpointError {
        CreateEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEndpointError {
    fn from(err: CredentialsError) -> CreateEndpointError {
        CreateEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEndpointError {
    fn from(err: HttpDispatchError) -> CreateEndpointError {
        CreateEndpointError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEndpointError {
    fn description(&self) -> &str {
        match *self {
            CreateEndpointError::AccessDeniedFault(ref cause) => cause,
            CreateEndpointError::InvalidResourceStateFault(ref cause) => cause,
            CreateEndpointError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateEndpointError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateEndpointError::ResourceNotFoundFault(ref cause) => cause,
            CreateEndpointError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateEndpointError::Validation(ref cause) => cause,
            CreateEndpointError::Credentials(ref err) => err.description(),
            CreateEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    ///<p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    ///<p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
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
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceAlreadyExistsFault" => CreateEventSubscriptionError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => CreateEventSubscriptionError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => CreateEventSubscriptionError::ResourceQuotaExceededFault(String::from(error_message)),
                    "SNSInvalidTopicFault" => CreateEventSubscriptionError::SNSInvalidTopicFault(String::from(error_message)),
                    "SNSNoAuthorizationFault" => CreateEventSubscriptionError::SNSNoAuthorizationFault(String::from(error_message)),
                    "ValidationException" => {
                        CreateEventSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => CreateEventSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEventSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEventSubscriptionError {
    fn from(err: serde_json::error::Error) -> CreateEventSubscriptionError {
        CreateEventSubscriptionError::Unknown(err.description().to_string())
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
impl fmt::Display for CreateEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            CreateEventSubscriptionError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
            CreateEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            CreateEventSubscriptionError::Validation(ref cause) => cause,
            CreateEventSubscriptionError::Credentials(ref err) => err.description(),
            CreateEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationInstance
#[derive(Debug, PartialEq)]
pub enum CreateReplicationInstanceError {
    ///<p>AWS DMS was denied access to the endpoint.</p>
    AccessDeniedFault(String),
    ///<p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    ///<p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateReplicationInstanceError {
    pub fn from_body(body: &str) -> CreateReplicationInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedFault" => CreateReplicationInstanceError::AccessDeniedFault(String::from(error_message)),
                    "InsufficientResourceCapacityFault" => CreateReplicationInstanceError::InsufficientResourceCapacityFault(String::from(error_message)),
                    "InvalidResourceStateFault" => CreateReplicationInstanceError::InvalidResourceStateFault(String::from(error_message)),
                    "InvalidSubnet" => {
                        CreateReplicationInstanceError::InvalidSubnet(String::from(error_message))
                    }
                    "KMSKeyNotAccessibleFault" => CreateReplicationInstanceError::KMSKeyNotAccessibleFault(String::from(error_message)),
                    "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(String::from(error_message)),
                    "ResourceAlreadyExistsFault" => CreateReplicationInstanceError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => CreateReplicationInstanceError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => CreateReplicationInstanceError::ResourceQuotaExceededFault(String::from(error_message)),
                    "StorageQuotaExceededFault" => CreateReplicationInstanceError::StorageQuotaExceededFault(String::from(error_message)),
                    "ValidationException" => {
                        CreateReplicationInstanceError::Validation(error_message.to_string())
                    }
                    _ => CreateReplicationInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateReplicationInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateReplicationInstanceError {
    fn from(err: serde_json::error::Error) -> CreateReplicationInstanceError {
        CreateReplicationInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateReplicationInstanceError {
    fn from(err: CredentialsError) -> CreateReplicationInstanceError {
        CreateReplicationInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationInstanceError {
    fn from(err: HttpDispatchError) -> CreateReplicationInstanceError {
        CreateReplicationInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationInstanceError::AccessDeniedFault(ref cause) => cause,
            CreateReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => cause,
            CreateReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            CreateReplicationInstanceError::InvalidSubnet(ref cause) => cause,
            CreateReplicationInstanceError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            CreateReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationInstanceError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateReplicationInstanceError::StorageQuotaExceededFault(ref cause) => cause,
            CreateReplicationInstanceError::Validation(ref cause) => cause,
            CreateReplicationInstanceError::Credentials(ref err) => err.description(),
            CreateReplicationInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateReplicationSubnetGroupError {
    ///<p>AWS DMS was denied access to the endpoint.</p>
    AccessDeniedFault(String),
    ///<p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    ///<p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateReplicationSubnetGroupError {
    pub fn from_body(body: &str) -> CreateReplicationSubnetGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedFault" => CreateReplicationSubnetGroupError::AccessDeniedFault(String::from(error_message)),
                    "InvalidSubnet" => CreateReplicationSubnetGroupError::InvalidSubnet(String::from(error_message)),
                    "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(String::from(error_message)),
                    "ResourceAlreadyExistsFault" => CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => CreateReplicationSubnetGroupError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(String::from(error_message)),
                    "ValidationException" => {
                        CreateReplicationSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => CreateReplicationSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateReplicationSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateReplicationSubnetGroupError {
    fn from(err: serde_json::error::Error) -> CreateReplicationSubnetGroupError {
        CreateReplicationSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateReplicationSubnetGroupError {
    fn from(err: CredentialsError) -> CreateReplicationSubnetGroupError {
        CreateReplicationSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationSubnetGroupError {
    fn from(err: HttpDispatchError) -> CreateReplicationSubnetGroupError {
        CreateReplicationSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationSubnetGroupError::AccessDeniedFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::InvalidSubnet(ref cause) => cause,
            CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateReplicationSubnetGroupError::Validation(ref cause) => cause,
            CreateReplicationSubnetGroupError::Credentials(ref err) => err.description(),
            CreateReplicationSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateReplicationTask
#[derive(Debug, PartialEq)]
pub enum CreateReplicationTaskError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateReplicationTaskError {
    pub fn from_body(body: &str) -> CreateReplicationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => CreateReplicationTaskError::InvalidResourceStateFault(String::from(error_message)),
                    "KMSKeyNotAccessibleFault" => CreateReplicationTaskError::KMSKeyNotAccessibleFault(String::from(error_message)),
                    "ResourceAlreadyExistsFault" => CreateReplicationTaskError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => CreateReplicationTaskError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => CreateReplicationTaskError::ResourceQuotaExceededFault(String::from(error_message)),
                    "ValidationException" => {
                        CreateReplicationTaskError::Validation(error_message.to_string())
                    }
                    _ => CreateReplicationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateReplicationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateReplicationTaskError {
    fn from(err: serde_json::error::Error) -> CreateReplicationTaskError {
        CreateReplicationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateReplicationTaskError {
    fn from(err: CredentialsError) -> CreateReplicationTaskError {
        CreateReplicationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationTaskError {
    fn from(err: HttpDispatchError) -> CreateReplicationTaskError {
        CreateReplicationTaskError::HttpDispatch(err)
    }
}
impl fmt::Display for CreateReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            CreateReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            CreateReplicationTaskError::ResourceQuotaExceededFault(ref cause) => cause,
            CreateReplicationTaskError::Validation(ref cause) => cause,
            CreateReplicationTaskError::Credentials(ref err) => err.description(),
            CreateReplicationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteCertificateError {
    pub fn from_body(body: &str) -> DeleteCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DeleteCertificateError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => {
                        DeleteCertificateError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteCertificateError::Validation(error_message.to_string())
                    }
                    _ => DeleteCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteCertificateError {
    fn from(err: serde_json::error::Error) -> DeleteCertificateError {
        DeleteCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteCertificateError {
    fn from(err: CredentialsError) -> DeleteCertificateError {
        DeleteCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteCertificateError {
    fn from(err: HttpDispatchError) -> DeleteCertificateError {
        DeleteCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteCertificateError {
    fn description(&self) -> &str {
        match *self {
            DeleteCertificateError::InvalidResourceStateFault(ref cause) => cause,
            DeleteCertificateError::ResourceNotFoundFault(ref cause) => cause,
            DeleteCertificateError::Validation(ref cause) => cause,
            DeleteCertificateError::Credentials(ref err) => err.description(),
            DeleteCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteEndpointError {
    pub fn from_body(body: &str) -> DeleteEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => {
                        DeleteEndpointError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        DeleteEndpointError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEndpointError::Validation(error_message.to_string())
                    }
                    _ => DeleteEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEndpointError {
    fn from(err: serde_json::error::Error) -> DeleteEndpointError {
        DeleteEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEndpointError {
    fn from(err: CredentialsError) -> DeleteEndpointError {
        DeleteEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEndpointError {
    fn from(err: HttpDispatchError) -> DeleteEndpointError {
        DeleteEndpointError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEndpointError {
    fn description(&self) -> &str {
        match *self {
            DeleteEndpointError::InvalidResourceStateFault(ref cause) => cause,
            DeleteEndpointError::ResourceNotFoundFault(ref cause) => cause,
            DeleteEndpointError::Validation(ref cause) => cause,
            DeleteEndpointError::Credentials(ref err) => err.description(),
            DeleteEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteEventSubscriptionError {
    pub fn from_body(body: &str) -> DeleteEventSubscriptionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DeleteEventSubscriptionError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DeleteEventSubscriptionError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DeleteEventSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => DeleteEventSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEventSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEventSubscriptionError {
    fn from(err: serde_json::error::Error) -> DeleteEventSubscriptionError {
        DeleteEventSubscriptionError::Unknown(err.description().to_string())
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
impl fmt::Display for DeleteEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventSubscriptionError::InvalidResourceStateFault(ref cause) => cause,
            DeleteEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
            DeleteEventSubscriptionError::Validation(ref cause) => cause,
            DeleteEventSubscriptionError::Credentials(ref err) => err.description(),
            DeleteEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationInstance
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationInstanceError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteReplicationInstanceError {
    pub fn from_body(body: &str) -> DeleteReplicationInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DeleteReplicationInstanceError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DeleteReplicationInstanceError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DeleteReplicationInstanceError::Validation(error_message.to_string())
                    }
                    _ => DeleteReplicationInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteReplicationInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteReplicationInstanceError {
    fn from(err: serde_json::error::Error) -> DeleteReplicationInstanceError {
        DeleteReplicationInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationInstanceError {
    fn from(err: CredentialsError) -> DeleteReplicationInstanceError {
        DeleteReplicationInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationInstanceError {
    fn from(err: HttpDispatchError) -> DeleteReplicationInstanceError {
        DeleteReplicationInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
            DeleteReplicationInstanceError::Validation(ref cause) => cause,
            DeleteReplicationInstanceError::Credentials(ref err) => err.description(),
            DeleteReplicationInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationSubnetGroupError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteReplicationSubnetGroupError {
    pub fn from_body(body: &str) -> DeleteReplicationSubnetGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DeleteReplicationSubnetGroupError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DeleteReplicationSubnetGroupError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DeleteReplicationSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => DeleteReplicationSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteReplicationSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteReplicationSubnetGroupError {
    fn from(err: serde_json::error::Error) -> DeleteReplicationSubnetGroupError {
        DeleteReplicationSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationSubnetGroupError {
    fn from(err: CredentialsError) -> DeleteReplicationSubnetGroupError {
        DeleteReplicationSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationSubnetGroupError {
    fn from(err: HttpDispatchError) -> DeleteReplicationSubnetGroupError {
        DeleteReplicationSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationSubnetGroupError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
            DeleteReplicationSubnetGroupError::Validation(ref cause) => cause,
            DeleteReplicationSubnetGroupError::Credentials(ref err) => err.description(),
            DeleteReplicationSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteReplicationTask
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationTaskError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DeleteReplicationTaskError {
    pub fn from_body(body: &str) -> DeleteReplicationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DeleteReplicationTaskError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DeleteReplicationTaskError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DeleteReplicationTaskError::Validation(error_message.to_string())
                    }
                    _ => DeleteReplicationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteReplicationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteReplicationTaskError {
    fn from(err: serde_json::error::Error) -> DeleteReplicationTaskError {
        DeleteReplicationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationTaskError {
    fn from(err: CredentialsError) -> DeleteReplicationTaskError {
        DeleteReplicationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationTaskError {
    fn from(err: HttpDispatchError) -> DeleteReplicationTaskError {
        DeleteReplicationTaskError::HttpDispatch(err)
    }
}
impl fmt::Display for DeleteReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            DeleteReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            DeleteReplicationTaskError::Validation(ref cause) => cause,
            DeleteReplicationTaskError::Credentials(ref err) => err.description(),
            DeleteReplicationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeAccountAttributesError {
    pub fn from_body(body: &str) -> DescribeAccountAttributesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeAccountAttributesError::Validation(error_message.to_string())
                    }
                    _ => DescribeAccountAttributesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAccountAttributesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAccountAttributesError {
    fn from(err: serde_json::error::Error) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAccountAttributesError {
    fn from(err: CredentialsError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAccountAttributesError {
    fn from(err: HttpDispatchError) -> DescribeAccountAttributesError {
        DescribeAccountAttributesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAccountAttributesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAccountAttributesError::Validation(ref cause) => cause,
            DescribeAccountAttributesError::Credentials(ref err) => err.description(),
            DescribeAccountAttributesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeAccountAttributesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeCertificatesError {
    ///<p>The resource could not be found.</p>
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


impl DescribeCertificatesError {
    pub fn from_body(body: &str) -> DescribeCertificatesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => DescribeCertificatesError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeCertificatesError::Validation(error_message.to_string())
                    }
                    _ => DescribeCertificatesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeCertificatesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeCertificatesError {
    fn from(err: serde_json::error::Error) -> DescribeCertificatesError {
        DescribeCertificatesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeCertificatesError {
    fn from(err: CredentialsError) -> DescribeCertificatesError {
        DescribeCertificatesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeCertificatesError {
    fn from(err: HttpDispatchError) -> DescribeCertificatesError {
        DescribeCertificatesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeCertificatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCertificatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeCertificatesError::ResourceNotFoundFault(ref cause) => cause,
            DescribeCertificatesError::Validation(ref cause) => cause,
            DescribeCertificatesError::Credentials(ref err) => err.description(),
            DescribeCertificatesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeCertificatesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    ///<p>The resource could not be found.</p>
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


impl DescribeConnectionsError {
    pub fn from_body(body: &str) -> DescribeConnectionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => {
                        DescribeConnectionsError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeConnectionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeConnectionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConnectionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConnectionsError {
    fn from(err: serde_json::error::Error) -> DescribeConnectionsError {
        DescribeConnectionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConnectionsError {
    fn from(err: CredentialsError) -> DescribeConnectionsError {
        DescribeConnectionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConnectionsError {
    fn from(err: HttpDispatchError) -> DescribeConnectionsError {
        DescribeConnectionsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeConnectionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConnectionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConnectionsError::ResourceNotFoundFault(ref cause) => cause,
            DescribeConnectionsError::Validation(ref cause) => cause,
            DescribeConnectionsError::Credentials(ref err) => err.description(),
            DescribeConnectionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConnectionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpointTypes
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointTypesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeEndpointTypesError {
    pub fn from_body(body: &str) -> DescribeEndpointTypesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeEndpointTypesError::Validation(error_message.to_string())
                    }
                    _ => DescribeEndpointTypesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEndpointTypesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEndpointTypesError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointTypesError {
        DescribeEndpointTypesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointTypesError {
    fn from(err: CredentialsError) -> DescribeEndpointTypesError {
        DescribeEndpointTypesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointTypesError {
    fn from(err: HttpDispatchError) -> DescribeEndpointTypesError {
        DescribeEndpointTypesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEndpointTypesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointTypesError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointTypesError::Validation(ref cause) => cause,
            DescribeEndpointTypesError::Credentials(ref err) => err.description(),
            DescribeEndpointTypesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEndpointTypesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    ///<p>The resource could not be found.</p>
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


impl DescribeEndpointsError {
    pub fn from_body(body: &str) -> DescribeEndpointsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => {
                        DescribeEndpointsError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeEndpointsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEndpointsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEndpointsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEndpointsError {
    fn from(err: serde_json::error::Error) -> DescribeEndpointsError {
        DescribeEndpointsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeEndpointsError {
    fn from(err: CredentialsError) -> DescribeEndpointsError {
        DescribeEndpointsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeEndpointsError {
    fn from(err: HttpDispatchError) -> DescribeEndpointsError {
        DescribeEndpointsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeEndpointsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEndpointsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEndpointsError::ResourceNotFoundFault(ref cause) => cause,
            DescribeEndpointsError::Validation(ref cause) => cause,
            DescribeEndpointsError::Credentials(ref err) => err.description(),
            DescribeEndpointsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeEndpointsError::Unknown(ref cause) => cause,
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
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeEventCategoriesError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventCategoriesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventCategoriesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventCategoriesError {
    fn from(err: serde_json::error::Error) -> DescribeEventCategoriesError {
        DescribeEventCategoriesError::Unknown(err.description().to_string())
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
    ///<p>The resource could not be found.</p>
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


impl DescribeEventSubscriptionsError {
    pub fn from_body(body: &str) -> DescribeEventSubscriptionsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => DescribeEventSubscriptionsError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeEventSubscriptionsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventSubscriptionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventSubscriptionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventSubscriptionsError {
    fn from(err: serde_json::error::Error) -> DescribeEventSubscriptionsError {
        DescribeEventSubscriptionsError::Unknown(err.description().to_string())
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
impl fmt::Display for DescribeEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeEventSubscriptionsError::ResourceNotFoundFault(ref cause) => cause,
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
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeEventsError::Validation(error_message.to_string())
                    }
                    _ => DescribeEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeEventsError {
    fn from(err: serde_json::error::Error) -> DescribeEventsError {
        DescribeEventsError::Unknown(err.description().to_string())
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
            DescribeEventsError::Validation(ref cause) => cause,
            DescribeEventsError::Credentials(ref err) => err.description(),
            DescribeEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeOrderableReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableReplicationInstancesError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeOrderableReplicationInstancesError {
    pub fn from_body(body: &str) -> DescribeOrderableReplicationInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ValidationException" => {
                        DescribeOrderableReplicationInstancesError::Validation(error_message
                                                                                   .to_string())
                    }
                    _ => DescribeOrderableReplicationInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeOrderableReplicationInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeOrderableReplicationInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeOrderableReplicationInstancesError {
        DescribeOrderableReplicationInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeOrderableReplicationInstancesError {
    fn from(err: CredentialsError) -> DescribeOrderableReplicationInstancesError {
        DescribeOrderableReplicationInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeOrderableReplicationInstancesError {
    fn from(err: HttpDispatchError) -> DescribeOrderableReplicationInstancesError {
        DescribeOrderableReplicationInstancesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeOrderableReplicationInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeOrderableReplicationInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeOrderableReplicationInstancesError::Validation(ref cause) => cause,
            DescribeOrderableReplicationInstancesError::Credentials(ref err) => err.description(),
            DescribeOrderableReplicationInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeOrderableReplicationInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRefreshSchemasStatus
#[derive(Debug, PartialEq)]
pub enum DescribeRefreshSchemasStatusError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DescribeRefreshSchemasStatusError {
    pub fn from_body(body: &str) -> DescribeRefreshSchemasStatusError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DescribeRefreshSchemasStatusError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DescribeRefreshSchemasStatusError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeRefreshSchemasStatusError::Validation(error_message.to_string())
                    }
                    _ => DescribeRefreshSchemasStatusError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeRefreshSchemasStatusError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeRefreshSchemasStatusError {
    fn from(err: serde_json::error::Error) -> DescribeRefreshSchemasStatusError {
        DescribeRefreshSchemasStatusError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeRefreshSchemasStatusError {
    fn from(err: CredentialsError) -> DescribeRefreshSchemasStatusError {
        DescribeRefreshSchemasStatusError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeRefreshSchemasStatusError {
    fn from(err: HttpDispatchError) -> DescribeRefreshSchemasStatusError {
        DescribeRefreshSchemasStatusError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeRefreshSchemasStatusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRefreshSchemasStatusError {
    fn description(&self) -> &str {
        match *self {
            DescribeRefreshSchemasStatusError::InvalidResourceStateFault(ref cause) => cause,
            DescribeRefreshSchemasStatusError::ResourceNotFoundFault(ref cause) => cause,
            DescribeRefreshSchemasStatusError::Validation(ref cause) => cause,
            DescribeRefreshSchemasStatusError::Credentials(ref err) => err.description(),
            DescribeRefreshSchemasStatusError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeRefreshSchemasStatusError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationInstancesError {
    ///<p>The resource could not be found.</p>
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


impl DescribeReplicationInstancesError {
    pub fn from_body(body: &str) -> DescribeReplicationInstancesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => DescribeReplicationInstancesError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeReplicationInstancesError::Validation(error_message.to_string())
                    }
                    _ => DescribeReplicationInstancesError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReplicationInstancesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeReplicationInstancesError {
    fn from(err: serde_json::error::Error) -> DescribeReplicationInstancesError {
        DescribeReplicationInstancesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeReplicationInstancesError {
    fn from(err: CredentialsError) -> DescribeReplicationInstancesError {
        DescribeReplicationInstancesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReplicationInstancesError {
    fn from(err: HttpDispatchError) -> DescribeReplicationInstancesError {
        DescribeReplicationInstancesError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReplicationInstancesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationInstancesError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationInstancesError::ResourceNotFoundFault(ref cause) => cause,
            DescribeReplicationInstancesError::Validation(ref cause) => cause,
            DescribeReplicationInstancesError::Credentials(ref err) => err.description(),
            DescribeReplicationInstancesError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReplicationInstancesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationSubnetGroupsError {
    ///<p>The resource could not be found.</p>
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


impl DescribeReplicationSubnetGroupsError {
    pub fn from_body(body: &str) -> DescribeReplicationSubnetGroupsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeReplicationSubnetGroupsError::Validation(error_message.to_string())
                    }
                    _ => DescribeReplicationSubnetGroupsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReplicationSubnetGroupsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeReplicationSubnetGroupsError {
    fn from(err: serde_json::error::Error) -> DescribeReplicationSubnetGroupsError {
        DescribeReplicationSubnetGroupsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeReplicationSubnetGroupsError {
    fn from(err: CredentialsError) -> DescribeReplicationSubnetGroupsError {
        DescribeReplicationSubnetGroupsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReplicationSubnetGroupsError {
    fn from(err: HttpDispatchError) -> DescribeReplicationSubnetGroupsError {
        DescribeReplicationSubnetGroupsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReplicationSubnetGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationSubnetGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(ref cause) => cause,
            DescribeReplicationSubnetGroupsError::Validation(ref cause) => cause,
            DescribeReplicationSubnetGroupsError::Credentials(ref err) => err.description(),
            DescribeReplicationSubnetGroupsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReplicationSubnetGroupsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeReplicationTasks
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTasksError {
    ///<p>The resource could not be found.</p>
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


impl DescribeReplicationTasksError {
    pub fn from_body(body: &str) -> DescribeReplicationTasksError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => DescribeReplicationTasksError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeReplicationTasksError::Validation(error_message.to_string())
                    }
                    _ => DescribeReplicationTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeReplicationTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeReplicationTasksError {
    fn from(err: serde_json::error::Error) -> DescribeReplicationTasksError {
        DescribeReplicationTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeReplicationTasksError {
    fn from(err: CredentialsError) -> DescribeReplicationTasksError {
        DescribeReplicationTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeReplicationTasksError {
    fn from(err: HttpDispatchError) -> DescribeReplicationTasksError {
        DescribeReplicationTasksError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeReplicationTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeReplicationTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeReplicationTasksError::ResourceNotFoundFault(ref cause) => cause,
            DescribeReplicationTasksError::Validation(ref cause) => cause,
            DescribeReplicationTasksError::Credentials(ref err) => err.description(),
            DescribeReplicationTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeReplicationTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeSchemas
#[derive(Debug, PartialEq)]
pub enum DescribeSchemasError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DescribeSchemasError {
    pub fn from_body(body: &str) -> DescribeSchemasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => {
                        DescribeSchemasError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        DescribeSchemasError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeSchemasError::Validation(error_message.to_string())
                    }
                    _ => DescribeSchemasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeSchemasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeSchemasError {
    fn from(err: serde_json::error::Error) -> DescribeSchemasError {
        DescribeSchemasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeSchemasError {
    fn from(err: CredentialsError) -> DescribeSchemasError {
        DescribeSchemasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeSchemasError {
    fn from(err: HttpDispatchError) -> DescribeSchemasError {
        DescribeSchemasError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeSchemasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeSchemasError {
    fn description(&self) -> &str {
        match *self {
            DescribeSchemasError::InvalidResourceStateFault(ref cause) => cause,
            DescribeSchemasError::ResourceNotFoundFault(ref cause) => cause,
            DescribeSchemasError::Validation(ref cause) => cause,
            DescribeSchemasError::Credentials(ref err) => err.description(),
            DescribeSchemasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeSchemasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTableStatistics
#[derive(Debug, PartialEq)]
pub enum DescribeTableStatisticsError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl DescribeTableStatisticsError {
    pub fn from_body(body: &str) -> DescribeTableStatisticsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => DescribeTableStatisticsError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => DescribeTableStatisticsError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        DescribeTableStatisticsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTableStatisticsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTableStatisticsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTableStatisticsError {
    fn from(err: serde_json::error::Error) -> DescribeTableStatisticsError {
        DescribeTableStatisticsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTableStatisticsError {
    fn from(err: CredentialsError) -> DescribeTableStatisticsError {
        DescribeTableStatisticsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTableStatisticsError {
    fn from(err: HttpDispatchError) -> DescribeTableStatisticsError {
        DescribeTableStatisticsError::HttpDispatch(err)
    }
}
impl fmt::Display for DescribeTableStatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTableStatisticsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTableStatisticsError::InvalidResourceStateFault(ref cause) => cause,
            DescribeTableStatisticsError::ResourceNotFoundFault(ref cause) => cause,
            DescribeTableStatisticsError::Validation(ref cause) => cause,
            DescribeTableStatisticsError::Credentials(ref err) => err.description(),
            DescribeTableStatisticsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeTableStatisticsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ImportCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateError {
    ///<p>The certificate was not valid.</p>
    InvalidCertificateFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ImportCertificateError {
    pub fn from_body(body: &str) -> ImportCertificateError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidCertificateFault" => {
                        ImportCertificateError::InvalidCertificateFault(String::from(error_message))
                    }
                    "ResourceAlreadyExistsFault" => ImportCertificateError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ValidationException" => {
                        ImportCertificateError::Validation(error_message.to_string())
                    }
                    _ => ImportCertificateError::Unknown(String::from(body)),
                }
            }
            Err(_) => ImportCertificateError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ImportCertificateError {
    fn from(err: serde_json::error::Error) -> ImportCertificateError {
        ImportCertificateError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportCertificateError {
    fn from(err: CredentialsError) -> ImportCertificateError {
        ImportCertificateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportCertificateError {
    fn from(err: HttpDispatchError) -> ImportCertificateError {
        ImportCertificateError::HttpDispatch(err)
    }
}
impl fmt::Display for ImportCertificateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportCertificateError {
    fn description(&self) -> &str {
        match *self {
            ImportCertificateError::InvalidCertificateFault(ref cause) => cause,
            ImportCertificateError::ResourceAlreadyExistsFault(ref cause) => cause,
            ImportCertificateError::Validation(ref cause) => cause,
            ImportCertificateError::Credentials(ref err) => err.description(),
            ImportCertificateError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportCertificateError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    ///<p>The resource could not be found.</p>
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


impl ListTagsForResourceError {
    pub fn from_body(body: &str) -> ListTagsForResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => {
                        ListTagsForResourceError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListTagsForResourceError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::Unknown(err.description().to_string())
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
            ListTagsForResourceError::ResourceNotFoundFault(ref cause) => cause,
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyEndpoint
#[derive(Debug, PartialEq)]
pub enum ModifyEndpointError {
    ///<p>AWS DMS was denied access to the endpoint.</p>
    AccessDeniedFault(String),
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
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


impl ModifyEndpointError {
    pub fn from_body(body: &str) -> ModifyEndpointError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedFault" => {
                        ModifyEndpointError::AccessDeniedFault(String::from(error_message))
                    }
                    "InvalidResourceStateFault" => {
                        ModifyEndpointError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        ModifyEndpointError::KMSKeyNotAccessibleFault(String::from(error_message))
                    }
                    "ResourceAlreadyExistsFault" => {
                        ModifyEndpointError::ResourceAlreadyExistsFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        ModifyEndpointError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ModifyEndpointError::Validation(error_message.to_string())
                    }
                    _ => ModifyEndpointError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyEndpointError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyEndpointError {
    fn from(err: serde_json::error::Error) -> ModifyEndpointError {
        ModifyEndpointError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyEndpointError {
    fn from(err: CredentialsError) -> ModifyEndpointError {
        ModifyEndpointError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyEndpointError {
    fn from(err: HttpDispatchError) -> ModifyEndpointError {
        ModifyEndpointError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyEndpointError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEndpointError {
    fn description(&self) -> &str {
        match *self {
            ModifyEndpointError::AccessDeniedFault(ref cause) => cause,
            ModifyEndpointError::InvalidResourceStateFault(ref cause) => cause,
            ModifyEndpointError::KMSKeyNotAccessibleFault(ref cause) => cause,
            ModifyEndpointError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyEndpointError::ResourceNotFoundFault(ref cause) => cause,
            ModifyEndpointError::Validation(ref cause) => cause,
            ModifyEndpointError::Credentials(ref err) => err.description(),
            ModifyEndpointError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ModifyEndpointError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    ///<p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    ///<p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
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
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => ModifyEventSubscriptionError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => ModifyEventSubscriptionError::ResourceQuotaExceededFault(String::from(error_message)),
                    "SNSInvalidTopicFault" => ModifyEventSubscriptionError::SNSInvalidTopicFault(String::from(error_message)),
                    "SNSNoAuthorizationFault" => ModifyEventSubscriptionError::SNSNoAuthorizationFault(String::from(error_message)),
                    "ValidationException" => {
                        ModifyEventSubscriptionError::Validation(error_message.to_string())
                    }
                    _ => ModifyEventSubscriptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyEventSubscriptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyEventSubscriptionError {
    fn from(err: serde_json::error::Error) -> ModifyEventSubscriptionError {
        ModifyEventSubscriptionError::Unknown(err.description().to_string())
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
impl fmt::Display for ModifyEventSubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyEventSubscriptionError {
    fn description(&self) -> &str {
        match *self {
            ModifyEventSubscriptionError::ResourceNotFoundFault(ref cause) => cause,
            ModifyEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => cause,
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => cause,
            ModifyEventSubscriptionError::Validation(ref cause) => cause,
            ModifyEventSubscriptionError::Credentials(ref err) => err.description(),
            ModifyEventSubscriptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyEventSubscriptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationInstance
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationInstanceError {
    ///<p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
    ///<p>An upgrade dependency is preventing the database migration.</p>
    UpgradeDependencyFailureFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyReplicationInstanceError {
    pub fn from_body(body: &str) -> ModifyReplicationInstanceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InsufficientResourceCapacityFault" => ModifyReplicationInstanceError::InsufficientResourceCapacityFault(String::from(error_message)),
                    "InvalidResourceStateFault" => ModifyReplicationInstanceError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceAlreadyExistsFault" => ModifyReplicationInstanceError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => ModifyReplicationInstanceError::ResourceNotFoundFault(String::from(error_message)),
                    "StorageQuotaExceededFault" => ModifyReplicationInstanceError::StorageQuotaExceededFault(String::from(error_message)),
                    "UpgradeDependencyFailureFault" => ModifyReplicationInstanceError::UpgradeDependencyFailureFault(String::from(error_message)),
                    "ValidationException" => {
                        ModifyReplicationInstanceError::Validation(error_message.to_string())
                    }
                    _ => ModifyReplicationInstanceError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyReplicationInstanceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyReplicationInstanceError {
    fn from(err: serde_json::error::Error) -> ModifyReplicationInstanceError {
        ModifyReplicationInstanceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyReplicationInstanceError {
    fn from(err: CredentialsError) -> ModifyReplicationInstanceError {
        ModifyReplicationInstanceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyReplicationInstanceError {
    fn from(err: HttpDispatchError) -> ModifyReplicationInstanceError {
        ModifyReplicationInstanceError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyReplicationInstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationInstanceError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => cause,
            ModifyReplicationInstanceError::InvalidResourceStateFault(ref cause) => cause,
            ModifyReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyReplicationInstanceError::ResourceNotFoundFault(ref cause) => cause,
            ModifyReplicationInstanceError::StorageQuotaExceededFault(ref cause) => cause,
            ModifyReplicationInstanceError::UpgradeDependencyFailureFault(ref cause) => cause,
            ModifyReplicationInstanceError::Validation(ref cause) => cause,
            ModifyReplicationInstanceError::Credentials(ref err) => err.description(),
            ModifyReplicationInstanceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyReplicationInstanceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationSubnetGroupError {
    ///<p>AWS DMS was denied access to the endpoint.</p>
    AccessDeniedFault(String),
    ///<p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    ///<p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    ///<p>The specified subnet is already in use.</p>
    SubnetAlreadyInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ModifyReplicationSubnetGroupError {
    pub fn from_body(body: &str) -> ModifyReplicationSubnetGroupError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AccessDeniedFault" => ModifyReplicationSubnetGroupError::AccessDeniedFault(String::from(error_message)),
                    "InvalidSubnet" => ModifyReplicationSubnetGroupError::InvalidSubnet(String::from(error_message)),
                    "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(String::from(error_message)),
                    "ResourceNotFoundFault" => ModifyReplicationSubnetGroupError::ResourceNotFoundFault(String::from(error_message)),
                    "ResourceQuotaExceededFault" => ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(String::from(error_message)),
                    "SubnetAlreadyInUse" => ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(String::from(error_message)),
                    "ValidationException" => {
                        ModifyReplicationSubnetGroupError::Validation(error_message.to_string())
                    }
                    _ => ModifyReplicationSubnetGroupError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyReplicationSubnetGroupError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyReplicationSubnetGroupError {
    fn from(err: serde_json::error::Error) -> ModifyReplicationSubnetGroupError {
        ModifyReplicationSubnetGroupError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyReplicationSubnetGroupError {
    fn from(err: CredentialsError) -> ModifyReplicationSubnetGroupError {
        ModifyReplicationSubnetGroupError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyReplicationSubnetGroupError {
    fn from(err: HttpDispatchError) -> ModifyReplicationSubnetGroupError {
        ModifyReplicationSubnetGroupError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyReplicationSubnetGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationSubnetGroupError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationSubnetGroupError::AccessDeniedFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::InvalidSubnet(ref cause) => cause,
            ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(ref cause) => cause,
            ModifyReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => cause,
            ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(ref cause) => cause,
            ModifyReplicationSubnetGroupError::Validation(ref cause) => cause,
            ModifyReplicationSubnetGroupError::Credentials(ref err) => err.description(),
            ModifyReplicationSubnetGroupError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyReplicationSubnetGroupError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ModifyReplicationTask
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationTaskError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    ///<p>The resource could not be found.</p>
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


impl ModifyReplicationTaskError {
    pub fn from_body(body: &str) -> ModifyReplicationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => ModifyReplicationTaskError::InvalidResourceStateFault(String::from(error_message)),
                    "KMSKeyNotAccessibleFault" => ModifyReplicationTaskError::KMSKeyNotAccessibleFault(String::from(error_message)),
                    "ResourceAlreadyExistsFault" => ModifyReplicationTaskError::ResourceAlreadyExistsFault(String::from(error_message)),
                    "ResourceNotFoundFault" => ModifyReplicationTaskError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        ModifyReplicationTaskError::Validation(error_message.to_string())
                    }
                    _ => ModifyReplicationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => ModifyReplicationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ModifyReplicationTaskError {
    fn from(err: serde_json::error::Error) -> ModifyReplicationTaskError {
        ModifyReplicationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ModifyReplicationTaskError {
    fn from(err: CredentialsError) -> ModifyReplicationTaskError {
        ModifyReplicationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ModifyReplicationTaskError {
    fn from(err: HttpDispatchError) -> ModifyReplicationTaskError {
        ModifyReplicationTaskError::HttpDispatch(err)
    }
}
impl fmt::Display for ModifyReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ModifyReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            ModifyReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            ModifyReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => cause,
            ModifyReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => cause,
            ModifyReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            ModifyReplicationTaskError::Validation(ref cause) => cause,
            ModifyReplicationTaskError::Credentials(ref err) => err.description(),
            ModifyReplicationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ModifyReplicationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RefreshSchemas
#[derive(Debug, PartialEq)]
pub enum RefreshSchemasError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RefreshSchemasError {
    pub fn from_body(body: &str) -> RefreshSchemasError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => {
                        RefreshSchemasError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        RefreshSchemasError::KMSKeyNotAccessibleFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        RefreshSchemasError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ResourceQuotaExceededFault" => {
                        RefreshSchemasError::ResourceQuotaExceededFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        RefreshSchemasError::Validation(error_message.to_string())
                    }
                    _ => RefreshSchemasError::Unknown(String::from(body)),
                }
            }
            Err(_) => RefreshSchemasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RefreshSchemasError {
    fn from(err: serde_json::error::Error) -> RefreshSchemasError {
        RefreshSchemasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RefreshSchemasError {
    fn from(err: CredentialsError) -> RefreshSchemasError {
        RefreshSchemasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RefreshSchemasError {
    fn from(err: HttpDispatchError) -> RefreshSchemasError {
        RefreshSchemasError::HttpDispatch(err)
    }
}
impl fmt::Display for RefreshSchemasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RefreshSchemasError {
    fn description(&self) -> &str {
        match *self {
            RefreshSchemasError::InvalidResourceStateFault(ref cause) => cause,
            RefreshSchemasError::KMSKeyNotAccessibleFault(ref cause) => cause,
            RefreshSchemasError::ResourceNotFoundFault(ref cause) => cause,
            RefreshSchemasError::ResourceQuotaExceededFault(ref cause) => cause,
            RefreshSchemasError::Validation(ref cause) => cause,
            RefreshSchemasError::Credentials(ref err) => err.description(),
            RefreshSchemasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RefreshSchemasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ReloadTables
#[derive(Debug, PartialEq)]
pub enum ReloadTablesError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl ReloadTablesError {
    pub fn from_body(body: &str) -> ReloadTablesError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => {
                        ReloadTablesError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        ReloadTablesError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        ReloadTablesError::Validation(error_message.to_string())
                    }
                    _ => ReloadTablesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ReloadTablesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ReloadTablesError {
    fn from(err: serde_json::error::Error) -> ReloadTablesError {
        ReloadTablesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ReloadTablesError {
    fn from(err: CredentialsError) -> ReloadTablesError {
        ReloadTablesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ReloadTablesError {
    fn from(err: HttpDispatchError) -> ReloadTablesError {
        ReloadTablesError::HttpDispatch(err)
    }
}
impl fmt::Display for ReloadTablesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ReloadTablesError {
    fn description(&self) -> &str {
        match *self {
            ReloadTablesError::InvalidResourceStateFault(ref cause) => cause,
            ReloadTablesError::ResourceNotFoundFault(ref cause) => cause,
            ReloadTablesError::Validation(ref cause) => cause,
            ReloadTablesError::Credentials(ref err) => err.description(),
            ReloadTablesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ReloadTablesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    ///<p>The resource could not be found.</p>
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


impl RemoveTagsFromResourceError {
    pub fn from_body(body: &str) -> RemoveTagsFromResourceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundFault" => RemoveTagsFromResourceError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        RemoveTagsFromResourceError::Validation(error_message.to_string())
                    }
                    _ => RemoveTagsFromResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemoveTagsFromResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemoveTagsFromResourceError {
    fn from(err: serde_json::error::Error) -> RemoveTagsFromResourceError {
        RemoveTagsFromResourceError::Unknown(err.description().to_string())
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
            RemoveTagsFromResourceError::ResourceNotFoundFault(ref cause) => cause,
            RemoveTagsFromResourceError::Validation(ref cause) => cause,
            RemoveTagsFromResourceError::Credentials(ref err) => err.description(),
            RemoveTagsFromResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveTagsFromResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartReplicationTask
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl StartReplicationTaskError {
    pub fn from_body(body: &str) -> StartReplicationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => StartReplicationTaskError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => StartReplicationTaskError::ResourceNotFoundFault(String::from(error_message)),
                    "ValidationException" => {
                        StartReplicationTaskError::Validation(error_message.to_string())
                    }
                    _ => StartReplicationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartReplicationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartReplicationTaskError {
    fn from(err: serde_json::error::Error) -> StartReplicationTaskError {
        StartReplicationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartReplicationTaskError {
    fn from(err: CredentialsError) -> StartReplicationTaskError {
        StartReplicationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartReplicationTaskError {
    fn from(err: HttpDispatchError) -> StartReplicationTaskError {
        StartReplicationTaskError::HttpDispatch(err)
    }
}
impl fmt::Display for StartReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            StartReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            StartReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            StartReplicationTaskError::Validation(ref cause) => cause,
            StartReplicationTaskError::Credentials(ref err) => err.description(),
            StartReplicationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartReplicationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopReplicationTask
#[derive(Debug, PartialEq)]
pub enum StopReplicationTaskError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>The resource could not be found.</p>
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


impl StopReplicationTaskError {
    pub fn from_body(body: &str) -> StopReplicationTaskError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => StopReplicationTaskError::InvalidResourceStateFault(String::from(error_message)),
                    "ResourceNotFoundFault" => {
                        StopReplicationTaskError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopReplicationTaskError::Validation(error_message.to_string())
                    }
                    _ => StopReplicationTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopReplicationTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopReplicationTaskError {
    fn from(err: serde_json::error::Error) -> StopReplicationTaskError {
        StopReplicationTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopReplicationTaskError {
    fn from(err: CredentialsError) -> StopReplicationTaskError {
        StopReplicationTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopReplicationTaskError {
    fn from(err: HttpDispatchError) -> StopReplicationTaskError {
        StopReplicationTaskError::HttpDispatch(err)
    }
}
impl fmt::Display for StopReplicationTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopReplicationTaskError {
    fn description(&self) -> &str {
        match *self {
            StopReplicationTaskError::InvalidResourceStateFault(ref cause) => cause,
            StopReplicationTaskError::ResourceNotFoundFault(ref cause) => cause,
            StopReplicationTaskError::Validation(ref cause) => cause,
            StopReplicationTaskError::Credentials(ref err) => err.description(),
            StopReplicationTaskError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopReplicationTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TestConnection
#[derive(Debug, PartialEq)]
pub enum TestConnectionError {
    ///<p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    ///<p>AWS DMS cannot access the KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    ///<p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    ///<p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl TestConnectionError {
    pub fn from_body(body: &str) -> TestConnectionError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidResourceStateFault" => {
                        TestConnectionError::InvalidResourceStateFault(String::from(error_message))
                    }
                    "KMSKeyNotAccessibleFault" => {
                        TestConnectionError::KMSKeyNotAccessibleFault(String::from(error_message))
                    }
                    "ResourceNotFoundFault" => {
                        TestConnectionError::ResourceNotFoundFault(String::from(error_message))
                    }
                    "ResourceQuotaExceededFault" => {
                        TestConnectionError::ResourceQuotaExceededFault(String::from(error_message))
                    }
                    "ValidationException" => {
                        TestConnectionError::Validation(error_message.to_string())
                    }
                    _ => TestConnectionError::Unknown(String::from(body)),
                }
            }
            Err(_) => TestConnectionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TestConnectionError {
    fn from(err: serde_json::error::Error) -> TestConnectionError {
        TestConnectionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TestConnectionError {
    fn from(err: CredentialsError) -> TestConnectionError {
        TestConnectionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TestConnectionError {
    fn from(err: HttpDispatchError) -> TestConnectionError {
        TestConnectionError::HttpDispatch(err)
    }
}
impl fmt::Display for TestConnectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TestConnectionError {
    fn description(&self) -> &str {
        match *self {
            TestConnectionError::InvalidResourceStateFault(ref cause) => cause,
            TestConnectionError::KMSKeyNotAccessibleFault(ref cause) => cause,
            TestConnectionError::ResourceNotFoundFault(ref cause) => cause,
            TestConnectionError::ResourceQuotaExceededFault(ref cause) => cause,
            TestConnectionError::Validation(ref cause) => cause,
            TestConnectionError::Credentials(ref err) => err.description(),
            TestConnectionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TestConnectionError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Database Migration Service API. AWS Database Migration Service clients implement this trait.
pub trait DatabaseMigrationService {
    #[doc="<p>Adds metadata tags to a DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceMessage)
                            -> Result<AddTagsToResourceResponse, AddTagsToResourceError>;


    #[doc="<p>Creates an endpoint using the provided settings.</p>"]
    fn create_endpoint(&self,
                       input: &CreateEndpointMessage)
                       -> Result<CreateEndpointResponse, CreateEndpointError>;


    #[doc="<p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a> in the AWS Database MIgration Service User Guide.</p>"]
    fn create_event_subscription
        (&self,
         input: &CreateEventSubscriptionMessage)
         -> Result<CreateEventSubscriptionResponse, CreateEventSubscriptionError>;


    #[doc="<p>Creates the replication instance using the specified parameters.</p>"]
    fn create_replication_instance
        (&self,
         input: &CreateReplicationInstanceMessage)
         -> Result<CreateReplicationInstanceResponse, CreateReplicationInstanceError>;


    #[doc="<p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>"]
    fn create_replication_subnet_group
        (&self,
         input: &CreateReplicationSubnetGroupMessage)
         -> Result<CreateReplicationSubnetGroupResponse, CreateReplicationSubnetGroupError>;


    #[doc="<p>Creates a replication task using the specified parameters.</p>"]
    fn create_replication_task
        (&self,
         input: &CreateReplicationTaskMessage)
         -> Result<CreateReplicationTaskResponse, CreateReplicationTaskError>;


    #[doc="<p>Deletes the specified certificate. </p>"]
    fn delete_certificate(&self,
                          input: &DeleteCertificateMessage)
                          -> Result<DeleteCertificateResponse, DeleteCertificateError>;


    #[doc="<p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/>"]
    fn delete_endpoint(&self,
                       input: &DeleteEndpointMessage)
                       -> Result<DeleteEndpointResponse, DeleteEndpointError>;


    #[doc="<p> Deletes an AWS DMS event subscription. </p>"]
    fn delete_event_subscription
        (&self,
         input: &DeleteEventSubscriptionMessage)
         -> Result<DeleteEventSubscriptionResponse, DeleteEventSubscriptionError>;


    #[doc="<p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/>"]
    fn delete_replication_instance
        (&self,
         input: &DeleteReplicationInstanceMessage)
         -> Result<DeleteReplicationInstanceResponse, DeleteReplicationInstanceError>;


    #[doc="<p>Deletes a subnet group.</p>"]
    fn delete_replication_subnet_group
        (&self,
         input: &DeleteReplicationSubnetGroupMessage)
         -> Result<DeleteReplicationSubnetGroupResponse, DeleteReplicationSubnetGroupError>;


    #[doc="<p>Deletes the specified replication task.</p>"]
    fn delete_replication_task
        (&self,
         input: &DeleteReplicationTaskMessage)
         -> Result<DeleteReplicationTaskResponse, DeleteReplicationTaskError>;


    #[doc="<p>Lists all of the AWS DMS attributes for a customer account. The attributes include AWS DMS quotas for the account, such as the number of replication instances allowed. The description for a quota includes the quota name, current usage toward that quota, and the quota's maximum value.</p> <p>This command does not take any parameters.</p>"]
    fn describe_account_attributes
        (&self,
         input: &DescribeAccountAttributesMessage)
         -> Result<DescribeAccountAttributesResponse, DescribeAccountAttributesError>;


    #[doc="<p>Provides a description of the certificate.</p>"]
    fn describe_certificates(&self,
                             input: &DescribeCertificatesMessage)
                             -> Result<DescribeCertificatesResponse, DescribeCertificatesError>;


    #[doc="<p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>"]
    fn describe_connections(&self,
                            input: &DescribeConnectionsMessage)
                            -> Result<DescribeConnectionsResponse, DescribeConnectionsError>;


    #[doc="<p>Returns information about the type of endpoints available.</p>"]
    fn describe_endpoint_types
        (&self,
         input: &DescribeEndpointTypesMessage)
         -> Result<DescribeEndpointTypesResponse, DescribeEndpointTypesError>;


    #[doc="<p>Returns information about the endpoints for your account in the current region.</p>"]
    fn describe_endpoints(&self,
                          input: &DescribeEndpointsMessage)
                          -> Result<DescribeEndpointsResponse, DescribeEndpointsError>;


    #[doc="<p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a> in the AWS Database Migration Service User Guide. </p>"]
    fn describe_event_categories
        (&self,
         input: &DescribeEventCategoriesMessage)
         -> Result<DescribeEventCategoriesResponse, DescribeEventCategoriesError>;


    #[doc="<p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>"]
    fn describe_event_subscriptions
        (&self,
         input: &DescribeEventSubscriptionsMessage)
         -> Result<DescribeEventSubscriptionsResponse, DescribeEventSubscriptionsError>;


    #[doc="<p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a>. </p>"]
    fn describe_events(&self,
                       input: &DescribeEventsMessage)
                       -> Result<DescribeEventsResponse, DescribeEventsError>;


    #[doc="<p>Returns information about the replication instance types that can be created in the specified region.</p>"]
    fn describe_orderable_replication_instances(&self, input: &DescribeOrderableReplicationInstancesMessage)  -> Result<DescribeOrderableReplicationInstancesResponse, DescribeOrderableReplicationInstancesError>;


    #[doc="<p>Returns the status of the RefreshSchemas operation.</p>"]
    fn describe_refresh_schemas_status
        (&self,
         input: &DescribeRefreshSchemasStatusMessage)
         -> Result<DescribeRefreshSchemasStatusResponse, DescribeRefreshSchemasStatusError>;


    #[doc="<p>Returns information about replication instances for your account in the current region.</p>"]
    fn describe_replication_instances
        (&self,
         input: &DescribeReplicationInstancesMessage)
         -> Result<DescribeReplicationInstancesResponse, DescribeReplicationInstancesError>;


    #[doc="<p>Returns information about the replication subnet groups.</p>"]
    fn describe_replication_subnet_groups
        (&self,
         input: &DescribeReplicationSubnetGroupsMessage)
         -> Result<DescribeReplicationSubnetGroupsResponse, DescribeReplicationSubnetGroupsError>;


    #[doc="<p>Returns information about replication tasks for your account in the current region.</p>"]
    fn describe_replication_tasks
        (&self,
         input: &DescribeReplicationTasksMessage)
         -> Result<DescribeReplicationTasksResponse, DescribeReplicationTasksError>;


    #[doc="<p>Returns information about the schema for the specified endpoint.</p> <p/>"]
    fn describe_schemas(&self,
                        input: &DescribeSchemasMessage)
                        -> Result<DescribeSchemasResponse, DescribeSchemasError>;


    #[doc="<p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p>"]
    fn describe_table_statistics
        (&self,
         input: &DescribeTableStatisticsMessage)
         -> Result<DescribeTableStatisticsResponse, DescribeTableStatisticsError>;


    #[doc="<p>Uploads the specified certificate.</p>"]
    fn import_certificate(&self,
                          input: &ImportCertificateMessage)
                          -> Result<ImportCertificateResponse, ImportCertificateError>;


    #[doc="<p>Lists all tags for an AWS DMS resource.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceMessage)
                              -> Result<ListTagsForResourceResponse, ListTagsForResourceError>;


    #[doc="<p>Modifies the specified endpoint.</p>"]
    fn modify_endpoint(&self,
                       input: &ModifyEndpointMessage)
                       -> Result<ModifyEndpointResponse, ModifyEndpointError>;


    #[doc="<p>Modifies an existing AWS DMS event notification subscription. </p>"]
    fn modify_event_subscription
        (&self,
         input: &ModifyEventSubscriptionMessage)
         -> Result<ModifyEventSubscriptionResponse, ModifyEventSubscriptionError>;


    #[doc="<p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/>"]
    fn modify_replication_instance
        (&self,
         input: &ModifyReplicationInstanceMessage)
         -> Result<ModifyReplicationInstanceResponse, ModifyReplicationInstanceError>;


    #[doc="<p>Modifies the settings for the specified replication subnet group.</p>"]
    fn modify_replication_subnet_group
        (&self,
         input: &ModifyReplicationSubnetGroupMessage)
         -> Result<ModifyReplicationSubnetGroupResponse, ModifyReplicationSubnetGroupError>;


    #[doc="<p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see the AWS DMS user guide at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html\"> Working with Migration Tasks </a> </p>"]
    fn modify_replication_task
        (&self,
         input: &ModifyReplicationTaskMessage)
         -> Result<ModifyReplicationTaskResponse, ModifyReplicationTaskError>;


    #[doc="<p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>"]
    fn refresh_schemas(&self,
                       input: &RefreshSchemasMessage)
                       -> Result<RefreshSchemasResponse, RefreshSchemasError>;


    #[doc="<p>Reloads the target database table with the source data. </p>"]
    fn reload_tables(&self,
                     input: &ReloadTablesMessage)
                     -> Result<ReloadTablesResponse, ReloadTablesError>;


    #[doc="<p>Removes metadata tags from a DMS resource.</p>"]
    fn remove_tags_from_resource
        (&self,
         input: &RemoveTagsFromResourceMessage)
         -> Result<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError>;


    #[doc="<p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see the AWS DMS user guide at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html\"> Working with Migration Tasks </a> </p>"]
    fn start_replication_task
        (&self,
         input: &StartReplicationTaskMessage)
         -> Result<StartReplicationTaskResponse, StartReplicationTaskError>;


    #[doc="<p>Stops the replication task.</p> <p/>"]
    fn stop_replication_task(&self,
                             input: &StopReplicationTaskMessage)
                             -> Result<StopReplicationTaskResponse, StopReplicationTaskError>;


    #[doc="<p>Tests the connection between the replication instance and the endpoint.</p>"]
    fn test_connection(&self,
                       input: &TestConnectionMessage)
                       -> Result<TestConnectionResponse, TestConnectionError>;
}
/// A client for the AWS Database Migration Service API.
pub struct DatabaseMigrationServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> DatabaseMigrationServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DatabaseMigrationServiceClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> DatabaseMigrationService for DatabaseMigrationServiceClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Adds metadata tags to a DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS.</p>"]
    fn add_tags_to_resource(&self,
                            input: &AddTagsToResourceMessage)
                            -> Result<AddTagsToResourceResponse, AddTagsToResourceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.AddTagsToResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<AddTagsToResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(AddTagsToResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Creates an endpoint using the provided settings.</p>"]
    fn create_endpoint(&self,
                       input: &CreateEndpointMessage)
                       -> Result<CreateEndpointResponse, CreateEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateEndpointResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateEndpointError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a> in the AWS Database MIgration Service User Guide.</p>"]
    fn create_event_subscription
        (&self,
         input: &CreateEventSubscriptionMessage)
         -> Result<CreateEventSubscriptionResponse, CreateEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEventSubscription");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateEventSubscriptionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateEventSubscriptionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates the replication instance using the specified parameters.</p>"]
    fn create_replication_instance
        (&self,
         input: &CreateReplicationInstanceMessage)
         -> Result<CreateReplicationInstanceResponse, CreateReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.CreateReplicationInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateReplicationInstanceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateReplicationInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>"]
    fn create_replication_subnet_group
        (&self,
         input: &CreateReplicationSubnetGroupMessage)
         -> Result<CreateReplicationSubnetGroupResponse, CreateReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.CreateReplicationSubnetGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateReplicationSubnetGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(CreateReplicationSubnetGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Creates a replication task using the specified parameters.</p>"]
    fn create_replication_task
        (&self,
         input: &CreateReplicationTaskMessage)
         -> Result<CreateReplicationTaskResponse, CreateReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateReplicationTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<CreateReplicationTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(CreateReplicationTaskError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified certificate. </p>"]
    fn delete_certificate(&self,
                          input: &DeleteCertificateMessage)
                          -> Result<DeleteCertificateResponse, DeleteCertificateError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteCertificate");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/>"]
    fn delete_endpoint(&self,
                       input: &DeleteEndpointMessage)
                       -> Result<DeleteEndpointResponse, DeleteEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteEndpointResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteEndpointError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p> Deletes an AWS DMS event subscription. </p>"]
    fn delete_event_subscription
        (&self,
         input: &DeleteEventSubscriptionMessage)
         -> Result<DeleteEventSubscriptionResponse, DeleteEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEventSubscription");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteEventSubscriptionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteEventSubscriptionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/>"]
    fn delete_replication_instance
        (&self,
         input: &DeleteReplicationInstanceMessage)
         -> Result<DeleteReplicationInstanceResponse, DeleteReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DeleteReplicationInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteReplicationInstanceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteReplicationInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes a subnet group.</p>"]
    fn delete_replication_subnet_group
        (&self,
         input: &DeleteReplicationSubnetGroupMessage)
         -> Result<DeleteReplicationSubnetGroupResponse, DeleteReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DeleteReplicationSubnetGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteReplicationSubnetGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DeleteReplicationSubnetGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Deletes the specified replication task.</p>"]
    fn delete_replication_task
        (&self,
         input: &DeleteReplicationTaskMessage)
         -> Result<DeleteReplicationTaskResponse, DeleteReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteReplicationTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DeleteReplicationTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DeleteReplicationTaskError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all of the AWS DMS attributes for a customer account. The attributes include AWS DMS quotas for the account, such as the number of replication instances allowed. The description for a quota includes the quota name, current usage toward that quota, and the quota's maximum value.</p> <p>This command does not take any parameters.</p>"]
    fn describe_account_attributes
        (&self,
         input: &DescribeAccountAttributesMessage)
         -> Result<DescribeAccountAttributesResponse, DescribeAccountAttributesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeAccountAttributes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeAccountAttributesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeAccountAttributesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Provides a description of the certificate.</p>"]
    fn describe_certificates(&self,
                             input: &DescribeCertificatesMessage)
                             -> Result<DescribeCertificatesResponse, DescribeCertificatesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeCertificates");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeCertificatesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeCertificatesError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>"]
    fn describe_connections(&self,
                            input: &DescribeConnectionsMessage)
                            -> Result<DescribeConnectionsResponse, DescribeConnectionsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeConnections");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeConnectionsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeConnectionsError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about the type of endpoints available.</p>"]
    fn describe_endpoint_types
        (&self,
         input: &DescribeEndpointTypesMessage)
         -> Result<DescribeEndpointTypesResponse, DescribeEndpointTypesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpointTypes");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEndpointTypesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEndpointTypesError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about the endpoints for your account in the current region.</p>"]
    fn describe_endpoints(&self,
                          input: &DescribeEndpointsMessage)
                          -> Result<DescribeEndpointsResponse, DescribeEndpointsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpoints");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEndpointsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEndpointsError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a> in the AWS Database Migration Service User Guide. </p>"]
    fn describe_event_categories
        (&self,
         input: &DescribeEventCategoriesMessage)
         -> Result<DescribeEventCategoriesResponse, DescribeEventCategoriesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEventCategories");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventCategoriesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeEventCategoriesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>"]
    fn describe_event_subscriptions
        (&self,
         input: &DescribeEventSubscriptionsMessage)
         -> Result<DescribeEventSubscriptionsResponse, DescribeEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeEventSubscriptions");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventSubscriptionsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeEventSubscriptionsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html\"> Working with Events and Notifications </a>. </p>"]
    fn describe_events(&self,
                       input: &DescribeEventsMessage)
                       -> Result<DescribeEventsResponse, DescribeEventsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEvents");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeEventsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeEventsError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Returns information about the replication instance types that can be created in the specified region.</p>"]
fn describe_orderable_replication_instances(&self, input: &DescribeOrderableReplicationInstancesMessage)  -> Result<DescribeOrderableReplicationInstancesResponse, DescribeOrderableReplicationInstancesError>{
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeOrderableReplicationInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeOrderableReplicationInstancesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeOrderableReplicationInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns the status of the RefreshSchemas operation.</p>"]
    fn describe_refresh_schemas_status
        (&self,
         input: &DescribeRefreshSchemasStatusMessage)
         -> Result<DescribeRefreshSchemasStatusResponse, DescribeRefreshSchemasStatusError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeRefreshSchemasStatus");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeRefreshSchemasStatusResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeRefreshSchemasStatusError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns information about replication instances for your account in the current region.</p>"]
    fn describe_replication_instances
        (&self,
         input: &DescribeReplicationInstancesMessage)
         -> Result<DescribeReplicationInstancesResponse, DescribeReplicationInstancesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeReplicationInstances");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeReplicationInstancesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeReplicationInstancesError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns information about the replication subnet groups.</p>"]
    fn describe_replication_subnet_groups
        (&self,
         input: &DescribeReplicationSubnetGroupsMessage)
         -> Result<DescribeReplicationSubnetGroupsResponse, DescribeReplicationSubnetGroupsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeReplicationSubnetGroups");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeReplicationSubnetGroupsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeReplicationSubnetGroupsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns information about replication tasks for your account in the current region.</p>"]
    fn describe_replication_tasks
        (&self,
         input: &DescribeReplicationTasksMessage)
         -> Result<DescribeReplicationTasksResponse, DescribeReplicationTasksError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.DescribeReplicationTasks");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeReplicationTasksResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeReplicationTasksError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Returns information about the schema for the specified endpoint.</p> <p/>"]
    fn describe_schemas(&self,
                        input: &DescribeSchemasMessage)
                        -> Result<DescribeSchemasResponse, DescribeSchemasError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeSchemas");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeSchemasResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(DescribeSchemasError::from_body(String::from_utf8_lossy(&response.body)
                                                        .as_ref()))
            }
        }
    }


    #[doc="<p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p>"]
    fn describe_table_statistics
        (&self,
         input: &DescribeTableStatisticsMessage)
         -> Result<DescribeTableStatisticsResponse, DescribeTableStatisticsError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeTableStatistics");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<DescribeTableStatisticsResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(DescribeTableStatisticsError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Uploads the specified certificate.</p>"]
    fn import_certificate(&self,
                          input: &ImportCertificateMessage)
                          -> Result<ImportCertificateResponse, ImportCertificateError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ImportCertificate");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ImportCertificateResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ImportCertificateError::from_body(String::from_utf8_lossy(&response.body)
                                                          .as_ref()))
            }
        }
    }


    #[doc="<p>Lists all tags for an AWS DMS resource.</p>"]
    fn list_tags_for_resource(&self,
                              input: &ListTagsForResourceMessage)
                              -> Result<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ListTagsForResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ListTagsForResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ListTagsForResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies the specified endpoint.</p>"]
    fn modify_endpoint(&self,
                       input: &ModifyEndpointMessage)
                       -> Result<ModifyEndpointResponse, ModifyEndpointError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEndpoint");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyEndpointResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ModifyEndpointError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Modifies an existing AWS DMS event notification subscription. </p>"]
    fn modify_event_subscription
        (&self,
         input: &ModifyEventSubscriptionMessage)
         -> Result<ModifyEventSubscriptionResponse, ModifyEventSubscriptionError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEventSubscription");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyEventSubscriptionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ModifyEventSubscriptionError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/>"]
    fn modify_replication_instance
        (&self,
         input: &ModifyReplicationInstanceMessage)
         -> Result<ModifyReplicationInstanceResponse, ModifyReplicationInstanceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.ModifyReplicationInstance");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyReplicationInstanceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ModifyReplicationInstanceError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Modifies the settings for the specified replication subnet group.</p>"]
    fn modify_replication_subnet_group
        (&self,
         input: &ModifyReplicationSubnetGroupMessage)
         -> Result<ModifyReplicationSubnetGroupResponse, ModifyReplicationSubnetGroupError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target",
                           "AmazonDMSv20160101.ModifyReplicationSubnetGroup");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyReplicationSubnetGroupResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => Err(ModifyReplicationSubnetGroupError::from_body(String::from_utf8_lossy(&response.body).as_ref())),
        }
    }


    #[doc="<p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see the AWS DMS user guide at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html\"> Working with Migration Tasks </a> </p>"]
    fn modify_replication_task
        (&self,
         input: &ModifyReplicationTaskMessage)
         -> Result<ModifyReplicationTaskResponse, ModifyReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyReplicationTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ModifyReplicationTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ModifyReplicationTaskError::from_body(String::from_utf8_lossy(&response.body)
                                                              .as_ref()))
            }
        }
    }


    #[doc="<p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>"]
    fn refresh_schemas(&self,
                       input: &RefreshSchemasMessage)
                       -> Result<RefreshSchemasResponse, RefreshSchemasError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.RefreshSchemas");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RefreshSchemasResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RefreshSchemasError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }


    #[doc="<p>Reloads the target database table with the source data. </p>"]
    fn reload_tables(&self,
                     input: &ReloadTablesMessage)
                     -> Result<ReloadTablesResponse, ReloadTablesError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.ReloadTables");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<ReloadTablesResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(ReloadTablesError::from_body(String::from_utf8_lossy(&response.body).as_ref()))
            }
        }
    }


    #[doc="<p>Removes metadata tags from a DMS resource.</p>"]
    fn remove_tags_from_resource
        (&self,
         input: &RemoveTagsFromResourceMessage)
         -> Result<RemoveTagsFromResourceResponse, RemoveTagsFromResourceError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.RemoveTagsFromResource");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<RemoveTagsFromResourceResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(RemoveTagsFromResourceError::from_body(String::from_utf8_lossy(&response.body)
                                                               .as_ref()))
            }
        }
    }


    #[doc="<p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see the AWS DMS user guide at <a href=\"http://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html\"> Working with Migration Tasks </a> </p>"]
    fn start_replication_task
        (&self,
         input: &StartReplicationTaskMessage)
         -> Result<StartReplicationTaskResponse, StartReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.StartReplicationTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StartReplicationTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StartReplicationTaskError::from_body(String::from_utf8_lossy(&response.body)
                                                             .as_ref()))
            }
        }
    }


    #[doc="<p>Stops the replication task.</p> <p/>"]
    fn stop_replication_task(&self,
                             input: &StopReplicationTaskMessage)
                             -> Result<StopReplicationTaskResponse, StopReplicationTaskError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.StopReplicationTask");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<StopReplicationTaskResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(StopReplicationTaskError::from_body(String::from_utf8_lossy(&response.body)
                                                            .as_ref()))
            }
        }
    }


    #[doc="<p>Tests the connection between the replication instance and the endpoint.</p>"]
    fn test_connection(&self,
                       input: &TestConnectionMessage)
                       -> Result<TestConnectionResponse, TestConnectionError> {
        let mut request = SignedRequest::new("POST", "dms", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AmazonDMSv20160101.TestConnection");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                            Ok(serde_json::from_str::<TestConnectionResponse>(String::from_utf8_lossy(&response.body).as_ref()).unwrap())
                        }
            _ => {
                Err(TestConnectionError::from_body(String::from_utf8_lossy(&response.body)
                                                       .as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
