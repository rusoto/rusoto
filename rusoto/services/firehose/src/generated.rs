
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

#[allow(warnings)]
use hyper::Client;
use hyper::status::StatusCode;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::region;

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;
use rusoto_core::request::HttpDispatchError;
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::signature::SignedRequest;
use serde_json::Value as SerdeJsonValue;
use serde_json::from_str;
#[doc="<p>Describes hints for the buffering to perform before delivering data to the destination. Please note that these options are treated as hints, and therefore Firehose may choose to use different values when it is optimal.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct BufferingHints {
    #[doc="<p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300.</p>"]
    #[serde(rename="IntervalInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    #[doc="<p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.</p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or higher.</p>"]
    #[serde(rename="SizeInMBs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

#[doc="<p>Describes the CloudWatch logging options for your delivery stream.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CloudWatchLoggingOptions {
    #[doc="<p>Enables or disables CloudWatch logging.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="<p>The CloudWatch group name for logging. This value is required if CloudWatch logging is enabled.</p>"]
    #[serde(rename="LogGroupName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_group_name: Option<String>,
    #[doc="<p>The CloudWatch log stream name for logging. This value is required if CloudWatch logging is enabled.</p>"]
    #[serde(rename="LogStreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub log_stream_name: Option<String>,
}

#[doc="<p>Describes a <code>COPY</code> command for Amazon Redshift.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CopyCommand {
    #[doc="<p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the \"Optional Parameters\" section of <a href=\"http://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html\">Amazon Redshift COPY command</a>. Some possible examples that would apply to Firehose are as follows:</p> <p> <code>delimiter '\\t' lzop;</code> - fields are delimited with \"\\t\" (TAB character) and compressed using lzop.</p> <p> <code>delimiter '|</code> - fields are delimited with \"|\" (this is the default delimiter).</p> <p> <code>delimiter '|' escape</code> - the delimiter should be escaped.</p> <p> <code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p> <p> <code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p> <p>For more examples, see <a href=\"http://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html\">Amazon Redshift COPY command examples</a>.</p>"]
    #[serde(rename="CopyOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub copy_options: Option<String>,
    #[doc="<p>A comma-separated list of column names.</p>"]
    #[serde(rename="DataTableColumns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_table_columns: Option<String>,
    #[doc="<p>The name of the target table. The table must already exist in the database.</p>"]
    #[serde(rename="DataTableName")]
    pub data_table_name: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct CreateDeliveryStreamInput {
    #[doc="<p>The name of the delivery stream. This name must be unique per AWS account in the same region. You can have multiple delivery streams with the same name if they are in different accounts or different regions.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The destination in Amazon ES. You can specify only one destination.</p>"]
    #[serde(rename="ElasticsearchDestinationConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,
    #[doc="<p>The destination in Amazon S3. You can specify only one destination.</p>"]
    #[serde(rename="ExtendedS3DestinationConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,
    #[doc="<p>The destination in Amazon Redshift. You can specify only one destination.</p>"]
    #[serde(rename="RedshiftDestinationConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub redshift_destination_configuration: Option<RedshiftDestinationConfiguration>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct CreateDeliveryStreamOutput {
    #[doc="<p>The ARN of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub delivery_stream_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDeliveryStreamInput {
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDeliveryStreamOutput;

#[doc="<p>Contains information about a delivery stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeliveryStreamDescription {
    #[doc="<p>The date and time that the delivery stream was created.</p>"]
    #[serde(rename="CreateTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub create_timestamp: Option<f64>,
    #[doc="<p>The Amazon Resource Name (ARN) of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamARN")]
    pub delivery_stream_arn: String,
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The status of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamStatus")]
    pub delivery_stream_status: String,
    #[doc="<p>The destinations.</p>"]
    #[serde(rename="Destinations")]
    pub destinations: Vec<DestinationDescription>,
    #[doc="<p>Indicates whether there are more destinations available to list.</p>"]
    #[serde(rename="HasMoreDestinations")]
    pub has_more_destinations: bool,
    #[doc="<p>The date and time that the delivery stream was last updated.</p>"]
    #[serde(rename="LastUpdateTimestamp")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    #[doc="<p>Each time the destination is updated for a delivery stream, the version ID is changed, and the current version ID is required when updating the destination. This is so that the service knows it is applying the changes to the correct version of the delivery stream.</p>"]
    #[serde(rename="VersionId")]
    pub version_id: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDeliveryStreamInput {
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The ID of the destination to start returning the destination information. Currently Firehose supports one destination per delivery stream.</p>"]
    #[serde(rename="ExclusiveStartDestinationId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_destination_id: Option<String>,
    #[doc="<p>The limit on the number of destinations to return. Currently, you can have one destination per delivery stream.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDeliveryStreamOutput {
    #[doc="<p>Information about the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamDescription")]
    pub delivery_stream_description: DeliveryStreamDescription,
}

#[doc="<p>Describes the destination for a delivery stream.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DestinationDescription {
    #[doc="<p>The ID of the destination.</p>"]
    #[serde(rename="DestinationId")]
    pub destination_id: String,
    #[doc="<p>The destination in Amazon ES.</p>"]
    #[serde(rename="ElasticsearchDestinationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elasticsearch_destination_description: Option<ElasticsearchDestinationDescription>,
    #[doc="<p>The destination in Amazon S3.</p>"]
    #[serde(rename="ExtendedS3DestinationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extended_s3_destination_description: Option<ExtendedS3DestinationDescription>,
    #[doc="<p>The destination in Amazon Redshift.</p>"]
    #[serde(rename="RedshiftDestinationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub redshift_destination_description: Option<RedshiftDestinationDescription>,
    #[doc="<p>[Deprecated] The destination in Amazon S3.</p>"]
    #[serde(rename="S3DestinationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
}

#[doc="<p>Describes the buffering to perform before delivering data to the Amazon ES destination.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ElasticsearchBufferingHints {
    #[doc="<p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300 (5 minutes).</p>"]
    #[serde(rename="IntervalInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    #[doc="<p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.</p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or higher.</p>"]
    #[serde(rename="SizeInMBs")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

#[doc="<p>Describes the configuration of a destination in Amazon ES.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ElasticsearchDestinationConfiguration {
    #[doc="<p>The buffering options. If no value is specified, the default values for <b>ElasticsearchBufferingHints</b> are used.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the role specified in <b>RoleARN</b>.</p>"]
    #[serde(rename="DomainARN")]
    pub domain_arn: String,
    #[doc="<p>The Elasticsearch index name.</p>"]
    #[serde(rename="IndexName")]
    pub index_name: String,
    #[doc="<p>The Elasticsearch index rotation period. Index rotation appends a timestamp to the IndexName to facilitate expiration of old data. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation\">Index Rotation for Amazon Elasticsearch Service Destination</a>. The default value is <code>OneDay</code>.</p>"]
    #[serde(rename="IndexRotationPeriod")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The retry behavior in the event that Firehose is unable to deliver documents to Amazon ES. The default value is 300 (5 minutes).</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[doc="<p>The ARN of the IAM role to be assumed by Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3\">Amazon S3 Bucket Access</a>.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>Defines how documents should be delivered to Amazon S3. When set to FailedDocumentsOnly, Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with elasticsearch-failed/ appended to the key prefix. When set to AllDocuments, Firehose delivers all incoming records to Amazon S3, and also writes failed documents with elasticsearch-failed/ appended to the prefix. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-s3-backup\">Amazon S3 Backup for Amazon Elasticsearch Service Destination</a>. Default value is FailedDocumentsOnly.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The configuration for the intermediate Amazon S3 location from which Amazon ES obtains data.</p>"]
    #[serde(rename="S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[doc="<p>The Elasticsearch type name.</p>"]
    #[serde(rename="TypeName")]
    pub type_name: String,
}

#[doc="<p>The destination description in Amazon ES.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ElasticsearchDestinationDescription {
    #[doc="<p>The buffering options.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[doc="<p>The CloudWatch logging options.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The ARN of the Amazon ES domain.</p>"]
    #[serde(rename="DomainARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain_arn: Option<String>,
    #[doc="<p>The Elasticsearch index name.</p>"]
    #[serde(rename="IndexName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_name: Option<String>,
    #[doc="<p>The Elasticsearch index rotation period</p>"]
    #[serde(rename="IndexRotationPeriod")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The Amazon ES retry options.</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The Amazon S3 destination.</p>"]
    #[serde(rename="S3DestinationDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    #[doc="<p>The Elasticsearch type name.</p>"]
    #[serde(rename="TypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_name: Option<String>,
}

#[doc="<p>Describes an update for a destination in Amazon ES.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ElasticsearchDestinationUpdate {
    #[doc="<p>The buffering options. If no value is specified, <b>ElasticsearchBufferingHints</b> object default values are used. </p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the IAM role specified in <b>RoleARN</b>.</p>"]
    #[serde(rename="DomainARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub domain_arn: Option<String>,
    #[doc="<p>The Elasticsearch index name.</p>"]
    #[serde(rename="IndexName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_name: Option<String>,
    #[doc="<p>The Elasticsearch index rotation period. Index rotation appends a timestamp to IndexName to facilitate the expiration of old data. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation\">Index Rotation for Amazon Elasticsearch Service Destination</a>. Default value is <code>OneDay</code>.</p>"]
    #[serde(rename="IndexRotationPeriod")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub index_rotation_period: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The retry behavior in the event that Firehose is unable to deliver documents to Amazon ES. Default value is 300 (5 minutes).</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    #[doc="<p>The ARN of the IAM role to be assumed by Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3\">Amazon S3 Bucket Access</a>.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The Amazon S3 destination.</p>"]
    #[serde(rename="S3Update")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[doc="<p>The Elasticsearch type name.</p>"]
    #[serde(rename="TypeName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub type_name: Option<String>,
}

#[doc="<p>Configures retry behavior in the event that Firehose is unable to deliver documents to Amazon ES.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ElasticsearchRetryOptions {
    #[doc="<p>After an initial failure to deliver to Amazon ES, the total amount of time during which Firehose re-attempts delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>"]
    #[serde(rename="DurationInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[doc="<p>Describes the encryption for a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct EncryptionConfiguration {
    #[doc="<p>The encryption key.</p>"]
    #[serde(rename="KMSEncryptionConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub kms_encryption_config: Option<KMSEncryptionConfig>,
    #[doc="<p>Specifically override existing encryption information to ensure no encryption is used.</p>"]
    #[serde(rename="NoEncryptionConfig")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub no_encryption_config: Option<String>,
}

#[doc="<p>Describes the configuration of a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ExtendedS3DestinationConfiguration {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>The buffering option.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is UNCOMPRESSED.</p>"]
    #[serde(rename="CompressionFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compression_format: Option<String>,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>The configuration for backup in Amazon S3.</p>"]
    #[serde(rename="S3BackupConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

#[doc="<p>Describes a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ExtendedS3DestinationDescription {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>The buffering option.</p>"]
    #[serde(rename="BufferingHints")]
    pub buffering_hints: BufferingHints,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>"]
    #[serde(rename="CompressionFormat")]
    pub compression_format: String,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>The configuration for backup in Amazon S3.</p>"]
    #[serde(rename="S3BackupDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

#[doc="<p>Describes an update for a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ExtendedS3DestinationUpdate {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_arn: Option<String>,
    #[doc="<p>The buffering option.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>. </p>"]
    #[serde(rename="CompressionFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compression_format: Option<String>,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>Enables or disables Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The Amazon S3 destination for backup.</p>"]
    #[serde(rename="S3BackupUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
}

#[doc="<p>Describes an encryption key for a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct KMSEncryptionConfig {
    #[doc="<p>The ARN of the encryption key. Must belong to the same region as the destination Amazon S3 bucket.</p>"]
    #[serde(rename="AWSKMSKeyARN")]
    pub awskms_key_arn: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDeliveryStreamsInput {
    #[doc="<p>The name of the delivery stream to start the list with.</p>"]
    #[serde(rename="ExclusiveStartDeliveryStreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub exclusive_start_delivery_stream_name: Option<String>,
    #[doc="<p>The maximum number of delivery streams to list.</p>"]
    #[serde(rename="Limit")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDeliveryStreamsOutput {
    #[doc="<p>The names of the delivery streams.</p>"]
    #[serde(rename="DeliveryStreamNames")]
    pub delivery_stream_names: Vec<String>,
    #[doc="<p>Indicates whether there are more delivery streams available to list.</p>"]
    #[serde(rename="HasMoreDeliveryStreams")]
    pub has_more_delivery_streams: bool,
}

#[doc="<p>Describes a data processing configuration.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ProcessingConfiguration {
    #[doc="<p>Enables or disables data processing.</p>"]
    #[serde(rename="Enabled")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub enabled: Option<bool>,
    #[doc="<p>The data processors.</p>"]
    #[serde(rename="Processors")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processors: Option<Vec<Processor>>,
}

#[doc="<p>Describes a data processor.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct Processor {
    #[doc="<p>The processor parameters.</p>"]
    #[serde(rename="Parameters")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<Vec<ProcessorParameter>>,
    #[doc="<p>The type of processor.</p>"]
    #[serde(rename="Type")]
    pub type_: String,
}

#[doc="<p>Describes the processor parameter.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct ProcessorParameter {
    #[doc="<p>The name of the parameter.</p>"]
    #[serde(rename="ParameterName")]
    pub parameter_name: String,
    #[doc="<p>The parameter value.</p>"]
    #[serde(rename="ParameterValue")]
    pub parameter_value: String,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRecordBatchInput {
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>One or more records.</p>"]
    #[serde(rename="Records")]
    pub records: Vec<Record>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordBatchOutput {
    #[doc="<p>The number of records that might have failed processing.</p>"]
    #[serde(rename="FailedPutCount")]
    pub failed_put_count: i64,
    #[doc="<p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>"]
    #[serde(rename="RequestResponses")]
    pub request_responses: Vec<PutRecordBatchResponseEntry>,
}

#[doc="<p>Contains the result for an individual record from a <a>PutRecordBatch</a> request. If the record is successfully added to your delivery stream, it receives a record ID. If the record fails to be added to your delivery stream, the result includes an error code and an error message.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordBatchResponseEntry {
    #[doc="<p>The error code for an individual record result.</p>"]
    #[serde(rename="ErrorCode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_code: Option<String>,
    #[doc="<p>The error message for an individual record result.</p>"]
    #[serde(rename="ErrorMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub error_message: Option<String>,
    #[doc="<p>The ID of the record.</p>"]
    #[serde(rename="RecordId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct PutRecordInput {
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The record.</p>"]
    #[serde(rename="Record")]
    pub record: Record,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct PutRecordOutput {
    #[doc="<p>The ID of the record.</p>"]
    #[serde(rename="RecordId")]
    pub record_id: String,
}

#[doc="<p>The unit of data in a delivery stream.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct Record {
    #[doc="<p>The data blob, which is base64-encoded when the blob is serialized. The maximum size of the data blob, before base64-encoding, is 1,000 KB.</p>"]
    #[serde(rename="Data")]
    #[serde(
                            deserialize_with="::rusoto_core::serialization::SerdeBlob::deserialize_blob",
                            serialize_with="::rusoto_core::serialization::SerdeBlob::serialize_blob",
                            default,
                        )]
    pub data: Vec<u8>,
}

#[doc="<p>Describes the configuration of a destination in Amazon Redshift.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RedshiftDestinationConfiguration {
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The database connection string.</p>"]
    #[serde(rename="ClusterJDBCURL")]
    pub cluster_jdbcurl: String,
    #[doc="<p>The <code>COPY</code> command.</p>"]
    #[serde(rename="CopyCommand")]
    pub copy_command: CopyCommand,
    #[doc="<p>The user password.</p>"]
    #[serde(rename="Password")]
    pub password: String,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The retry behavior in the event that Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>The configuration for backup in Amazon S3.</p>"]
    #[serde(rename="S3BackupConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The configuration for the intermediate Amazon S3 location from which Amazon Redshift obtains data. Restrictions are described in the topic for <a>CreateDeliveryStream</a>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <b>RedshiftDestinationConfiguration.S3Configuration</b> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p>"]
    #[serde(rename="S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    #[doc="<p>The name of the user.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Describes a destination in Amazon Redshift.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RedshiftDestinationDescription {
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The database connection string.</p>"]
    #[serde(rename="ClusterJDBCURL")]
    pub cluster_jdbcurl: String,
    #[doc="<p>The <code>COPY</code> command.</p>"]
    #[serde(rename="CopyCommand")]
    pub copy_command: CopyCommand,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The retry behavior in the event that Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
    #[doc="<p>The configuration for backup in Amazon S3.</p>"]
    #[serde(rename="S3BackupDescription")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The Amazon S3 destination.</p>"]
    #[serde(rename="S3DestinationDescription")]
    pub s3_destination_description: S3DestinationDescription,
    #[doc="<p>The name of the user.</p>"]
    #[serde(rename="Username")]
    pub username: String,
}

#[doc="<p>Describes an update for a destination in Amazon Redshift.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RedshiftDestinationUpdate {
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The database connection string.</p>"]
    #[serde(rename="ClusterJDBCURL")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cluster_jdbcurl: Option<String>,
    #[doc="<p>The <code>COPY</code> command.</p>"]
    #[serde(rename="CopyCommand")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub copy_command: Option<CopyCommand>,
    #[doc="<p>The user password.</p>"]
    #[serde(rename="Password")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub password: Option<String>,
    #[doc="<p>The data processing configuration.</p>"]
    #[serde(rename="ProcessingConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[doc="<p>The retry behavior in the event that Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>"]
    #[serde(rename="RetryOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="<p>The Amazon S3 backup mode.</p>"]
    #[serde(rename="S3BackupMode")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_mode: Option<String>,
    #[doc="<p>The Amazon S3 destination for backup.</p>"]
    #[serde(rename="S3BackupUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
    #[doc="<p>The Amazon S3 destination.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <b>RedshiftDestinationUpdate.S3Update</b> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p>"]
    #[serde(rename="S3Update")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    #[doc="<p>The name of the user.</p>"]
    #[serde(rename="Username")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub username: Option<String>,
}

#[doc="<p>Configures retry behavior in the event that Firehose is unable to deliver documents to Amazon Redshift.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct RedshiftRetryOptions {
    #[doc="<p>The length of time during which Firehose retries delivery after a failure, starting from the initial request and including the first attempt. The default value is 3600 seconds (60 minutes). Firehose does not retry if the value of <code>DurationInSeconds</code> is 0 (zero) or if the first delivery attempt takes longer than the current value.</p>"]
    #[serde(rename="DurationInSeconds")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[doc="<p>Describes the configuration of a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct S3DestinationConfiguration {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>The buffering option. If no value is specified, <b>BufferingHints</b> object default values are used.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>"]
    #[serde(rename="CompressionFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compression_format: Option<String>,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p>Describes a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct S3DestinationDescription {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    pub bucket_arn: String,
    #[doc="<p>The buffering option. If no value is specified, <b>BufferingHints</b> object default values are used.</p>"]
    #[serde(rename="BufferingHints")]
    pub buffering_hints: BufferingHints,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>"]
    #[serde(rename="CompressionFormat")]
    pub compression_format: String,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    pub role_arn: String,
}

#[doc="<p>Describes an update for a destination in Amazon S3.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct S3DestinationUpdate {
    #[doc="<p>The ARN of the S3 bucket.</p>"]
    #[serde(rename="BucketARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bucket_arn: Option<String>,
    #[doc="<p>The buffering option. If no value is specified, <b>BufferingHints</b> object default values are used.</p>"]
    #[serde(rename="BufferingHints")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    #[doc="<p>The CloudWatch logging options for your delivery stream.</p>"]
    #[serde(rename="CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    #[doc="<p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>"]
    #[serde(rename="CompressionFormat")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub compression_format: Option<String>,
    #[doc="<p>The encryption configuration. If no value is specified, the default is no encryption.</p>"]
    #[serde(rename="EncryptionConfiguration")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    #[doc="<p>The \"YYYY/MM/DD/HH\" time format prefix is automatically used for delivered S3 files. You can specify an extra prefix to be added in front of the time format prefix. Note that if the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html\">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    #[serde(rename="Prefix")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub prefix: Option<String>,
    #[doc="<p>The ARN of the AWS credentials.</p>"]
    #[serde(rename="RoleARN")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateDestinationInput {
    #[doc="<p>Obtain this value from the <b>VersionId</b> result of <a>DeliveryStreamDescription</a>. This value is required, and helps the service to perform conditional operations. For example, if there is a interleaving update and this value is null, then the update destination fails. After the update is successful, the <b>VersionId</b> value is updated. The service then performs a merge of the old configuration with the new configuration.</p>"]
    #[serde(rename="CurrentDeliveryStreamVersionId")]
    pub current_delivery_stream_version_id: String,
    #[doc="<p>The name of the delivery stream.</p>"]
    #[serde(rename="DeliveryStreamName")]
    pub delivery_stream_name: String,
    #[doc="<p>The ID of the destination.</p>"]
    #[serde(rename="DestinationId")]
    pub destination_id: String,
    #[doc="<p>Describes an update for a destination in Amazon ES.</p>"]
    #[serde(rename="ElasticsearchDestinationUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub elasticsearch_destination_update: Option<ElasticsearchDestinationUpdate>,
    #[doc="<p>Describes an update for a destination in Amazon S3.</p>"]
    #[serde(rename="ExtendedS3DestinationUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub extended_s3_destination_update: Option<ExtendedS3DestinationUpdate>,
    #[doc="<p>Describes an update for a destination in Amazon Redshift.</p>"]
    #[serde(rename="RedshiftDestinationUpdate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub redshift_destination_update: Option<RedshiftDestinationUpdate>,
}

#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateDestinationOutput;

/// Errors returned by CreateDeliveryStream
#[derive(Debug, PartialEq)]
pub enum CreateDeliveryStreamError {
    ///<p>The specified input parameter has an value that is not valid.</p>
    InvalidArgument(String),
    ///<p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    ///<p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl CreateDeliveryStreamError {
    pub fn from_body(body: &str) -> CreateDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        CreateDeliveryStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateDeliveryStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateDeliveryStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => CreateDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> CreateDeliveryStreamError {
        CreateDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDeliveryStreamError {
    fn from(err: CredentialsError) -> CreateDeliveryStreamError {
        CreateDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDeliveryStreamError {
    fn from(err: HttpDispatchError) -> CreateDeliveryStreamError {
        CreateDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDeliveryStreamError {
    fn from(err: io::Error) -> CreateDeliveryStreamError {
        CreateDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            CreateDeliveryStreamError::InvalidArgument(ref cause) => cause,
            CreateDeliveryStreamError::LimitExceeded(ref cause) => cause,
            CreateDeliveryStreamError::ResourceInUse(ref cause) => cause,
            CreateDeliveryStreamError::Validation(ref cause) => cause,
            CreateDeliveryStreamError::Credentials(ref err) => err.description(),
            CreateDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDeliveryStream
#[derive(Debug, PartialEq)]
pub enum DeleteDeliveryStreamError {
    ///<p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDeliveryStreamError {
    pub fn from_body(body: &str) -> DeleteDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceInUseException" => {
                        DeleteDeliveryStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDeliveryStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => DeleteDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> DeleteDeliveryStreamError {
        DeleteDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDeliveryStreamError {
    fn from(err: CredentialsError) -> DeleteDeliveryStreamError {
        DeleteDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDeliveryStreamError {
    fn from(err: HttpDispatchError) -> DeleteDeliveryStreamError {
        DeleteDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDeliveryStreamError {
    fn from(err: io::Error) -> DeleteDeliveryStreamError {
        DeleteDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            DeleteDeliveryStreamError::ResourceInUse(ref cause) => cause,
            DeleteDeliveryStreamError::ResourceNotFound(ref cause) => cause,
            DeleteDeliveryStreamError::Validation(ref cause) => cause,
            DeleteDeliveryStreamError::Credentials(ref err) => err.description(),
            DeleteDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDeliveryStream
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryStreamError {
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDeliveryStreamError {
    pub fn from_body(body: &str) -> DescribeDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ResourceNotFoundException" => {
                        DescribeDeliveryStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => DescribeDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> DescribeDeliveryStreamError {
        DescribeDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDeliveryStreamError {
    fn from(err: CredentialsError) -> DescribeDeliveryStreamError {
        DescribeDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDeliveryStreamError {
    fn from(err: HttpDispatchError) -> DescribeDeliveryStreamError {
        DescribeDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDeliveryStreamError {
    fn from(err: io::Error) -> DescribeDeliveryStreamError {
        DescribeDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            DescribeDeliveryStreamError::ResourceNotFound(ref cause) => cause,
            DescribeDeliveryStreamError::Validation(ref cause) => cause,
            DescribeDeliveryStreamError::Credentials(ref err) => err.description(),
            DescribeDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDeliveryStreams
#[derive(Debug, PartialEq)]
pub enum ListDeliveryStreamsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDeliveryStreamsError {
    pub fn from_body(body: &str) -> ListDeliveryStreamsError {
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
                        ListDeliveryStreamsError::Validation(error_message.to_string())
                    }
                    _ => ListDeliveryStreamsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDeliveryStreamsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDeliveryStreamsError {
    fn from(err: serde_json::error::Error) -> ListDeliveryStreamsError {
        ListDeliveryStreamsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDeliveryStreamsError {
    fn from(err: CredentialsError) -> ListDeliveryStreamsError {
        ListDeliveryStreamsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDeliveryStreamsError {
    fn from(err: HttpDispatchError) -> ListDeliveryStreamsError {
        ListDeliveryStreamsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDeliveryStreamsError {
    fn from(err: io::Error) -> ListDeliveryStreamsError {
        ListDeliveryStreamsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListDeliveryStreamsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDeliveryStreamsError {
    fn description(&self) -> &str {
        match *self {
            ListDeliveryStreamsError::Validation(ref cause) => cause,
            ListDeliveryStreamsError::Credentials(ref err) => err.description(),
            ListDeliveryStreamsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListDeliveryStreamsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRecord
#[derive(Debug, PartialEq)]
pub enum PutRecordError {
    ///<p>The specified input parameter has an value that is not valid.</p>
    InvalidArgument(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>The service is unavailable, back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Firehose Limits</a>.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutRecordError {
    pub fn from_body(body: &str) -> PutRecordError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        PutRecordError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutRecordError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutRecordError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => PutRecordError::Validation(error_message.to_string()),
                    _ => PutRecordError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRecordError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRecordError {
    fn from(err: serde_json::error::Error) -> PutRecordError {
        PutRecordError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRecordError {
    fn from(err: CredentialsError) -> PutRecordError {
        PutRecordError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRecordError {
    fn from(err: HttpDispatchError) -> PutRecordError {
        PutRecordError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRecordError {
    fn from(err: io::Error) -> PutRecordError {
        PutRecordError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordError {
    fn description(&self) -> &str {
        match *self {
            PutRecordError::InvalidArgument(ref cause) => cause,
            PutRecordError::ResourceNotFound(ref cause) => cause,
            PutRecordError::ServiceUnavailable(ref cause) => cause,
            PutRecordError::Validation(ref cause) => cause,
            PutRecordError::Credentials(ref err) => err.description(),
            PutRecordError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRecordError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRecordBatch
#[derive(Debug, PartialEq)]
pub enum PutRecordBatchError {
    ///<p>The specified input parameter has an value that is not valid.</p>
    InvalidArgument(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    ///<p>The service is unavailable, back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Firehose Limits</a>.</p>
    ServiceUnavailable(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl PutRecordBatchError {
    pub fn from_body(body: &str) -> PutRecordBatchError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        PutRecordBatchError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PutRecordBatchError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceUnavailableException" => {
                        PutRecordBatchError::ServiceUnavailable(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutRecordBatchError::Validation(error_message.to_string())
                    }
                    _ => PutRecordBatchError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutRecordBatchError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutRecordBatchError {
    fn from(err: serde_json::error::Error) -> PutRecordBatchError {
        PutRecordBatchError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutRecordBatchError {
    fn from(err: CredentialsError) -> PutRecordBatchError {
        PutRecordBatchError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutRecordBatchError {
    fn from(err: HttpDispatchError) -> PutRecordBatchError {
        PutRecordBatchError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutRecordBatchError {
    fn from(err: io::Error) -> PutRecordBatchError {
        PutRecordBatchError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutRecordBatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutRecordBatchError {
    fn description(&self) -> &str {
        match *self {
            PutRecordBatchError::InvalidArgument(ref cause) => cause,
            PutRecordBatchError::ResourceNotFound(ref cause) => cause,
            PutRecordBatchError::ServiceUnavailable(ref cause) => cause,
            PutRecordBatchError::Validation(ref cause) => cause,
            PutRecordBatchError::Credentials(ref err) => err.description(),
            PutRecordBatchError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PutRecordBatchError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDestination
#[derive(Debug, PartialEq)]
pub enum UpdateDestinationError {
    ///<p>Another modification has already happened. Fetch <b>VersionId</b> again and use it to update the destination.</p>
    ConcurrentModification(String),
    ///<p>The specified input parameter has an value that is not valid.</p>
    InvalidArgument(String),
    ///<p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    ///<p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateDestinationError {
    pub fn from_body(body: &str) -> UpdateDestinationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => {
                        UpdateDestinationError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        UpdateDestinationError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateDestinationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateDestinationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateDestinationError::Validation(error_message.to_string())
                    }
                    _ => UpdateDestinationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateDestinationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateDestinationError {
    fn from(err: serde_json::error::Error) -> UpdateDestinationError {
        UpdateDestinationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDestinationError {
    fn from(err: CredentialsError) -> UpdateDestinationError {
        UpdateDestinationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDestinationError {
    fn from(err: HttpDispatchError) -> UpdateDestinationError {
        UpdateDestinationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDestinationError {
    fn from(err: io::Error) -> UpdateDestinationError {
        UpdateDestinationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDestinationError {
    fn description(&self) -> &str {
        match *self {
            UpdateDestinationError::ConcurrentModification(ref cause) => cause,
            UpdateDestinationError::InvalidArgument(ref cause) => cause,
            UpdateDestinationError::ResourceInUse(ref cause) => cause,
            UpdateDestinationError::ResourceNotFound(ref cause) => cause,
            UpdateDestinationError::Validation(ref cause) => cause,
            UpdateDestinationError::Credentials(ref err) => err.description(),
            UpdateDestinationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateDestinationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Firehose API. Firehose clients implement this trait.
pub trait KinesisFirehose {
    #[doc="<p>Creates a delivery stream.</p> <p>By default, you can create up to 20 delivery streams per region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon Elasticsearch Service, or Amazon Redshift. You must specify only one of the following destination configuration parameters: <b>ExtendedS3DestinationConfiguration</b>, <b>S3DestinationConfiguration</b>, <b>ElasticsearchDestinationConfiguration</b>, or <b>RedshiftDestinationConfiguration</b>.</p> <p>When you specify <b>S3DestinationConfiguration</b>, you can also provide the following optional values: <b>BufferingHints</b>, <b>EncryptionConfiguration</b>, and <b>CompressionFormat</b>. By default, if no <b>BufferingHints</b> value is provided, Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. Note that <b>BufferingHints</b> is a hint, so there are some cases where the service cannot adhere to these conditions strictly; for example, record boundaries are such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location, as Firehose first delivers data to S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <b>RedshiftDestinationConfiguration.S3Configuration</b> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <b>RedshiftDestinationConfiguration.S3Configuration</b> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Firehose principal to assume the role, and the role should have permissions that allows the service to deliver the data. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3\">Amazon S3 Bucket Access</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    fn create_delivery_stream(&self,
                              input: &CreateDeliveryStreamInput)
                              -> Result<CreateDeliveryStreamOutput, CreateDeliveryStreamError>;


    #[doc="<p>Deletes a delivery stream and its data.</p> <p>You can delete a delivery stream only if it is in <code>ACTIVE</code> or <code>DELETING</code> state, and not in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>While the delivery stream is <code>DELETING</code> state, the service may continue to accept the records, but the service doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, you should first stop any applications that are sending records before deleting a delivery stream.</p>"]
    fn delete_delivery_stream(&self,
                              input: &DeleteDeliveryStreamInput)
                              -> Result<DeleteDeliveryStreamOutput, DeleteDeliveryStreamError>;


    #[doc="<p>Describes the specified delivery stream and gets the status. For example, after your delivery stream is created, call <a>DescribeDeliveryStream</a> to see if the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it.</p>"]
    fn describe_delivery_stream
        (&self,
         input: &DescribeDeliveryStreamInput)
         -> Result<DescribeDeliveryStreamOutput, DescribeDeliveryStreamError>;


    #[doc="<p>Lists your delivery streams.</p> <p>The number of delivery streams might be too large to return using a single call to <a>ListDeliveryStreams</a>. You can limit the number of delivery streams returned, using the <b>Limit</b> parameter. To determine whether there are more delivery streams to list, check the value of <b>HasMoreDeliveryStreams</b> in the output. If there are more delivery streams to list, you can request them by specifying the name of the last delivery stream returned in the call in the <b>ExclusiveStartDeliveryStreamName</b> parameter of a subsequent call.</p>"]
    fn list_delivery_streams(&self,
                             input: &ListDeliveryStreamsInput)
                             -> Result<ListDeliveryStreamsOutput, ListDeliveryStreamsError>;


    #[doc="<p>Writes a single data record into an Amazon Kinesis Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. Note that if you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/limits.html\">Amazon Kinesis Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data, for example, a segment from a log file, geographic location data, web site clickstream data, etc.</p> <p>Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\\n</code>) or some other character unique within the data. This allows the consumer application(s) to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecord</a> operation returns a <b>RecordId</b>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <a>PutRecord</a> operation throws a <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>"]
    fn put_record(&self, input: &PutRecordInput) -> Result<PutRecordOutput, PutRecordError>;


    #[doc="<p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. Note that if you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/limits.html\">Amazon Kinesis Firehose Limits</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data, for example, a segment from a log file, geographic location data, web site clickstream data, and so on.</p> <p>Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\\n</code>) or some other character unique within the data. This allows the consumer application(s) to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <b>FailedPutCount</b>, and an array of responses, <b>RequestResponses</b>. Each entry in the <b>RequestResponses</b> array provides additional information about the processed record, and directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <b>RequestResponses</b> includes both successfully and unsuccessfully processed records. Firehose attempts to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes a <b>RecordId</b> value, which is unique for the record. An unsuccessfully processed record includes <b>ErrorCode</b> and <b>ErrorMessage</b> values. <b>ErrorCode</b> reflects the type of error, and is one of the following values: <code>ServiceUnavailable</code> or <code>InternalFailure</code>. <b>ErrorMessage</b> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <b>FailedPutCount</b> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>"]
    fn put_record_batch(&self,
                        input: &PutRecordBatchInput)
                        -> Result<PutRecordBatchOutput, PutRecordBatchError>;


    #[doc="<p>Updates the specified destination of the specified delivery stream.</p> <p>You can use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Note that switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified then the existing <a>EncryptionConfiguration</a> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Firehose uses <b>CurrentDeliveryStreamVersionId</b> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. You should use the new version ID to set <b>CurrentDeliveryStreamVersionId</b> in the next call.</p>"]
    fn update_destination(&self,
                          input: &UpdateDestinationInput)
                          -> Result<UpdateDestinationOutput, UpdateDestinationError>;
}
/// A client for the Firehose API.
pub struct KinesisFirehoseClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> KinesisFirehoseClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KinesisFirehoseClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> KinesisFirehose for KinesisFirehoseClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Creates a delivery stream.</p> <p>By default, you can create up to 20 delivery streams per region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon Elasticsearch Service, or Amazon Redshift. You must specify only one of the following destination configuration parameters: <b>ExtendedS3DestinationConfiguration</b>, <b>S3DestinationConfiguration</b>, <b>ElasticsearchDestinationConfiguration</b>, or <b>RedshiftDestinationConfiguration</b>.</p> <p>When you specify <b>S3DestinationConfiguration</b>, you can also provide the following optional values: <b>BufferingHints</b>, <b>EncryptionConfiguration</b>, and <b>CompressionFormat</b>. By default, if no <b>BufferingHints</b> value is provided, Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. Note that <b>BufferingHints</b> is a hint, so there are some cases where the service cannot adhere to these conditions strictly; for example, record boundaries are such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location, as Firehose first delivers data to S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <b>RedshiftDestinationConfiguration.S3Configuration</b> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <b>RedshiftDestinationConfiguration.S3Configuration</b> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Firehose principal to assume the role, and the role should have permissions that allows the service to deliver the data. For more information, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3\">Amazon S3 Bucket Access</a> in the <i>Amazon Kinesis Firehose Developer Guide</i>.</p>"]
    fn create_delivery_stream(&self,
                              input: &CreateDeliveryStreamInput)
                              -> Result<CreateDeliveryStreamOutput, CreateDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.CreateDeliveryStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<CreateDeliveryStreamOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(CreateDeliveryStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes a delivery stream and its data.</p> <p>You can delete a delivery stream only if it is in <code>ACTIVE</code> or <code>DELETING</code> state, and not in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>While the delivery stream is <code>DELETING</code> state, the service may continue to accept the records, but the service doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, you should first stop any applications that are sending records before deleting a delivery stream.</p>"]
    fn delete_delivery_stream(&self,
                              input: &DeleteDeliveryStreamInput)
                              -> Result<DeleteDeliveryStreamOutput, DeleteDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.DeleteDeliveryStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DeleteDeliveryStreamOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDeliveryStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Describes the specified delivery stream and gets the status. For example, after your delivery stream is created, call <a>DescribeDeliveryStream</a> to see if the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it.</p>"]
    fn describe_delivery_stream
        (&self,
         input: &DescribeDeliveryStreamInput)
         -> Result<DescribeDeliveryStreamOutput, DescribeDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.DescribeDeliveryStream");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<DescribeDeliveryStreamOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeDeliveryStreamError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Lists your delivery streams.</p> <p>The number of delivery streams might be too large to return using a single call to <a>ListDeliveryStreams</a>. You can limit the number of delivery streams returned, using the <b>Limit</b> parameter. To determine whether there are more delivery streams to list, check the value of <b>HasMoreDeliveryStreams</b> in the output. If there are more delivery streams to list, you can request them by specifying the name of the last delivery stream returned in the call in the <b>ExclusiveStartDeliveryStreamName</b> parameter of a subsequent call.</p>"]
    fn list_delivery_streams(&self,
                             input: &ListDeliveryStreamsInput)
                             -> Result<ListDeliveryStreamsOutput, ListDeliveryStreamsError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.ListDeliveryStreams");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<ListDeliveryStreamsOutput>(String::from_utf8_lossy(&body).as_ref()).unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDeliveryStreamsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Writes a single data record into an Amazon Kinesis Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. Note that if you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/limits.html\">Amazon Kinesis Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data, for example, a segment from a log file, geographic location data, web site clickstream data, etc.</p> <p>Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\\n</code>) or some other character unique within the data. This allows the consumer application(s) to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecord</a> operation returns a <b>RecordId</b>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <a>PutRecord</a> operation throws a <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>"]
    fn put_record(&self, input: &PutRecordInput) -> Result<PutRecordOutput, PutRecordError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.PutRecord");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutRecordOutput>(String::from_utf8_lossy(&body).as_ref())
                       .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutRecordError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. Note that if you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits, see <a href=\"http://docs.aws.amazon.com/firehose/latest/dev/limits.html\">Amazon Kinesis Firehose Limits</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data, for example, a segment from a log file, geographic location data, web site clickstream data, and so on.</p> <p>Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\\n</code>) or some other character unique within the data. This allows the consumer application(s) to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <b>FailedPutCount</b>, and an array of responses, <b>RequestResponses</b>. Each entry in the <b>RequestResponses</b> array provides additional information about the processed record, and directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <b>RequestResponses</b> includes both successfully and unsuccessfully processed records. Firehose attempts to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes a <b>RecordId</b> value, which is unique for the record. An unsuccessfully processed record includes <b>ErrorCode</b> and <b>ErrorMessage</b> values. <b>ErrorCode</b> reflects the type of error, and is one of the following values: <code>ServiceUnavailable</code> or <code>InternalFailure</code>. <b>ErrorMessage</b> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <b>FailedPutCount</b> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>"]
    fn put_record_batch(&self,
                        input: &PutRecordBatchInput)
                        -> Result<PutRecordBatchOutput, PutRecordBatchError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.PutRecordBatch");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<PutRecordBatchOutput>(String::from_utf8_lossy(&body)
                                                                    .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(PutRecordBatchError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Updates the specified destination of the specified delivery stream.</p> <p>You can use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Note that switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified then the existing <a>EncryptionConfiguration</a> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Firehose uses <b>CurrentDeliveryStreamVersionId</b> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. You should use the new version ID to set <b>CurrentDeliveryStreamVersionId</b> in the next call.</p>"]
    fn update_destination(&self,
                          input: &UpdateDestinationInput)
                          -> Result<UpdateDestinationOutput, UpdateDestinationError> {
        let mut request = SignedRequest::new("POST", "firehose", self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.UpdateDestination");
        let encoded = serde_json::to_string(input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        request.sign(&try!(self.credentials_provider.credentials()));

        let mut response = try!(self.dispatcher.dispatch(&request));

        match response.status {
            StatusCode::Ok => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Ok(serde_json::from_str::<UpdateDestinationOutput>(String::from_utf8_lossy(&body)
                                                                       .as_ref())
                           .unwrap())
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateDestinationError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
