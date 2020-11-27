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

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl DatabaseMigrationServiceClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "dms", &self.region, request_uri);

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
/// <p>Describes a quota for an AWS account, for example, the number of replication instances allowed.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountQuota {
    /// <p>The name of the AWS DMS quota for this AWS account.</p>
    #[serde(rename = "AccountQuotaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quota_name: Option<String>,
    /// <p>The maximum allowed value for the quota.</p>
    #[serde(rename = "Max")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    /// <p>The amount currently used toward the quota maximum.</p>
    #[serde(rename = "Used")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

/// <p>Associates a set of tags with an AWS DMS resource.</p>
/// see [DatabaseMigrationService::add_tags_to_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AddTagsToResourceMessage {
    /// <p>Identifies the AWS DMS resource to which tags should be added. The value for this parameter is an Amazon Resource Name (ARN).</p> <p>For AWS DMS, you can tag a replication instance, an endpoint, or a replication task.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>One or more tags to be assigned to the resource.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::add_tags_to_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddTagsToResourceResponse {}

/// <p><p/></p>
/// see [DatabaseMigrationService::apply_pending_maintenance_action]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ApplyPendingMaintenanceActionMessage {
    /// <p>The pending maintenance action to apply to this resource.</p>
    #[serde(rename = "ApplyAction")]
    pub apply_action: String,
    /// <p><p>A value that specifies the type of opt-in request, or undoes an opt-in request. You can&#39;t undo an opt-in request of type <code>immediate</code>.</p> <p>Valid values:</p> <ul> <li> <p> <code>immediate</code> - Apply the maintenance action immediately.</p> </li> <li> <p> <code>next-maintenance</code> - Apply the maintenance action during the next maintenance window for the resource.</p> </li> <li> <p> <code>undo-opt-in</code> - Cancel any existing <code>next-maintenance</code> opt-in requests.</p> </li> </ul></p>
    #[serde(rename = "OptInType")]
    pub opt_in_type: String,
    /// <p>The Amazon Resource Name (ARN) of the AWS DMS resource that the pending maintenance action applies to.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::apply_pending_maintenance_action]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ApplyPendingMaintenanceActionResponse {
    /// <p>The AWS DMS resource that the pending maintenance action will be applied to.</p>
    #[serde(rename = "ResourcePendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

/// <p>The name of an Availability Zone for use during database migration. <code>AvailabilityZone</code> is an optional parameter to the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_CreateReplicationInstance.html"> <code>CreateReplicationInstance</code> </a> operation, and it’s value relates to the AWS Region of an endpoint. For example, the availability zone of an endpoint in the us-east-1 region might be us-east-1a, us-east-1b, us-east-1c, or us-east-1d.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AvailabilityZone {
    /// <p>The name of the Availability Zone.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::cancel_replication_task_assessment_run]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelReplicationTaskAssessmentRunMessage {
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be canceled.</p>
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    pub replication_task_assessment_run_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::cancel_replication_task_assessment_run]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelReplicationTaskAssessmentRunResponse {
    /// <p>The <code>ReplicationTaskAssessmentRun</code> object for the canceled assessment run.</p>
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

/// <p>The SSL certificate that can be used to encrypt connections between the endpoints and the replication instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Certificate {
    /// <p>The Amazon Resource Name (ARN) for the certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The date that the certificate was created.</p>
    #[serde(rename = "CertificateCreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_creation_date: Option<f64>,
    /// <p>A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "CertificateIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_identifier: Option<String>,
    /// <p>The owner of the certificate.</p>
    #[serde(rename = "CertificateOwner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_owner: Option<String>,
    /// <p>The contents of a <code>.pem</code> file, which contains an X.509 certificate.</p>
    #[serde(rename = "CertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The location of an imported Oracle Wallet certificate for use with SSL.</p>
    #[serde(rename = "CertificateWallet")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<bytes::Bytes>,
    /// <p>The key length of the cryptographic algorithm being used.</p>
    #[serde(rename = "KeyLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_length: Option<i64>,
    /// <p>The signing algorithm for the certificate.</p>
    #[serde(rename = "SigningAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_algorithm: Option<String>,
    /// <p>The beginning date that the certificate is valid.</p>
    #[serde(rename = "ValidFromDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from_date: Option<f64>,
    /// <p>The final date that the certificate is valid.</p>
    #[serde(rename = "ValidToDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to_date: Option<f64>,
}

/// <p>Status of the connection between an endpoint and a replication instance, including Amazon Resource Names (ARNs) and the last error message issued.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Connection {
    /// <p>The ARN string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The identifier of the endpoint. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The error message when the connection last failed.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The ARN of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p><p>The connection status. This parameter can return one of the following values:</p> <ul> <li> <p> <code>&quot;successful&quot;</code> </p> </li> <li> <p> <code>&quot;testing&quot;</code> </p> </li> <li> <p> <code>&quot;failed&quot;</code> </p> </li> <li> <p> <code>&quot;deleting&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_endpoint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) for the certificate.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the endpoint database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Possible settings include the following:</p> <ul> <li> <p> <code>ServiceAccessRoleArn</code> - The IAM role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p> <code>BucketName</code> - The name of the S3 bucket to use.</p> </li> <li> <p> <code>CompressionType</code> - An optional parameter to use GZIP to compress the target files. To use GZIP, set this value to <code>NONE</code> (the default). To keep the files uncompressed, don't use this value.</p> </li> </ul> <p>Shorthand syntax for these settings is as follows: <code>ServiceAccessRoleArn=string,BucketName=string,CompressionType=string</code> </p> <p>JSON syntax for these settings is as follows: <code>{ "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </code> </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    #[serde(rename = "DocDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    /// <p>Settings in JSON format for the target Amazon DynamoDB endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html">Using Object Mapping to Migrate Data to DynamoDB</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>Settings in JSON format for the target Elasticsearch endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Elasticsearch.html#CHAP_Target.Elasticsearch.Configuration">Extra Connection Attributes When Using Elasticsearch as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen, or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    pub endpoint_identifier: String,
    /// <p>The type of endpoint. Valid values are <code>source</code> and <code>target</code>.</p>
    #[serde(rename = "EndpointType")]
    pub endpoint_type: String,
    /// <p>The type of engine for the endpoint. Valid values, depending on the <code>EndpointType</code> value, include <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>, <code>"aurora-postgresql"</code>, <code>"redshift"</code>, <code>"s3"</code>, <code>"db2"</code>, <code>"azuredb"</code>, <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>, <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>, <code>"docdb"</code>, <code>"sqlserver"</code>, and <code>"neptune"</code>.</p>
    #[serde(rename = "EngineName")]
    pub engine_name: String,
    /// <p>The external table definition. </p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional attributes associated with the connection. Each attribute is specified as a name-value pair associated by an equal sign (=). Multiple attributes are separated by a semicolon (;) with no additional white space. For information on the attributes available for connecting your source or target endpoint, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Endpoints.html">Working with AWS DMS Endpoints</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>Settings in JSON format for the source IBM Db2 LUW endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.DB2.html">Extra connection attributes when using Db2 LUW as a source for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "IBMDb2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db_2_settings: Option<IBMDb2Settings>,
    /// <p>Settings in JSON format for the target Apache Kafka endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kafka.html">Using Apache Kafka as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "KafkaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    /// <p>Settings in JSON format for the target endpoint for Amazon Kinesis Data Streams. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kinesis.html">Using Amazon Kinesis Data Streams as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>An AWS KMS key identifier that is used to encrypt the connection parameters for the endpoint.</p> <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key.</p> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>Settings in JSON format for the source and target Microsoft SQL Server endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SQLServer.html">Extra connection attributes when using SQL Server as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SQLServer.html"> Extra connection attributes when using SQL Server as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_sql_server_settings: Option<MicrosoftSQLServerSettings>,
    /// <p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html#CHAP_Source.MongoDB.Configuration">Using MongoDB as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>Settings in JSON format for the source and target MySQL endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MySQL.html">Extra connection attributes when using MySQL as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.MySQL.html">Extra connection attributes when using a MySQL-compatible database as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MySQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_settings: Option<MySQLSettings>,
    /// <p>Settings in JSON format for the target Amazon Neptune endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Neptune.html#CHAP_Target.Neptune.EndpointSettings">Specifying Endpoint Settings for Amazon Neptune as a Target</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "NeptuneSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    /// <p>Settings in JSON format for the source and target Oracle endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html">Extra connection attributes when using Oracle as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Oracle.html"> Extra connection attributes when using Oracle as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "OracleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    /// <p>The password to be used to log in to the endpoint database.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port used by the endpoint database.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Settings in JSON format for the source and target PostgreSQL endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html">Extra connection attributes when using PostgreSQL as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.PostgreSQL.html"> Extra connection attributes when using PostgreSQL as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_settings: Option<PostgreSQLSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, AWS DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    /// <p>Settings in JSON format for the target Amazon S3 endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring">Extra Connection Attributes When Using Amazon S3 as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server where the endpoint database resides.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for the service access role that you want to use to create the endpoint. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The Secure Sockets Layer (SSL) mode to use for the SSL connection. The default is <code>none</code> </p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>Settings in JSON format for the source and target SAP ASE endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SAP.html">Extra connection attributes when using SAP ASE as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SAP.html">Extra connection attributes when using SAP ASE as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "SybaseSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    /// <p>One or more tags to be assigned to the endpoint.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The user name to be used to log in to the endpoint database.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_endpoint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEndpointResponse {
    /// <p>The endpoint that was created.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_event_subscription]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventSubscriptionMessage {
    /// <p> A Boolean value; set to <code>true</code> to activate the subscription, or set to <code>false</code> to create the subscription but not activate it. </p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A list of event categories for a source type that you want to subscribe to. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it. </p>
    #[serde(rename = "SnsTopicArn")]
    pub sns_topic_arn: String,
    /// <p>A list of identifiers for which AWS DMS provides notification events.</p> <p>If you don't specify a value, notifications are provided for all sources.</p> <p>If you specify multiple values, they must be of the same type. For example, if you specify a database instance ID, then all of the other values must be database instance IDs.</p>
    #[serde(rename = "SourceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates the events. For example, if you want to be notified of events generated by a replication instance, you set this parameter to <code>replication-instance</code>. If this value isn't specified, all events are returned. </p> <p>Valid values: <code>replication-instance</code> | <code>replication-task</code> </p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The name of the AWS DMS event notification subscription. This name must be less than 255 characters.</p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
    /// <p>One or more tags to be assigned to the event subscription.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_event_subscription]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateEventSubscriptionResponse {
    /// <p>The event subscription that was created.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_instance]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReplicationInstanceMessage {
    /// <p>The amount of storage (in gigabytes) to be initially allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>A value that indicates whether minor engine upgrades are applied automatically to the replication instance during the maintenance window. This parameter defaults to <code>true</code>.</p> <p>Default: <code>true</code> </p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Availability Zone where the replication instance will be created. The default value is a random, system-chosen Availability Zone in the endpoint's AWS Region, for example: <code>us-east-1d</code> </p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>A list of custom DNS name servers supported for the replication instance to access your on-premise source or target database. This list overrides the default name servers supported by the replication instance. You can specify a comma-separated list of internet addresses for up to four on-premise DNS name servers. For example: <code>"1.1.1.1,2.2.2.2,3.3.3.3,4.4.4.4"</code> </p>
    #[serde(rename = "DnsNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    /// <p>The engine version number of the replication instance.</p> <p>If an engine version number is not specified when a replication instance is created, the default is the latest engine version available.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>An AWS KMS key identifier that is used to encrypt the data on the replication instance.</p> <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key.</p> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The weekly time range during which system maintenance can occur, in Universal Coordinated Time (UTC).</p> <p> Format: <code>ddd:hh24:mi-ddd:hh24:mi</code> </p> <p>Default: A 30-minute window selected at random from an 8-hour block of time per AWS Region, occurring on a random day of the week.</p> <p>Valid Days: Mon, Tue, Wed, Thu, Fri, Sat, Sun</p> <p>Constraints: Minimum 30-minute window.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p> <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.html#CHAP_ReplicationInstance.InDepth"> Selecting the right AWS DMS replication instance for your migration</a>. </p>
    #[serde(rename = "ReplicationInstanceClass")]
    pub replication_instance_class: String,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain 1-63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Can't end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    pub replication_instance_identifier: String,
    /// <p>A subnet group to associate with the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, AWS DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    /// <p>One or more tags to be assigned to the replication instance.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_instance]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateReplicationInstanceResponse {
    /// <p>The replication instance that was created.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_subnet_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReplicationSubnetGroupMessage {
    /// <p>The description for the subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    pub replication_subnet_group_description: String,
    /// <p>The name for the replication subnet group. This value is stored as a lowercase string.</p> <p>Constraints: Must contain no more than 255 alphanumeric characters, periods, spaces, underscores, or hyphens. Must not be "default".</p> <p>Example: <code>mySubnetgroup</code> </p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// <p>One or more subnet IDs to be assigned to the subnet group.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>One or more tags to be assigned to the subnet group.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_subnet_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateReplicationSubnetGroupResponse {
    /// <p>The replication subnet group that was created.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateReplicationTaskMessage {
    /// <p><p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position &quot;checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93&quot;</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p> <note> <p>When you use this task setting with a source PostgreSQL database, a logical replication slot should already be created and associated with the source endpoint. You can verify this by setting the <code>slotName</code> extra connection attribute to the name of this logical replication slot. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra Connection Attributes When Using PostgreSQL as a Source for AWS DMS</a>.</p> </note></p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 2018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The migration type. Valid values: <code>full-load</code> | <code>cdc</code> | <code>full-load-and-cdc</code> </p>
    #[serde(rename = "MigrationType")]
    pub migration_type: String,
    /// <p>The Amazon Resource Name (ARN) of a replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// <p><p>An identifier for the replication task.</p> <p>Constraints:</p> <ul> <li> <p>Must contain 1-255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    pub replication_task_identifier: String,
    /// <p>Overall settings for the task, in JSON format. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TaskSettings.html">Specifying Task Settings for AWS Database Migration Service Tasks</a> in the <i>AWS Database Migration User Guide.</i> </p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>A friendly name for the resource identifier at the end of the <code>EndpointArn</code> response parameter that is returned in the created <code>Endpoint</code> object. The value for this parameter can have up to 31 characters. It can contain only ASCII letters, digits, and hyphen ('-'). Also, it can't end with a hyphen or contain two consecutive hyphens, and can only begin with a letter, such as <code>Example-App-ARN1</code>. For example, this value might result in the <code>EndpointArn</code> value <code>arn:aws:dms:eu-west-1:012345678901:rep:Example-App-ARN1</code>. If you don't specify a <code>ResourceIdentifier</code> value, AWS DMS generates a default identifier value for the end of <code>EndpointArn</code>.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies the source endpoint.</p>
    #[serde(rename = "SourceEndpointArn")]
    pub source_endpoint_arn: String,
    /// <p>The table mappings for the task, in JSON format. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.CustomizingTasks.TableMapping.html">Using Table Mapping to Specify Task Settings</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "TableMappings")]
    pub table_mappings: String,
    /// <p>One or more tags to be assigned to the replication task.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>An Amazon Resource Name (ARN) that uniquely identifies the target endpoint.</p>
    #[serde(rename = "TargetEndpointArn")]
    pub target_endpoint_arn: String,
    /// <p>Supplemental information that the task requires to migrate the data for certain source and target endpoints. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">Specifying Supplemental Data for Task Settings</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "TaskData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::create_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateReplicationTaskResponse {
    /// <p>The replication task that was created.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// see [DatabaseMigrationService::delete_certificate]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCertificateMessage {
    /// <p>The Amazon Resource Name (ARN) of the deleted certificate.</p>
    #[serde(rename = "CertificateArn")]
    pub certificate_arn: String,
}

/// see [DatabaseMigrationService::delete_certificate]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCertificateResponse {
    /// <p>The Secure Sockets Layer (SSL) certificate.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_connection]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteConnectionMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_connection]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteConnectionResponse {
    /// <p>The connection that is being deleted.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_endpoint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_endpoint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEndpointResponse {
    /// <p>The endpoint that was deleted.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_event_subscription]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventSubscriptionMessage {
    /// <p>The name of the DMS event notification subscription to be deleted.</p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_event_subscription]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteEventSubscriptionResponse {
    /// <p>The event subscription that was deleted.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_instance]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationInstanceMessage {
    /// <p>The Amazon Resource Name (ARN) of the replication instance to be deleted.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_instance]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReplicationInstanceResponse {
    /// <p>The replication instance that was deleted.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_subnet_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationSubnetGroupMessage {
    /// <p>The subnet group name of the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_subnet_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReplicationSubnetGroupResponse {}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_task_assessment_run]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationTaskAssessmentRunMessage {
    /// <p>Amazon Resource Name (ARN) of the premigration assessment run to be deleted.</p>
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    pub replication_task_assessment_run_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_task_assessment_run]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReplicationTaskAssessmentRunResponse {
    /// <p>The <code>ReplicationTaskAssessmentRun</code> object for the deleted assessment run.</p>
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteReplicationTaskMessage {
    /// <p>The Amazon Resource Name (ARN) of the replication task to be deleted.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::delete_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteReplicationTaskResponse {
    /// <p>The deleted replication task.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_account_attributes]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAccountAttributesMessage {}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_account_attributes]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAccountAttributesResponse {
    /// <p>Account quota information.</p>
    #[serde(rename = "AccountQuotas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_quotas: Option<Vec<AccountQuota>>,
    /// <p><p>A unique AWS DMS identifier for an account in a particular AWS Region. The value of this identifier has the following format: <code>c99999999999</code>. DMS uses this identifier to name artifacts. For example, DMS uses this identifier to name the default Amazon S3 bucket for storing task assessment reports in a given AWS Region. The format of this S3 bucket name is the following: <code>dms-<i>AccountNumber</i>-<i>UniqueAccountIdentifier</i>.</code> Here is an example name for this default S3 bucket: <code>dms-111122223333-c44445555666</code>.</p> <note> <p>AWS DMS supports the <code>UniqueAccountIdentifier</code> parameter in versions 3.1.4 and later.</p> </note></p>
    #[serde(rename = "UniqueAccountIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_account_identifier: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_applicable_individual_assessments]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeApplicableIndividualAssessmentsMessage {
    /// <p>Optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>Name of the migration type that each provided individual assessment must support.</p>
    #[serde(rename = "MigrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// <p>ARN of a replication instance on which you want to base the default list of individual assessments.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>Amazon Resource Name (ARN) of a migration task on which you want to base the default list of individual assessments.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>Name of a database engine that the specified replication instance supports as a source.</p>
    #[serde(rename = "SourceEngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_engine_name: Option<String>,
    /// <p>Name of a database engine that the specified replication instance supports as a target.</p>
    #[serde(rename = "TargetEngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_engine_name: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_applicable_individual_assessments]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeApplicableIndividualAssessmentsResponse {
    /// <p>List of names for the individual assessments supported by the premigration assessment run that you start based on the specified request parameters. For more information on the available individual assessments, including compatibility with different migration task configurations, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.AssessmentReport.html">Working with premigration assessment runs</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "IndividualAssessmentNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_names: Option<Vec<String>>,
    /// <p>Pagination token returned for you to pass to a subsequent request. If you pass this token as the <code>Marker</code> value in a subsequent request, the response includes only records beyond the marker, up to the value specified in the request by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

/// see [DatabaseMigrationService::describe_certificates]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeCertificatesMessage {
    /// <p>Filters applied to the certificates described in the form of key-value pairs.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 10</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeCertificatesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// see [DatabaseMigrationService::describe_certificates]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeCertificatesResponse {
    /// <p>The Secure Sockets Layer (SSL) certificates associated with the replication instance.</p>
    #[serde(rename = "Certificates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<Vec<Certificate>>,
    /// <p>The pagination token.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeCertificatesResponse {
    fn pagination_page_opt(self) -> Option<Vec<Certificate>> {
        Some(self.certificates.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeCertificatesResponse {
    type Item = Certificate;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Certificate> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_connections]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConnectionsMessage {
    /// <p>The filters applied to the connection.</p> <p>Valid filter names: endpoint-arn | replication-instance-arn</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeConnectionsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_connections]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConnectionsResponse {
    /// <p>A description of the connections.</p>
    #[serde(rename = "Connections")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeConnectionsResponse {
    fn pagination_page_opt(self) -> Option<Vec<Connection>> {
        Some(self.connections.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeConnectionsResponse {
    type Item = Connection;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Connection> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_endpoint_types]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointTypesMessage {
    /// <p>Filters applied to the endpoint types.</p> <p>Valid filter names: engine-name | endpoint-type</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeEndpointTypesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_endpoint_types]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointTypesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The types of endpoints that are supported.</p>
    #[serde(rename = "SupportedEndpointTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_endpoint_types: Option<Vec<SupportedEndpointType>>,
}

impl DescribeEndpointTypesResponse {
    fn pagination_page_opt(self) -> Option<Vec<SupportedEndpointType>> {
        Some(self.supported_endpoint_types.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeEndpointTypesResponse {
    type Item = SupportedEndpointType;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<SupportedEndpointType> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_endpoints]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEndpointsMessage {
    /// <p>Filters applied to the endpoints.</p> <p>Valid filter names: endpoint-arn | endpoint-type | endpoint-id | engine-name</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeEndpointsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_endpoints]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEndpointsResponse {
    /// <p>Endpoint description.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeEndpointsResponse {
    fn pagination_page_opt(self) -> Option<Vec<Endpoint>> {
        Some(self.endpoints.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeEndpointsResponse {
    type Item = Endpoint;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<Endpoint> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_event_categories]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventCategoriesMessage {
    /// <p>Filters applied to the event categories.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_event_categories]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventCategoriesResponse {
    /// <p>A list of event categories.</p>
    #[serde(rename = "EventCategoryGroupList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_category_group_list: Option<Vec<EventCategoryGroup>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_event_subscriptions]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventSubscriptionsMessage {
    /// <p>Filters applied to event subscriptions.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The name of the AWS DMS event subscription to be described.</p>
    #[serde(rename = "SubscriptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

impl PagedRequest for DescribeEventSubscriptionsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_event_subscriptions]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventSubscriptionsResponse {
    /// <p>A list of event subscriptions.</p>
    #[serde(rename = "EventSubscriptionsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions_list: Option<Vec<EventSubscription>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeEventSubscriptionsResponse {
    fn pagination_page_opt(self) -> Option<Vec<EventSubscription>> {
        Some(self.event_subscriptions_list.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeEventSubscriptionsResponse {
    type Item = EventSubscription;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<EventSubscription> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_events]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeEventsMessage {
    /// <p>The duration of the events to be listed.</p>
    #[serde(rename = "Duration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// <p>The end time for the events to be listed.</p>
    #[serde(rename = "EndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>A list of event categories for the source type that you've chosen.</p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>Filters applied to events.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p> The identifier of an event source.</p>
    #[serde(rename = "SourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// <p>The type of AWS DMS resource that generates events.</p> <p>Valid values: replication-instance | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The start time for the events to be listed.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

impl PagedRequest for DescribeEventsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_events]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeEventsResponse {
    /// <p>The events described.</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<Event>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

impl DescribeEventsResponse {
    fn pagination_page_opt(self) -> Option<Vec<Event>> {
        Some(self.events.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeEventsResponse {
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

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_orderable_replication_instances]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeOrderableReplicationInstancesMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeOrderableReplicationInstancesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_orderable_replication_instances]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeOrderableReplicationInstancesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The order-able replication instances available.</p>
    #[serde(rename = "OrderableReplicationInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_replication_instances: Option<Vec<OrderableReplicationInstance>>,
}

impl DescribeOrderableReplicationInstancesResponse {
    fn pagination_page_opt(self) -> Option<Vec<OrderableReplicationInstance>> {
        Some(self.orderable_replication_instances.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeOrderableReplicationInstancesResponse {
    type Item = OrderableReplicationInstance;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<OrderableReplicationInstance> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_pending_maintenance_actions]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribePendingMaintenanceActionsMessage {
    /// <p><p/></p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_pending_maintenance_actions]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribePendingMaintenanceActionsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The pending maintenance action.</p>
    #[serde(rename = "PendingMaintenanceActions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<Vec<ResourcePendingMaintenanceActions>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_refresh_schemas_status]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeRefreshSchemasStatusMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_refresh_schemas_status]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeRefreshSchemasStatusResponse {
    /// <p>The status of the schema.</p>
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

/// see [DatabaseMigrationService::describe_replication_instance_task_logs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationInstanceTaskLogsMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// see [DatabaseMigrationService::describe_replication_instance_task_logs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationInstanceTaskLogsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>An array of replication task log metadata. Each member of the array contains the replication task name, ARN, and task log size (in bytes). </p>
    #[serde(rename = "ReplicationInstanceTaskLogs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_logs: Option<Vec<ReplicationInstanceTaskLog>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_instances]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationInstancesMessage {
    /// <p>Filters applied to replication instances.</p> <p>Valid filter names: replication-instance-arn | replication-instance-id | replication-instance-class | engine-version</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeReplicationInstancesMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_instances]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationInstancesResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The replication instances described.</p>
    #[serde(rename = "ReplicationInstances")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instances: Option<Vec<ReplicationInstance>>,
}

impl DescribeReplicationInstancesResponse {
    fn pagination_page_opt(self) -> Option<Vec<ReplicationInstance>> {
        Some(self.replication_instances.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeReplicationInstancesResponse {
    type Item = ReplicationInstance;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReplicationInstance> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_subnet_groups]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationSubnetGroupsMessage {
    /// <p>Filters applied to replication subnet groups.</p> <p>Valid filter names: replication-subnet-group-id</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeReplicationSubnetGroupsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_subnet_groups]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationSubnetGroupsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>A description of the replication subnet groups.</p>
    #[serde(rename = "ReplicationSubnetGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_groups: Option<Vec<ReplicationSubnetGroup>>,
}

impl DescribeReplicationSubnetGroupsResponse {
    fn pagination_page_opt(self) -> Option<Vec<ReplicationSubnetGroup>> {
        Some(self.replication_subnet_groups.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeReplicationSubnetGroupsResponse {
    type Item = ReplicationSubnetGroup;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReplicationSubnetGroup> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_assessment_results]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationTaskAssessmentResultsMessage {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the task. When this input parameter is specified, the API returns only one result and ignore the values of the <code>MaxRecords</code> and <code>Marker</code> parameters. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

impl PagedRequest for DescribeReplicationTaskAssessmentResultsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_assessment_results]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationTaskAssessmentResultsResponse {
    /// <p>- The Amazon S3 bucket where the task assessment report is located. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The task assessment report. </p>
    #[serde(rename = "ReplicationTaskAssessmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_results: Option<Vec<ReplicationTaskAssessmentResult>>,
}

impl DescribeReplicationTaskAssessmentResultsResponse {
    fn pagination_page_opt(self) -> Option<Vec<ReplicationTaskAssessmentResult>> {
        Some(self.replication_task_assessment_results.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeReplicationTaskAssessmentResultsResponse {
    type Item = ReplicationTaskAssessmentResult;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReplicationTaskAssessmentResult> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_assessment_runs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationTaskAssessmentRunsMessage {
    /// <p>Filters applied to the premigration assessment runs described in the form of key-value pairs.</p> <p>Valid filter names: <code>replication-task-assessment-run-arn</code>, <code>replication-task-arn</code>, <code>replication-instance-arn</code>, <code>status</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_assessment_runs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationTaskAssessmentRunsResponse {
    /// <p>A pagination token returned for you to pass to a subsequent request. If you pass this token as the <code>Marker</code> value in a subsequent request, the response includes only records beyond the marker, up to the value specified in the request by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>One or more premigration assessment runs as specified by <code>Filters</code>.</p>
    #[serde(rename = "ReplicationTaskAssessmentRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_runs: Option<Vec<ReplicationTaskAssessmentRun>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_individual_assessments]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationTaskIndividualAssessmentsMessage {
    /// <p>Filters applied to the individual assessments described in the form of key-value pairs.</p> <p>Valid filter names: <code>replication-task-assessment-run-arn</code>, <code>replication-task-arn</code>, <code>status</code> </p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_task_individual_assessments]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationTaskIndividualAssessmentsResponse {
    /// <p>A pagination token returned for you to pass to a subsequent request. If you pass this token as the <code>Marker</code> value in a subsequent request, the response includes only records beyond the marker, up to the value specified in the request by <code>MaxRecords</code>.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>One or more individual assessments as specified by <code>Filters</code>.</p>
    #[serde(rename = "ReplicationTaskIndividualAssessments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessments: Option<Vec<ReplicationTaskIndividualAssessment>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_tasks]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeReplicationTasksMessage {
    /// <p>Filters applied to replication tasks.</p> <p>Valid filter names: replication-task-arn | replication-task-id | migration-type | endpoint-arn | replication-instance-arn</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>An option to set to avoid returning information about settings. Use this to reduce overhead when setting information is too large. To use this option, choose <code>true</code>; otherwise, choose <code>false</code> (the default).</p>
    #[serde(rename = "WithoutSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_settings: Option<bool>,
}

impl PagedRequest for DescribeReplicationTasksMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_replication_tasks]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeReplicationTasksResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>A description of the replication tasks.</p>
    #[serde(rename = "ReplicationTasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_tasks: Option<Vec<ReplicationTask>>,
}

impl DescribeReplicationTasksResponse {
    fn pagination_page_opt(self) -> Option<Vec<ReplicationTask>> {
        Some(self.replication_tasks.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeReplicationTasksResponse {
    type Item = ReplicationTask;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<ReplicationTask> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_schemas]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeSchemasMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 100.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
}

impl PagedRequest for DescribeSchemasMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_schemas]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeSchemasResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The described schema.</p>
    #[serde(rename = "Schemas")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schemas: Option<Vec<String>>,
}

impl DescribeSchemasResponse {
    fn pagination_page_opt(self) -> Option<Vec<String>> {
        Some(self.schemas.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeSchemasResponse {
    type Item = String;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<String> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_table_statistics]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTableStatisticsMessage {
    /// <p>Filters applied to table statistics.</p> <p>Valid filter names: schema-name | table-name | table-state</p> <p>A combination of filters creates an AND condition where each record matches all specified filters.</p>
    #[serde(rename = "Filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p> The maximum number of records to include in the response. If more records exist than the specified <code>MaxRecords</code> value, a pagination token called a marker is included in the response so that the remaining results can be retrieved. </p> <p>Default: 100</p> <p>Constraints: Minimum 20, maximum 500.</p>
    #[serde(rename = "MaxRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

impl PagedRequest for DescribeTableStatisticsMessage {
    type Token = Option<String>;
    fn with_pagination_token(mut self, key: Option<String>) -> Self {
        self.marker = key;
        self
    }
}

/// <p><p/></p>
/// see [DatabaseMigrationService::describe_table_statistics]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTableStatisticsResponse {
    /// <p> An optional pagination token provided by a previous request. If this parameter is specified, the response includes only records beyond the marker, up to the value specified by <code>MaxRecords</code>. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The table statistics.</p>
    #[serde(rename = "TableStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_statistics: Option<Vec<TableStatistics>>,
}

impl DescribeTableStatisticsResponse {
    fn pagination_page_opt(self) -> Option<Vec<TableStatistics>> {
        Some(self.table_statistics.as_ref()?.clone())
    }
}

impl PagedOutput for DescribeTableStatisticsResponse {
    type Item = TableStatistics;
    type Token = Option<String>;
    fn pagination_token(&self) -> Option<String> {
        Some(self.marker.as_ref()?.clone())
    }

    fn into_pagination_page(self) -> Vec<TableStatistics> {
        self.pagination_page_opt().unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        {
            self.pagination_token().is_some()
        }
    }
}

/// <p> The settings in JSON format for the DMS Transfer type source endpoint. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DmsTransferSettings {
    /// <p> The name of the S3 bucket to use. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p> The IAM role that has permission to access the Amazon S3 bucket. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

/// <p>Provides information that defines a DocumentDB endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DocDbSettings {
    /// <p> The database name on the DocumentDB source endpoint. </p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p> Indicates the number of documents to preview to determine the document organization. Use this setting when <code>NestingLevel</code> is set to <code>"one"</code>. </p> <p>Must be a positive value greater than <code>0</code>. Default value is <code>1000</code>.</p>
    #[serde(rename = "DocsToInvestigate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<i64>,
    /// <p> Specifies the document ID. Use this setting when <code>NestingLevel</code> is set to <code>"none"</code>. </p> <p>Default value is <code>"false"</code>. </p>
    #[serde(rename = "ExtractDocId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<bool>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies either document or table mode. </p> <p>Default value is <code>"none"</code>. Specify <code>"none"</code> to use document mode. Specify <code>"one"</code> to use table mode.</p>
    #[serde(rename = "NestingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<String>,
    /// <p> The password for the user account you use to access the DocumentDB source endpoint. </p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p> The port value for the DocumentDB source endpoint. </p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the DocumentDB endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the DocumentDB endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p> The name of the server on the DocumentDB source endpoint. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The user name you use to access the DocumentDB source endpoint. </p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Provides the Amazon Resource Name (ARN) of the AWS Identity and Access Management (IAM) role used to define an Amazon DynamoDB target endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DynamoDbSettings {
    /// <p> The Amazon Resource Name (ARN) used by the service access IAM role. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

/// <p>Provides information that defines an Elasticsearch endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ElasticsearchSettings {
    /// <p>The endpoint for the Elasticsearch cluster. AWS DMS uses HTTPS if a transport protocol (http/https) is not specified.</p>
    #[serde(rename = "EndpointUri")]
    pub endpoint_uri: String,
    /// <p>The maximum number of seconds for which DMS retries failed API requests to the Elasticsearch cluster.</p>
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i64>,
    /// <p>The maximum percentage of records that can fail to be written before a full load operation stops.</p> <p>To avoid early failure, this counter is only effective after 1000 records are transferred. Elasticsearch also has the concept of error monitoring during the last 10 minutes of an Observation Window. If transfer of all records fail in the last 10 minutes, the full load operation stops. </p>
    #[serde(rename = "FullLoadErrorPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_percentage: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) used by service to access the IAM role.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

/// <p><p>Describes an endpoint of a database instance in response to operations such as the following:</p> <ul> <li> <p> <code>CreateEndpoint</code> </p> </li> <li> <p> <code>DescribeEndpoint</code> </p> </li> <li> <p> <code>DescribeEndpointTypes</code> </p> </li> <li> <p> <code>ModifyEndpoint</code> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>The Amazon Resource Name (ARN) used for SSL connection to the endpoint.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the database at the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Possible settings include the following:</p> <ul> <li> <p> <code>ServiceAccessRoleArn</code> - The IAM role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p> <code>BucketName</code> - The name of the S3 bucket to use.</p> </li> <li> <p> <code>CompressionType</code> - An optional parameter to use GZIP to compress the target files. To use GZIP, set this value to <code>NONE</code> (the default). To keep the files uncompressed, don't use this value.</p> </li> </ul> <p>Shorthand syntax for these settings is as follows: <code>ServiceAccessRoleArn=string,BucketName=string,CompressionType=string</code> </p> <p>JSON syntax for these settings is as follows: <code>{ "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </code> </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    #[serde(rename = "DocDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    /// <p>The settings for the DynamoDB target endpoint. For more information, see the <code>DynamoDBSettings</code> structure.</p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>The settings for the Elasticsearch source endpoint. For more information, see the <code>ElasticsearchSettings</code> structure.</p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The type of endpoint. Valid values are <code>source</code> and <code>target</code>.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The expanded name for the engine name. For example, if the <code>EngineName</code> parameter is "aurora," this value would be "Amazon Aurora MySQL."</p>
    #[serde(rename = "EngineDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    /// <p>The database engine name. Valid values, depending on the EndpointType, include <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>, <code>"aurora-postgresql"</code>, <code>"redshift"</code>, <code>"s3"</code>, <code>"db2"</code>, <code>"azuredb"</code>, <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>, <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>, <code>"documentdb"</code>, <code>"sqlserver"</code>, and <code>"neptune"</code>.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p> Value returned by a call to CreateEndpoint that can be used for cross-account validation. Use it on a subsequent call to CreateEndpoint to create the endpoint with a cross-account. </p>
    #[serde(rename = "ExternalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// <p>The external table definition.</p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional connection attributes used to connect to the endpoint.</p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>The settings for the IBM Db2 LUW source endpoint. For more information, see the <code>IBMDb2Settings</code> structure. </p>
    #[serde(rename = "IBMDb2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db_2_settings: Option<IBMDb2Settings>,
    /// <p>The settings for the Apache Kafka target endpoint. For more information, see the <code>KafkaSettings</code> structure.</p>
    #[serde(rename = "KafkaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    /// <p>The settings for the Amazon Kinesis target endpoint. For more information, see the <code>KinesisSettings</code> structure.</p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>An AWS KMS key identifier that is used to encrypt the connection parameters for the endpoint.</p> <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key.</p> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The settings for the Microsoft SQL Server source and target endpoint. For more information, see the <code>MicrosoftSQLServerSettings</code> structure.</p>
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_sql_server_settings: Option<MicrosoftSQLServerSettings>,
    /// <p>The settings for the MongoDB source endpoint. For more information, see the <code>MongoDbSettings</code> structure.</p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>The settings for the MySQL source and target endpoint. For more information, see the <code>MySQLSettings</code> structure.</p>
    #[serde(rename = "MySQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_settings: Option<MySQLSettings>,
    /// <p>The settings for the Amazon Neptune target endpoint. For more information, see the <code>NeptuneSettings</code> structure.</p>
    #[serde(rename = "NeptuneSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    /// <p>The settings for the Oracle source and target endpoint. For more information, see the <code>OracleSettings</code> structure.</p>
    #[serde(rename = "OracleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    /// <p>The port value used to access the endpoint.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>The settings for the PostgreSQL source and target endpoint. For more information, see the <code>PostgreSQLSettings</code> structure.</p>
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_settings: Option<PostgreSQLSettings>,
    /// <p>Settings for the Amazon Redshift endpoint.</p>
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>The settings for the S3 target endpoint. For more information, see the <code>S3Settings</code> structure.</p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server at the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The Amazon Resource Name (ARN) used by the service access IAM role.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The SSL mode used to connect to the endpoint. The default value is <code>none</code>.</p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>The status of the endpoint.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The settings for the SAP ASE source and target endpoint. For more information, see the <code>SybaseSettings</code> structure.</p>
    #[serde(rename = "SybaseSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    /// <p>The user name used to connect to the endpoint.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Describes an identifiable significant activity that affects a replication instance or task. This object can provide the message, the available event categories, the date and source of the event, and the AWS DMS resource type.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Event {
    /// <p>The date of the event.</p>
    #[serde(rename = "Date")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    /// <p>The event categories available for the specified source type.</p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p>The event message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p> The identifier of an event source.</p>
    #[serde(rename = "SourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | endpoint | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>Lists categories of events subscribed to, and generated by, the applicable AWS DMS resource type. This data type appears in response to the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_EventCategoryGroup.html"> <code>DescribeEventCategories</code> </a> action.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventCategoryGroup {
    /// <p> A list of event categories from a source type that you've chosen.</p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

/// <p>Describes an event notification subscription created by the <code>CreateEventSubscription</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventSubscription {
    /// <p>The AWS DMS event notification subscription Id.</p>
    #[serde(rename = "CustSubscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    /// <p>The AWS customer account associated with the AWS DMS event notification subscription.</p>
    #[serde(rename = "CustomerAwsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    /// <p>Boolean value that indicates if the event subscription is enabled.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>A lists of event categories.</p>
    #[serde(rename = "EventCategoriesList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<Vec<String>>,
    /// <p>The topic ARN of the AWS DMS event notification subscription.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p>A list of source Ids for the event subscription.</p>
    #[serde(rename = "SourceIdsList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<Vec<String>>,
    /// <p> The type of AWS DMS resource that generates events. </p> <p>Valid values: replication-instance | replication-server | security-group | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The status of the AWS DMS event notification subscription.</p> <p>Constraints:</p> <p>Can be one of the following: creating | modifying | deleting | active | no-permission | topic-not-exist</p> <p>The status "no-permission" indicates that AWS DMS no longer has permission to post to the SNS topic. The status "topic-not-exist" indicates that the topic was deleted after the subscription was created.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The time the AWS DMS event notification subscription was created.</p>
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

/// <p>Identifies the name and value of a filter object. This filter is used to limit the number and type of AWS DMS objects that are returned for a particular <code>Describe*</code> call or similar operation. Filters are used as an optional parameter for certain API operations. </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>The name of the filter as specified for a <code>Describe*</code> or similar operation.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The filter value, which can specify one or more values used to narrow the returned results.</p>
    #[serde(rename = "Values")]
    pub values: Vec<String>,
}

/// <p>Provides information that defines an IBM Db2 LUW endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct IBMDb2Settings {
    /// <p>For ongoing replication (CDC), use CurrentLSN to specify a log sequence number (LSN) where you want the replication to start.</p>
    #[serde(rename = "CurrentLsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_lsn: Option<String>,
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Maximum number of bytes per read, as a NUMBER value. The default is 64 KB.</p>
    #[serde(rename = "MaxKBytesPerRead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_k_bytes_per_read: Option<i64>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the Db2 LUW endpoint. </p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the Db2 LUW endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Enables ongoing replication (CDC) as a BOOLEAN value. The default is true.</p>
    #[serde(rename = "SetDataCaptureChanges")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_data_capture_changes: Option<bool>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// see [DatabaseMigrationService::import_certificate]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportCertificateMessage {
    /// <p>A customer-assigned name for the certificate. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "CertificateIdentifier")]
    pub certificate_identifier: String,
    /// <p>The contents of a <code>.pem</code> file, which contains an X.509 certificate.</p>
    #[serde(rename = "CertificatePem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_pem: Option<String>,
    /// <p>The location of an imported Oracle Wallet certificate for use with SSL.</p>
    #[serde(rename = "CertificateWallet")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_wallet: Option<bytes::Bytes>,
    /// <p>The tags associated with the certificate.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// see [DatabaseMigrationService::import_certificate]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportCertificateResponse {
    /// <p>The certificate to be uploaded.</p>
    #[serde(rename = "Certificate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate: Option<Certificate>,
}

/// <p>Provides information that describes an Apache Kafka endpoint. This information includes the output format of records applied to the endpoint and details of transaction and control table data information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KafkaSettings {
    /// <p>The broker location and port of the Kafka broker that hosts your Kafka instance. Specify the broker in the form <code> <i>broker-hostname-or-ip</i>:<i>port</i> </code>. For example, <code>"ec2-12-345-678-901.compute-1.amazonaws.com:2345"</code>.</p>
    #[serde(rename = "Broker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker: Option<String>,
    /// <p>Shows detailed control information for table definition, column definition, and table and column changes in the Kafka message output. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeControlDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,
    /// <p>Include NULL and empty columns for records migrated to the endpoint. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,
    /// <p>Shows the partition value within the Kafka message output, unless the partition type is <code>schema-table-type</code>. The default is <code>false</code>.</p>
    #[serde(rename = "IncludePartitionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,
    /// <p>Includes any data definition language (DDL) operations that change the table in the control data, such as <code>rename-table</code>, <code>drop-table</code>, <code>add-column</code>, <code>drop-column</code>, and <code>rename-column</code>. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,
    /// <p>Provides detailed transaction information from the source database. This information includes a commit timestamp, a log position, and values for <code>transaction_id</code>, previous <code>transaction_id</code>, and <code>transaction_record_id</code> (the record offset within a transaction). The default is <code>false</code>.</p>
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,
    /// <p>The output format for the records created on the endpoint. The message format is <code>JSON</code> (default) or <code>JSON_UNFORMATTED</code> (a single line with no tab).</p>
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>The maximum size in bytes for records created on the endpoint The default is 1,000,000.</p>
    #[serde(rename = "MessageMaxBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_max_bytes: Option<i64>,
    /// <p>Prefixes schema and table names to partition values, when the partition type is <code>primary-key-type</code>. Doing this increases data distribution among Kafka partitions. For example, suppose that a SysBench schema has thousands of tables and each table has only limited range for a primary key. In this case, the same primary key is sent from thousands of tables to the same partition, which causes throttling. The default is <code>false</code>.</p>
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,
    /// <p>The topic to which you migrate the data. If you don't specify a topic, AWS DMS specifies <code>"kafka-default-topic"</code> as the migration topic.</p>
    #[serde(rename = "Topic")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
}

/// <p>Provides information that describes an Amazon Kinesis Data Stream endpoint. This information includes the output format of records applied to the endpoint and details of transaction and control table data information.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KinesisSettings {
    /// <p>Shows detailed control information for table definition, column definition, and table and column changes in the Kinesis message output. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeControlDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_control_details: Option<bool>,
    /// <p>Include NULL and empty columns for records migrated to the endpoint. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeNullAndEmpty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_null_and_empty: Option<bool>,
    /// <p>Shows the partition value within the Kinesis message output, unless the partition type is <code>schema-table-type</code>. The default is <code>false</code>.</p>
    #[serde(rename = "IncludePartitionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_partition_value: Option<bool>,
    /// <p>Includes any data definition language (DDL) operations that change the table in the control data, such as <code>rename-table</code>, <code>drop-table</code>, <code>add-column</code>, <code>drop-column</code>, and <code>rename-column</code>. The default is <code>false</code>.</p>
    #[serde(rename = "IncludeTableAlterOperations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_table_alter_operations: Option<bool>,
    /// <p>Provides detailed transaction information from the source database. This information includes a commit timestamp, a log position, and values for <code>transaction_id</code>, previous <code>transaction_id</code>, and <code>transaction_record_id</code> (the record offset within a transaction). The default is <code>false</code>.</p>
    #[serde(rename = "IncludeTransactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_transaction_details: Option<bool>,
    /// <p>The output format for the records created on the endpoint. The message format is <code>JSON</code> (default) or <code>JSON_UNFORMATTED</code> (a single line with no tab).</p>
    #[serde(rename = "MessageFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_format: Option<String>,
    /// <p>Prefixes schema and table names to partition values, when the partition type is <code>primary-key-type</code>. Doing this increases data distribution among Kinesis shards. For example, suppose that a SysBench schema has thousands of tables and each table has only limited range for a primary key. In this case, the same primary key is sent from thousands of tables to the same shard, which causes throttling. The default is <code>false</code>.</p>
    #[serde(rename = "PartitionIncludeSchemaTable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partition_include_schema_table: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) for the AWS Identity and Access Management (IAM) role that AWS DMS uses to write to the Kinesis data stream.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for the Amazon Kinesis Data Streams endpoint.</p>
    #[serde(rename = "StreamArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_arn: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::list_tags_for_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the AWS DMS resource.</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::list_tags_for_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A list of tags for the resource.</p>
    #[serde(rename = "TagList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

/// <p>Provides information that defines a Microsoft SQL Server endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MicrosoftSQLServerSettings {
    /// <p>The maximum size of the packets (in bytes) used to transfer data using BCP.</p>
    #[serde(rename = "BcpPacketSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcp_packet_size: Option<i64>,
    /// <p>Specifies a file group for the AWS DMS internal tables. When the replication task starts, all the internal AWS DMS control tables (awsdms_ apply_exception, awsdms_apply, awsdms_changes) are created for the specified file group.</p>
    #[serde(rename = "ControlTablesFileGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_tables_file_group: Option<String>,
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>When this attribute is set to <code>Y</code>, AWS DMS only reads changes from transaction log backups and doesn't read from the active transaction log file during ongoing replication. Setting this parameter to <code>Y</code> enables you to control active transaction log file growth during full load and ongoing replication tasks. However, it can add some source latency to ongoing replication.</p>
    #[serde(rename = "ReadBackupOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_backup_only: Option<bool>,
    /// <p>Use this attribute to minimize the need to access the backup log and enable AWS DMS to prevent truncation using one of the following two methods.</p> <p> <i>Start transactions in the database:</i> This is the default method. When this method is used, AWS DMS prevents TLOG truncation by mimicking a transaction in the database. As long as such a transaction is open, changes that appear after the transaction started aren't truncated. If you need Microsoft Replication to be enabled in your database, then you must choose this method.</p> <p> <i>Exclusively use sp_repldone within a single task</i>: When this method is used, AWS DMS reads the changes and then uses sp_repldone to mark the TLOG transactions as ready for truncation. Although this method doesn't involve any transactional activities, it can only be used when Microsoft Replication isn't running. Also, when using this method, only one AWS DMS task can access the database at any given time. Therefore, if you need to run parallel AWS DMS tasks against the same database, use the default method.</p>
    #[serde(rename = "SafeguardPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safeguard_policy: Option<String>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the SQL Server endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the SQL Server endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Use this to attribute to transfer data for full-load operations using BCP. When the target table contains an identity column that does not exist in the source table, you must disable the use BCP for loading table option.</p>
    #[serde(rename = "UseBcpFullLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_bcp_full_load: Option<bool>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_endpoint]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyEndpointMessage {
    /// <p>The Amazon Resource Name (ARN) of the certificate used for SSL connection.</p>
    #[serde(rename = "CertificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The name of the endpoint database.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The settings in JSON format for the DMS transfer type of source endpoint. </p> <p>Attributes include the following:</p> <ul> <li> <p>serviceAccessRoleArn - The AWS Identity and Access Management (IAM) role that has permission to access the Amazon S3 bucket.</p> </li> <li> <p>BucketName - The name of the S3 bucket to use.</p> </li> <li> <p>compressionType - An optional parameter to use GZIP to compress the target files. Either set this parameter to NONE (the default) or don't use it to leave the files uncompressed.</p> </li> </ul> <p>Shorthand syntax for these settings is as follows: <code>ServiceAccessRoleArn=string ,BucketName=string,CompressionType=string</code> </p> <p>JSON syntax for these settings is as follows: <code>{ "ServiceAccessRoleArn": "string", "BucketName": "string", "CompressionType": "none"|"gzip" } </code> </p>
    #[serde(rename = "DmsTransferSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dms_transfer_settings: Option<DmsTransferSettings>,
    /// <p>Settings in JSON format for the source DocumentDB endpoint. For more information about the available settings, see the configuration properties section in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.DocumentDB.html"> Using DocumentDB as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "DocDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doc_db_settings: Option<DocDbSettings>,
    /// <p>Settings in JSON format for the target Amazon DynamoDB endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.DynamoDB.html">Using Object Mapping to Migrate Data to DynamoDB</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "DynamoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamo_db_settings: Option<DynamoDbSettings>,
    /// <p>Settings in JSON format for the target Elasticsearch endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Elasticsearch.html#CHAP_Target.Elasticsearch.Configuration">Extra Connection Attributes When Using Elasticsearch as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "ElasticsearchSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_settings: Option<ElasticsearchSettings>,
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The database endpoint identifier. Identifiers must begin with a letter and must contain only ASCII letters, digits, and hyphens. They can't end with a hyphen or contain two consecutive hyphens.</p>
    #[serde(rename = "EndpointIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_identifier: Option<String>,
    /// <p>The type of endpoint. Valid values are <code>source</code> and <code>target</code>.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The type of engine for the endpoint. Valid values, depending on the EndpointType, include <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>, <code>"aurora-postgresql"</code>, <code>"redshift"</code>, <code>"s3"</code>, <code>"db2"</code>, <code>"azuredb"</code>, <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>, <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>, <code>"documentdb"</code>, <code>"sqlserver"</code>, and <code>"neptune"</code>.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p>The external table definition.</p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p>Additional attributes associated with the connection. To reset this parameter, pass the empty string ("") as an argument.</p>
    #[serde(rename = "ExtraConnectionAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_connection_attributes: Option<String>,
    /// <p>Settings in JSON format for the source IBM Db2 LUW endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.DB2.ConnectionAttrib">Extra connection attributes when using Db2 LUW as a source for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "IBMDb2Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ibm_db_2_settings: Option<IBMDb2Settings>,
    /// <p>Settings in JSON format for the target Apache Kafka endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kafka.html">Using Apache Kafka as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "KafkaSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kafka_settings: Option<KafkaSettings>,
    /// <p>Settings in JSON format for the target endpoint for Amazon Kinesis Data Streams. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Kinesis.html">Using Amazon Kinesis Data Streams as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "KinesisSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_settings: Option<KinesisSettings>,
    /// <p>Settings in JSON format for the source and target Microsoft SQL Server endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SQLServer.ConnectionAttrib">Extra connection attributes when using SQL Server as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SQLServer.ConnectionAttrib"> Extra connection attributes when using SQL Server as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MicrosoftSQLServerSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub microsoft_sql_server_settings: Option<MicrosoftSQLServerSettings>,
    /// <p>Settings in JSON format for the source MongoDB endpoint. For more information about the available settings, see the configuration properties section in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MongoDB.html"> Using MongoDB as a Target for AWS Database Migration Service</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MongoDbSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mongo_db_settings: Option<MongoDbSettings>,
    /// <p>Settings in JSON format for the source and target MySQL endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.MySQL.ConnectionAttrib">Extra connection attributes when using MySQL as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.MySQL.ConnectionAttrib">Extra connection attributes when using a MySQL-compatible database as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "MySQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_sql_settings: Option<MySQLSettings>,
    /// <p>Settings in JSON format for the target Amazon Neptune endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Neptune.html#CHAP_Target.Neptune.EndpointSettings">Specifying Endpoint Settings for Amazon Neptune as a Target</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "NeptuneSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neptune_settings: Option<NeptuneSettings>,
    /// <p>Settings in JSON format for the source and target Oracle endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.ConnectionAttrib">Extra connection attributes when using Oracle as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Oracle.ConnectionAttrib"> Extra connection attributes when using Oracle as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "OracleSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_settings: Option<OracleSettings>,
    /// <p>The password to be used to login to the endpoint database.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port used by the endpoint database.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Settings in JSON format for the source and target PostgreSQL endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.ConnectionAttrib">Extra connection attributes when using PostgreSQL as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.PostgreSQL.ConnectionAttrib"> Extra connection attributes when using PostgreSQL as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "PostgreSQLSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postgre_sql_settings: Option<PostgreSQLSettings>,
    #[serde(rename = "RedshiftSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_settings: Option<RedshiftSettings>,
    /// <p>Settings in JSON format for the target Amazon S3 endpoint. For more information about the available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring">Extra Connection Attributes When Using Amazon S3 as a Target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "S3Settings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_settings: Option<S3Settings>,
    /// <p>The name of the server where the endpoint database resides.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p> The Amazon Resource Name (ARN) for the service access role you want to use to modify the endpoint. </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The SSL mode used to connect to the endpoint. The default value is <code>none</code>.</p>
    #[serde(rename = "SslMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_mode: Option<String>,
    /// <p>Settings in JSON format for the source and target SAP ASE endpoint. For information about other available settings, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.SAP.ConnectionAttrib">Extra connection attributes when using SAP ASE as a source for AWS DMS</a> and <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.SAP.ConnectionAttrib">Extra connection attributes when using SAP ASE as a target for AWS DMS</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "SybaseSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sybase_settings: Option<SybaseSettings>,
    /// <p>The user name to be used to login to the endpoint database.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_endpoint]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyEndpointResponse {
    /// <p>The modified endpoint.</p>
    #[serde(rename = "Endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_event_subscription]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyEventSubscriptionMessage {
    /// <p> A Boolean value; set to <b>true</b> to activate the subscription. </p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p> A list of event categories for a source type that you want to subscribe to. Use the <code>DescribeEventCategories</code> action to see a list of event categories. </p>
    #[serde(rename = "EventCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<Vec<String>>,
    /// <p> The Amazon Resource Name (ARN) of the Amazon SNS topic created for event notification. The ARN is created by Amazon SNS when you create a topic and subscribe to it.</p>
    #[serde(rename = "SnsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    /// <p> The type of AWS DMS resource that generates the events you want to subscribe to. </p> <p>Valid values: replication-instance | replication-task</p>
    #[serde(rename = "SourceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    /// <p>The name of the AWS DMS event notification subscription to be modified.</p>
    #[serde(rename = "SubscriptionName")]
    pub subscription_name: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_event_subscription]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyEventSubscriptionResponse {
    /// <p>The modified event subscription.</p>
    #[serde(rename = "EventSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_instance]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReplicationInstanceMessage {
    /// <p>The amount of storage (in gigabytes) to be allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Indicates that major version upgrades are allowed. Changing this parameter does not result in an outage, and the change is asynchronously applied as soon as possible.</p> <p>This parameter must be set to <code>true</code> when specifying a value for the <code>EngineVersion</code> parameter that is a different major version than the replication instance's current version.</p>
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    /// <p>Indicates whether the changes should be applied immediately or during the next maintenance window.</p>
    #[serde(rename = "ApplyImmediately")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    /// <p><p>A value that indicates that minor version upgrades are applied automatically to the replication instance during the maintenance window. Changing this parameter doesn&#39;t result in an outage, except in the case described following. The change is asynchronously applied as soon as possible. </p> <p>An outage does result if these factors apply: </p> <ul> <li> <p>This parameter is set to <code>true</code> during the maintenance window.</p> </li> <li> <p>A newer minor version is available. </p> </li> <li> <p>AWS DMS has enabled automatic patching for the given engine version. </p> </li> </ul></p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The engine version number of the replication instance.</p> <p>When modifying a major engine version of an instance, also set <code>AllowMajorVersionUpgrade</code> to <code>true</code>.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The weekly time range (in UTC) during which system maintenance can occur, which might result in an outage. Changing this parameter does not result in an outage, except in the following situation, and the change is asynchronously applied as soon as possible. If moving this window to the current time, there must be at least 30 minutes between the current time and end of the window to ensure pending changes are applied.</p> <p>Default: Uses existing setting</p> <p>Format: ddd:hh24:mi-ddd:hh24:mi</p> <p>Valid Days: Mon | Tue | Wed | Thu | Fri | Sat | Sun</p> <p>Constraints: Must be at least 30 minutes</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p> <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.html#CHAP_ReplicationInstance.InDepth"> Selecting the right AWS DMS replication instance for your migration</a>. </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The replication instance identifier. This parameter is stored as a lowercase string.</p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p> Specifies the VPC security group to be used with the replication instance. The VPC security group must work with the VPC containing the replication instance. </p>
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<Vec<String>>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_instance]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyReplicationInstanceResponse {
    /// <p>The modified replication instance.</p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_subnet_group]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReplicationSubnetGroupMessage {
    /// <p>A description for the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    /// <p>The name of the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    pub replication_subnet_group_identifier: String,
    /// <p>A list of subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_subnet_group]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyReplicationSubnetGroupResponse {
    /// <p>The modified replication subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ModifyReplicationTaskMessage {
    /// <p><p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position &quot;checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93&quot;</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p> <note> <p>When you use this task setting with a source PostgreSQL database, a logical replication slot should already be created and associated with the source endpoint. You can verify this by setting the <code>slotName</code> extra connection attribute to the name of this logical replication slot. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra Connection Attributes When Using PostgreSQL as a Source for AWS DMS</a>.</p> </note></p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 2018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The migration type. Valid values: <code>full-load</code> | <code>cdc</code> | <code>full-load-and-cdc</code> </p>
    #[serde(rename = "MigrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p><p>The replication task identifier.</p> <p>Constraints:</p> <ul> <li> <p>Must contain 1-255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>JSON file that contains settings for the task, such as task metadata settings.</p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>When using the AWS CLI or boto3, provide the path of the JSON file that contains the table mappings. Precede the path with <code>file://</code>. When working with the DMS API, provide the JSON as the parameter value, for example: <code>--table-mappings file://mappingfile.json</code> </p>
    #[serde(rename = "TableMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    /// <p>Supplemental information that the task requires to migrate the data for certain source and target endpoints. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">Specifying Supplemental Data for Task Settings</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "TaskData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::modify_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ModifyReplicationTaskResponse {
    /// <p>The replication task that was modified.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p>Provides information that defines a MongoDB endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MongoDbSettings {
    /// <p> The authentication mechanism you use to access the MongoDB source endpoint.</p> <p>For the default value, in MongoDB version 2.x, <code>"default"</code> is <code>"mongodb_cr"</code>. For MongoDB version 3.x or later, <code>"default"</code> is <code>"scram_sha_1"</code>. This setting isn't used when <code>AuthType</code> is set to <code>"no"</code>.</p>
    #[serde(rename = "AuthMechanism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_mechanism: Option<String>,
    /// <p> The MongoDB database name. This setting isn't used when <code>AuthType</code> is set to <code>"no"</code>. </p> <p>The default is <code>"admin"</code>.</p>
    #[serde(rename = "AuthSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_source: Option<String>,
    /// <p> The authentication type you use to access the MongoDB source endpoint.</p> <p>When when set to <code>"no"</code>, user name and password parameters are not used and can be empty. </p>
    #[serde(rename = "AuthType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    /// <p> The database name on the MongoDB source endpoint. </p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p> Indicates the number of documents to preview to determine the document organization. Use this setting when <code>NestingLevel</code> is set to <code>"one"</code>. </p> <p>Must be a positive value greater than <code>0</code>. Default value is <code>1000</code>.</p>
    #[serde(rename = "DocsToInvestigate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub docs_to_investigate: Option<String>,
    /// <p> Specifies the document ID. Use this setting when <code>NestingLevel</code> is set to <code>"none"</code>. </p> <p>Default value is <code>"false"</code>. </p>
    #[serde(rename = "ExtractDocId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extract_doc_id: Option<String>,
    /// <p>The AWS KMS key identifier that is used to encrypt the content on the replication instance. If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key. AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies either document or table mode. </p> <p>Default value is <code>"none"</code>. Specify <code>"none"</code> to use document mode. Specify <code>"one"</code> to use table mode.</p>
    #[serde(rename = "NestingLevel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nesting_level: Option<String>,
    /// <p> The password for the user account you use to access the MongoDB source endpoint. </p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p> The port value for the MongoDB source endpoint. </p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the MongoDB endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the MongoDB endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p> The name of the server on the MongoDB source endpoint. </p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The user name you use to access the MongoDB source endpoint. </p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::move_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct MoveReplicationTaskMessage {
    /// <p>The Amazon Resource Name (ARN) of the task that you want to move.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p>The ARN of the replication instance where you want to move the task to.</p>
    #[serde(rename = "TargetReplicationInstanceArn")]
    pub target_replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::move_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct MoveReplicationTaskResponse {
    /// <p>The replication task that was moved.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p>Provides information that defines a MySQL endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MySQLSettings {
    /// <p>Specifies a script to run immediately after AWS DMS connects to the endpoint. The migration task continues running regardless if the SQL statement succeeds or fails.</p>
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Specifies how often to check the binary log for new changes/events when the database is idle.</p> <p>Example: <code>eventsPollInterval=5;</code> </p> <p>In the example, AWS DMS checks for changes in the binary logs every five seconds.</p>
    #[serde(rename = "EventsPollInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_poll_interval: Option<i64>,
    /// <p>Specifies the maximum size (in KB) of any .csv file used to transfer data to a MySQL-compatible database.</p> <p>Example: <code>maxFileSize=512</code> </p>
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    /// <p>Improves performance when loading data into the MySQL-compatible target database. Specifies how many threads to use to load the data into the MySQL-compatible target database. Setting a large number of threads can have an adverse effect on database performance, because a separate connection is required for each thread.</p> <p>Example: <code>parallelLoadThreads=1</code> </p>
    #[serde(rename = "ParallelLoadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_load_threads: Option<i64>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the MySQL endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the MySQL endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Specifies the time zone for the source MySQL database.</p> <p>Example: <code>serverTimezone=US/Pacific;</code> </p> <p>Note: Do not enclose time zones in single quotes.</p>
    #[serde(rename = "ServerTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_timezone: Option<String>,
    /// <p>Specifies where to migrate source tables on the target, either to a single database or multiple databases.</p> <p>Example: <code>targetDbType=MULTIPLE_DATABASES</code> </p>
    #[serde(rename = "TargetDbType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_db_type: Option<String>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Provides information that defines an Amazon Neptune endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct NeptuneSettings {
    /// <p>The number of milliseconds for AWS DMS to wait to retry a bulk-load of migrated graph data to the Neptune target database before raising an error. The default is 250.</p>
    #[serde(rename = "ErrorRetryDuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_retry_duration: Option<i64>,
    /// <p>If you want AWS Identity and Access Management (IAM) authorization enabled for this endpoint, set this parameter to <code>true</code>. Then attach the appropriate IAM policy document to your service role specified by <code>ServiceAccessRoleArn</code>. The default is <code>false</code>.</p>
    #[serde(rename = "IamAuthEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_auth_enabled: Option<bool>,
    /// <p>The maximum size in kilobytes of migrated graph data stored in a .csv file before AWS DMS bulk-loads the data to the Neptune target database. The default is 1,048,576 KB. If the bulk load is successful, AWS DMS clears the bucket, ready to store the next batch of migrated graph data.</p>
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    /// <p>The number of times for AWS DMS to retry a bulk load of migrated graph data to the Neptune target database before raising an error. The default is 5.</p>
    #[serde(rename = "MaxRetryCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_retry_count: Option<i64>,
    /// <p>A folder path where you want AWS DMS to store migrated graph data in the S3 bucket specified by <code>S3BucketName</code> </p>
    #[serde(rename = "S3BucketFolder")]
    pub s3_bucket_folder: String,
    /// <p>The name of the Amazon S3 bucket where AWS DMS can temporarily store migrated graph data in .csv files before bulk-loading it to the Neptune target database. AWS DMS maps the SQL source data to graph data before storing it in these .csv files.</p>
    #[serde(rename = "S3BucketName")]
    pub s3_bucket_name: String,
    /// <p>The Amazon Resource Name (ARN) of the service role that you created for the Neptune target endpoint. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.Neptune.html#CHAP_Target.Neptune.ServiceRole">Creating an IAM Service Role for Accessing Amazon Neptune as a Target</a> in the <i>AWS Database Migration Service User Guide. </i> </p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
}

/// <p>Provides information that defines an Oracle endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OracleSettings {
    /// <p>Set this attribute to <code>false</code> in order to use the Binary Reader to capture change data for an Amazon RDS for Oracle as the source. This tells the DMS instance to not access redo logs through any specified path prefix replacement using direct file access.</p>
    #[serde(rename = "AccessAlternateDirectly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_alternate_directly: Option<bool>,
    /// <p>Set this attribute to set up table-level supplemental logging for the Oracle database. This attribute enables PRIMARY KEY supplemental logging on all tables selected for a migration task.</p> <p>If you use this option, you still need to enable database-level supplemental logging.</p>
    #[serde(rename = "AddSupplementalLogging")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_supplemental_logging: Option<bool>,
    /// <p>Set this attribute with <code>archivedLogDestId</code> in a primary/ standby setup. This attribute is useful in the case of a switchover. In this case, AWS DMS needs to know which destination to get archive redo logs from to read changes. This need arises because the previous primary instance is now a standby instance after switchover.</p>
    #[serde(rename = "AdditionalArchivedLogDestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_archived_log_dest_id: Option<i64>,
    /// <p>Set this attribute to <code>true</code> to enable replication of Oracle tables containing columns that are nested tables or defined types.</p>
    #[serde(rename = "AllowSelectNestedTables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_select_nested_tables: Option<bool>,
    /// <p>Specifies the destination of the archived redo logs. The value should be the same as the DEST_ID number in the v$archived_log table. When working with multiple log destinations (DEST_ID), we recommend that you to specify an archived redo logs location identifier. Doing this improves performance by ensuring that the correct logs are accessed from the outset.</p>
    #[serde(rename = "ArchivedLogDestId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_log_dest_id: Option<i64>,
    /// <p>When this field is set to <code>Y</code>, AWS DMS only accesses the archived redo logs. If the archived redo logs are stored on Oracle ASM only, the AWS DMS user account needs to be granted ASM privileges.</p>
    #[serde(rename = "ArchivedLogsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived_logs_only: Option<bool>,
    /// <p>For an Oracle source endpoint, your Oracle Automatic Storage Management (ASM) password. You can set this value from the <code> <i>asm_user_password</i> </code> value. You set this value as part of the comma-separated value that you set to the <code>Password</code> request parameter when you create the endpoint to access transaction logs using Binary Reader. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.CDC.Configuration">Configuration for change data capture (CDC) on an Oracle source database</a>.</p>
    #[serde(rename = "AsmPassword")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_password: Option<String>,
    /// <p>For an Oracle source endpoint, your ASM server address. You can set this value from the <code>asm_server</code> value. You set <code>asm_server</code> as part of the extra connection attribute string to access an Oracle server with Binary Reader that uses ASM. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.CDC.Configuration">Configuration for change data capture (CDC) on an Oracle source database</a>.</p>
    #[serde(rename = "AsmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_server: Option<String>,
    /// <p>For an Oracle source endpoint, your ASM user name. You can set this value from the <code>asm_user</code> value. You set <code>asm_user</code> as part of the extra connection attribute string to access an Oracle server with Binary Reader that uses ASM. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.CDC.Configuration">Configuration for change data capture (CDC) on an Oracle source database</a>.</p>
    #[serde(rename = "AsmUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asm_user: Option<String>,
    /// <p>Specifies whether the length of a character column is in bytes or in characters. To indicate that the character column length is in characters, set this attribute to <code>CHAR</code>. Otherwise, the character column length is in bytes.</p> <p>Example: <code>charLengthSemantics=CHAR;</code> </p>
    #[serde(rename = "CharLengthSemantics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_length_semantics: Option<String>,
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>When set to <code>true</code>, this attribute helps to increase the commit rate on the Oracle target database by writing directly to tables and not writing a trail to database logs.</p>
    #[serde(rename = "DirectPathNoLog")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_no_log: Option<bool>,
    /// <p>When set to <code>true</code>, this attribute specifies a parallel load when <code>useDirectPathFullLoad</code> is set to <code>Y</code>. This attribute also only applies when you use the AWS DMS parallel load feature. Note that the target table cannot have any constraints or indexes.</p>
    #[serde(rename = "DirectPathParallelLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_path_parallel_load: Option<bool>,
    /// <p>Set this attribute to enable homogenous tablespace replication and create existing tables or indexes under the same tablespace on the target.</p>
    #[serde(rename = "EnableHomogenousTablespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_homogenous_tablespace: Option<bool>,
    /// <p>When set to <code>true</code>, this attribute causes a task to fail if the actual size of an LOB column is greater than the specified <code>LobMaxSize</code>.</p> <p>If a task is set to limited LOB mode and this option is set to <code>true</code>, the task fails instead of truncating the LOB data.</p>
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    /// <p>Specifies the number scale. You can select a scale up to 38, or you can select FLOAT. By default, the NUMBER data type is converted to precision 38, scale 10.</p> <p>Example: <code>numberDataTypeScale=12</code> </p>
    #[serde(rename = "NumberDatatypeScale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_datatype_scale: Option<i64>,
    /// <p>Set this string attribute to the required value in order to use the Binary Reader to capture change data for an Amazon RDS for Oracle as the source. This value specifies the default Oracle root used to access the redo logs.</p>
    #[serde(rename = "OraclePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oracle_path_prefix: Option<String>,
    /// <p>Set this attribute to change the number of threads that DMS configures to perform a Change Data Capture (CDC) load using Oracle Automatic Storage Management (ASM). You can specify an integer value between 2 (the default) and 8 (the maximum). Use this attribute together with the <code>readAheadBlocks</code> attribute.</p>
    #[serde(rename = "ParallelAsmReadThreads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_asm_read_threads: Option<i64>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>Set this attribute to change the number of read-ahead blocks that DMS configures to perform a Change Data Capture (CDC) load using Oracle Automatic Storage Management (ASM). You can specify an integer value between 1000 (the default) and 200,000 (the maximum).</p>
    #[serde(rename = "ReadAheadBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_ahead_blocks: Option<i64>,
    /// <p>When set to <code>true</code>, this attribute supports tablespace replication.</p>
    #[serde(rename = "ReadTableSpaceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_table_space_name: Option<bool>,
    /// <p>Set this attribute to true in order to use the Binary Reader to capture change data for an Amazon RDS for Oracle as the source. This setting tells DMS instance to replace the default Oracle root with the specified <code>usePathPrefix</code> setting to access the redo logs.</p>
    #[serde(rename = "ReplacePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_path_prefix: Option<bool>,
    /// <p>Specifies the number of seconds that the system waits before resending a query.</p> <p>Example: <code>retryInterval=6;</code> </p>
    #[serde(rename = "RetryInterval")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_interval: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the Oracle endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the Oracle endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>For an Oracle source endpoint, the transparent data encryption (TDE) password required by AWM DMS to access Oracle redo logs encrypted by TDE using Binary Reader. It is also the <code> <i>TDE_Password</i> </code> part of the comma-separated value you set to the <code>Password</code> request parameter when you create the endpoint. The <code>SecurityDbEncryptian</code> setting is related to this <code>SecurityDbEncryptionName</code> setting. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.Encryption"> Supported encryption methods for using Oracle as a source for AWS DMS</a> in the <i>AWS Database Migration Service User Guide</i>. </p>
    #[serde(rename = "SecurityDbEncryption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption: Option<String>,
    /// <p>For an Oracle source endpoint, the name of a key used for the transparent data encryption (TDE) of the columns and tablespaces in an Oracle source database that is encrypted using TDE. The key value is the value of the <code>SecurityDbEncryption</code> setting. For more information on setting the key name value of <code>SecurityDbEncryptionName</code>, see the information and example for setting the <code>securityDbEncryptionName</code> extra connection attribute in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.Oracle.html#CHAP_Source.Oracle.Encryption"> Supported encryption methods for using Oracle as a source for AWS DMS</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    #[serde(rename = "SecurityDbEncryptionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_db_encryption_name: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Set this attribute to <code>true</code> in order to use the Binary Reader to capture change data for an Amazon RDS for Oracle as the source. This tells the DMS instance to use any specified prefix replacement to access all online redo logs.</p>
    #[serde(rename = "UseAlternateFolderForOnline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_alternate_folder_for_online: Option<bool>,
    /// <p>Set this string attribute to the required value in order to use the Binary Reader to capture change data for an Amazon RDS for Oracle as the source. This value specifies the path prefix used to replace the default Oracle root to access the redo logs.</p>
    #[serde(rename = "UsePathPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_path_prefix: Option<String>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>In response to the <code>DescribeOrderableReplicationInstances</code> operation, this object describes an available replication instance. This description includes the replication instance's type, engine version, and allocated storage.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct OrderableReplicationInstance {
    /// <p>List of Availability Zones for this replication instance.</p>
    #[serde(rename = "AvailabilityZones")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    /// <p>The default amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "DefaultAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_allocated_storage: Option<i64>,
    /// <p>The version of the replication engine.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "IncludedAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub included_allocated_storage: Option<i64>,
    /// <p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>
    #[serde(rename = "MaxAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_allocated_storage: Option<i64>,
    /// <p>The minimum amount of storage (in gigabytes) that can be allocated for the replication instance.</p>
    #[serde(rename = "MinAllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_allocated_storage: Option<i64>,
    /// <p><p>The value returned when the specified <code>EngineVersion</code> of the replication instance is in Beta or test mode. This indicates some features might not work as expected.</p> <note> <p>AWS DMS supports the <code>ReleaseStatus</code> parameter in versions 3.1.4 and later.</p> </note></p>
    #[serde(rename = "ReleaseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_status: Option<String>,
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. For example to specify the instance class dms.c4.large, set this parameter to <code>"dms.c4.large"</code>.</p> <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.html#CHAP_ReplicationInstance.InDepth"> Selecting the right AWS DMS replication instance for your migration</a>. </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The type of storage used by the replication instance.</p>
    #[serde(rename = "StorageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

/// <p>Describes a maintenance action pending for an AWS DMS resource, including when and how it will be applied. This data type is a response element to the <code>DescribePendingMaintenanceActions</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PendingMaintenanceAction {
    /// <p>The type of pending maintenance action that is available for the resource.</p>
    #[serde(rename = "Action")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    /// <p>The date of the maintenance window when the action is to be applied. The maintenance action is applied to the resource during its first maintenance window after this date. If this date is specified, any <code>next-maintenance</code> opt-in requests are ignored.</p>
    #[serde(rename = "AutoAppliedAfterDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_applied_after_date: Option<f64>,
    /// <p>The effective date when the pending maintenance action will be applied to the resource. This date takes into account opt-in requests received from the <code>ApplyPendingMaintenanceAction</code> API operation, and also the <code>AutoAppliedAfterDate</code> and <code>ForcedApplyDate</code> parameter values. This value is blank if an opt-in request has not been received and nothing has been specified for <code>AutoAppliedAfterDate</code> or <code>ForcedApplyDate</code>.</p>
    #[serde(rename = "CurrentApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<f64>,
    /// <p>A description providing more detail about the maintenance action.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date when the maintenance action will be automatically applied. The maintenance action is applied to the resource on this date regardless of the maintenance window for the resource. If this date is specified, any <code>immediate</code> opt-in requests are ignored.</p>
    #[serde(rename = "ForcedApplyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_apply_date: Option<f64>,
    /// <p>The type of opt-in request that has been received for the resource.</p>
    #[serde(rename = "OptInStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
}

/// <p>Provides information that defines a PostgreSQL endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PostgreSQLSettings {
    /// <p>For use with change data capture (CDC) only, this attribute has AWS DMS bypass foreign keys and user triggers to reduce the time it takes to bulk load data.</p> <p>Example: <code>afterConnectScript=SET session_replication_role='replica'</code> </p>
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    /// <p>To capture DDL events, AWS DMS creates various artifacts in the PostgreSQL database when the task starts. You can later remove these artifacts.</p> <p>If this value is set to <code>N</code>, you don't have to create tables or triggers on the source database.</p>
    #[serde(rename = "CaptureDdls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_ddls: Option<bool>,
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The schema in which the operational DDL database artifacts are created.</p> <p>Example: <code>ddlArtifactsSchema=xyzddlschema;</code> </p>
    #[serde(rename = "DdlArtifactsSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddl_artifacts_schema: Option<String>,
    /// <p>Sets the client statement timeout for the PostgreSQL instance, in seconds. The default value is 60 seconds.</p> <p>Example: <code>executeTimeout=100;</code> </p>
    #[serde(rename = "ExecuteTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execute_timeout: Option<i64>,
    /// <p>When set to <code>true</code>, this value causes a task to fail if the actual size of a LOB column is greater than the specified <code>LobMaxSize</code>.</p> <p>If task is set to Limited LOB mode and this option is set to true, the task fails instead of truncating the LOB data.</p>
    #[serde(rename = "FailTasksOnLobTruncation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_tasks_on_lob_truncation: Option<bool>,
    /// <p>Specifies the maximum size (in KB) of any .csv file used to transfer data to PostgreSQL.</p> <p>Example: <code>maxFileSize=512</code> </p>
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the PostgreSQL endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the PostgreSQL endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Sets the name of a previously created logical replication slot for a CDC load of the PostgreSQL source instance.</p> <p>When used with the AWS DMS API <code>CdcStartPosition</code> request parameter, this attribute also enables using native CDC start points.</p>
    #[serde(rename = "SlotName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slot_name: Option<String>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// see [DatabaseMigrationService::reboot_replication_instance]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RebootReplicationInstanceMessage {
    /// <p>If this parameter is <code>true</code>, the reboot is conducted through a Multi-AZ failover. (If the instance isn't configured for Multi-AZ, then you can't specify <code>true</code>.)</p>
    #[serde(rename = "ForceFailover")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_failover: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// see [DatabaseMigrationService::reboot_replication_instance]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RebootReplicationInstanceResponse {
    /// <p>The replication instance that is being rebooted. </p>
    #[serde(rename = "ReplicationInstance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance: Option<ReplicationInstance>,
}

/// <p>Provides information that defines an Amazon Redshift endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedshiftSettings {
    /// <p>A value that indicates to allow any date format, including invalid formats such as 00/00/00 00:00:00, to be loaded without generating an error. You can choose <code>true</code> or <code>false</code> (the default).</p> <p>This parameter applies only to TIMESTAMP and DATE columns. Always use ACCEPTANYDATE with the DATEFORMAT parameter. If the date format for the data doesn't match the DATEFORMAT specification, Amazon Redshift inserts a NULL value into that field. </p>
    #[serde(rename = "AcceptAnyDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_any_date: Option<bool>,
    /// <p>Code to run after connecting. This parameter should contain the code itself, not the name of a file containing the code.</p>
    #[serde(rename = "AfterConnectScript")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_connect_script: Option<String>,
    /// <p>An S3 folder where the comma-separated-value (.csv) files are stored before being uploaded to the target Redshift cluster. </p> <p>For full load mode, AWS DMS converts source records into .csv files and loads them to the <i>BucketFolder/TableID</i> path. AWS DMS uses the Redshift <code>COPY</code> command to upload the .csv files to the target table. The files are deleted once the <code>COPY</code> operation has finished. For more information, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">COPY</a> in the <i>Amazon Redshift Database Developer Guide</i>.</p> <p>For change-data-capture (CDC) mode, AWS DMS creates a <i>NetChanges</i> table, and loads the .csv files to this <i>BucketFolder/NetChangesTableID</i> path.</p>
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    /// <p>The name of the intermediate S3 bucket used to store .csv files before uploading data to Redshift.</p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p>If Amazon Redshift is configured to support case sensitive schema names, set <code>CaseSensitiveNames</code> to <code>true</code>. The default is <code>false</code>.</p>
    #[serde(rename = "CaseSensitiveNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_sensitive_names: Option<bool>,
    /// <p>If you set <code>CompUpdate</code> to <code>true</code> Amazon Redshift applies automatic compression if the table is empty. This applies even if the table columns already have encodings other than <code>RAW</code>. If you set <code>CompUpdate</code> to <code>false</code>, automatic compression is disabled and existing column encodings aren't changed. The default is <code>true</code>.</p>
    #[serde(rename = "CompUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp_update: Option<bool>,
    /// <p>A value that sets the amount of time to wait (in milliseconds) before timing out, beginning from when you initially establish a connection.</p>
    #[serde(rename = "ConnectionTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_timeout: Option<i64>,
    /// <p>The name of the Amazon Redshift data warehouse (service) that you are working with.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>The date format that you are using. Valid values are <code>auto</code> (case-sensitive), your date format string enclosed in quotes, or NULL. If this parameter is left unset (NULL), it defaults to a format of 'YYYY-MM-DD'. Using <code>auto</code> recognizes most strings, even some that aren't supported when you use a date format string. </p> <p>If your date and time values use formats different from each other, set this to <code>auto</code>. </p>
    #[serde(rename = "DateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    /// <p>A value that specifies whether AWS DMS should migrate empty CHAR and VARCHAR fields as NULL. A value of <code>true</code> sets empty CHAR and VARCHAR fields to null. The default is <code>false</code>.</p>
    #[serde(rename = "EmptyAsNull")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_as_null: Option<bool>,
    /// <p>The type of server-side encryption that you want to use for your data. This encryption type is part of the endpoint settings or the extra connections attributes for Amazon S3. You can choose either <code>SSE_S3</code> (the default) or <code>SSE_KMS</code>. </p> <note> <p>For the <code>ModifyEndpoint</code> operation, you can change the existing value of the <code>EncryptionMode</code> parameter from <code>SSE_KMS</code> to <code>SSE_S3</code>. But you can’t change the existing value from <code>SSE_S3</code> to <code>SSE_KMS</code>.</p> </note> <p>To use <code>SSE_S3</code>, create an AWS Identity and Access Management (IAM) role with a policy that allows <code>"arn:aws:s3:::*"</code> to use the following actions: <code>"s3:PutObject", "s3:ListBucket"</code> </p>
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    /// <p>This setting is only valid for a full-load migration task. Set <code>ExplicitIds</code> to <code>true</code> to have tables with <code>IDENTITY</code> columns override their auto-generated values with explicit values loaded from the source data files used to populate the tables. The default is <code>false</code>.</p>
    #[serde(rename = "ExplicitIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_ids: Option<bool>,
    /// <p>The number of threads used to upload a single file. This parameter accepts a value from 1 through 64. It defaults to 10.</p> <p>The number of parallel streams used to upload a single .csv file to an S3 bucket using S3 Multipart Upload. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/mpuoverview.html">Multipart upload overview</a>. </p> <p> <code>FileTransferUploadStreams</code> accepts a value from 1 through 64. It defaults to 10.</p>
    #[serde(rename = "FileTransferUploadStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_transfer_upload_streams: Option<i64>,
    /// <p>The amount of time to wait (in milliseconds) before timing out of operations performed by AWS DMS on a Redshift cluster, such as Redshift COPY, INSERT, DELETE, and UPDATE.</p>
    #[serde(rename = "LoadTimeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_timeout: Option<i64>,
    /// <p>The maximum size (in KB) of any .csv file used to load data on an S3 bucket and transfer data to Amazon Redshift. It defaults to 1048576KB (1 GB).</p>
    #[serde(rename = "MaxFileSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_file_size: Option<i64>,
    /// <p>The password for the user named in the <code>username</code> property.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The port number for Amazon Redshift. The default value is 5439.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p>A value that specifies to remove surrounding quotation marks from strings in the incoming data. All characters within the quotation marks, including delimiters, are retained. Choose <code>true</code> to remove quotation marks. The default is <code>false</code>.</p>
    #[serde(rename = "RemoveQuotes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_quotes: Option<bool>,
    /// <p>A value that specifies to replaces the invalid characters specified in <code>ReplaceInvalidChars</code>, substituting the specified characters instead. The default is <code>"?"</code>.</p>
    #[serde(rename = "ReplaceChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_chars: Option<String>,
    /// <p>A list of characters that you want to replace. Use with <code>ReplaceChars</code>.</p>
    #[serde(rename = "ReplaceInvalidChars")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_invalid_chars: Option<String>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the Amazon Redshift endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the Amazon Redshift endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>The name of the Amazon Redshift cluster you are using.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>The AWS KMS key ID. If you are using <code>SSE_KMS</code> for the <code>EncryptionMode</code>, provide this key ID. The key that you use needs an attached policy that enables IAM user permissions and allows use of the key.</p>
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that has access to the Amazon Redshift service.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>The time format that you want to use. Valid values are <code>auto</code> (case-sensitive), <code>'timeformat_string'</code>, <code>'epochsecs'</code>, or <code>'epochmillisecs'</code>. It defaults to 10. Using <code>auto</code> recognizes most strings, even some that aren't supported when you use a time format string. </p> <p>If your date and time values use formats different from each other, set this parameter to <code>auto</code>. </p>
    #[serde(rename = "TimeFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_format: Option<String>,
    /// <p>A value that specifies to remove the trailing white space characters from a VARCHAR string. This parameter applies only to columns with a VARCHAR data type. Choose <code>true</code> to remove unneeded white space. The default is <code>false</code>.</p>
    #[serde(rename = "TrimBlanks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trim_blanks: Option<bool>,
    /// <p>A value that specifies to truncate data in columns to the appropriate number of characters, so that the data fits in the column. This parameter applies only to columns with a VARCHAR or CHAR data type, and rows with a size of 4 MB or less. Choose <code>true</code> to truncate data. The default is <code>false</code>.</p>
    #[serde(rename = "TruncateColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncate_columns: Option<bool>,
    /// <p>An Amazon Redshift user name for a registered user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// <p>The size (in KB) of the in-memory file write buffer used when generating .csv files on the local disk at the DMS replication instance. The default value is 1000 (buffer size is 1000KB).</p>
    #[serde(rename = "WriteBufferSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_buffer_size: Option<i64>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::refresh_schemas]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RefreshSchemasMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::refresh_schemas]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RefreshSchemasResponse {
    /// <p>The status of the refreshed schema.</p>
    #[serde(rename = "RefreshSchemasStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_schemas_status: Option<RefreshSchemasStatus>,
}

/// <p>Provides information that describes status of a schema at an endpoint specified by the <code>DescribeRefreshSchemaStatus</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RefreshSchemasStatus {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_arn: Option<String>,
    /// <p>The last failure message for the schema.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The date the schema was last refreshed.</p>
    #[serde(rename = "LastRefreshDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refresh_date: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The status of the schema.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// see [DatabaseMigrationService::reload_tables]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ReloadTablesMessage {
    /// <p>Options for reload. Specify <code>data-reload</code> to reload the data and re-validate it if validation is enabled. Specify <code>validate-only</code> to re-validate the table. This option applies only when validation is enabled for the task. </p> <p>Valid values: data-reload, validate-only</p> <p>Default value is data-reload.</p>
    #[serde(rename = "ReloadOption")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reload_option: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p>The name and schema of the table to be reloaded. </p>
    #[serde(rename = "TablesToReload")]
    pub tables_to_reload: Vec<TableToReload>,
}

/// see [DatabaseMigrationService::reload_tables]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReloadTablesResponse {
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
}

/// <p>Removes one or more tags from an AWS DMS resource.</p>
/// see [DatabaseMigrationService::remove_tags_from_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct RemoveTagsFromResourceMessage {
    /// <p>An AWS DMS resource from which you want to remove tag(s). The value for this parameter is an Amazon Resource Name (ARN).</p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The tag key (name) of the tag to be removed.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::remove_tags_from_resource]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RemoveTagsFromResourceResponse {}

/// <p>Provides information that defines a replication instance.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationInstance {
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>Boolean value indicating if minor version upgrades will be automatically applied to the instance.</p>
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    /// <p>The Availability Zone for the instance.</p>
    #[serde(rename = "AvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    /// <p>The DNS name servers supported for the replication instance to access your on-premise source or target database.</p>
    #[serde(rename = "DnsNameServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_name_servers: Option<String>,
    /// <p>The engine version number of the replication instance.</p> <p>If an engine version number is not specified when a replication instance is created, the default is the latest engine version available.</p> <p>When modifying a major engine version of an instance, also set <code>AllowMajorVersionUpgrade</code> to <code>true</code>.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> The expiration date of the free replication instance that is part of the Free DMS program. </p>
    #[serde(rename = "FreeUntil")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_until: Option<f64>,
    /// <p>The time the replication instance was created.</p>
    #[serde(rename = "InstanceCreateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<f64>,
    /// <p>An AWS KMS key identifier that is used to encrypt the data on the replication instance.</p> <p>If you don't specify a value for the <code>KmsKeyId</code> parameter, then AWS DMS uses your default encryption key.</p> <p>AWS KMS creates the default encryption key for your AWS account. Your AWS account has a different default encryption key for each AWS Region.</p>
    #[serde(rename = "KmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p> Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The pending modification values.</p>
    #[serde(rename = "PendingModifiedValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ReplicationPendingModifiedValues>,
    /// <p>The maintenance window times for the replication instance. Any pending upgrades to the replication instance are performed during this time.</p>
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    /// <p> Specifies the accessibility options for the replication instance. A value of <code>true</code> represents an instance with a public IP address. A value of <code>false</code> represents an instance with a private IP address. The default value is <code>true</code>. </p>
    #[serde(rename = "PubliclyAccessible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class. It is a required parameter, although a defualt value is pre-selected in the DMS console.</p> <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.html#CHAP_ReplicationInstance.InDepth"> Selecting the right AWS DMS replication instance for your migration</a>. </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
    /// <p>The replication instance identifier is a required parameter. This parameter is stored as a lowercase string.</p> <p>Constraints:</p> <ul> <li> <p>Must contain 1-63 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul> <p>Example: <code>myrepinstance</code> </p>
    #[serde(rename = "ReplicationInstanceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_identifier: Option<String>,
    /// <p>One or more private IP addresses for the replication instance.</p>
    #[serde(rename = "ReplicationInstancePrivateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_private_ip_addresses: Option<Vec<String>>,
    /// <p>One or more public IP addresses for the replication instance.</p>
    #[serde(rename = "ReplicationInstancePublicIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_public_ip_addresses: Option<Vec<String>>,
    /// <p><p>The status of the replication instance. The possible return values include:</p> <ul> <li> <p> <code>&quot;available&quot;</code> </p> </li> <li> <p> <code>&quot;creating&quot;</code> </p> </li> <li> <p> <code>&quot;deleted&quot;</code> </p> </li> <li> <p> <code>&quot;deleting&quot;</code> </p> </li> <li> <p> <code>&quot;failed&quot;</code> </p> </li> <li> <p> <code>&quot;modifying&quot;</code> </p> </li> <li> <p> <code>&quot;upgrading&quot;</code> </p> </li> <li> <p> <code>&quot;rebooting&quot;</code> </p> </li> <li> <p> <code>&quot;resetting-master-credentials&quot;</code> </p> </li> <li> <p> <code>&quot;storage-full&quot;</code> </p> </li> <li> <p> <code>&quot;incompatible-credentials&quot;</code> </p> </li> <li> <p> <code>&quot;incompatible-network&quot;</code> </p> </li> <li> <p> <code>&quot;maintenance&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "ReplicationInstanceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_status: Option<String>,
    /// <p>The subnet group for the replication instance.</p>
    #[serde(rename = "ReplicationSubnetGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group: Option<ReplicationSubnetGroup>,
    /// <p>The Availability Zone of the standby replication instance in a Multi-AZ deployment.</p>
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    /// <p>The VPC security group for the instance.</p>
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<Vec<VpcSecurityGroupMembership>>,
}

/// <p>Contains metadata for a replication instance task log.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationInstanceTaskLog {
    /// <p>The size, in bytes, of the replication task log.</p>
    #[serde(rename = "ReplicationInstanceTaskLogSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_task_log_size: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The name of the replication task.</p>
    #[serde(rename = "ReplicationTaskName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_name: Option<String>,
}

/// <p>Provides information about the values of pending modifications to a replication instance. This data type is an object of the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_ReplicationInstance.html"> <code>ReplicationInstance</code> </a> user-defined data type. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationPendingModifiedValues {
    /// <p>The amount of storage (in gigabytes) that is allocated for the replication instance.</p>
    #[serde(rename = "AllocatedStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i64>,
    /// <p>The engine version number of the replication instance.</p>
    #[serde(rename = "EngineVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    /// <p> Specifies whether the replication instance is a Multi-AZ deployment. You can't set the <code>AvailabilityZone</code> parameter if the Multi-AZ parameter is set to <code>true</code>. </p>
    #[serde(rename = "MultiAZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_az: Option<bool>,
    /// <p>The compute and memory capacity of the replication instance as defined for the specified replication instance class.</p> <p>For more information on the settings and capacities for the available replication instance classes, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_ReplicationInstance.html#CHAP_ReplicationInstance.InDepth"> Selecting the right AWS DMS replication instance for your migration</a>. </p>
    #[serde(rename = "ReplicationInstanceClass")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_class: Option<String>,
}

/// <p>Describes a subnet group in response to a request by the <code>DescribeReplicationSubnetGroups</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationSubnetGroup {
    /// <p>A description for the replication subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_description: Option<String>,
    /// <p>The identifier of the replication instance subnet group.</p>
    #[serde(rename = "ReplicationSubnetGroupIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_subnet_group_identifier: Option<String>,
    /// <p>The status of the subnet group.</p>
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    /// <p>The subnets that are in the subnet group.</p>
    #[serde(rename = "Subnets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<Subnet>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// <p>Provides information that describes a replication task created by the <code>CreateReplicationTask</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTask {
    /// <p>Indicates when you want a change data capture (CDC) operation to start. Use either <code>CdcStartPosition</code> or <code>CdcStartTime</code> to specify when you want the CDC operation to start. Specifying both values results in an error.</p> <p>The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position "checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93"</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 2018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The last error (failure) message generated for the replication task.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>The type of migration.</p>
    #[serde(rename = "MigrationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration_type: Option<String>,
    /// <p>Indicates the last checkpoint that occurred during a change data capture (CDC) operation. You can provide this value to the <code>CdcStartPosition</code> parameter to start a CDC operation that begins at that checkpoint.</p>
    #[serde(rename = "RecoveryCheckpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_checkpoint: Option<String>,
    /// <p>The ARN of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>The date the replication task was created.</p>
    #[serde(rename = "ReplicationTaskCreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_creation_date: Option<f64>,
    /// <p><p>The user-assigned replication task identifier or name.</p> <p>Constraints:</p> <ul> <li> <p>Must contain 1-255 alphanumeric characters or hyphens.</p> </li> <li> <p>First character must be a letter.</p> </li> <li> <p>Cannot end with a hyphen or contain two consecutive hyphens.</p> </li> </ul></p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>The settings for the replication task.</p>
    #[serde(rename = "ReplicationTaskSettings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_settings: Option<String>,
    /// <p>The date the replication task is scheduled to start.</p>
    #[serde(rename = "ReplicationTaskStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_start_date: Option<f64>,
    /// <p>The statistics for the task, including elapsed time, tables loaded, and table errors.</p>
    #[serde(rename = "ReplicationTaskStats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_stats: Option<ReplicationTaskStats>,
    /// <p>The Amazon Resource Name (ARN) that uniquely identifies the endpoint.</p>
    #[serde(rename = "SourceEndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_endpoint_arn: Option<String>,
    /// <p><p>The status of the replication task. This response parameter can return one of the following values:</p> <ul> <li> <p> <code>&quot;moving&quot;</code> – The task is being moved in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_MoveReplicationTask.html"> <code>MoveReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;creating&quot;</code> – The task is being created in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_CreateReplicationTask.html"> <code>CreateReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;deleting&quot;</code> – The task is being deleted in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_DeleteReplicationTask.html"> <code>DeleteReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;failed&quot;</code> – The task failed to successfully complete the database migration in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTask.html"> <code>StartReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;failed-move&quot;</code> – The task failed to move in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_MoveReplicationTask.html"> <code>MoveReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;modifying&quot;</code> – The task definition is being modified in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_ModifyReplicationTask.html"> <code>ModifyReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;ready&quot;</code> – The task is in a <code>ready</code> state where it can respond to other task operations, such as <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTask.html"> <code>StartReplicationTask</code> </a> or <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_DeleteReplicationTask.html"> <code>DeleteReplicationTask</code> </a>. </p> </li> <li> <p> <code>&quot;running&quot;</code> – The task is performing a database migration in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTask.html"> <code>StartReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;starting&quot;</code> – The task is preparing to perform a database migration in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTask.html"> <code>StartReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;stopped&quot;</code> – The task has stopped in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StopReplicationTask.html"> <code>StopReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;stopping&quot;</code> – The task is preparing to stop in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StopReplicationTask.html"> <code>StopReplicationTask</code> </a> operation.</p> </li> <li> <p> <code>&quot;testing&quot;</code> – The database migration specified for this task is being tested in response to running either the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTaskAssessmentRun.html"> <code>StartReplicationTaskAssessmentRun</code> </a> or the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTaskAssessment.html"> <code>StartReplicationTaskAssessment</code> </a> operation.</p> <note> <p> <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTaskAssessmentRun.html"> <code>StartReplicationTaskAssessmentRun</code> </a> is an improved premigration task assessment operation. The <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTaskAssessment.html"> <code>StartReplicationTaskAssessment</code> </a> operation assesses data type compatibility only between the source and target database of a given migration task. In contrast, <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTaskAssessmentRun.html"> <code>StartReplicationTaskAssessmentRun</code> </a> enables you to specify a variety of premigration task assessments in addition to data type compatibility. These assessments include ones for the validity of primary key definitions and likely issues with database migration performance, among others.</p> </note> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>The reason the replication task was stopped. This response parameter can return one of the following values:</p> <ul> <li> <p> <code>&quot;STOP<em>REASON</em>FULL<em>LOAD</em>COMPLETED&quot;</code> – Full-load migration completed.</p> </li> <li> <p> <code>&quot;STOP<em>REASON</em>CACHED<em>CHANGES</em>APPLIED&quot;</code> – Change data capture (CDC) load completed.</p> </li> <li> <p> <code>&quot;STOP<em>REASON</em>CACHED<em>CHANGES</em>NOT<em>APPLIED&quot;</code> – In a full-load and CDC migration, the full load stopped as specified before starting the CDC migration.</p> </li> <li> <p> <code>&quot;STOP</em>REASON<em>SERVER</em>TIME&quot;</code> – The migration stopped at the specified server time.</p> </li> </ul></p>
    #[serde(rename = "StopReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<String>,
    /// <p>Table mappings specified in the task.</p>
    #[serde(rename = "TableMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_mappings: Option<String>,
    /// <p>The ARN that uniquely identifies the endpoint.</p>
    #[serde(rename = "TargetEndpointArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_endpoint_arn: Option<String>,
    /// <p>The ARN of the replication instance to which this task is moved in response to running the <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_MoveReplicationTask.html"> <code>MoveReplicationTask</code> </a> operation. Otherwise, this response parameter isn't a member of the <code>ReplicationTask</code> object.</p>
    #[serde(rename = "TargetReplicationInstanceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_replication_instance_arn: Option<String>,
    /// <p>Supplemental information that the task requires to migrate the data for certain source and target endpoints. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.TaskData.html">Specifying Supplemental Data for Task Settings</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    #[serde(rename = "TaskData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_data: Option<String>,
}

/// <p> The task assessment report in JSON format. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTaskAssessmentResult {
    /// <p> The task assessment results in JSON format. </p>
    #[serde(rename = "AssessmentResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results: Option<String>,
    /// <p> The file containing the results of the task assessment. </p>
    #[serde(rename = "AssessmentResultsFile")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_results_file: Option<String>,
    /// <p> The status of the task assessment. </p>
    #[serde(rename = "AssessmentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_status: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p> The replication task identifier of the task on which the task assessment was run. </p>
    #[serde(rename = "ReplicationTaskIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_identifier: Option<String>,
    /// <p>The date the task assessment was completed. </p>
    #[serde(rename = "ReplicationTaskLastAssessmentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_last_assessment_date: Option<f64>,
    /// <p> The URL of the S3 object containing the task assessment results. </p>
    #[serde(rename = "S3ObjectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_url: Option<String>,
}

/// <p>Provides information that describes a premigration assessment run that you have started using the <code>StartReplicationTaskAssessmentRun</code> operation.</p> <p>Some of the information appears based on other operations that can return the <code>ReplicationTaskAssessmentRun</code> object.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTaskAssessmentRun {
    /// <p>Indication of the completion progress for the individual assessments specified to run.</p>
    #[serde(rename = "AssessmentProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_progress: Option<ReplicationTaskAssessmentRunProgress>,
    /// <p>Unique name of the assessment run.</p>
    #[serde(rename = "AssessmentRunName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_name: Option<String>,
    /// <p>Last message generated by an individual assessment failure.</p>
    #[serde(rename = "LastFailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    /// <p>ARN of the migration task associated with this premigration assessment run.</p>
    #[serde(rename = "ReplicationTaskArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_arn: Option<String>,
    /// <p>Amazon Resource Name (ARN) of this assessment run.</p>
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_arn: Option<String>,
    /// <p>Date on which the assessment run was created using the <code>StartReplicationTaskAssessmentRun</code> operation.</p>
    #[serde(rename = "ReplicationTaskAssessmentRunCreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_creation_date: Option<f64>,
    /// <p>Encryption mode used to encrypt the assessment run results.</p>
    #[serde(rename = "ResultEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_encryption_mode: Option<String>,
    /// <p>ARN of the AWS KMS encryption key used to encrypt the assessment run results.</p>
    #[serde(rename = "ResultKmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_kms_key_arn: Option<String>,
    /// <p>Amazon S3 bucket where AWS DMS stores the results of this assessment run.</p>
    #[serde(rename = "ResultLocationBucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_bucket: Option<String>,
    /// <p>Folder in an Amazon S3 bucket where AWS DMS stores the results of this assessment run.</p>
    #[serde(rename = "ResultLocationFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_folder: Option<String>,
    /// <p>ARN of the service role used to start the assessment run using the <code>StartReplicationTaskAssessmentRun</code> operation.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p><p>Assessment run status. </p> <p>This status can have one of the following values:</p> <ul> <li> <p> <code>&quot;cancelling&quot;</code> – The assessment run was canceled by the <code>CancelReplicationTaskAssessmentRun</code> operation.</p> </li> <li> <p> <code>&quot;deleting&quot;</code> – The assessment run was deleted by the <code>DeleteReplicationTaskAssessmentRun</code> operation.</p> </li> <li> <p> <code>&quot;failed&quot;</code> – At least one individual assessment completed with a <code>failed</code> status.</p> </li> <li> <p> <code>&quot;error-provisioning&quot;</code> – An internal error occurred while resources were provisioned (during <code>provisioning</code> status).</p> </li> <li> <p> <code>&quot;error-executing&quot;</code> – An internal error occurred while individual assessments ran (during <code>running</code> status).</p> </li> <li> <p> <code>&quot;invalid state&quot;</code> – The assessment run is in an unknown state.</p> </li> <li> <p> <code>&quot;passed&quot;</code> – All individual assessments have completed, and none has a <code>failed</code> status.</p> </li> <li> <p> <code>&quot;provisioning&quot;</code> – Resources required to run individual assessments are being provisioned.</p> </li> <li> <p> <code>&quot;running&quot;</code> – Individual assessments are being run.</p> </li> <li> <p> <code>&quot;starting&quot;</code> – The assessment run is starting, but resources are not yet being provisioned for individual assessments.</p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The progress values reported by the <code>AssessmentProgress</code> response element.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTaskAssessmentRunProgress {
    /// <p>The number of individual assessments that have completed, successfully or not.</p>
    #[serde(rename = "IndividualAssessmentCompletedCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_completed_count: Option<i64>,
    /// <p>The number of individual assessments that are specified to run.</p>
    #[serde(rename = "IndividualAssessmentCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_count: Option<i64>,
}

/// <p>Provides information that describes an individual assessment from a premigration assessment run.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTaskIndividualAssessment {
    /// <p>Name of this individual assessment.</p>
    #[serde(rename = "IndividualAssessmentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual_assessment_name: Option<String>,
    /// <p>ARN of the premigration assessment run that is created to run this individual assessment.</p>
    #[serde(rename = "ReplicationTaskAssessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run_arn: Option<String>,
    /// <p>Amazon Resource Name (ARN) of this individual assessment.</p>
    #[serde(rename = "ReplicationTaskIndividualAssessmentArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessment_arn: Option<String>,
    /// <p>Date when this individual assessment was started as part of running the <code>StartReplicationTaskAssessmentRun</code> operation.</p>
    #[serde(rename = "ReplicationTaskIndividualAssessmentStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_individual_assessment_start_date: Option<f64>,
    /// <p><p>Individual assessment status.</p> <p>This status can have one of the following values:</p> <ul> <li> <p> <code>&quot;cancelled&quot;</code> </p> </li> <li> <p> <code>&quot;error&quot;</code> </p> </li> <li> <p> <code>&quot;failed&quot;</code> </p> </li> <li> <p> <code>&quot;passed&quot;</code> </p> </li> <li> <p> <code>&quot;pending&quot;</code> </p> </li> <li> <p> <code>&quot;running&quot;</code> </p> </li> </ul></p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>In response to a request by the <code>DescribeReplicationTasks</code> operation, this object provides a collection of statistics about a replication task.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ReplicationTaskStats {
    /// <p>The elapsed time of the task, in milliseconds.</p>
    #[serde(rename = "ElapsedTimeMillis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_millis: Option<i64>,
    /// <p>The date the replication task was started either with a fresh start or a target reload.</p>
    #[serde(rename = "FreshStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fresh_start_date: Option<f64>,
    /// <p>The date the replication task full load was completed.</p>
    #[serde(rename = "FullLoadFinishDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_finish_date: Option<f64>,
    /// <p>The percent complete for the full load migration task.</p>
    #[serde(rename = "FullLoadProgressPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_progress_percent: Option<i64>,
    /// <p>The date the replication task full load was started.</p>
    #[serde(rename = "FullLoadStartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_start_date: Option<f64>,
    /// <p>The date the replication task was started either with a fresh start or a resume. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_StartReplicationTask.html#DMS-StartReplicationTask-request-StartReplicationTaskType">StartReplicationTaskType</a>.</p>
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<f64>,
    /// <p>The date the replication task was stopped.</p>
    #[serde(rename = "StopDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_date: Option<f64>,
    /// <p>The number of errors that have occurred during this task.</p>
    #[serde(rename = "TablesErrored")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_errored: Option<i64>,
    /// <p>The number of tables loaded for this task.</p>
    #[serde(rename = "TablesLoaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loaded: Option<i64>,
    /// <p>The number of tables currently loading for this task.</p>
    #[serde(rename = "TablesLoading")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_loading: Option<i64>,
    /// <p>The number of tables queued for this task.</p>
    #[serde(rename = "TablesQueued")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tables_queued: Option<i64>,
}

/// <p>Identifies an AWS DMS resource and any pending actions for it.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ResourcePendingMaintenanceActions {
    /// <p>Detailed information about the pending maintenance action.</p>
    #[serde(rename = "PendingMaintenanceActionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_action_details: Option<Vec<PendingMaintenanceAction>>,
    /// <p>The Amazon Resource Name (ARN) of the DMS resource that the pending maintenance action applies to. For information about creating an ARN, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Introduction.AWS.ARN.html"> Constructing an Amazon Resource Name (ARN) for AWS DMS</a> in the DMS documentation.</p>
    #[serde(rename = "ResourceIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

/// <p>Settings for exporting data to Amazon S3. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3Settings {
    /// <p> An optional parameter to set a folder name in the S3 bucket. If provided, tables are created in the path <code> <i>bucketFolder</i>/<i>schema_name</i>/<i>table_name</i>/</code>. If this parameter isn't specified, then the path used is <code> <i>schema_name</i>/<i>table_name</i>/</code>. </p>
    #[serde(rename = "BucketFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_folder: Option<String>,
    /// <p> The name of the S3 bucket. </p>
    #[serde(rename = "BucketName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    /// <p><p>A value that enables a change data capture (CDC) load to write INSERT and UPDATE operations to .csv or .parquet (columnar storage) output files. The default setting is <code>false</code>, but when <code>CdcInsertsAndUpdates</code> is set to <code>true</code> or <code>y</code>, only INSERTs and UPDATEs from the source database are migrated to the .csv or .parquet file. </p> <p>For .csv file format only, how these INSERTs and UPDATEs are recorded depends on the value of the <code>IncludeOpForFullLoad</code> parameter. If <code>IncludeOpForFullLoad</code> is set to <code>true</code>, the first field of every CDC record is set to either <code>I</code> or <code>U</code> to indicate INSERT and UPDATE operations at the source. But if <code>IncludeOpForFullLoad</code> is set to <code>false</code>, CDC records are written without an indication of INSERT or UPDATE operations at the source. For more information about how these settings work together, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring.InsertOps">Indicating Source DB Operations in Migrated S3 Data</a> in the <i>AWS Database Migration Service User Guide.</i>.</p> <note> <p>AWS DMS supports the use of the <code>CdcInsertsAndUpdates</code> parameter in versions 3.3.1 and later.</p> <p> <code>CdcInsertsOnly</code> and <code>CdcInsertsAndUpdates</code> can&#39;t both be set to <code>true</code> for the same endpoint. Set either <code>CdcInsertsOnly</code> or <code>CdcInsertsAndUpdates</code> to <code>true</code> for the same endpoint, but not both.</p> </note></p>
    #[serde(rename = "CdcInsertsAndUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_and_updates: Option<bool>,
    /// <p><p>A value that enables a change data capture (CDC) load to write only INSERT operations to .csv or columnar storage (.parquet) output files. By default (the <code>false</code> setting), the first field in a .csv or .parquet record contains the letter I (INSERT), U (UPDATE), or D (DELETE). These values indicate whether the row was inserted, updated, or deleted at the source database for a CDC load to the target.</p> <p>If <code>CdcInsertsOnly</code> is set to <code>true</code> or <code>y</code>, only INSERTs from the source database are migrated to the .csv or .parquet file. For .csv format only, how these INSERTs are recorded depends on the value of <code>IncludeOpForFullLoad</code>. If <code>IncludeOpForFullLoad</code> is set to <code>true</code>, the first field of every CDC record is set to I to indicate the INSERT operation at the source. If <code>IncludeOpForFullLoad</code> is set to <code>false</code>, every CDC record is written without a first field to indicate the INSERT operation at the source. For more information about how these settings work together, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring.InsertOps">Indicating Source DB Operations in Migrated S3 Data</a> in the <i>AWS Database Migration Service User Guide.</i>.</p> <note> <p>AWS DMS supports the interaction described preceding between the <code>CdcInsertsOnly</code> and <code>IncludeOpForFullLoad</code> parameters in versions 3.1.4 and later. </p> <p> <code>CdcInsertsOnly</code> and <code>CdcInsertsAndUpdates</code> can&#39;t both be set to <code>true</code> for the same endpoint. Set either <code>CdcInsertsOnly</code> or <code>CdcInsertsAndUpdates</code> to <code>true</code> for the same endpoint, but not both.</p> </note></p>
    #[serde(rename = "CdcInsertsOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_inserts_only: Option<bool>,
    /// <p><p>Specifies the folder path of CDC files. For an S3 source, this setting is required if a task captures change data; otherwise, it&#39;s optional. If <code>CdcPath</code> is set, AWS DMS reads CDC files from this path and replicates the data changes to the target endpoint. For an S3 target if you set <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-PreserveTransactions"> <code>PreserveTransactions</code> </a> to <code>true</code>, AWS DMS verifies that you have set this parameter to a folder path on your S3 target where AWS DMS can save the transaction order for the CDC load. AWS DMS creates this CDC folder path in either your S3 target working directory or the S3 target location specified by <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-BucketFolder"> <code>BucketFolder</code> </a> and <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-BucketName"> <code>BucketName</code> </a>.</p> <p>For example, if you specify <code>CdcPath</code> as <code>MyChangedData</code>, and you specify <code>BucketName</code> as <code>MyTargetBucket</code> but do not specify <code>BucketFolder</code>, AWS DMS creates the CDC folder path following: <code>MyTargetBucket/MyChangedData</code>.</p> <p>If you specify the same <code>CdcPath</code>, and you specify <code>BucketName</code> as <code>MyTargetBucket</code> and <code>BucketFolder</code> as <code>MyTargetData</code>, AWS DMS creates the CDC folder path following: <code>MyTargetBucket/MyTargetData/MyChangedData</code>.</p> <p>For more information on CDC including transaction order on an S3 target, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.EndpointSettings.CdcPath">Capturing data changes (CDC) including transaction order on the S3 target</a>.</p> <note> <p>This setting is supported in AWS DMS versions 3.4.2 and later.</p> </note></p>
    #[serde(rename = "CdcPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_path: Option<String>,
    /// <p>An optional parameter to use GZIP to compress the target files. Set to GZIP to compress the target files. Either set this parameter to NONE (the default) or don't use it to leave the files uncompressed. This parameter applies to both .csv and .parquet file formats. </p>
    #[serde(rename = "CompressionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_type: Option<String>,
    /// <p> The delimiter used to separate columns in the .csv file for both source and target. The default is a comma. </p>
    #[serde(rename = "CsvDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_delimiter: Option<String>,
    /// <p><p>This setting only applies if your Amazon S3 output files during a change data capture (CDC) load are written in .csv format. If <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-UseCsvNoSupValue"> <code>UseCsvNoSupValue</code> </a> is set to true, specify a string value that you want AWS DMS to use for all columns not included in the supplemental log. If you do not specify a string value, AWS DMS uses the null value for these columns regardless of the <code>UseCsvNoSupValue</code> setting.</p> <note> <p>This setting is supported in AWS DMS versions 3.4.1 and later.</p> </note></p>
    #[serde(rename = "CsvNoSupValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_no_sup_value: Option<String>,
    /// <p> The delimiter used to separate rows in the .csv file for both source and target. The default is a carriage return (<code>\n</code>). </p>
    #[serde(rename = "CsvRowDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_row_delimiter: Option<String>,
    /// <p><p>The format of the data that you want to use for output. You can choose one of the following: </p> <ul> <li> <p> <code>csv</code> : This is a row-based file format with comma-separated values (.csv). </p> </li> <li> <p> <code>parquet</code> : Apache Parquet (.parquet) is a columnar storage file format that features efficient compression and provides faster query response. </p> </li> </ul></p>
    #[serde(rename = "DataFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
    /// <p>The size of one data page in bytes. This parameter defaults to 1024 * 1024 bytes (1 MiB). This number is used for .parquet file format only. </p>
    #[serde(rename = "DataPageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_page_size: Option<i64>,
    /// <p>Specifies a date separating delimiter to use during folder partitioning. The default value is <code>SLASH</code>. Use this parameter when <code>DatePartitionedEnabled</code> is set to <code>true</code>.</p>
    #[serde(rename = "DatePartitionDelimiter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_delimiter: Option<String>,
    /// <p>When set to <code>true</code>, this parameter partitions S3 bucket folders based on transaction commit dates. The default value is <code>false</code>. For more information about date-based folder partitoning, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.DatePartitioning">Using date-based folder partitioning</a>.</p>
    #[serde(rename = "DatePartitionEnabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_enabled: Option<bool>,
    /// <p>Identifies the sequence of the date format to use during folder partitioning. The default value is <code>YYYYMMDD</code>. Use this parameter when <code>DatePartitionedEnabled</code> is set to <code>true</code>.</p>
    #[serde(rename = "DatePartitionSequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_partition_sequence: Option<String>,
    /// <p>The maximum size of an encoded dictionary page of a column. If the dictionary page exceeds this, this column is stored using an encoding type of <code>PLAIN</code>. This parameter defaults to 1024 * 1024 bytes (1 MiB), the maximum size of a dictionary page before it reverts to <code>PLAIN</code> encoding. This size is used for .parquet file format only. </p>
    #[serde(rename = "DictPageSizeLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_page_size_limit: Option<i64>,
    /// <p>A value that enables statistics for Parquet pages and row groups. Choose <code>true</code> to enable statistics, <code>false</code> to disable. Statistics include <code>NULL</code>, <code>DISTINCT</code>, <code>MAX</code>, and <code>MIN</code> values. This parameter defaults to <code>true</code>. This value is used for .parquet file format only.</p>
    #[serde(rename = "EnableStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_statistics: Option<bool>,
    /// <p><p>The type of encoding you are using: </p> <ul> <li> <p> <code>RLE<em>DICTIONARY</code> uses a combination of bit-packing and run-length encoding to store repeated values more efficiently. This is the default.</p> </li> <li> <p> <code>PLAIN</code> doesn&#39;t use encoding at all. Values are stored as they are.</p> </li> <li> <p> <code>PLAIN</em>DICTIONARY</code> builds a dictionary of the values encountered in a given column. The dictionary is stored in a dictionary page for each column chunk.</p> </li> </ul></p>
    #[serde(rename = "EncodingType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding_type: Option<String>,
    /// <p><p>The type of server-side encryption that you want to use for your data. This encryption type is part of the endpoint settings or the extra connections attributes for Amazon S3. You can choose either <code>SSE<em>S3</code> (the default) or <code>SSE</em>KMS</code>. </p> <note> <p>For the <code>ModifyEndpoint</code> operation, you can change the existing value of the <code>EncryptionMode</code> parameter from <code>SSE<em>KMS</code> to <code>SSE</em>S3</code>. But you can’t change the existing value from <code>SSE<em>S3</code> to <code>SSE</em>KMS</code>.</p> </note> <p>To use <code>SSE_S3</code>, you need an AWS Identity and Access Management (IAM) role with permission to allow <code>&quot;arn:aws:s3:::dms-*&quot;</code> to use the following actions:</p> <ul> <li> <p> <code>s3:CreateBucket</code> </p> </li> <li> <p> <code>s3:ListBucket</code> </p> </li> <li> <p> <code>s3:DeleteBucket</code> </p> </li> <li> <p> <code>s3:GetBucketLocation</code> </p> </li> <li> <p> <code>s3:GetObject</code> </p> </li> <li> <p> <code>s3:PutObject</code> </p> </li> <li> <p> <code>s3:DeleteObject</code> </p> </li> <li> <p> <code>s3:GetObjectVersion</code> </p> </li> <li> <p> <code>s3:GetBucketPolicy</code> </p> </li> <li> <p> <code>s3:PutBucketPolicy</code> </p> </li> <li> <p> <code>s3:DeleteBucketPolicy</code> </p> </li> </ul></p>
    #[serde(rename = "EncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_mode: Option<String>,
    /// <p> Specifies how tables are defined in the S3 source files only. </p>
    #[serde(rename = "ExternalTableDefinition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_table_definition: Option<String>,
    /// <p><p>A value that enables a full load to write INSERT operations to the comma-separated value (.csv) output files only to indicate how the rows were added to the source database.</p> <note> <p>AWS DMS supports the <code>IncludeOpForFullLoad</code> parameter in versions 3.1.4 and later.</p> </note> <p>For full load, records can only be inserted. By default (the <code>false</code> setting), no information is recorded in these output files for a full load to indicate that the rows were inserted at the source database. If <code>IncludeOpForFullLoad</code> is set to <code>true</code> or <code>y</code>, the INSERT is recorded as an I annotation in the first field of the .csv file. This allows the format of your target records from a full load to be consistent with the target records from a CDC load.</p> <note> <p>This setting works together with the <code>CdcInsertsOnly</code> and the <code>CdcInsertsAndUpdates</code> parameters for output to .csv files only. For more information about how these settings work together, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.Configuring.InsertOps">Indicating Source DB Operations in Migrated S3 Data</a> in the <i>AWS Database Migration Service User Guide.</i>.</p> </note></p>
    #[serde(rename = "IncludeOpForFullLoad")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_op_for_full_load: Option<bool>,
    /// <p><p>A value that specifies the precision of any <code>TIMESTAMP</code> column values that are written to an Amazon S3 object file in .parquet format.</p> <note> <p>AWS DMS supports the <code>ParquetTimestampInMillisecond</code> parameter in versions 3.1.4 and later.</p> </note> <p>When <code>ParquetTimestampInMillisecond</code> is set to <code>true</code> or <code>y</code>, AWS DMS writes all <code>TIMESTAMP</code> columns in a .parquet formatted file with millisecond precision. Otherwise, DMS writes them with microsecond precision.</p> <p>Currently, Amazon Athena and AWS Glue can handle only millisecond precision for <code>TIMESTAMP</code> values. Set this parameter to <code>true</code> for S3 endpoint object files that are .parquet formatted only if you plan to query or process the data with Athena or AWS Glue.</p> <note> <p>AWS DMS writes any <code>TIMESTAMP</code> column values written to an S3 file in .csv format with microsecond precision.</p> <p>Setting <code>ParquetTimestampInMillisecond</code> has no effect on the string format of the timestamp column value that is inserted by setting the <code>TimestampColumnName</code> parameter.</p> </note></p>
    #[serde(rename = "ParquetTimestampInMillisecond")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_timestamp_in_millisecond: Option<bool>,
    /// <p>The version of the Apache Parquet format that you want to use: <code>parquet_1_0</code> (the default) or <code>parquet_2_0</code>.</p>
    #[serde(rename = "ParquetVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_version: Option<String>,
    /// <p><p>If set to <code>true</code>, AWS DMS saves the transaction order for a change data capture (CDC) load on the Amazon S3 target specified by <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-CdcPath"> <code>CdcPath</code> </a>. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Target.S3.html#CHAP_Target.S3.EndpointSettings.CdcPath">Capturing data changes (CDC) including transaction order on the S3 target</a>.</p> <note> <p>This setting is supported in AWS DMS versions 3.4.2 and later.</p> </note></p>
    #[serde(rename = "PreserveTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_transactions: Option<bool>,
    /// <p>The number of rows in a row group. A smaller row group size provides faster reads. But as the number of row groups grows, the slower writes become. This parameter defaults to 10,000 rows. This number is used for .parquet file format only. </p> <p>If you choose a value larger than the maximum, <code>RowGroupLength</code> is set to the max row group length in bytes (64 * 1024 * 1024). </p>
    #[serde(rename = "RowGroupLength")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_group_length: Option<i64>,
    /// <p>If you are using <code>SSE_KMS</code> for the <code>EncryptionMode</code>, provide the AWS KMS key ID. The key that you use needs an attached policy that enables AWS Identity and Access Management (IAM) user permissions and allows use of the key.</p> <p>Here is a CLI example: <code>aws dms create-endpoint --endpoint-identifier <i>value</i> --endpoint-type target --engine-name s3 --s3-settings ServiceAccessRoleArn=<i>value</i>,BucketFolder=<i>value</i>,BucketName=<i>value</i>,EncryptionMode=SSE_KMS,ServerSideEncryptionKmsKeyId=<i>value</i> </code> </p>
    #[serde(rename = "ServerSideEncryptionKmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_side_encryption_kms_key_id: Option<String>,
    /// <p> The Amazon Resource Name (ARN) used by the service access IAM role. It is a required parameter that enables DMS to write and read objects from an 3S bucket.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_access_role_arn: Option<String>,
    /// <p>A value that when nonblank causes AWS DMS to add a column with timestamp information to the endpoint data for an Amazon S3 target.</p> <note> <p>AWS DMS supports the <code>TimestampColumnName</code> parameter in versions 3.1.4 and later.</p> </note> <p>DMS includes an additional <code>STRING</code> column in the .csv or .parquet object files of your migrated data when you set <code>TimestampColumnName</code> to a nonblank value.</p> <p>For a full load, each row of this timestamp column contains a timestamp for when the data was transferred from the source to the target by DMS. </p> <p>For a change data capture (CDC) load, each row of the timestamp column contains the timestamp for the commit of that row in the source database.</p> <p>The string format for this timestamp column value is <code>yyyy-MM-dd HH:mm:ss.SSSSSS</code>. By default, the precision of this value is in microseconds. For a CDC load, the rounding of the precision depends on the commit timestamp supported by DMS for the source database.</p> <p>When the <code>AddColumnName</code> parameter is set to <code>true</code>, DMS also includes a name for the timestamp column that you set with <code>TimestampColumnName</code>.</p>
    #[serde(rename = "TimestampColumnName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_column_name: Option<String>,
    /// <p><p>This setting applies if the S3 output files during a change data capture (CDC) load are written in .csv format. If set to <code>true</code> for columns not included in the supplemental log, AWS DMS uses the value specified by <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_S3Settings.html#DMS-Type-S3Settings-CsvNoSupValue"> <code>CsvNoSupValue</code> </a>. If not set or set to <code>false</code>, AWS DMS uses the null value for these columns.</p> <note> <p>This setting is supported in AWS DMS versions 3.4.1 and later.</p> </note></p>
    #[serde(rename = "UseCsvNoSupValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_csv_no_sup_value: Option<bool>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task_assessment]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReplicationTaskAssessmentMessage {
    /// <p> The Amazon Resource Name (ARN) of the replication task. </p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task_assessment]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReplicationTaskAssessmentResponse {
    /// <p> The assessed replication task. </p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task_assessment_run]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReplicationTaskAssessmentRunMessage {
    /// <p>Unique name to identify the assessment run.</p>
    #[serde(rename = "AssessmentRunName")]
    pub assessment_run_name: String,
    /// <p><p>Space-separated list of names for specific individual assessments that you want to exclude. These names come from the default list of individual assessments that AWS DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note> <p>You can&#39;t set a value for <code>Exclude</code> if you also set a value for <code>IncludeOnly</code> in the API operation.</p> <p>To identify the names of the default individual assessments that AWS DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p> </note></p>
    #[serde(rename = "Exclude")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
    /// <p><p>Space-separated list of names for specific individual assessments that you want to include. These names come from the default list of individual assessments that AWS DMS supports for the associated migration task. This task is specified by <code>ReplicationTaskArn</code>.</p> <note> <p>You can&#39;t set a value for <code>IncludeOnly</code> if you also set a value for <code>Exclude</code> in the API operation. </p> <p>To identify the names of the default individual assessments that AWS DMS supports for the associated migration task, run the <code>DescribeApplicableIndividualAssessments</code> operation using its own <code>ReplicationTaskArn</code> request parameter.</p> </note></p>
    #[serde(rename = "IncludeOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_only: Option<Vec<String>>,
    /// <p>Amazon Resource Name (ARN) of the migration task associated with the premigration assessment run that you want to start.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p><p>Encryption mode that you can specify to encrypt the results of this assessment run. If you don&#39;t specify this request parameter, AWS DMS stores the assessment run results without encryption. You can specify one of the options following:</p> <ul> <li> <p> <code>&quot;SSE<em>S3&quot;</code> – The server-side encryption provided as a default by Amazon S3.</p> </li> <li> <p> <code>&quot;SSE</em>KMS&quot;</code> – AWS Key Management Service (AWS KMS) encryption. This encryption can use either a custom KMS encryption key that you specify or the default KMS encryption key that DMS provides.</p> </li> </ul></p>
    #[serde(rename = "ResultEncryptionMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_encryption_mode: Option<String>,
    /// <p>ARN of a custom KMS encryption key that you specify when you set <code>ResultEncryptionMode</code> to <code>"SSE_KMS</code>".</p>
    #[serde(rename = "ResultKmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_kms_key_arn: Option<String>,
    /// <p>Amazon S3 bucket where you want AWS DMS to store the results of this assessment run.</p>
    #[serde(rename = "ResultLocationBucket")]
    pub result_location_bucket: String,
    /// <p>Folder within an Amazon S3 bucket where you want AWS DMS to store the results of this assessment run.</p>
    #[serde(rename = "ResultLocationFolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_location_folder: Option<String>,
    /// <p>ARN of a service role needed to start the assessment run.</p>
    #[serde(rename = "ServiceAccessRoleArn")]
    pub service_access_role_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task_assessment_run]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReplicationTaskAssessmentRunResponse {
    /// <p>The premigration assessment run that was started.</p>
    #[serde(rename = "ReplicationTaskAssessmentRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task_assessment_run: Option<ReplicationTaskAssessmentRun>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartReplicationTaskMessage {
    /// <p><p>Indicates when you want a change data capture (CDC) operation to start. Use either CdcStartPosition or CdcStartTime to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p> The value can be in date, checkpoint, or LSN/SCN format.</p> <p>Date Example: --cdc-start-position “2018-03-08T12:12:12”</p> <p>Checkpoint Example: --cdc-start-position &quot;checkpoint:V1#27#mysql-bin-changelog.157832:1975:-1:2002:677883278264080:mysql-bin-changelog.157832:1876#0#0#*#0#93&quot;</p> <p>LSN Example: --cdc-start-position “mysql-bin-changelog.000024:373”</p> <note> <p>When you use this task setting with a source PostgreSQL database, a logical replication slot should already be created and associated with the source endpoint. You can verify this by setting the <code>slotName</code> extra connection attribute to the name of this logical replication slot. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Source.PostgreSQL.html#CHAP_Source.PostgreSQL.ConnectionAttrib">Extra Connection Attributes When Using PostgreSQL as a Source for AWS DMS</a>.</p> </note></p>
    #[serde(rename = "CdcStartPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_position: Option<String>,
    /// <p>Indicates the start time for a change data capture (CDC) operation. Use either CdcStartTime or CdcStartPosition to specify when you want a CDC operation to start. Specifying both values results in an error.</p> <p>Timestamp Example: --cdc-start-time “2018-03-08T12:12:12”</p>
    #[serde(rename = "CdcStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_start_time: Option<f64>,
    /// <p>Indicates when you want a change data capture (CDC) operation to stop. The value can be either server time or commit time.</p> <p>Server time example: --cdc-stop-position “server_time:2018-02-09T12:12:12”</p> <p>Commit time example: --cdc-stop-position “commit_time: 2018-02-09T12:12:12 “</p>
    #[serde(rename = "CdcStopPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdc_stop_position: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the replication task to be started.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
    /// <p>A type of replication task.</p>
    #[serde(rename = "StartReplicationTaskType")]
    pub start_replication_task_type: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::start_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartReplicationTaskResponse {
    /// <p>The replication task started.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::stop_replication_task]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopReplicationTaskMessage {
    /// <p>The Amazon Resource Name(ARN) of the replication task to be stopped.</p>
    #[serde(rename = "ReplicationTaskArn")]
    pub replication_task_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::stop_replication_task]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopReplicationTaskResponse {
    /// <p>The replication task stopped.</p>
    #[serde(rename = "ReplicationTask")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_task: Option<ReplicationTask>,
}

/// <p>In response to a request by the <code>DescribeReplicationSubnetGroups</code> operation, this object identifies a subnet by its given Availability Zone, subnet identifier, and status.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Subnet {
    /// <p>The Availability Zone of the subnet.</p>
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    /// <p>The subnet identifier.</p>
    #[serde(rename = "SubnetIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    /// <p>The status of the subnet.</p>
    #[serde(rename = "SubnetStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

/// <p>Provides information about types of supported endpoints in response to a request by the <code>DescribeEndpointTypes</code> operation. This information includes the type of endpoint, the database engine name, and whether change data capture (CDC) is supported.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SupportedEndpointType {
    /// <p>The type of endpoint. Valid values are <code>source</code> and <code>target</code>.</p>
    #[serde(rename = "EndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    /// <p>The expanded name for the engine name. For example, if the <code>EngineName</code> parameter is "aurora," this value would be "Amazon Aurora MySQL."</p>
    #[serde(rename = "EngineDisplayName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_display_name: Option<String>,
    /// <p>The database engine name. Valid values, depending on the EndpointType, include <code>"mysql"</code>, <code>"oracle"</code>, <code>"postgres"</code>, <code>"mariadb"</code>, <code>"aurora"</code>, <code>"aurora-postgresql"</code>, <code>"redshift"</code>, <code>"s3"</code>, <code>"db2"</code>, <code>"azuredb"</code>, <code>"sybase"</code>, <code>"dynamodb"</code>, <code>"mongodb"</code>, <code>"kinesis"</code>, <code>"kafka"</code>, <code>"elasticsearch"</code>, <code>"documentdb"</code>, <code>"sqlserver"</code>, and <code>"neptune"</code>.</p>
    #[serde(rename = "EngineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_name: Option<String>,
    /// <p>The earliest AWS DMS engine version that supports this endpoint engine. Note that endpoint engines released with AWS DMS versions earlier than 3.1.1 do not return a value for this parameter.</p>
    #[serde(rename = "ReplicationInstanceEngineMinimumVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_instance_engine_minimum_version: Option<String>,
    /// <p>Indicates if Change Data Capture (CDC) is supported.</p>
    #[serde(rename = "SupportsCDC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_cdc: Option<bool>,
}

/// <p>Provides information that defines a SAP ASE endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SybaseSettings {
    /// <p>Database name for the endpoint.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>Endpoint connection password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>Endpoint TCP port.</p>
    #[serde(rename = "Port")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    /// <p><p>The full Amazon Resource Name (ARN) of the IAM role that specifies AWS DMS as the trusted entity and grants the required permissions to access the value in <code>SecretsManagerSecret</code>. <code>SecretsManagerSecret</code> has the value of the AWS Secrets Manager secret that allows access to the SAP ASE endpoint.</p> <note> <p>You can specify one of two sets of values for these permissions. You can specify the values for this setting and <code>SecretsManagerSecretId</code>. Or you can specify clear-text values for <code>UserName</code>, <code>Password</code>, <code>ServerName</code>, and <code>Port</code>. You can&#39;t specify both. For more information on creating this <code>SecretsManagerSecret</code> and the <code>SecretsManagerAccessRoleArn</code> and <code>SecretsManagerSecretId</code> required to access it, see <a href="https://docs.aws.amazon.com/https:/docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#security-iam-secretsmanager">Using secrets to access AWS Database Migration Service resources</a> in the <i>AWS Database Migration Service User Guide</i>.</p> </note></p>
    #[serde(rename = "SecretsManagerAccessRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_access_role_arn: Option<String>,
    /// <p>The full ARN, partial ARN, or friendly name of the <code>SecretsManagerSecret</code> that contains the SAP SAE endpoint connection details.</p>
    #[serde(rename = "SecretsManagerSecretId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets_manager_secret_id: Option<String>,
    /// <p>Fully qualified domain name of the endpoint.</p>
    #[serde(rename = "ServerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_name: Option<String>,
    /// <p>Endpoint connection user name.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Provides a collection of table statistics in response to a request by the <code>DescribeTableStatistics</code> operation.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TableStatistics {
    /// <p>The data definition language (DDL) used to build and modify the structure of your tables.</p>
    #[serde(rename = "Ddls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ddls: Option<i64>,
    /// <p>The number of delete actions performed on a table.</p>
    #[serde(rename = "Deletes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    /// <p>The number of rows that failed conditional checks during the full load operation (valid only for migrations where DynamoDB is the target).</p>
    #[serde(rename = "FullLoadCondtnlChkFailedRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_condtnl_chk_failed_rows: Option<i64>,
    /// <p>The time when the full load operation completed.</p>
    #[serde(rename = "FullLoadEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_end_time: Option<f64>,
    /// <p>The number of rows that failed to load during the full load operation (valid only for migrations where DynamoDB is the target).</p>
    #[serde(rename = "FullLoadErrorRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_error_rows: Option<i64>,
    /// <p>A value that indicates if the table was reloaded (<code>true</code>) or loaded as part of a new full load operation (<code>false</code>).</p>
    #[serde(rename = "FullLoadReloaded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_reloaded: Option<bool>,
    /// <p>The number of rows added during the full load operation.</p>
    #[serde(rename = "FullLoadRows")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_rows: Option<i64>,
    /// <p>The time when the full load operation started.</p>
    #[serde(rename = "FullLoadStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_load_start_time: Option<f64>,
    /// <p>The number of insert actions performed on a table.</p>
    #[serde(rename = "Inserts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inserts: Option<i64>,
    /// <p>The last time a table was updated.</p>
    #[serde(rename = "LastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The schema name.</p>
    #[serde(rename = "SchemaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_name: Option<String>,
    /// <p>The name of the table.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>The state of the tables described.</p> <p>Valid states: Table does not exist | Before load | Full load | Table completed | Table cancelled | Table error | Table all | Table updates | Table is being reloaded</p>
    #[serde(rename = "TableState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_state: Option<String>,
    /// <p>The number of update actions performed on a table.</p>
    #[serde(rename = "Updates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<i64>,
    /// <p>The number of records that failed validation.</p>
    #[serde(rename = "ValidationFailedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_failed_records: Option<i64>,
    /// <p>The number of records that have yet to be validated.</p>
    #[serde(rename = "ValidationPendingRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_pending_records: Option<i64>,
    /// <p><p>The validation state of the table.</p> <p>This parameter can have the following values:</p> <ul> <li> <p>Not enabled – Validation isn&#39;t enabled for the table in the migration task.</p> </li> <li> <p>Pending records – Some records in the table are waiting for validation.</p> </li> <li> <p>Mismatched records – Some records in the table don&#39;t match between the source and target.</p> </li> <li> <p>Suspended records – Some records in the table couldn&#39;t be validated.</p> </li> <li> <p>No primary key –The table couldn&#39;t be validated because it has no primary key.</p> </li> <li> <p>Table error – The table wasn&#39;t validated because it&#39;s in an error state and some data wasn&#39;t migrated.</p> </li> <li> <p>Validated – All rows in the table are validated. If the table is updated, the status can change from Validated.</p> </li> <li> <p>Error – The table couldn&#39;t be validated because of an unexpected error.</p> </li> <li> <p>Pending validation – The table is waiting validation.</p> </li> <li> <p>Preparing table – Preparing the table enabled in the migration task for validation.</p> </li> <li> <p>Pending revalidation – All rows in the table are pending validation after the table was updated.</p> </li> </ul></p>
    #[serde(rename = "ValidationState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state: Option<String>,
    /// <p>Additional details about the state of validation.</p>
    #[serde(rename = "ValidationStateDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_state_details: Option<String>,
    /// <p>The number of records that couldn't be validated.</p>
    #[serde(rename = "ValidationSuspendedRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_suspended_records: Option<i64>,
}

/// <p>Provides the name of the schema and table to be reloaded.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TableToReload {
    /// <p>The schema name of the table to be reloaded.</p>
    #[serde(rename = "SchemaName")]
    pub schema_name: String,
    /// <p>The table name of the table to be reloaded.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p><p>A user-defined key-value pair that describes metadata added to an AWS DMS resource and that is used by operations such as the following:</p> <ul> <li> <p> <code>AddTagsToResource</code> </p> </li> <li> <p> <code>ListTagsForResource</code> </p> </li> <li> <p> <code>RemoveTagsFromResource</code> </p> </li> </ul></p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A key is the required name of the tag. The string value can be 1-128 Unicode characters in length and can't be prefixed with "aws:" or "dms:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regular expressions: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>A value is the optional value of the tag. The string value can be 1-256 Unicode characters in length and can't be prefixed with "aws:" or "dms:". The string can only contain only the set of Unicode letters, digits, white-space, '_', '.', '/', '=', '+', '-' (Java regular expressions: "^([\\p{L}\\p{Z}\\p{N}_.:/=+\\-]*)$").</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::test_connection]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TestConnectionMessage {
    /// <p>The Amazon Resource Name (ARN) string that uniquely identifies the endpoint.</p>
    #[serde(rename = "EndpointArn")]
    pub endpoint_arn: String,
    /// <p>The Amazon Resource Name (ARN) of the replication instance.</p>
    #[serde(rename = "ReplicationInstanceArn")]
    pub replication_instance_arn: String,
}

/// <p><p/></p>
/// see [DatabaseMigrationService::test_connection]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TestConnectionResponse {
    /// <p>The connection tested.</p>
    #[serde(rename = "Connection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<Connection>,
}

/// <p>Describes the status of a security group associated with the virtual private cloud (VPC) hosting your replication and DB instances.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcSecurityGroupMembership {
    /// <p>The status of the VPC security group.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The VPC security group ID.</p>
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

/// Errors returned by AddTagsToResource
#[derive(Debug, PartialEq)]
pub enum AddTagsToResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl AddTagsToResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddTagsToResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(AddTagsToResourceError::ResourceNotFoundFault(
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
impl fmt::Display for AddTagsToResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddTagsToResourceError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddTagsToResourceError {}
/// Errors returned by ApplyPendingMaintenanceAction
#[derive(Debug, PartialEq)]
pub enum ApplyPendingMaintenanceActionError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ApplyPendingMaintenanceActionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ApplyPendingMaintenanceActionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ApplyPendingMaintenanceActionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
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
/// Errors returned by CancelReplicationTaskAssessmentRun
#[derive(Debug, PartialEq)]
pub enum CancelReplicationTaskAssessmentRunError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl CancelReplicationTaskAssessmentRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CancelReplicationTaskAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        CancelReplicationTaskAssessmentRunError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        CancelReplicationTaskAssessmentRunError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CancelReplicationTaskAssessmentRunError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelReplicationTaskAssessmentRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelReplicationTaskAssessmentRunError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelReplicationTaskAssessmentRunError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CancelReplicationTaskAssessmentRunError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CancelReplicationTaskAssessmentRunError {}
/// Errors returned by CreateEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateEndpointError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>Insufficient privileges are preventing access to an Amazon S3 object.</p>
    S3AccessDeniedFault(String),
}

impl CreateEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateEndpointError::AccessDeniedFault(err.msg))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(CreateEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(CreateEndpointError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(CreateEndpointError::ResourceQuotaExceededFault(
                        err.msg,
                    ))
                }
                "S3AccessDeniedFault" => {
                    return RusotoError::Service(CreateEndpointError::S3AccessDeniedFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEndpointError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::ResourceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
            CreateEndpointError::S3AccessDeniedFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEndpointError {}
/// Errors returned by CreateEventSubscription
#[derive(Debug, PartialEq)]
pub enum CreateEventSubscriptionError {
    /// <p>The ciphertext references a key that doesn't exist or that the DMS account doesn't have access to.</p>
    KMSAccessDeniedFault(String),
    /// <p>The specified master key (CMK) isn't enabled.</p>
    KMSDisabledFault(String),
    /// <p>The state of the specified AWS KMS resource isn't valid for this request.</p>
    KMSInvalidStateFault(String),
    /// <p>The specified AWS KMS entity or resource can't be found.</p>
    KMSNotFoundFault(String),
    /// <p>This request triggered AWS KMS request throttling.</p>
    KMSThrottlingFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
}

impl CreateEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "KMSAccessDeniedFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::KMSAccessDeniedFault(err.msg),
                    )
                }
                "KMSDisabledFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSDisabledFault(
                        err.msg,
                    ))
                }
                "KMSInvalidStateFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::KMSInvalidStateFault(err.msg),
                    )
                }
                "KMSNotFoundFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSNotFoundFault(
                        err.msg,
                    ))
                }
                "KMSThrottlingFault" => {
                    return RusotoError::Service(CreateEventSubscriptionError::KMSThrottlingFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SNSInvalidTopicFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::SNSInvalidTopicFault(err.msg),
                    )
                }
                "SNSNoAuthorizationFault" => {
                    return RusotoError::Service(
                        CreateEventSubscriptionError::SNSNoAuthorizationFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventSubscriptionError::KMSAccessDeniedFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::KMSDisabledFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::KMSInvalidStateFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::KMSNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::KMSThrottlingFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSubscriptionError::SNSInvalidTopicFault(ref cause) => write!(f, "{}", cause),
            CreateEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateEventSubscriptionError {}
/// Errors returned by CreateReplicationInstance
#[derive(Debug, PartialEq)]
pub enum CreateReplicationInstanceError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
}

impl CreateReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateReplicationInstanceError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InsufficientResourceCapacityFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::InsufficientResourceCapacityFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(CreateReplicationInstanceError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                            err.msg,
                        ),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "StorageQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationInstanceError::StorageQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateReplicationInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReplicationInstanceError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            CreateReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateReplicationInstanceError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationInstanceError::StorageQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateReplicationInstanceError {}
/// Errors returned by CreateReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum CreateReplicationSubnetGroupError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl CreateReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CreateReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(CreateReplicationSubnetGroupError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => return RusotoError::Service(
                    CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                        err.msg,
                    ),
                ),
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateReplicationSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReplicationSubnetGroupError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            CreateReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => write!(f, "{}", cause),
            CreateReplicationSubnetGroupError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateReplicationSubnetGroupError {}
/// Errors returned by CreateReplicationTask
#[derive(Debug, PartialEq)]
pub enum CreateReplicationTaskError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl CreateReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(CreateReplicationTaskError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(CreateReplicationTaskError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        CreateReplicationTaskError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateReplicationTaskError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            CreateReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
            CreateReplicationTaskError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for CreateReplicationTaskError {}
/// Errors returned by DeleteCertificate
#[derive(Debug, PartialEq)]
pub enum DeleteCertificateError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DeleteCertificateError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteCertificateError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCertificateError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            DeleteCertificateError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCertificateError {}
/// Errors returned by DeleteConnection
#[derive(Debug, PartialEq)]
pub enum DeleteConnectionError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(DeleteConnectionError::AccessDeniedFault(err.msg))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DeleteConnectionError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteConnectionError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteConnectionError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            DeleteConnectionError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteConnectionError {}
/// Errors returned by DeleteEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteEndpointError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DeleteEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteEndpointError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEndpointError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            DeleteEndpointError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEndpointError {}
/// Errors returned by DeleteEventSubscription
#[derive(Debug, PartialEq)]
pub enum DeleteEventSubscriptionError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteEventSubscriptionError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventSubscriptionError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteEventSubscriptionError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteEventSubscriptionError {}
/// Errors returned by DeleteReplicationInstance
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationInstanceError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReplicationInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationInstanceError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationInstanceError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteReplicationInstanceError {}
/// Errors returned by DeleteReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationSubnetGroupError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationSubnetGroupError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReplicationSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationSubnetGroupError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteReplicationSubnetGroupError {}
/// Errors returned by DeleteReplicationTask
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DeleteReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for DeleteReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteReplicationTaskError {}
/// Errors returned by DeleteReplicationTaskAssessmentRun
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationTaskAssessmentRunError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DeleteReplicationTaskAssessmentRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteReplicationTaskAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        DeleteReplicationTaskAssessmentRunError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DeleteReplicationTaskAssessmentRunError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DeleteReplicationTaskAssessmentRunError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteReplicationTaskAssessmentRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteReplicationTaskAssessmentRunError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationTaskAssessmentRunError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteReplicationTaskAssessmentRunError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteReplicationTaskAssessmentRunError {}
/// Errors returned by DescribeAccountAttributes
#[derive(Debug, PartialEq)]
pub enum DescribeAccountAttributesError {}

impl DescribeAccountAttributesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAccountAttributesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAccountAttributesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeAccountAttributesError {}
/// Errors returned by DescribeApplicableIndividualAssessments
#[derive(Debug, PartialEq)]
pub enum DescribeApplicableIndividualAssessmentsError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeApplicableIndividualAssessmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeApplicableIndividualAssessmentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        DescribeApplicableIndividualAssessmentsError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeApplicableIndividualAssessmentsError::InvalidResourceStateFault(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeApplicableIndividualAssessmentsError::ResourceNotFoundFault(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeApplicableIndividualAssessmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeApplicableIndividualAssessmentsError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeApplicableIndividualAssessmentsError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeApplicableIndividualAssessmentsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeApplicableIndividualAssessmentsError {}
/// Errors returned by DescribeCertificates
#[derive(Debug, PartialEq)]
pub enum DescribeCertificatesError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeCertificatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeCertificatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeCertificatesError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeCertificatesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeCertificatesError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeCertificatesError {}
/// Errors returned by DescribeConnections
#[derive(Debug, PartialEq)]
pub enum DescribeConnectionsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeConnectionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConnectionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeConnectionsError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeConnectionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConnectionsError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConnectionsError {}
/// Errors returned by DescribeEndpointTypes
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointTypesError {}

impl DescribeEndpointTypesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointTypesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEndpointTypesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEndpointTypesError {}
/// Errors returned by DescribeEndpoints
#[derive(Debug, PartialEq)]
pub enum DescribeEndpointsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEndpointsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeEndpointsError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEndpointsError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeEndpointsError {}
/// Errors returned by DescribeEventCategories
#[derive(Debug, PartialEq)]
pub enum DescribeEventCategoriesError {}

impl DescribeEventCategoriesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeEventCategoriesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
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
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeEventSubscriptionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeEventSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeEventSubscriptionsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventSubscriptionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeEventSubscriptionsError::ResourceNotFoundFault(ref cause) => {
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
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeEventsError {}
/// Errors returned by DescribeOrderableReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeOrderableReplicationInstancesError {}

impl DescribeOrderableReplicationInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeOrderableReplicationInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeOrderableReplicationInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for DescribeOrderableReplicationInstancesError {}
/// Errors returned by DescribePendingMaintenanceActions
#[derive(Debug, PartialEq)]
pub enum DescribePendingMaintenanceActionsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribePendingMaintenanceActionsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribePendingMaintenanceActionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribePendingMaintenanceActionsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
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
/// Errors returned by DescribeRefreshSchemasStatus
#[derive(Debug, PartialEq)]
pub enum DescribeRefreshSchemasStatusError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeRefreshSchemasStatusError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeRefreshSchemasStatusError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeRefreshSchemasStatusError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeRefreshSchemasStatusError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeRefreshSchemasStatusError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeRefreshSchemasStatusError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeRefreshSchemasStatusError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeRefreshSchemasStatusError {}
/// Errors returned by DescribeReplicationInstanceTaskLogs
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationInstanceTaskLogsError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationInstanceTaskLogsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationInstanceTaskLogsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstanceTaskLogsError::InvalidResourceStateFault(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstanceTaskLogsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationInstanceTaskLogsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationInstanceTaskLogsError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeReplicationInstanceTaskLogsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationInstanceTaskLogsError {}
/// Errors returned by DescribeReplicationInstances
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationInstancesError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationInstancesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationInstancesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationInstancesError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationInstancesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationInstancesError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationInstancesError {}
/// Errors returned by DescribeReplicationSubnetGroups
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationSubnetGroupsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationSubnetGroupsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationSubnetGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationSubnetGroupsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationSubnetGroupsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationSubnetGroupsError {}
/// Errors returned by DescribeReplicationTaskAssessmentResults
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTaskAssessmentResultsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTaskAssessmentResultsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationTaskAssessmentResultsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTaskAssessmentResultsError::ResourceNotFoundFault(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationTaskAssessmentResultsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationTaskAssessmentResultsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationTaskAssessmentResultsError {}
/// Errors returned by DescribeReplicationTaskAssessmentRuns
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTaskAssessmentRunsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTaskAssessmentRunsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationTaskAssessmentRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTaskAssessmentRunsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationTaskAssessmentRunsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationTaskAssessmentRunsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationTaskAssessmentRunsError {}
/// Errors returned by DescribeReplicationTaskIndividualAssessments
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTaskIndividualAssessmentsError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTaskIndividualAssessmentsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeReplicationTaskIndividualAssessmentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTaskIndividualAssessmentsError::ResourceNotFoundFault(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationTaskIndividualAssessmentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationTaskIndividualAssessmentsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationTaskIndividualAssessmentsError {}
/// Errors returned by DescribeReplicationTasks
#[derive(Debug, PartialEq)]
pub enum DescribeReplicationTasksError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeReplicationTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeReplicationTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeReplicationTasksError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeReplicationTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeReplicationTasksError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeReplicationTasksError {}
/// Errors returned by DescribeSchemas
#[derive(Debug, PartialEq)]
pub enum DescribeSchemasError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeSchemasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(DescribeSchemasError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(DescribeSchemasError::ResourceNotFoundFault(
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
impl fmt::Display for DescribeSchemasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeSchemasError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            DescribeSchemasError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeSchemasError {}
/// Errors returned by DescribeTableStatistics
#[derive(Debug, PartialEq)]
pub enum DescribeTableStatisticsError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl DescribeTableStatisticsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTableStatisticsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        DescribeTableStatisticsError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        DescribeTableStatisticsError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTableStatisticsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTableStatisticsError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeTableStatisticsError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeTableStatisticsError {}
/// Errors returned by ImportCertificate
#[derive(Debug, PartialEq)]
pub enum ImportCertificateError {
    /// <p>The certificate was not valid.</p>
    InvalidCertificateFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl ImportCertificateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ImportCertificateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidCertificateFault" => {
                    return RusotoError::Service(ImportCertificateError::InvalidCertificateFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ImportCertificateError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ImportCertificateError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ImportCertificateError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ImportCertificateError::InvalidCertificateFault(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::ResourceAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            ImportCertificateError::ResourceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ImportCertificateError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFoundFault(
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
            ListTagsForResourceError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ModifyEndpoint
#[derive(Debug, PartialEq)]
pub enum ModifyEndpointError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ModifyEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyEndpointError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(ModifyEndpointError::AccessDeniedFault(err.msg))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(ModifyEndpointError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(ModifyEndpointError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(ModifyEndpointError::ResourceAlreadyExistsFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ModifyEndpointError::ResourceNotFoundFault(
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
impl fmt::Display for ModifyEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyEndpointError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            ModifyEndpointError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            ModifyEndpointError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            ModifyEndpointError::ResourceAlreadyExistsFault(ref cause) => write!(f, "{}", cause),
            ModifyEndpointError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyEndpointError {}
/// Errors returned by ModifyEventSubscription
#[derive(Debug, PartialEq)]
pub enum ModifyEventSubscriptionError {
    /// <p>The ciphertext references a key that doesn't exist or that the DMS account doesn't have access to.</p>
    KMSAccessDeniedFault(String),
    /// <p>The specified master key (CMK) isn't enabled.</p>
    KMSDisabledFault(String),
    /// <p>The state of the specified AWS KMS resource isn't valid for this request.</p>
    KMSInvalidStateFault(String),
    /// <p>The specified AWS KMS entity or resource can't be found.</p>
    KMSNotFoundFault(String),
    /// <p>This request triggered AWS KMS request throttling.</p>
    KMSThrottlingFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The SNS topic is invalid.</p>
    SNSInvalidTopicFault(String),
    /// <p>You are not authorized for the SNS subscription.</p>
    SNSNoAuthorizationFault(String),
}

impl ModifyEventSubscriptionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyEventSubscriptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "KMSAccessDeniedFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::KMSAccessDeniedFault(err.msg),
                    )
                }
                "KMSDisabledFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSDisabledFault(
                        err.msg,
                    ))
                }
                "KMSInvalidStateFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::KMSInvalidStateFault(err.msg),
                    )
                }
                "KMSNotFoundFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSNotFoundFault(
                        err.msg,
                    ))
                }
                "KMSThrottlingFault" => {
                    return RusotoError::Service(ModifyEventSubscriptionError::KMSThrottlingFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SNSInvalidTopicFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::SNSInvalidTopicFault(err.msg),
                    )
                }
                "SNSNoAuthorizationFault" => {
                    return RusotoError::Service(
                        ModifyEventSubscriptionError::SNSNoAuthorizationFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyEventSubscriptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyEventSubscriptionError::KMSAccessDeniedFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::KMSDisabledFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::KMSInvalidStateFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::KMSNotFoundFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::KMSThrottlingFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyEventSubscriptionError::SNSInvalidTopicFault(ref cause) => write!(f, "{}", cause),
            ModifyEventSubscriptionError::SNSNoAuthorizationFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyEventSubscriptionError {}
/// Errors returned by ModifyReplicationInstance
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationInstanceError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>There are not enough resources allocated to the database migration.</p>
    InsufficientResourceCapacityFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The storage quota has been exceeded.</p>
    StorageQuotaExceededFault(String),
    /// <p>An upgrade dependency is preventing the database migration.</p>
    UpgradeDependencyFailureFault(String),
}

impl ModifyReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(ModifyReplicationInstanceError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InsufficientResourceCapacityFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::InsufficientResourceCapacityFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "StorageQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::StorageQuotaExceededFault(err.msg),
                    )
                }
                "UpgradeDependencyFailureFault" => {
                    return RusotoError::Service(
                        ModifyReplicationInstanceError::UpgradeDependencyFailureFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyReplicationInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReplicationInstanceError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            ModifyReplicationInstanceError::InsufficientResourceCapacityFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationInstanceError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationInstanceError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationInstanceError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationInstanceError::StorageQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationInstanceError::UpgradeDependencyFailureFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyReplicationInstanceError {}
/// Errors returned by ModifyReplicationSubnetGroup
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationSubnetGroupError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The subnet provided is invalid.</p>
    InvalidSubnet(String),
    /// <p>The replication subnet group does not cover enough Availability Zones (AZs). Edit the replication subnet group and add more AZs.</p>
    ReplicationSubnetGroupDoesNotCoverEnoughAZs(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
    /// <p>The specified subnet is already in use.</p>
    SubnetAlreadyInUse(String),
}

impl ModifyReplicationSubnetGroupError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ModifyReplicationSubnetGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidSubnet" => {
                    return RusotoError::Service(ModifyReplicationSubnetGroupError::InvalidSubnet(
                        err.msg,
                    ))
                }
                "ReplicationSubnetGroupDoesNotCoverEnoughAZs" => return RusotoError::Service(
                    ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                        err.msg,
                    ),
                ),
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(err.msg),
                    )
                }
                "SubnetAlreadyInUse" => {
                    return RusotoError::Service(
                        ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ModifyReplicationSubnetGroupError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReplicationSubnetGroupError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationSubnetGroupError::InvalidSubnet(ref cause) => write!(f, "{}", cause),
            ModifyReplicationSubnetGroupError::ReplicationSubnetGroupDoesNotCoverEnoughAZs(
                ref cause,
            ) => write!(f, "{}", cause),
            ModifyReplicationSubnetGroupError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationSubnetGroupError::ResourceQuotaExceededFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationSubnetGroupError::SubnetAlreadyInUse(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ModifyReplicationSubnetGroupError {}
/// Errors returned by ModifyReplicationTask
#[derive(Debug, PartialEq)]
pub enum ModifyReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ModifyReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ModifyReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        ModifyReplicationTaskError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ModifyReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for ModifyReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModifyReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationTaskError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationTaskError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            ModifyReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ModifyReplicationTaskError {}
/// Errors returned by MoveReplicationTask
#[derive(Debug, PartialEq)]
pub enum MoveReplicationTaskError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl MoveReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<MoveReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(MoveReplicationTaskError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        MoveReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(MoveReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for MoveReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MoveReplicationTaskError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            MoveReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            MoveReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for MoveReplicationTaskError {}
/// Errors returned by RebootReplicationInstance
#[derive(Debug, PartialEq)]
pub enum RebootReplicationInstanceError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl RebootReplicationInstanceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RebootReplicationInstanceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        RebootReplicationInstanceError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        RebootReplicationInstanceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RebootReplicationInstanceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RebootReplicationInstanceError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            RebootReplicationInstanceError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for RebootReplicationInstanceError {}
/// Errors returned by RefreshSchemas
#[derive(Debug, PartialEq)]
pub enum RefreshSchemasError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl RefreshSchemasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RefreshSchemasError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(RefreshSchemasError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(RefreshSchemasError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(RefreshSchemasError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(RefreshSchemasError::ResourceQuotaExceededFault(
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
impl fmt::Display for RefreshSchemasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RefreshSchemasError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            RefreshSchemasError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            RefreshSchemasError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
            RefreshSchemasError::ResourceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RefreshSchemasError {}
/// Errors returned by ReloadTables
#[derive(Debug, PartialEq)]
pub enum ReloadTablesError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl ReloadTablesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ReloadTablesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(ReloadTablesError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(ReloadTablesError::ResourceNotFoundFault(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ReloadTablesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ReloadTablesError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            ReloadTablesError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ReloadTablesError {}
/// Errors returned by RemoveTagsFromResource
#[derive(Debug, PartialEq)]
pub enum RemoveTagsFromResourceError {
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl RemoveTagsFromResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemoveTagsFromResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        RemoveTagsFromResourceError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveTagsFromResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveTagsFromResourceError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveTagsFromResourceError {}
/// Errors returned by StartReplicationTask
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StartReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(StartReplicationTaskError::AccessDeniedFault(
                        err.msg,
                    ))
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(StartReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for StartReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReplicationTaskError::AccessDeniedFault(ref cause) => write!(f, "{}", cause),
            StartReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartReplicationTaskError {}
/// Errors returned by StartReplicationTaskAssessment
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskAssessmentError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StartReplicationTaskAssessmentError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartReplicationTaskAssessmentError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentError::ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartReplicationTaskAssessmentError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReplicationTaskAssessmentError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartReplicationTaskAssessmentError {}
/// Errors returned by StartReplicationTaskAssessmentRun
#[derive(Debug, PartialEq)]
pub enum StartReplicationTaskAssessmentRunError {
    /// <p>AWS DMS was denied access to the endpoint. Check that the role is correctly configured.</p>
    AccessDeniedFault(String),
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The ciphertext references a key that doesn't exist or that the DMS account doesn't have access to.</p>
    KMSAccessDeniedFault(String),
    /// <p>The specified master key (CMK) isn't enabled.</p>
    KMSDisabledFault(String),
    /// <p>An AWS Key Management Service (AWS KMS) error is preventing access to AWS KMS.</p>
    KMSFault(String),
    /// <p>The state of the specified AWS KMS resource isn't valid for this request.</p>
    KMSInvalidStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The specified AWS KMS entity or resource can't be found.</p>
    KMSNotFoundFault(String),
    /// <p>The resource you are attempting to create already exists.</p>
    ResourceAlreadyExistsFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>Insufficient privileges are preventing access to an Amazon S3 object.</p>
    S3AccessDeniedFault(String),
    /// <p>A specified Amazon S3 bucket, bucket folder, or other object can't be found.</p>
    S3ResourceNotFoundFault(String),
}

impl StartReplicationTaskAssessmentRunError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartReplicationTaskAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::AccessDeniedFault(err.msg),
                    )
                }
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::InvalidResourceStateFault(err.msg),
                    )
                }
                "KMSAccessDeniedFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::KMSAccessDeniedFault(err.msg),
                    )
                }
                "KMSDisabledFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::KMSDisabledFault(err.msg),
                    )
                }
                "KMSFault" => {
                    return RusotoError::Service(StartReplicationTaskAssessmentRunError::KMSFault(
                        err.msg,
                    ))
                }
                "KMSInvalidStateFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::KMSInvalidStateFault(err.msg),
                    )
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::KMSKeyNotAccessibleFault(err.msg),
                    )
                }
                "KMSNotFoundFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::KMSNotFoundFault(err.msg),
                    )
                }
                "ResourceAlreadyExistsFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::ResourceAlreadyExistsFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::ResourceNotFoundFault(err.msg),
                    )
                }
                "S3AccessDeniedFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::S3AccessDeniedFault(err.msg),
                    )
                }
                "S3ResourceNotFoundFault" => {
                    return RusotoError::Service(
                        StartReplicationTaskAssessmentRunError::S3ResourceNotFoundFault(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartReplicationTaskAssessmentRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartReplicationTaskAssessmentRunError::AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::KMSAccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::KMSDisabledFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::KMSFault(ref cause) => write!(f, "{}", cause),
            StartReplicationTaskAssessmentRunError::KMSInvalidStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::KMSKeyNotAccessibleFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::KMSNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::ResourceAlreadyExistsFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::S3AccessDeniedFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StartReplicationTaskAssessmentRunError::S3ResourceNotFoundFault(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartReplicationTaskAssessmentRunError {}
/// Errors returned by StopReplicationTask
#[derive(Debug, PartialEq)]
pub enum StopReplicationTaskError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
}

impl StopReplicationTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopReplicationTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(
                        StopReplicationTaskError::InvalidResourceStateFault(err.msg),
                    )
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(StopReplicationTaskError::ResourceNotFoundFault(
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
impl fmt::Display for StopReplicationTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopReplicationTaskError::InvalidResourceStateFault(ref cause) => {
                write!(f, "{}", cause)
            }
            StopReplicationTaskError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopReplicationTaskError {}
/// Errors returned by TestConnection
#[derive(Debug, PartialEq)]
pub enum TestConnectionError {
    /// <p>The resource is in a state that prevents it from being used for database migration.</p>
    InvalidResourceStateFault(String),
    /// <p>AWS DMS cannot access the AWS KMS key.</p>
    KMSKeyNotAccessibleFault(String),
    /// <p>The resource could not be found.</p>
    ResourceNotFoundFault(String),
    /// <p>The quota for this resource quota has been exceeded.</p>
    ResourceQuotaExceededFault(String),
}

impl TestConnectionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TestConnectionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidResourceStateFault" => {
                    return RusotoError::Service(TestConnectionError::InvalidResourceStateFault(
                        err.msg,
                    ))
                }
                "KMSKeyNotAccessibleFault" => {
                    return RusotoError::Service(TestConnectionError::KMSKeyNotAccessibleFault(
                        err.msg,
                    ))
                }
                "ResourceNotFoundFault" => {
                    return RusotoError::Service(TestConnectionError::ResourceNotFoundFault(
                        err.msg,
                    ))
                }
                "ResourceQuotaExceededFault" => {
                    return RusotoError::Service(TestConnectionError::ResourceQuotaExceededFault(
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
impl fmt::Display for TestConnectionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TestConnectionError::InvalidResourceStateFault(ref cause) => write!(f, "{}", cause),
            TestConnectionError::KMSKeyNotAccessibleFault(ref cause) => write!(f, "{}", cause),
            TestConnectionError::ResourceNotFoundFault(ref cause) => write!(f, "{}", cause),
            TestConnectionError::ResourceQuotaExceededFault(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TestConnectionError {}
/// Trait representing the capabilities of the AWS Database Migration Service API. AWS Database Migration Service clients implement this trait.
#[async_trait]
pub trait DatabaseMigrationService: Clone + Sync + Send + 'static {
    /// <p>Adds metadata tags to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<AddTagsToResourceResponse, RusotoError<AddTagsToResourceError>>;

    /// <p>Applies a pending maintenance action to a resource (for example, to a replication instance).</p>
    async fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> Result<
        ApplyPendingMaintenanceActionResponse,
        RusotoError<ApplyPendingMaintenanceActionError>,
    >;

    /// <p>Cancels a single premigration assessment run.</p> <p>This operation prevents any individual assessments from running if they haven't started running. It also attempts to cancel any individual assessments that are currently running.</p>
    async fn cancel_replication_task_assessment_run(
        &self,
        input: CancelReplicationTaskAssessmentRunMessage,
    ) -> Result<
        CancelReplicationTaskAssessmentRunResponse,
        RusotoError<CancelReplicationTaskAssessmentRunError>,
    >;

    /// <p>Creates an endpoint using the provided settings.</p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointMessage,
    ) -> Result<CreateEndpointResponse, RusotoError<CreateEndpointError>>;

    /// <p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> Result<CreateEventSubscriptionResponse, RusotoError<CreateEventSubscriptionError>>;

    /// <p>Creates the replication instance using the specified parameters.</p> <p>AWS DMS requires that your account have certain roles with appropriate permissions before you can create a replication instance. For information on the required roles, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.APIRole">Creating the IAM Roles to Use With the AWS CLI and AWS DMS API</a>. For information on the required permissions, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.IAMPermissions">IAM Permissions Needed to Use AWS DMS</a>.</p>
    async fn create_replication_instance(
        &self,
        input: CreateReplicationInstanceMessage,
    ) -> Result<CreateReplicationInstanceResponse, RusotoError<CreateReplicationInstanceError>>;

    /// <p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>
    async fn create_replication_subnet_group(
        &self,
        input: CreateReplicationSubnetGroupMessage,
    ) -> Result<CreateReplicationSubnetGroupResponse, RusotoError<CreateReplicationSubnetGroupError>>;

    /// <p>Creates a replication task using the specified parameters.</p>
    async fn create_replication_task(
        &self,
        input: CreateReplicationTaskMessage,
    ) -> Result<CreateReplicationTaskResponse, RusotoError<CreateReplicationTaskError>>;

    /// <p>Deletes the specified certificate. </p>
    async fn delete_certificate(
        &self,
        input: DeleteCertificateMessage,
    ) -> Result<DeleteCertificateResponse, RusotoError<DeleteCertificateError>>;

    /// <p>Deletes the connection between a replication instance and an endpoint.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionMessage,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>>;

    /// <p><p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/></p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointMessage,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>>;

    /// <p> Deletes an AWS DMS event subscription. </p>
    async fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> Result<DeleteEventSubscriptionResponse, RusotoError<DeleteEventSubscriptionError>>;

    /// <p><p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/></p>
    async fn delete_replication_instance(
        &self,
        input: DeleteReplicationInstanceMessage,
    ) -> Result<DeleteReplicationInstanceResponse, RusotoError<DeleteReplicationInstanceError>>;

    /// <p>Deletes a subnet group.</p>
    async fn delete_replication_subnet_group(
        &self,
        input: DeleteReplicationSubnetGroupMessage,
    ) -> Result<DeleteReplicationSubnetGroupResponse, RusotoError<DeleteReplicationSubnetGroupError>>;

    /// <p>Deletes the specified replication task.</p>
    async fn delete_replication_task(
        &self,
        input: DeleteReplicationTaskMessage,
    ) -> Result<DeleteReplicationTaskResponse, RusotoError<DeleteReplicationTaskError>>;

    /// <p>Deletes the record of a single premigration assessment run.</p> <p>This operation removes all metadata that AWS DMS maintains about this assessment run. However, the operation leaves untouched all information about this assessment run that is stored in your Amazon S3 bucket.</p>
    async fn delete_replication_task_assessment_run(
        &self,
        input: DeleteReplicationTaskAssessmentRunMessage,
    ) -> Result<
        DeleteReplicationTaskAssessmentRunResponse,
        RusotoError<DeleteReplicationTaskAssessmentRunError>,
    >;

    /// <p>Lists all of the AWS DMS attributes for a customer account. These attributes include AWS DMS quotas for the account and a unique account identifier in a particular DMS region. DMS quotas include a list of resource quotas supported by the account, such as the number of replication instances allowed. The description for each resource quota, includes the quota name, current usage toward that quota, and the quota's maximum value. DMS uses the unique account identifier to name each artifact used by DMS in the given region.</p> <p>This command does not take any parameters.</p>
    async fn describe_account_attributes(
        &self,
    ) -> Result<DescribeAccountAttributesResponse, RusotoError<DescribeAccountAttributesError>>;

    /// <p>Provides a list of individual assessments that you can specify for a new premigration assessment run, given one or more parameters.</p> <p>If you specify an existing migration task, this operation provides the default individual assessments you can specify for that task. Otherwise, the specified parameters model elements of a possible migration task on which to base a premigration assessment run.</p> <p>To use these migration task modeling parameters, you must specify an existing replication instance, a source database engine, a target database engine, and a migration type. This combination of parameters potentially limits the default individual assessments available for an assessment run created for a corresponding migration task.</p> <p>If you specify no parameters, this operation provides a list of all possible individual assessments that you can specify for an assessment run. If you specify any one of the task modeling parameters, you must specify all of them or the operation cannot provide a list of individual assessments. The only parameter that you can specify alone is for an existing migration task. The specified task definition then determines the default list of individual assessments that you can specify in an assessment run for the task.</p>
    async fn describe_applicable_individual_assessments(
        &self,
        input: DescribeApplicableIndividualAssessmentsMessage,
    ) -> Result<
        DescribeApplicableIndividualAssessmentsResponse,
        RusotoError<DescribeApplicableIndividualAssessmentsError>,
    >;

    /// <p>Provides a description of the certificate.</p>
    async fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> Result<DescribeCertificatesResponse, RusotoError<DescribeCertificatesError>>;

    /// Auto-paginating version of `describe_certificates`
    fn describe_certificates_pages(
        &self,
        input: DescribeCertificatesMessage,
    ) -> RusotoStream<Certificate, DescribeCertificatesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_certificates(state.clone())
        })
    }

    /// <p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>
    async fn describe_connections(
        &self,
        input: DescribeConnectionsMessage,
    ) -> Result<DescribeConnectionsResponse, RusotoError<DescribeConnectionsError>>;

    /// Auto-paginating version of `describe_connections`
    fn describe_connections_pages(
        &self,
        input: DescribeConnectionsMessage,
    ) -> RusotoStream<Connection, DescribeConnectionsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_connections(state.clone())
        })
    }

    /// <p>Returns information about the type of endpoints available.</p>
    async fn describe_endpoint_types(
        &self,
        input: DescribeEndpointTypesMessage,
    ) -> Result<DescribeEndpointTypesResponse, RusotoError<DescribeEndpointTypesError>>;

    /// Auto-paginating version of `describe_endpoint_types`
    fn describe_endpoint_types_pages(
        &self,
        input: DescribeEndpointTypesMessage,
    ) -> RusotoStream<SupportedEndpointType, DescribeEndpointTypesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_endpoint_types(state.clone())
        })
    }

    /// <p>Returns information about the endpoints for your account in the current region.</p>
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsMessage,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>>;

    /// Auto-paginating version of `describe_endpoints`
    fn describe_endpoints_pages(
        &self,
        input: DescribeEndpointsMessage,
    ) -> RusotoStream<Endpoint, DescribeEndpointsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_endpoints(state.clone())
        })
    }

    /// <p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> Result<DescribeEventCategoriesResponse, RusotoError<DescribeEventCategoriesError>>;

    /// <p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>
    async fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> Result<DescribeEventSubscriptionsResponse, RusotoError<DescribeEventSubscriptionsError>>;

    /// Auto-paginating version of `describe_event_subscriptions`
    fn describe_event_subscriptions_pages(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> RusotoStream<EventSubscription, DescribeEventSubscriptionsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_event_subscriptions(state.clone())
        })
    }

    /// <p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration User Guide.</i> </p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>>;

    /// Auto-paginating version of `describe_events`
    fn describe_events_pages(
        &self,
        input: DescribeEventsMessage,
    ) -> RusotoStream<Event, DescribeEventsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_events(state.clone())
        })
    }

    /// <p>Returns information about the replication instance types that can be created in the specified region.</p>
    async fn describe_orderable_replication_instances(
        &self,
        input: DescribeOrderableReplicationInstancesMessage,
    ) -> Result<
        DescribeOrderableReplicationInstancesResponse,
        RusotoError<DescribeOrderableReplicationInstancesError>,
    >;

    /// Auto-paginating version of `describe_orderable_replication_instances`
    fn describe_orderable_replication_instances_pages(
        &self,
        input: DescribeOrderableReplicationInstancesMessage,
    ) -> RusotoStream<OrderableReplicationInstance, DescribeOrderableReplicationInstancesError>
    {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_orderable_replication_instances(state.clone())
        })
    }

    /// <p>For internal use only</p>
    async fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> Result<
        DescribePendingMaintenanceActionsResponse,
        RusotoError<DescribePendingMaintenanceActionsError>,
    >;

    /// <p>Returns the status of the RefreshSchemas operation.</p>
    async fn describe_refresh_schemas_status(
        &self,
        input: DescribeRefreshSchemasStatusMessage,
    ) -> Result<DescribeRefreshSchemasStatusResponse, RusotoError<DescribeRefreshSchemasStatusError>>;

    /// <p>Returns information about the task logs for the specified task.</p>
    async fn describe_replication_instance_task_logs(
        &self,
        input: DescribeReplicationInstanceTaskLogsMessage,
    ) -> Result<
        DescribeReplicationInstanceTaskLogsResponse,
        RusotoError<DescribeReplicationInstanceTaskLogsError>,
    >;

    /// <p>Returns information about replication instances for your account in the current region.</p>
    async fn describe_replication_instances(
        &self,
        input: DescribeReplicationInstancesMessage,
    ) -> Result<DescribeReplicationInstancesResponse, RusotoError<DescribeReplicationInstancesError>>;

    /// Auto-paginating version of `describe_replication_instances`
    fn describe_replication_instances_pages(
        &self,
        input: DescribeReplicationInstancesMessage,
    ) -> RusotoStream<ReplicationInstance, DescribeReplicationInstancesError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_replication_instances(state.clone())
        })
    }

    /// <p>Returns information about the replication subnet groups.</p>
    async fn describe_replication_subnet_groups(
        &self,
        input: DescribeReplicationSubnetGroupsMessage,
    ) -> Result<
        DescribeReplicationSubnetGroupsResponse,
        RusotoError<DescribeReplicationSubnetGroupsError>,
    >;

    /// Auto-paginating version of `describe_replication_subnet_groups`
    fn describe_replication_subnet_groups_pages(
        &self,
        input: DescribeReplicationSubnetGroupsMessage,
    ) -> RusotoStream<ReplicationSubnetGroup, DescribeReplicationSubnetGroupsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_replication_subnet_groups(state.clone())
        })
    }

    /// <p>Returns the task assessment results from Amazon S3. This action always returns the latest results.</p>
    async fn describe_replication_task_assessment_results(
        &self,
        input: DescribeReplicationTaskAssessmentResultsMessage,
    ) -> Result<
        DescribeReplicationTaskAssessmentResultsResponse,
        RusotoError<DescribeReplicationTaskAssessmentResultsError>,
    >;

    /// Auto-paginating version of `describe_replication_task_assessment_results`
    fn describe_replication_task_assessment_results_pages(
        &self,
        input: DescribeReplicationTaskAssessmentResultsMessage,
    ) -> RusotoStream<ReplicationTaskAssessmentResult, DescribeReplicationTaskAssessmentResultsError>
    {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_replication_task_assessment_results(state.clone())
        })
    }

    /// <p><p>Returns a paginated list of premigration assessment runs based on filter settings.</p> <p>These filter settings can specify a combination of premigration assessment runs, migration tasks, replication instances, and assessment run status values.</p> <note> <p>This operation doesn&#39;t return information about individual assessments. For this information, see the <code>DescribeReplicationTaskIndividualAssessments</code> operation. </p> </note></p>
    async fn describe_replication_task_assessment_runs(
        &self,
        input: DescribeReplicationTaskAssessmentRunsMessage,
    ) -> Result<
        DescribeReplicationTaskAssessmentRunsResponse,
        RusotoError<DescribeReplicationTaskAssessmentRunsError>,
    >;

    /// <p>Returns a paginated list of individual assessments based on filter settings.</p> <p>These filter settings can specify a combination of premigration assessment runs, migration tasks, and assessment status values.</p>
    async fn describe_replication_task_individual_assessments(
        &self,
        input: DescribeReplicationTaskIndividualAssessmentsMessage,
    ) -> Result<
        DescribeReplicationTaskIndividualAssessmentsResponse,
        RusotoError<DescribeReplicationTaskIndividualAssessmentsError>,
    >;

    /// <p>Returns information about replication tasks for your account in the current region.</p>
    async fn describe_replication_tasks(
        &self,
        input: DescribeReplicationTasksMessage,
    ) -> Result<DescribeReplicationTasksResponse, RusotoError<DescribeReplicationTasksError>>;

    /// Auto-paginating version of `describe_replication_tasks`
    fn describe_replication_tasks_pages(
        &self,
        input: DescribeReplicationTasksMessage,
    ) -> RusotoStream<ReplicationTask, DescribeReplicationTasksError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_replication_tasks(state.clone())
        })
    }

    /// <p><p>Returns information about the schema for the specified endpoint.</p> <p/></p>
    async fn describe_schemas(
        &self,
        input: DescribeSchemasMessage,
    ) -> Result<DescribeSchemasResponse, RusotoError<DescribeSchemasError>>;

    /// Auto-paginating version of `describe_schemas`
    fn describe_schemas_pages(
        &self,
        input: DescribeSchemasMessage,
    ) -> RusotoStream<String, DescribeSchemasError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_schemas(state.clone())
        })
    }

    /// <p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p> <p>Note that the "last updated" column the DMS console only indicates the time that AWS DMS last updated the table statistics record for a table. It does not indicate the time of the last update to the table.</p>
    async fn describe_table_statistics(
        &self,
        input: DescribeTableStatisticsMessage,
    ) -> Result<DescribeTableStatisticsResponse, RusotoError<DescribeTableStatisticsError>>;

    /// Auto-paginating version of `describe_table_statistics`
    fn describe_table_statistics_pages(
        &self,
        input: DescribeTableStatisticsMessage,
    ) -> RusotoStream<TableStatistics, DescribeTableStatisticsError> {
        all_pages(self.clone(), input, move |client, state| {
            client.describe_table_statistics(state.clone())
        })
    }

    /// <p>Uploads the specified certificate.</p>
    async fn import_certificate(
        &self,
        input: ImportCertificateMessage,
    ) -> Result<ImportCertificateResponse, RusotoError<ImportCertificateError>>;

    /// <p>Lists all metadata tags attached to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Modifies the specified endpoint.</p>
    async fn modify_endpoint(
        &self,
        input: ModifyEndpointMessage,
    ) -> Result<ModifyEndpointResponse, RusotoError<ModifyEndpointError>>;

    /// <p>Modifies an existing AWS DMS event notification subscription. </p>
    async fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> Result<ModifyEventSubscriptionResponse, RusotoError<ModifyEventSubscriptionError>>;

    /// <p><p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/></p>
    async fn modify_replication_instance(
        &self,
        input: ModifyReplicationInstanceMessage,
    ) -> Result<ModifyReplicationInstanceResponse, RusotoError<ModifyReplicationInstanceError>>;

    /// <p>Modifies the settings for the specified replication subnet group.</p>
    async fn modify_replication_subnet_group(
        &self,
        input: ModifyReplicationSubnetGroupMessage,
    ) -> Result<ModifyReplicationSubnetGroupResponse, RusotoError<ModifyReplicationSubnetGroupError>>;

    /// <p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    async fn modify_replication_task(
        &self,
        input: ModifyReplicationTaskMessage,
    ) -> Result<ModifyReplicationTaskResponse, RusotoError<ModifyReplicationTaskError>>;

    /// <p>Moves a replication task from its current replication instance to a different target replication instance using the specified parameters. The target replication instance must be created with the same or later AWS DMS version as the current replication instance.</p>
    async fn move_replication_task(
        &self,
        input: MoveReplicationTaskMessage,
    ) -> Result<MoveReplicationTaskResponse, RusotoError<MoveReplicationTaskError>>;

    /// <p>Reboots a replication instance. Rebooting results in a momentary outage, until the replication instance becomes available again.</p>
    async fn reboot_replication_instance(
        &self,
        input: RebootReplicationInstanceMessage,
    ) -> Result<RebootReplicationInstanceResponse, RusotoError<RebootReplicationInstanceError>>;

    /// <p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>
    async fn refresh_schemas(
        &self,
        input: RefreshSchemasMessage,
    ) -> Result<RefreshSchemasResponse, RusotoError<RefreshSchemasError>>;

    /// <p>Reloads the target database table with the source data. </p>
    async fn reload_tables(
        &self,
        input: ReloadTablesMessage,
    ) -> Result<ReloadTablesResponse, RusotoError<ReloadTablesError>>;

    /// <p>Removes metadata tags from an AWS DMS resource, including replication instance, endpoint, security group, and migration task. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<RemoveTagsFromResourceResponse, RusotoError<RemoveTagsFromResourceError>>;

    /// <p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks </a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn start_replication_task(
        &self,
        input: StartReplicationTaskMessage,
    ) -> Result<StartReplicationTaskResponse, RusotoError<StartReplicationTaskError>>;

    /// <p> Starts the replication task assessment for unsupported data types in the source database. </p>
    async fn start_replication_task_assessment(
        &self,
        input: StartReplicationTaskAssessmentMessage,
    ) -> Result<
        StartReplicationTaskAssessmentResponse,
        RusotoError<StartReplicationTaskAssessmentError>,
    >;

    /// <p>Starts a new premigration assessment run for one or more individual assessments of a migration task.</p> <p>The assessments that you can specify depend on the source and target database engine and the migration type defined for the given task. To run this operation, your migration task must already be created. After you run this operation, you can review the status of each individual assessment. You can also run the migration task manually after the assessment run and its individual assessments complete.</p>
    async fn start_replication_task_assessment_run(
        &self,
        input: StartReplicationTaskAssessmentRunMessage,
    ) -> Result<
        StartReplicationTaskAssessmentRunResponse,
        RusotoError<StartReplicationTaskAssessmentRunError>,
    >;

    /// <p>Stops the replication task.</p>
    async fn stop_replication_task(
        &self,
        input: StopReplicationTaskMessage,
    ) -> Result<StopReplicationTaskResponse, RusotoError<StopReplicationTaskError>>;

    /// <p>Tests the connection between the replication instance and the endpoint.</p>
    async fn test_connection(
        &self,
        input: TestConnectionMessage,
    ) -> Result<TestConnectionResponse, RusotoError<TestConnectionError>>;
}
/// A client for the AWS Database Migration Service API.
#[derive(Clone)]
pub struct DatabaseMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl DatabaseMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DatabaseMigrationServiceClient {
        DatabaseMigrationServiceClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DatabaseMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DatabaseMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(
        client: Client,
        region: region::Region,
    ) -> DatabaseMigrationServiceClient {
        DatabaseMigrationServiceClient { client, region }
    }
}

#[async_trait]
impl DatabaseMigrationService for DatabaseMigrationServiceClient {
    /// <p>Adds metadata tags to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. These tags can also be used with cost allocation reporting to track cost associated with DMS resources, or used in a Condition statement in an IAM policy for DMS. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn add_tags_to_resource(
        &self,
        input: AddTagsToResourceMessage,
    ) -> Result<AddTagsToResourceResponse, RusotoError<AddTagsToResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.AddTagsToResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, AddTagsToResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<AddTagsToResourceResponse, _>()
    }

    /// <p>Applies a pending maintenance action to a resource (for example, to a replication instance).</p>
    async fn apply_pending_maintenance_action(
        &self,
        input: ApplyPendingMaintenanceActionMessage,
    ) -> Result<
        ApplyPendingMaintenanceActionResponse,
        RusotoError<ApplyPendingMaintenanceActionError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ApplyPendingMaintenanceAction",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ApplyPendingMaintenanceActionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ApplyPendingMaintenanceActionResponse, _>()
    }

    /// <p>Cancels a single premigration assessment run.</p> <p>This operation prevents any individual assessments from running if they haven't started running. It also attempts to cancel any individual assessments that are currently running.</p>
    async fn cancel_replication_task_assessment_run(
        &self,
        input: CancelReplicationTaskAssessmentRunMessage,
    ) -> Result<
        CancelReplicationTaskAssessmentRunResponse,
        RusotoError<CancelReplicationTaskAssessmentRunError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.CancelReplicationTaskAssessmentRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                CancelReplicationTaskAssessmentRunError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CancelReplicationTaskAssessmentRunResponse, _>()
    }

    /// <p>Creates an endpoint using the provided settings.</p>
    async fn create_endpoint(
        &self,
        input: CreateEndpointMessage,
    ) -> Result<CreateEndpointResponse, RusotoError<CreateEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEndpointError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateEndpointResponse, _>()
    }

    /// <p> Creates an AWS DMS event notification subscription. </p> <p>You can specify the type of source (<code>SourceType</code>) you want to be notified of, provide a list of AWS DMS source IDs (<code>SourceIds</code>) that triggers the events, and provide a list of event categories (<code>EventCategories</code>) for events you want to be notified of. If you specify both the <code>SourceType</code> and <code>SourceIds</code>, such as <code>SourceType = replication-instance</code> and <code>SourceIdentifier = my-replinstance</code>, you will be notified of all the replication instance events for the specified source. If you specify a <code>SourceType</code> but don't specify a <code>SourceIdentifier</code>, you receive notice of the events for that source type for all your AWS DMS sources. If you don't specify either <code>SourceType</code> nor <code>SourceIdentifier</code>, you will be notified of events generated from all AWS DMS sources belonging to your customer account.</p> <p>For more information about AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn create_event_subscription(
        &self,
        input: CreateEventSubscriptionMessage,
    ) -> Result<CreateEventSubscriptionResponse, RusotoError<CreateEventSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateEventSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateEventSubscriptionResponse, _>()
    }

    /// <p>Creates the replication instance using the specified parameters.</p> <p>AWS DMS requires that your account have certain roles with appropriate permissions before you can create a replication instance. For information on the required roles, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.APIRole">Creating the IAM Roles to Use With the AWS CLI and AWS DMS API</a>. For information on the required permissions, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Security.html#CHAP_Security.IAMPermissions">IAM Permissions Needed to Use AWS DMS</a>.</p>
    async fn create_replication_instance(
        &self,
        input: CreateReplicationInstanceMessage,
    ) -> Result<CreateReplicationInstanceResponse, RusotoError<CreateReplicationInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.CreateReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateReplicationInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateReplicationInstanceResponse, _>()
    }

    /// <p>Creates a replication subnet group given a list of the subnet IDs in a VPC.</p>
    async fn create_replication_subnet_group(
        &self,
        input: CreateReplicationSubnetGroupMessage,
    ) -> Result<CreateReplicationSubnetGroupResponse, RusotoError<CreateReplicationSubnetGroupError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.CreateReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateReplicationSubnetGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateReplicationSubnetGroupResponse, _>()
    }

    /// <p>Creates a replication task using the specified parameters.</p>
    async fn create_replication_task(
        &self,
        input: CreateReplicationTaskMessage,
    ) -> Result<CreateReplicationTaskResponse, RusotoError<CreateReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.CreateReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<CreateReplicationTaskResponse, _>()
    }

    /// <p>Deletes the specified certificate. </p>
    async fn delete_certificate(
        &self,
        input: DeleteCertificateMessage,
    ) -> Result<DeleteCertificateResponse, RusotoError<DeleteCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteCertificateResponse, _>()
    }

    /// <p>Deletes the connection between a replication instance and an endpoint.</p>
    async fn delete_connection(
        &self,
        input: DeleteConnectionMessage,
    ) -> Result<DeleteConnectionResponse, RusotoError<DeleteConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteConnectionResponse, _>()
    }

    /// <p><p>Deletes the specified endpoint.</p> <note> <p>All tasks associated with the endpoint must be deleted before you can delete the endpoint.</p> </note> <p/></p>
    async fn delete_endpoint(
        &self,
        input: DeleteEndpointMessage,
    ) -> Result<DeleteEndpointResponse, RusotoError<DeleteEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEndpointError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteEndpointResponse, _>()
    }

    /// <p> Deletes an AWS DMS event subscription. </p>
    async fn delete_event_subscription(
        &self,
        input: DeleteEventSubscriptionMessage,
    ) -> Result<DeleteEventSubscriptionResponse, RusotoError<DeleteEventSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteEventSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteEventSubscriptionResponse, _>()
    }

    /// <p><p>Deletes the specified replication instance.</p> <note> <p>You must delete any migration tasks that are associated with the replication instance before you can delete it.</p> </note> <p/></p>
    async fn delete_replication_instance(
        &self,
        input: DeleteReplicationInstanceMessage,
    ) -> Result<DeleteReplicationInstanceResponse, RusotoError<DeleteReplicationInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DeleteReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReplicationInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReplicationInstanceResponse, _>()
    }

    /// <p>Deletes a subnet group.</p>
    async fn delete_replication_subnet_group(
        &self,
        input: DeleteReplicationSubnetGroupMessage,
    ) -> Result<DeleteReplicationSubnetGroupResponse, RusotoError<DeleteReplicationSubnetGroupError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DeleteReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReplicationSubnetGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReplicationSubnetGroupResponse, _>()
    }

    /// <p>Deletes the specified replication task.</p>
    async fn delete_replication_task(
        &self,
        input: DeleteReplicationTaskMessage,
    ) -> Result<DeleteReplicationTaskResponse, RusotoError<DeleteReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DeleteReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReplicationTaskResponse, _>()
    }

    /// <p>Deletes the record of a single premigration assessment run.</p> <p>This operation removes all metadata that AWS DMS maintains about this assessment run. However, the operation leaves untouched all information about this assessment run that is stored in your Amazon S3 bucket.</p>
    async fn delete_replication_task_assessment_run(
        &self,
        input: DeleteReplicationTaskAssessmentRunMessage,
    ) -> Result<
        DeleteReplicationTaskAssessmentRunResponse,
        RusotoError<DeleteReplicationTaskAssessmentRunError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DeleteReplicationTaskAssessmentRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DeleteReplicationTaskAssessmentRunError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DeleteReplicationTaskAssessmentRunResponse, _>()
    }

    /// <p>Lists all of the AWS DMS attributes for a customer account. These attributes include AWS DMS quotas for the account and a unique account identifier in a particular DMS region. DMS quotas include a list of resource quotas supported by the account, such as the number of replication instances allowed. The description for each resource quota, includes the quota name, current usage toward that quota, and the quota's maximum value. DMS uses the unique account identifier to name each artifact used by DMS in the given region.</p> <p>This command does not take any parameters.</p>
    async fn describe_account_attributes(
        &self,
    ) -> Result<DescribeAccountAttributesResponse, RusotoError<DescribeAccountAttributesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeAccountAttributes",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let response = self
            .sign_and_dispatch(request, DescribeAccountAttributesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeAccountAttributesResponse, _>()
    }

    /// <p>Provides a list of individual assessments that you can specify for a new premigration assessment run, given one or more parameters.</p> <p>If you specify an existing migration task, this operation provides the default individual assessments you can specify for that task. Otherwise, the specified parameters model elements of a possible migration task on which to base a premigration assessment run.</p> <p>To use these migration task modeling parameters, you must specify an existing replication instance, a source database engine, a target database engine, and a migration type. This combination of parameters potentially limits the default individual assessments available for an assessment run created for a corresponding migration task.</p> <p>If you specify no parameters, this operation provides a list of all possible individual assessments that you can specify for an assessment run. If you specify any one of the task modeling parameters, you must specify all of them or the operation cannot provide a list of individual assessments. The only parameter that you can specify alone is for an existing migration task. The specified task definition then determines the default list of individual assessments that you can specify in an assessment run for the task.</p>
    async fn describe_applicable_individual_assessments(
        &self,
        input: DescribeApplicableIndividualAssessmentsMessage,
    ) -> Result<
        DescribeApplicableIndividualAssessmentsResponse,
        RusotoError<DescribeApplicableIndividualAssessmentsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeApplicableIndividualAssessments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeApplicableIndividualAssessmentsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeApplicableIndividualAssessmentsResponse, _>()
    }

    /// <p>Provides a description of the certificate.</p>
    async fn describe_certificates(
        &self,
        input: DescribeCertificatesMessage,
    ) -> Result<DescribeCertificatesResponse, RusotoError<DescribeCertificatesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeCertificates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeCertificatesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeCertificatesResponse, _>()
    }

    /// <p>Describes the status of the connections that have been made between the replication instance and an endpoint. Connections are created when you test an endpoint.</p>
    async fn describe_connections(
        &self,
        input: DescribeConnectionsMessage,
    ) -> Result<DescribeConnectionsResponse, RusotoError<DescribeConnectionsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeConnections");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeConnectionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeConnectionsResponse, _>()
    }

    /// <p>Returns information about the type of endpoints available.</p>
    async fn describe_endpoint_types(
        &self,
        input: DescribeEndpointTypesMessage,
    ) -> Result<DescribeEndpointTypesResponse, RusotoError<DescribeEndpointTypesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpointTypes");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEndpointTypesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEndpointTypesResponse, _>()
    }

    /// <p>Returns information about the endpoints for your account in the current region.</p>
    async fn describe_endpoints(
        &self,
        input: DescribeEndpointsMessage,
    ) -> Result<DescribeEndpointsResponse, RusotoError<DescribeEndpointsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEndpoints");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEndpointsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEndpointsResponse, _>()
    }

    /// <p>Lists categories for all event source types, or, if specified, for a specified source type. You can see a list of the event categories and source types in <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn describe_event_categories(
        &self,
        input: DescribeEventCategoriesMessage,
    ) -> Result<DescribeEventCategoriesResponse, RusotoError<DescribeEventCategoriesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEventCategories");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventCategoriesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventCategoriesResponse, _>()
    }

    /// <p>Lists all the event subscriptions for a customer account. The description of a subscription includes <code>SubscriptionName</code>, <code>SNSTopicARN</code>, <code>CustomerID</code>, <code>SourceType</code>, <code>SourceID</code>, <code>CreationTime</code>, and <code>Status</code>. </p> <p>If you specify <code>SubscriptionName</code>, this action lists the description for that subscription.</p>
    async fn describe_event_subscriptions(
        &self,
        input: DescribeEventSubscriptionsMessage,
    ) -> Result<DescribeEventSubscriptionsResponse, RusotoError<DescribeEventSubscriptionsError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeEventSubscriptions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventSubscriptionsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeEventSubscriptionsResponse, _>()
    }

    /// <p> Lists events for a given source identifier and source type. You can also specify a start and end time. For more information on AWS DMS events, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Events.html">Working with Events and Notifications</a> in the <i>AWS Database Migration User Guide.</i> </p>
    async fn describe_events(
        &self,
        input: DescribeEventsMessage,
    ) -> Result<DescribeEventsResponse, RusotoError<DescribeEventsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeEvents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeEventsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeEventsResponse, _>()
    }

    /// <p>Returns information about the replication instance types that can be created in the specified region.</p>
    async fn describe_orderable_replication_instances(
        &self,
        input: DescribeOrderableReplicationInstancesMessage,
    ) -> Result<
        DescribeOrderableReplicationInstancesResponse,
        RusotoError<DescribeOrderableReplicationInstancesError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeOrderableReplicationInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeOrderableReplicationInstancesError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeOrderableReplicationInstancesResponse, _>()
    }

    /// <p>For internal use only</p>
    async fn describe_pending_maintenance_actions(
        &self,
        input: DescribePendingMaintenanceActionsMessage,
    ) -> Result<
        DescribePendingMaintenanceActionsResponse,
        RusotoError<DescribePendingMaintenanceActionsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribePendingMaintenanceActions",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribePendingMaintenanceActionsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribePendingMaintenanceActionsResponse, _>()
    }

    /// <p>Returns the status of the RefreshSchemas operation.</p>
    async fn describe_refresh_schemas_status(
        &self,
        input: DescribeRefreshSchemasStatusMessage,
    ) -> Result<DescribeRefreshSchemasStatusResponse, RusotoError<DescribeRefreshSchemasStatusError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeRefreshSchemasStatus",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeRefreshSchemasStatusError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeRefreshSchemasStatusResponse, _>()
    }

    /// <p>Returns information about the task logs for the specified task.</p>
    async fn describe_replication_instance_task_logs(
        &self,
        input: DescribeReplicationInstanceTaskLogsMessage,
    ) -> Result<
        DescribeReplicationInstanceTaskLogsResponse,
        RusotoError<DescribeReplicationInstanceTaskLogsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationInstanceTaskLogs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeReplicationInstanceTaskLogsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationInstanceTaskLogsResponse, _>()
    }

    /// <p>Returns information about replication instances for your account in the current region.</p>
    async fn describe_replication_instances(
        &self,
        input: DescribeReplicationInstancesMessage,
    ) -> Result<DescribeReplicationInstancesResponse, RusotoError<DescribeReplicationInstancesError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationInstances",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReplicationInstancesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationInstancesResponse, _>()
    }

    /// <p>Returns information about the replication subnet groups.</p>
    async fn describe_replication_subnet_groups(
        &self,
        input: DescribeReplicationSubnetGroupsMessage,
    ) -> Result<
        DescribeReplicationSubnetGroupsResponse,
        RusotoError<DescribeReplicationSubnetGroupsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationSubnetGroups",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReplicationSubnetGroupsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationSubnetGroupsResponse, _>()
    }

    /// <p>Returns the task assessment results from Amazon S3. This action always returns the latest results.</p>
    async fn describe_replication_task_assessment_results(
        &self,
        input: DescribeReplicationTaskAssessmentResultsMessage,
    ) -> Result<
        DescribeReplicationTaskAssessmentResultsResponse,
        RusotoError<DescribeReplicationTaskAssessmentResultsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTaskAssessmentResults",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeReplicationTaskAssessmentResultsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationTaskAssessmentResultsResponse, _>()
    }

    /// <p><p>Returns a paginated list of premigration assessment runs based on filter settings.</p> <p>These filter settings can specify a combination of premigration assessment runs, migration tasks, replication instances, and assessment run status values.</p> <note> <p>This operation doesn&#39;t return information about individual assessments. For this information, see the <code>DescribeReplicationTaskIndividualAssessments</code> operation. </p> </note></p>
    async fn describe_replication_task_assessment_runs(
        &self,
        input: DescribeReplicationTaskAssessmentRunsMessage,
    ) -> Result<
        DescribeReplicationTaskAssessmentRunsResponse,
        RusotoError<DescribeReplicationTaskAssessmentRunsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTaskAssessmentRuns",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeReplicationTaskAssessmentRunsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationTaskAssessmentRunsResponse, _>()
    }

    /// <p>Returns a paginated list of individual assessments based on filter settings.</p> <p>These filter settings can specify a combination of premigration assessment runs, migration tasks, and assessment status values.</p>
    async fn describe_replication_task_individual_assessments(
        &self,
        input: DescribeReplicationTaskIndividualAssessmentsMessage,
    ) -> Result<
        DescribeReplicationTaskIndividualAssessmentsResponse,
        RusotoError<DescribeReplicationTaskIndividualAssessmentsError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTaskIndividualAssessments",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                DescribeReplicationTaskIndividualAssessmentsError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationTaskIndividualAssessmentsResponse, _>()
    }

    /// <p>Returns information about replication tasks for your account in the current region.</p>
    async fn describe_replication_tasks(
        &self,
        input: DescribeReplicationTasksMessage,
    ) -> Result<DescribeReplicationTasksResponse, RusotoError<DescribeReplicationTasksError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.DescribeReplicationTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeReplicationTasksError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeReplicationTasksResponse, _>()
    }

    /// <p><p>Returns information about the schema for the specified endpoint.</p> <p/></p>
    async fn describe_schemas(
        &self,
        input: DescribeSchemasMessage,
    ) -> Result<DescribeSchemasResponse, RusotoError<DescribeSchemasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeSchemas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeSchemasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DescribeSchemasResponse, _>()
    }

    /// <p>Returns table statistics on the database migration task, including table name, rows inserted, rows updated, and rows deleted.</p> <p>Note that the "last updated" column the DMS console only indicates the time that AWS DMS last updated the table statistics record for a table. It does not indicate the time of the last update to the table.</p>
    async fn describe_table_statistics(
        &self,
        input: DescribeTableStatisticsMessage,
    ) -> Result<DescribeTableStatisticsResponse, RusotoError<DescribeTableStatisticsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.DescribeTableStatistics");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeTableStatisticsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeTableStatisticsResponse, _>()
    }

    /// <p>Uploads the specified certificate.</p>
    async fn import_certificate(
        &self,
        input: ImportCertificateMessage,
    ) -> Result<ImportCertificateResponse, RusotoError<ImportCertificateError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ImportCertificate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ImportCertificateError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ImportCertificateResponse, _>()
    }

    /// <p>Lists all metadata tags attached to an AWS DMS resource, including replication instance, endpoint, security group, and migration task. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceMessage,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListTagsForResourceResponse, _>()
    }

    /// <p>Modifies the specified endpoint.</p>
    async fn modify_endpoint(
        &self,
        input: ModifyEndpointMessage,
    ) -> Result<ModifyEndpointResponse, RusotoError<ModifyEndpointError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEndpoint");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyEndpointError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ModifyEndpointResponse, _>()
    }

    /// <p>Modifies an existing AWS DMS event notification subscription. </p>
    async fn modify_event_subscription(
        &self,
        input: ModifyEventSubscriptionMessage,
    ) -> Result<ModifyEventSubscriptionResponse, RusotoError<ModifyEventSubscriptionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyEventSubscription");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyEventSubscriptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyEventSubscriptionResponse, _>()
    }

    /// <p><p>Modifies the replication instance to apply new settings. You can change one or more parameters by specifying these parameters and the new values in the request.</p> <p>Some settings are applied during the maintenance window.</p> <p/></p>
    async fn modify_replication_instance(
        &self,
        input: ModifyReplicationInstanceMessage,
    ) -> Result<ModifyReplicationInstanceResponse, RusotoError<ModifyReplicationInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ModifyReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyReplicationInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyReplicationInstanceResponse, _>()
    }

    /// <p>Modifies the settings for the specified replication subnet group.</p>
    async fn modify_replication_subnet_group(
        &self,
        input: ModifyReplicationSubnetGroupMessage,
    ) -> Result<ModifyReplicationSubnetGroupResponse, RusotoError<ModifyReplicationSubnetGroupError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.ModifyReplicationSubnetGroup",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyReplicationSubnetGroupError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyReplicationSubnetGroupResponse, _>()
    }

    /// <p>Modifies the specified replication task.</p> <p>You can't modify the task endpoints. The task must be stopped before you can modify it. </p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks</a> in the <i>AWS Database Migration Service User Guide</i>.</p>
    async fn modify_replication_task(
        &self,
        input: ModifyReplicationTaskMessage,
    ) -> Result<ModifyReplicationTaskResponse, RusotoError<ModifyReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ModifyReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ModifyReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ModifyReplicationTaskResponse, _>()
    }

    /// <p>Moves a replication task from its current replication instance to a different target replication instance using the specified parameters. The target replication instance must be created with the same or later AWS DMS version as the current replication instance.</p>
    async fn move_replication_task(
        &self,
        input: MoveReplicationTaskMessage,
    ) -> Result<MoveReplicationTaskResponse, RusotoError<MoveReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.MoveReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, MoveReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<MoveReplicationTaskResponse, _>()
    }

    /// <p>Reboots a replication instance. Rebooting results in a momentary outage, until the replication instance becomes available again.</p>
    async fn reboot_replication_instance(
        &self,
        input: RebootReplicationInstanceMessage,
    ) -> Result<RebootReplicationInstanceResponse, RusotoError<RebootReplicationInstanceError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.RebootReplicationInstance",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RebootReplicationInstanceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RebootReplicationInstanceResponse, _>()
    }

    /// <p>Populates the schema for the specified endpoint. This is an asynchronous operation and can take several minutes. You can check the status of this operation by calling the DescribeRefreshSchemasStatus operation.</p>
    async fn refresh_schemas(
        &self,
        input: RefreshSchemasMessage,
    ) -> Result<RefreshSchemasResponse, RusotoError<RefreshSchemasError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.RefreshSchemas");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RefreshSchemasError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<RefreshSchemasResponse, _>()
    }

    /// <p>Reloads the target database table with the source data. </p>
    async fn reload_tables(
        &self,
        input: ReloadTablesMessage,
    ) -> Result<ReloadTablesResponse, RusotoError<ReloadTablesError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.ReloadTables");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ReloadTablesError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ReloadTablesResponse, _>()
    }

    /// <p>Removes metadata tags from an AWS DMS resource, including replication instance, endpoint, security group, and migration task. For more information, see <a href="https://docs.aws.amazon.com/dms/latest/APIReference/API_Tag.html"> <code>Tag</code> </a> data type description.</p>
    async fn remove_tags_from_resource(
        &self,
        input: RemoveTagsFromResourceMessage,
    ) -> Result<RemoveTagsFromResourceResponse, RusotoError<RemoveTagsFromResourceError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.RemoveTagsFromResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, RemoveTagsFromResourceError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<RemoveTagsFromResourceResponse, _>()
    }

    /// <p>Starts the replication task.</p> <p>For more information about AWS DMS tasks, see <a href="https://docs.aws.amazon.com/dms/latest/userguide/CHAP_Tasks.html">Working with Migration Tasks </a> in the <i>AWS Database Migration Service User Guide.</i> </p>
    async fn start_replication_task(
        &self,
        input: StartReplicationTaskMessage,
    ) -> Result<StartReplicationTaskResponse, RusotoError<StartReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.StartReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartReplicationTaskResponse, _>()
    }

    /// <p> Starts the replication task assessment for unsupported data types in the source database. </p>
    async fn start_replication_task_assessment(
        &self,
        input: StartReplicationTaskAssessmentMessage,
    ) -> Result<
        StartReplicationTaskAssessmentResponse,
        RusotoError<StartReplicationTaskAssessmentError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.StartReplicationTaskAssessment",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartReplicationTaskAssessmentError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartReplicationTaskAssessmentResponse, _>()
    }

    /// <p>Starts a new premigration assessment run for one or more individual assessments of a migration task.</p> <p>The assessments that you can specify depend on the source and target database engine and the migration type defined for the given task. To run this operation, your migration task must already be created. After you run this operation, you can review the status of each individual assessment. You can also run the migration task manually after the assessment run and its individual assessments complete.</p>
    async fn start_replication_task_assessment_run(
        &self,
        input: StartReplicationTaskAssessmentRunMessage,
    ) -> Result<
        StartReplicationTaskAssessmentRunResponse,
        RusotoError<StartReplicationTaskAssessmentRunError>,
    > {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "AmazonDMSv20160101.StartReplicationTaskAssessmentRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(
                request,
                StartReplicationTaskAssessmentRunError::from_response,
            )
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartReplicationTaskAssessmentRunResponse, _>()
    }

    /// <p>Stops the replication task.</p>
    async fn stop_replication_task(
        &self,
        input: StopReplicationTaskMessage,
    ) -> Result<StopReplicationTaskResponse, RusotoError<StopReplicationTaskError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.StopReplicationTask");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopReplicationTaskError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<StopReplicationTaskResponse, _>()
    }

    /// <p>Tests the connection between the replication instance and the endpoint.</p>
    async fn test_connection(
        &self,
        input: TestConnectionMessage,
    ) -> Result<TestConnectionResponse, RusotoError<TestConnectionError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "AmazonDMSv20160101.TestConnection");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TestConnectionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TestConnectionResponse, _>()
    }
}
