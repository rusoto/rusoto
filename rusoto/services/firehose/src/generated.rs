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
use rusoto_core::region;
use rusoto_core::request::DispatchSignedRequest;
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Describes hints for the buffering to perform before delivering data to the destination. These options are treated as hints, and therefore Kinesis Data Firehose might choose to use different values when it is optimal.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BufferingHints {
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300.</p>
    #[serde(rename = "IntervalInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.</p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or higher.</p>
    #[serde(rename = "SizeInMBs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

/// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CloudWatchLoggingOptions {
    /// <p>Enables or disables CloudWatch logging.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The CloudWatch group name for logging. This value is required if CloudWatch logging is enabled.</p>
    #[serde(rename = "LogGroupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_group_name: Option<String>,
    /// <p>The CloudWatch log stream name for logging. This value is required if CloudWatch logging is enabled.</p>
    #[serde(rename = "LogStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
}

/// <p>Describes a <code>COPY</code> command for Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CopyCommand {
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Kinesis Data Firehose are as follows:</p> <p> <code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p> <p> <code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p> <p> <code>delimiter '|' escape</code> - the delimiter should be escaped.</p> <p> <code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p> <p> <code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p> <p>For more examples, see <a href="http://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
    #[serde(rename = "CopyOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_options: Option<String>,
    /// <p>A comma-separated list of column names.</p>
    #[serde(rename = "DataTableColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_table_columns: Option<String>,
    /// <p>The name of the target table. The table must already exist in the database.</p>
    #[serde(rename = "DataTableName")]
    pub data_table_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDeliveryStreamInput {
    /// <p>The name of the delivery stream. This name must be unique per AWS account in the same AWS Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p><p>The delivery stream type. This parameter can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul></p>
    #[serde(rename = "DeliveryStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<String>,
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,
    /// <p>When a Kinesis data stream is used as the source for the delivery stream, a <a>KinesisStreamSourceConfiguration</a> containing the Kinesis data stream Amazon Resource Name (ARN) and the role ARN for the source stream.</p>
    #[serde(rename = "KinesisStreamSourceConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_configuration: Option<KinesisStreamSourceConfiguration>,
    /// <p>The destination in Amazon Redshift. You can specify only one destination.</p>
    #[serde(rename = "RedshiftDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_configuration: Option<RedshiftDestinationConfiguration>,
    /// <p>The destination in Splunk. You can specify only one destination.</p>
    #[serde(rename = "SplunkDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_configuration: Option<SplunkDestinationConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateDeliveryStreamOutput {
    /// <p>The ARN of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_arn: Option<String>,
}

/// <p>Specifies that you want Kinesis Data Firehose to convert data from the JSON format to the Parquet or ORC format before writing it to Amazon S3. Kinesis Data Firehose uses the serializer and deserializer that you specify, in addition to the column information from the AWS Glue table, to deserialize your input data from JSON and then serialize it to the Parquet or ORC format. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/record-format-conversion.html">Kinesis Data Firehose Record Format Conversion</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataFormatConversionConfiguration {
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies the deserializer that you want Kinesis Data Firehose to use to convert the format of your data from JSON.</p>
    #[serde(rename = "InputFormatConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_configuration: Option<InputFormatConfiguration>,
    /// <p>Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data to the Parquet or ORC format.</p>
    #[serde(rename = "OutputFormatConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format_configuration: Option<OutputFormatConfiguration>,
    /// <p>Specifies the AWS Glue Data Catalog table that contains the column information.</p>
    #[serde(rename = "SchemaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_configuration: Option<SchemaConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDeliveryStreamInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteDeliveryStreamOutput {}

/// <p>Contains information about a delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeliveryStreamDescription {
    /// <p>The date and time that the delivery stream was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the delivery stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The status of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamStatus")]
    pub delivery_stream_status: String,
    /// <p><p>The delivery stream type. This can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul></p>
    #[serde(rename = "DeliveryStreamType")]
    pub delivery_stream_type: String,
    /// <p>The destinations.</p>
    #[serde(rename = "Destinations")]
    pub destinations: Vec<DestinationDescription>,
    /// <p>Indicates whether there are more destinations available to list.</p>
    #[serde(rename = "HasMoreDestinations")]
    pub has_more_destinations: bool,
    /// <p>The date and time that the delivery stream was last updated.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>If the <code>DeliveryStreamType</code> parameter is <code>KinesisStreamAsSource</code>, a <a>SourceDescription</a> object describing the source Kinesis data stream.</p>
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<SourceDescription>,
    /// <p>Each time the destination is updated for a delivery stream, the version ID is changed, and the current version ID is required when updating the destination. This is so that the service knows it is applying the changes to the correct version of the delivery stream.</p>
    #[serde(rename = "VersionId")]
    pub version_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDeliveryStreamInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The ID of the destination to start returning the destination information. Kinesis Data Firehose supports one destination per delivery stream.</p>
    #[serde(rename = "ExclusiveStartDestinationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_destination_id: Option<String>,
    /// <p>The limit on the number of destinations to return. You can have one destination per delivery stream.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeDeliveryStreamOutput {
    /// <p>Information about the delivery stream.</p>
    #[serde(rename = "DeliveryStreamDescription")]
    pub delivery_stream_description: DeliveryStreamDescription,
}

/// <p>The deserializer you want Kinesis Data Firehose to use for converting the input data from JSON. Kinesis Data Firehose then serializes the data to its final format using the <a>Serializer</a>. Kinesis Data Firehose supports two types of deserializers: the <a href="https://cwiki.apache.org/confluence/display/Hive/LanguageManual+DDL#LanguageManualDDL-JSON">Apache Hive JSON SerDe</a> and the <a href="https://github.com/rcongiu/Hive-JSON-Serde">OpenX JSON SerDe</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Deserializer {
    /// <p>The native Hive / HCatalog JsonSerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the OpenX SerDe.</p>
    #[serde(rename = "HiveJsonSerDe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hive_json_ser_de: Option<HiveJsonSerDe>,
    /// <p>The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the native Hive / HCatalog JsonSerDe.</p>
    #[serde(rename = "OpenXJsonSerDe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_x_json_ser_de: Option<OpenXJsonSerDe>,
}

/// <p>Describes the destination for a delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DestinationDescription {
    /// <p>The ID of the destination.</p>
    #[serde(rename = "DestinationId")]
    pub destination_id: String,
    /// <p>The destination in Amazon ES.</p>
    #[serde(rename = "ElasticsearchDestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_description: Option<ElasticsearchDestinationDescription>,
    /// <p>The destination in Amazon S3.</p>
    #[serde(rename = "ExtendedS3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_description: Option<ExtendedS3DestinationDescription>,
    /// <p>The destination in Amazon Redshift.</p>
    #[serde(rename = "RedshiftDestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_description: Option<RedshiftDestinationDescription>,
    /// <p>[Deprecated] The destination in Amazon S3.</p>
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    /// <p>The destination in Splunk.</p>
    #[serde(rename = "SplunkDestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_description: Option<SplunkDestinationDescription>,
}

/// <p>Describes the buffering to perform before delivering data to the Amazon ES destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchBufferingHints {
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300 (5 minutes).</p>
    #[serde(rename = "IntervalInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5.</p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or higher.</p>
    #[serde(rename = "SizeInMBs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

/// <p>Describes the configuration of a destination in Amazon ES.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ElasticsearchDestinationConfiguration {
    /// <p>The buffering options. If no value is specified, the default values for <code>ElasticsearchBufferingHints</code> are used.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the role specified in <b>RoleARN</b>. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "DomainARN")]
    pub domain_arn: String,
    /// <p>The Elasticsearch index name.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p>The Elasticsearch index rotation period. Index rotation appends a time stamp to the <code>IndexName</code> to facilitate the expiration of old data. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation">Index Rotation for the Amazon ES Destination</a>. The default value is <code>OneDay</code>.</p>
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES. The default value is 300 (5 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with <code>elasticsearch-failed/</code> appended to the key prefix. When set to <code>AllDocuments</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents with <code>elasticsearch-failed/</code> appended to the prefix. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-s3-backup">Amazon S3 Backup for the Amazon ES Destination</a>. Default value is <code>FailedDocumentsOnly</code>.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The configuration for the backup Amazon S3 location.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    /// <p>The Elasticsearch type name. For Elasticsearch 6.x, there can be only one type per index. If you try to specify a new type for an existing index that already has another type, Kinesis Data Firehose returns an error during run time.</p>
    #[serde(rename = "TypeName")]
    pub type_name: String,
}

/// <p>The destination description in Amazon ES.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ElasticsearchDestinationDescription {
    /// <p>The buffering options.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The Amazon CloudWatch logging options.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The ARN of the Amazon ES domain. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "DomainARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The Elasticsearch index name.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The Elasticsearch index rotation period</p>
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The Amazon ES retry options.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    /// <p>The Elasticsearch type name.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// <p>Describes an update for a destination in Amazon ES.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ElasticsearchDestinationUpdate {
    /// <p>The buffering options. If no value is specified, <b>ElasticsearchBufferingHints</b> object default values are used. </p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the IAM role specified in <b>RoleARN</b>. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "DomainARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The Elasticsearch index name.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The Elasticsearch index rotation period. Index rotation appends a time stamp to <code>IndexName</code> to facilitate the expiration of old data. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation">Index Rotation for the Amazon ES Destination</a>. Default value is <code>OneDay</code>.</p>
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES. The default value is 300 (5 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    /// <p>The Elasticsearch type name. For Elasticsearch 6.x, there can be only one type per index. If you try to specify a new type for an existing index that already has another type, Kinesis Data Firehose returns an error during runtime.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ElasticsearchRetryOptions {
    /// <p>After an initial failure to deliver to Amazon ES, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

/// <p>Describes the encryption for a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EncryptionConfiguration {
    /// <p>The encryption key.</p>
    #[serde(rename = "KMSEncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_config: Option<KMSEncryptionConfig>,
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    #[serde(rename = "NoEncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_encryption_config: Option<String>,
}

/// <p>Describes the configuration of a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExtendedS3DestinationConfiguration {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>The buffering option.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is UNCOMPRESSED.</p>
    #[serde(rename = "CompressionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The configuration for backup in Amazon S3.</p>
    #[serde(rename = "S3BackupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

/// <p>Describes a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExtendedS3DestinationDescription {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>The buffering option.</p>
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: BufferingHints,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    #[serde(rename = "CompressionFormat")]
    pub compression_format: String,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The configuration for backup in Amazon S3.</p>
    #[serde(rename = "S3BackupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
}

/// <p>Describes an update for a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ExtendedS3DestinationUpdate {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    /// <p>The buffering option.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>. </p>
    #[serde(rename = "CompressionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Enables or disables Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The Amazon S3 destination for backup.</p>
    #[serde(rename = "S3BackupUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
}

/// <p>The native Hive / HCatalog JsonSerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the OpenX SerDe.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HiveJsonSerDe {
    /// <p>Indicates how you want Kinesis Data Firehose to parse the date and time stamps that may be present in your input data JSON. To specify these format strings, follow the pattern syntax of JodaTime's DateTimeFormat format strings. For more information, see <a href="https://www.joda.org/joda-time/apidocs/org/joda/time/format/DateTimeFormat.html">Class DateTimeFormat</a>. You can also use the special value <code>millis</code> to parse time stamps in epoch milliseconds. If you don't specify a format, Kinesis Data Firehose uses <code>java.sql.Timestamp::valueOf</code> by default.</p>
    #[serde(rename = "TimestampFormats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_formats: Option<Vec<String>>,
}

/// <p>Specifies the deserializer you want to use to convert the format of the input data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputFormatConfiguration {
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    #[serde(rename = "Deserializer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deserializer: Option<Deserializer>,
}

/// <p>Describes an encryption key for a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KMSEncryptionConfig {
    /// <p>The Amazon Resource Name (ARN) of the encryption key. Must belong to the same AWS Region as the destination Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "AWSKMSKeyARN")]
    pub awskms_key_arn: String,
}

/// <p>The stream and role Amazon Resource Names (ARNs) for a Kinesis data stream used as the source for a delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisStreamSourceConfiguration {
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    #[serde(rename = "KinesisStreamARN")]
    pub kinesis_stream_arn: String,
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM) ARN Format</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Details about a Kinesis data stream used as the source for a Kinesis Data Firehose delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KinesisStreamSourceDescription {
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this time stamp.</p>
    #[serde(rename = "DeliveryStartTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_start_timestamp: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    #[serde(rename = "KinesisStreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_arn: Option<String>,
    /// <p>The ARN of the role used by the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM) ARN Format</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDeliveryStreamsInput {
    /// <p>The delivery stream type. This can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul> <p>This parameter is optional. If this parameter is omitted, delivery streams of all types are returned.</p>
    #[serde(rename = "DeliveryStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<String>,
    /// <p>The name of the delivery stream to start the list with.</p>
    #[serde(rename = "ExclusiveStartDeliveryStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_delivery_stream_name: Option<String>,
    /// <p>The maximum number of delivery streams to list. The default value is 10.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListDeliveryStreamsOutput {
    /// <p>The names of the delivery streams.</p>
    #[serde(rename = "DeliveryStreamNames")]
    pub delivery_stream_names: Vec<String>,
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    #[serde(rename = "HasMoreDeliveryStreams")]
    pub has_more_delivery_streams: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForDeliveryStreamInput {
    /// <p>The name of the delivery stream whose tags you want to list.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The key to use as the starting point for the list of tags. If you set this parameter, <code>ListTagsForDeliveryStream</code> gets all tags that occur after <code>ExclusiveStartTagKey</code>.</p>
    #[serde(rename = "ExclusiveStartTagKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_tag_key: Option<String>,
    /// <p>The number of tags to return. If this number is less than the total number of tags associated with the delivery stream, <code>HasMoreTags</code> is set to <code>true</code> in the response. To list additional tags, set <code>ExclusiveStartTagKey</code> to the last key in the response. </p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsForDeliveryStreamOutput {
    /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
    #[serde(rename = "HasMoreTags")]
    pub has_more_tags: bool,
    /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

/// <p>The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the native Hive / HCatalog JsonSerDe.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenXJsonSerDe {
    /// <p>When set to <code>true</code>, which is the default, Kinesis Data Firehose converts JSON keys to lowercase before deserializing them.</p>
    #[serde(rename = "CaseInsensitive")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_insensitive: Option<bool>,
    /// <p>Maps column names to JSON keys that aren't identical to the column names. This is useful when the JSON contains keys that are Hive keywords. For example, <code>timestamp</code> is a Hive keyword. If you have a JSON key named <code>timestamp</code>, set this parameter to <code>{"ts": "timestamp"}</code> to map this key to a column named <code>ts</code>.</p>
    #[serde(rename = "ColumnToJsonKeyMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_to_json_key_mappings: Option<::std::collections::HashMap<String, String>>,
    /// <p>When set to <code>true</code>, specifies that the names of the keys include dots and that you want Kinesis Data Firehose to replace them with underscores. This is useful because Apache Hive does not allow dots in column names. For example, if the JSON contains a key whose name is "a.b", you can define the column name to be "a_b" when using this option.</p> <p>The default is <code>false</code>.</p>
    #[serde(rename = "ConvertDotsInJsonKeysToUnderscores")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub convert_dots_in_json_keys_to_underscores: Option<bool>,
}

/// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrcSerDe {
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.</p>
    #[serde(rename = "BlockSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i64>,
    /// <p>The column names for which you want Kinesis Data Firehose to create bloom filters. The default is <code>null</code>.</p>
    #[serde(rename = "BloomFilterColumns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloom_filter_columns: Option<Vec<String>>,
    /// <p>The Bloom filter false positive probability (FPP). The lower the FPP, the bigger the Bloom filter. The default value is 0.05, the minimum is 0, and the maximum is 1.</p>
    #[serde(rename = "BloomFilterFalsePositiveProbability")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bloom_filter_false_positive_probability: Option<f64>,
    /// <p>The compression code to use over data blocks. The default is <code>SNAPPY</code>.</p>
    #[serde(rename = "Compression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    /// <p>Represents the fraction of the total number of non-null rows. To turn off dictionary encoding, set this fraction to a number that is less than the number of distinct keys in a dictionary. To always use dictionary encoding, set this threshold to 1.</p>
    #[serde(rename = "DictionaryKeyThreshold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dictionary_key_threshold: Option<f64>,
    /// <p>Set this to <code>true</code> to indicate that you want stripes to be padded to the HDFS block boundaries. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is <code>false</code>.</p>
    #[serde(rename = "EnablePadding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_padding: Option<bool>,
    /// <p>The version of the file to write. The possible values are <code>V0_11</code> and <code>V0_12</code>. The default is <code>V0_12</code>.</p>
    #[serde(rename = "FormatVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format_version: Option<String>,
    /// <p>A number between 0 and 1 that defines the tolerance for block padding as a decimal fraction of stripe size. The default value is 0.05, which means 5 percent of stripe size.</p> <p>For the default values of 64 MiB ORC stripes and 256 MiB HDFS blocks, the default block padding tolerance of 5 percent reserves a maximum of 3.2 MiB for padding within the 256 MiB block. In such a case, if the available size within the block is more than 3.2 MiB, a new, smaller stripe is inserted to fit within that space. This ensures that no stripe crosses block boundaries and causes remote reads within a node-local task.</p> <p>Kinesis Data Firehose ignores this parameter when <a>OrcSerDe$EnablePadding</a> is <code>false</code>.</p>
    #[serde(rename = "PaddingTolerance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub padding_tolerance: Option<f64>,
    /// <p>The number of rows between index entries. The default is 10,000 and the minimum is 1,000.</p>
    #[serde(rename = "RowIndexStride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_index_stride: Option<i64>,
    /// <p>The number of bytes in each stripe. The default is 64 MiB and the minimum is 8 MiB.</p>
    #[serde(rename = "StripeSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_size_bytes: Option<i64>,
}

/// <p>Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data before it writes it to Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputFormatConfiguration {
    /// <p>Specifies which serializer to use. You can choose either the ORC SerDe or the Parquet SerDe. If both are non-null, the server rejects the request.</p>
    #[serde(rename = "Serializer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializer: Option<Serializer>,
}

/// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParquetSerDe {
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.</p>
    #[serde(rename = "BlockSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i64>,
    /// <p>The compression code to use over data blocks. The possible values are <code>UNCOMPRESSED</code>, <code>SNAPPY</code>, and <code>GZIP</code>, with the default being <code>SNAPPY</code>. Use <code>SNAPPY</code> for higher decompression speed. Use <code>GZIP</code> if the compression ration is more important than speed.</p>
    #[serde(rename = "Compression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<String>,
    /// <p>Indicates whether to enable dictionary compression.</p>
    #[serde(rename = "EnableDictionaryCompression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dictionary_compression: Option<bool>,
    /// <p>The maximum amount of padding to apply. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 0.</p>
    #[serde(rename = "MaxPaddingBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_padding_bytes: Option<i64>,
    /// <p>The Parquet page size. Column chunks are divided into pages. A page is conceptually an indivisible unit (in terms of compression and encoding). The minimum value is 64 KiB and the default is 1 MiB.</p>
    #[serde(rename = "PageSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size_bytes: Option<i64>,
    /// <p>Indicates the version of row format to output. The possible values are <code>V1</code> and <code>V2</code>. The default is <code>V1</code>.</p>
    #[serde(rename = "WriterVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writer_version: Option<String>,
}

/// <p>Describes a data processing configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessingConfiguration {
    /// <p>Enables or disables data processing.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The data processors.</p>
    #[serde(rename = "Processors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<Processor>>,
}

/// <p>Describes a data processor.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Processor {
    /// <p>The processor parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ProcessorParameter>>,
    /// <p>The type of processor.</p>
    #[serde(rename = "Type")]
    pub type_: String,
}

/// <p>Describes the processor parameter.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProcessorParameter {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    pub parameter_name: String,
    /// <p>The parameter value.</p>
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRecordBatchInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>One or more records.</p>
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutRecordBatchOutput {
    /// <p>The number of records that might have failed processing.</p>
    #[serde(rename = "FailedPutCount")]
    pub failed_put_count: i64,
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    #[serde(rename = "RequestResponses")]
    pub request_responses: Vec<PutRecordBatchResponseEntry>,
}

/// <p>Contains the result for an individual record from a <a>PutRecordBatch</a> request. If the record is successfully added to your delivery stream, it receives a record ID. If the record fails to be added to your delivery stream, the result includes an error code and an error message.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutRecordBatchResponseEntry {
    /// <p>The error code for an individual record result.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message for an individual record result.</p>
    #[serde(rename = "ErrorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the record.</p>
    #[serde(rename = "RecordId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutRecordInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The record.</p>
    #[serde(rename = "Record")]
    pub record: Record,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct PutRecordOutput {
    /// <p>The ID of the record.</p>
    #[serde(rename = "RecordId")]
    pub record_id: String,
}

/// <p>The unit of data in a delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Record {
    /// <p>The data blob, which is base64-encoded when the blob is serialized. The maximum size of the data blob, before base64-encoding, is 1,000 KB.</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub data: Vec<u8>,
}

/// <p>Describes the configuration of a destination in Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RedshiftDestinationConfiguration {
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The database connection string.</p>
    #[serde(rename = "ClusterJDBCURL")]
    pub cluster_jdbcurl: String,
    /// <p>The <code>COPY</code> command.</p>
    #[serde(rename = "CopyCommand")]
    pub copy_command: CopyCommand,
    /// <p>The user password.</p>
    #[serde(rename = "Password")]
    pub password: String,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The configuration for backup in Amazon S3.</p>
    #[serde(rename = "S3BackupConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_configuration: Option<S3DestinationConfiguration>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The configuration for the intermediate Amazon S3 location from which Amazon Redshift obtains data. Restrictions are described in the topic for <a>CreateDeliveryStream</a>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes a destination in Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct RedshiftDestinationDescription {
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The database connection string.</p>
    #[serde(rename = "ClusterJDBCURL")]
    pub cluster_jdbcurl: String,
    /// <p>The <code>COPY</code> command.</p>
    #[serde(rename = "CopyCommand")]
    pub copy_command: CopyCommand,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The configuration for backup in Amazon S3.</p>
    #[serde(rename = "S3BackupDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_description: Option<S3DestinationDescription>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3DestinationDescription")]
    pub s3_destination_description: S3DestinationDescription,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes an update for a destination in Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RedshiftDestinationUpdate {
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The database connection string.</p>
    #[serde(rename = "ClusterJDBCURL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_jdbcurl: Option<String>,
    /// <p>The <code>COPY</code> command.</p>
    #[serde(rename = "CopyCommand")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_command: Option<CopyCommand>,
    /// <p>The user password.</p>
    #[serde(rename = "Password")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon Redshift. Default value is 3600 (60 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<RedshiftRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon S3 backup mode.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The Amazon S3 destination for backup.</p>
    #[serde(rename = "S3BackupUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
    /// <p>The Amazon S3 destination.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationUpdate.S3Update</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p>
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon Redshift.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RedshiftRetryOptions {
    /// <p>The length of time during which Kinesis Data Firehose retries delivery after a failure, starting from the initial request and including the first attempt. The default value is 3600 seconds (60 minutes). Kinesis Data Firehose does not retry if the value of <code>DurationInSeconds</code> is 0 (zero) or if the first delivery attempt takes longer than the current value.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

/// <p>Describes the configuration of a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3DestinationConfiguration {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    #[serde(rename = "CompressionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Describes a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct S3DestinationDescription {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    #[serde(rename = "BufferingHints")]
    pub buffering_hints: BufferingHints,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p>
    #[serde(rename = "CompressionFormat")]
    pub compression_format: String,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Describes an update for a destination in Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3DestinationUpdate {
    /// <p>The ARN of the S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "BucketARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn: Option<String>,
    /// <p>The buffering option. If no value is specified, <code>BufferingHints</code> object default values are used.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<BufferingHints>,
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The compression format. If no value is specified, the default is <code>UNCOMPRESSED</code>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified for Amazon Redshift destinations because they are not supported by the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket.</p>
    #[serde(rename = "CompressionFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression_format: Option<String>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can specify an extra prefix to be added in front of the time format prefix. If the prefix ends with a slash, it appears as a folder in the S3 bucket. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#s3-object-name">Amazon S3 Object Name Format</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Specifies the schema to which you want Kinesis Data Firehose to configure your data before it writes it to Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchemaConfiguration {
    /// <p>The ID of the AWS Glue Data Catalog. If you don't supply this, the AWS account ID is used by default.</p>
    #[serde(rename = "CatalogId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_id: Option<String>,
    /// <p>Specifies the name of the AWS Glue database that contains the schema for the output data.</p>
    #[serde(rename = "DatabaseName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    /// <p>If you don't specify an AWS Region, the default is the current Region.</p>
    #[serde(rename = "Region")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// <p>The role that Kinesis Data Firehose can use to access AWS Glue. This role must be in the same account you use for Kinesis Data Firehose. Cross-account roles aren't allowed.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Specifies the AWS Glue table that contains the column information that constitutes your data schema.</p>
    #[serde(rename = "TableName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// <p>Specifies the table version for the output data schema. If you don't specify this version ID, or if you set it to <code>LATEST</code>, Kinesis Data Firehose uses the most recent version. This means that any updates to the table are automatically picked up.</p>
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

/// <p>The serializer that you want Kinesis Data Firehose to use to convert data to the target format before writing it to Amazon S3. Kinesis Data Firehose supports two types of serializers: the <a href="https://hive.apache.org/javadocs/r1.2.2/api/org/apache/hadoop/hive/ql/io/orc/OrcSerde.html">ORC SerDe</a> and the <a href="https://hive.apache.org/javadocs/r1.2.2/api/org/apache/hadoop/hive/ql/io/parquet/serde/ParquetHiveSerDe.html">Parquet SerDe</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Serializer {
    /// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
    #[serde(rename = "OrcSerDe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orc_ser_de: Option<OrcSerDe>,
    /// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
    #[serde(rename = "ParquetSerDe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parquet_ser_de: Option<ParquetSerDe>,
}

/// <p>Details about a Kinesis data stream used as the source for a Kinesis Data Firehose delivery stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SourceDescription {
    /// <p>The <a>KinesisStreamSourceDescription</a> value for the source Kinesis data stream.</p>
    #[serde(rename = "KinesisStreamSourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_description: Option<KinesisStreamSourceDescription>,
}

/// <p>Describes the configuration of a destination in Splunk.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SplunkDestinationConfiguration {
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The amount of time that Kinesis Data Firehose waits to receive an acknowledgment from Splunk after it sends it data. At the end of the timeout period, Kinesis Data Firehose either tries to send the data again or considers it an error, based on your retry settings.</p>
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_acknowledgment_timeout_in_seconds: Option<i64>,
    /// <p>The HTTP Event Collector (HEC) endpoint to which Kinesis Data Firehose sends your data.</p>
    #[serde(rename = "HECEndpoint")]
    pub hec_endpoint: String,
    /// <p>This type can be either "Raw" or "Event."</p>
    #[serde(rename = "HECEndpointType")]
    pub hec_endpoint_type: String,
    /// <p>This is a GUID that you obtain from your Splunk cluster when you create a new HEC endpoint.</p>
    #[serde(rename = "HECToken")]
    pub hec_token: String,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver data to Splunk, or if it doesn't receive an acknowledgment of receipt from Splunk.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    /// <p>Defines how documents should be delivered to Amazon S3. When set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any data that could not be indexed to the configured Amazon S3 destination. When set to <code>AllDocuments</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents to Amazon S3. Default value is <code>FailedDocumentsOnly</code>. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The configuration for the backup Amazon S3 location.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
}

/// <p>Describes a destination in Splunk.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct SplunkDestinationDescription {
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The amount of time that Kinesis Data Firehose waits to receive an acknowledgment from Splunk after it sends it data. At the end of the timeout period, Kinesis Data Firehose either tries to send the data again or considers it an error, based on your retry settings.</p>
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_acknowledgment_timeout_in_seconds: Option<i64>,
    /// <p>The HTTP Event Collector (HEC) endpoint to which Kinesis Data Firehose sends your data.</p>
    #[serde(rename = "HECEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_endpoint: Option<String>,
    /// <p>This type can be either "Raw" or "Event."</p>
    #[serde(rename = "HECEndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_endpoint_type: Option<String>,
    /// <p>A GUID you obtain from your Splunk cluster when you create a new HEC endpoint.</p>
    #[serde(rename = "HECToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_token: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver data to Splunk or if it doesn't receive an acknowledgment of receipt from Splunk.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    /// <p>Defines how documents should be delivered to Amazon S3. When set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any data that could not be indexed to the configured Amazon S3 destination. When set to <code>AllDocuments</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents to Amazon S3. Default value is <code>FailedDocumentsOnly</code>. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>The Amazon S3 destination.&gt;</p>
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
}

/// <p>Describes an update for a destination in Splunk.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SplunkDestinationUpdate {
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The amount of time that Kinesis Data Firehose waits to receive an acknowledgment from Splunk after it sends data. At the end of the timeout period, Kinesis Data Firehose either tries to send the data again or considers it an error, based on your retry settings.</p>
    #[serde(rename = "HECAcknowledgmentTimeoutInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_acknowledgment_timeout_in_seconds: Option<i64>,
    /// <p>The HTTP Event Collector (HEC) endpoint to which Kinesis Data Firehose sends your data.</p>
    #[serde(rename = "HECEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_endpoint: Option<String>,
    /// <p>This type can be either "Raw" or "Event."</p>
    #[serde(rename = "HECEndpointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_endpoint_type: Option<String>,
    /// <p>A GUID that you obtain from your Splunk cluster when you create a new HEC endpoint.</p>
    #[serde(rename = "HECToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hec_token: Option<String>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver data to Splunk or if it doesn't receive an acknowledgment of receipt from Splunk.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<SplunkRetryOptions>,
    /// <p>Defines how documents should be delivered to Amazon S3. When set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any data that could not be indexed to the configured Amazon S3 destination. When set to <code>AllDocuments</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents to Amazon S3. Default value is <code>FailedDocumentsOnly</code>. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<String>,
    /// <p>Your update to the configuration of the backup Amazon S3 location.</p>
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
}

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to Splunk, or if it doesn't receive an acknowledgment from Splunk.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SplunkRetryOptions {
    /// <p>The total amount of time that Kinesis Data Firehose spends on retries. This duration starts after the initial attempt to send data to Splunk fails. It doesn't include the periods during which Kinesis Data Firehose waits for acknowledgment from Splunk after each attempt.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

/// <p>Metadata that you can assign to a delivery stream, consisting of a key-value pair.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A unique identifier for the tag. Maximum length: 128 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An optional string, which you can use to describe or define the tag. Maximum length: 256 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagDeliveryStreamInput {
    /// <p>The name of the delivery stream to which you want to add the tags.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>A set of key-value pairs to use to create the tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TagDeliveryStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagDeliveryStreamInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>A list of tag keys. Each corresponding tag is removed from the delivery stream.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UntagDeliveryStreamOutput {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDestinationInput {
    /// <p>Obtain this value from the <b>VersionId</b> result of <a>DeliveryStreamDescription</a>. This value is required, and helps the service perform conditional operations. For example, if there is an interleaving update and this value is null, then the update destination fails. After the update is successful, the <code>VersionId</code> value is updated. The service then performs a merge of the old configuration with the new configuration.</p>
    #[serde(rename = "CurrentDeliveryStreamVersionId")]
    pub current_delivery_stream_version_id: String,
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The ID of the destination.</p>
    #[serde(rename = "DestinationId")]
    pub destination_id: String,
    /// <p>Describes an update for a destination in Amazon ES.</p>
    #[serde(rename = "ElasticsearchDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_update: Option<ElasticsearchDestinationUpdate>,
    /// <p>Describes an update for a destination in Amazon S3.</p>
    #[serde(rename = "ExtendedS3DestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_update: Option<ExtendedS3DestinationUpdate>,
    /// <p>Describes an update for a destination in Amazon Redshift.</p>
    #[serde(rename = "RedshiftDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_update: Option<RedshiftDestinationUpdate>,
    /// <p>Describes an update for a destination in Splunk.</p>
    #[serde(rename = "SplunkDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_update: Option<SplunkDestinationUpdate>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateDestinationOutput {}

/// Errors returned by CreateDeliveryStream
#[derive(Debug, PartialEq)]
pub enum CreateDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>The specified resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by ListTagsForDeliveryStream
#[derive(Debug, PartialEq)]
pub enum ListTagsForDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
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

impl ListTagsForDeliveryStreamError {
    pub fn from_body(body: &str) -> ListTagsForDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        ListTagsForDeliveryStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        ListTagsForDeliveryStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsForDeliveryStreamError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        ListTagsForDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => ListTagsForDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsForDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsForDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> ListTagsForDeliveryStreamError {
        ListTagsForDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForDeliveryStreamError {
    fn from(err: CredentialsError) -> ListTagsForDeliveryStreamError {
        ListTagsForDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForDeliveryStreamError {
    fn from(err: HttpDispatchError) -> ListTagsForDeliveryStreamError {
        ListTagsForDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForDeliveryStreamError {
    fn from(err: io::Error) -> ListTagsForDeliveryStreamError {
        ListTagsForDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsForDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForDeliveryStreamError::InvalidArgument(ref cause) => cause,
            ListTagsForDeliveryStreamError::LimitExceeded(ref cause) => cause,
            ListTagsForDeliveryStreamError::ResourceNotFound(ref cause) => cause,
            ListTagsForDeliveryStreamError::Validation(ref cause) => cause,
            ListTagsForDeliveryStreamError::Credentials(ref err) => err.description(),
            ListTagsForDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutRecord
#[derive(Debug, PartialEq)]
pub enum PutRecordError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p>
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
                let raw_error_type = json
                    .get("__type")
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
/// Errors returned by TagDeliveryStream
#[derive(Debug, PartialEq)]
pub enum TagDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
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

impl TagDeliveryStreamError {
    pub fn from_body(body: &str) -> TagDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        TagDeliveryStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        TagDeliveryStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        TagDeliveryStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagDeliveryStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => TagDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> TagDeliveryStreamError {
        TagDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagDeliveryStreamError {
    fn from(err: CredentialsError) -> TagDeliveryStreamError {
        TagDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagDeliveryStreamError {
    fn from(err: HttpDispatchError) -> TagDeliveryStreamError {
        TagDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagDeliveryStreamError {
    fn from(err: io::Error) -> TagDeliveryStreamError {
        TagDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TagDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            TagDeliveryStreamError::InvalidArgument(ref cause) => cause,
            TagDeliveryStreamError::LimitExceeded(ref cause) => cause,
            TagDeliveryStreamError::ResourceInUse(ref cause) => cause,
            TagDeliveryStreamError::ResourceNotFound(ref cause) => cause,
            TagDeliveryStreamError::Validation(ref cause) => cause,
            TagDeliveryStreamError::Credentials(ref err) => err.description(),
            TagDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            TagDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagDeliveryStream
#[derive(Debug, PartialEq)]
pub enum UntagDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
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

impl UntagDeliveryStreamError {
    pub fn from_body(body: &str) -> UntagDeliveryStreamError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json
                    .get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidArgumentException" => {
                        UntagDeliveryStreamError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UntagDeliveryStreamError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UntagDeliveryStreamError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UntagDeliveryStreamError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagDeliveryStreamError::Validation(error_message.to_string())
                    }
                    _ => UntagDeliveryStreamError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagDeliveryStreamError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagDeliveryStreamError {
    fn from(err: serde_json::error::Error) -> UntagDeliveryStreamError {
        UntagDeliveryStreamError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagDeliveryStreamError {
    fn from(err: CredentialsError) -> UntagDeliveryStreamError {
        UntagDeliveryStreamError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagDeliveryStreamError {
    fn from(err: HttpDispatchError) -> UntagDeliveryStreamError {
        UntagDeliveryStreamError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagDeliveryStreamError {
    fn from(err: io::Error) -> UntagDeliveryStreamError {
        UntagDeliveryStreamError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UntagDeliveryStreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagDeliveryStreamError {
    fn description(&self) -> &str {
        match *self {
            UntagDeliveryStreamError::InvalidArgument(ref cause) => cause,
            UntagDeliveryStreamError::LimitExceeded(ref cause) => cause,
            UntagDeliveryStreamError::ResourceInUse(ref cause) => cause,
            UntagDeliveryStreamError::ResourceNotFound(ref cause) => cause,
            UntagDeliveryStreamError::Validation(ref cause) => cause,
            UntagDeliveryStreamError::Credentials(ref err) => err.description(),
            UntagDeliveryStreamError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UntagDeliveryStreamError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDestination
#[derive(Debug, PartialEq)]
pub enum UpdateDestinationError {
    /// <p>Another modification has already happened. Fetch <b>VersionId</b> again and use it to update the destination.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
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
                let raw_error_type = json
                    .get("__type")
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
    /// <p>Creates a Kinesis Data Firehose delivery stream.</p> <p>By default, you can create up to 50 delivery streams per AWS Region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>A Kinesis Data Firehose delivery stream can be configured to receive records directly from providers using <a>PutRecord</a> or <a>PutRecordBatch</a>, or it can be configured to use an existing Kinesis stream as its source. To specify a Kinesis data stream as input, set the <code>DeliveryStreamType</code> parameter to <code>KinesisStreamAsSource</code>, and provide the Kinesis stream Amazon Resource Name (ARN) and role ARN in the <code>KinesisStreamSourceConfiguration</code> parameter.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon ES, Amazon Redshift, or Splunk. You must specify only one of the following destination configuration parameters: <b>ExtendedS3DestinationConfiguration</b>, <b>S3DestinationConfiguration</b>, <b>ElasticsearchDestinationConfiguration</b>, <b>RedshiftDestinationConfiguration</b>, or <b>SplunkDestinationConfiguration</b>.</p> <p>When you specify <b>S3DestinationConfiguration</b>, you can also provide the following optional values: <b>BufferingHints</b>, <b>EncryptionConfiguration</b>, and <b>CompressionFormat</b>. By default, if no <b>BufferingHints</b> value is provided, Kinesis Data Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. <b>BufferingHints</b> is a hint, so there are some cases where the service cannot adhere to these conditions strictly. For example, record boundaries might be such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location. Kinesis Data Firehose first delivers data to Amazon S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <b>RedshiftDestinationConfiguration.S3Configuration</b> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Kinesis Data Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Kinesis Data Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Kinesis Data Firehose principal to assume the role, and the role should have permissions that allow the service to deliver the data. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    fn create_delivery_stream(
        &self,
        input: CreateDeliveryStreamInput,
    ) -> RusotoFuture<CreateDeliveryStreamOutput, CreateDeliveryStreamError>;

    /// <p>Deletes a delivery stream and its data.</p> <p>You can delete a delivery stream only if it is in <code>ACTIVE</code> or <code>DELETING</code> state, and not in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>While the delivery stream is <code>DELETING</code> state, the service might continue to accept the records, but it doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, you should first stop any applications that are sending records before deleting a delivery stream.</p>
    fn delete_delivery_stream(
        &self,
        input: DeleteDeliveryStreamInput,
    ) -> RusotoFuture<DeleteDeliveryStreamOutput, DeleteDeliveryStreamError>;

    /// <p>Describes the specified delivery stream and gets the status. For example, after your delivery stream is created, call <code>DescribeDeliveryStream</code> to see whether the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it.</p>
    fn describe_delivery_stream(
        &self,
        input: DescribeDeliveryStreamInput,
    ) -> RusotoFuture<DescribeDeliveryStreamOutput, DescribeDeliveryStreamError>;

    /// <p>Lists your delivery streams.</p> <p>The number of delivery streams might be too large to return using a single call to <code>ListDeliveryStreams</code>. You can limit the number of delivery streams returned, using the <b>Limit</b> parameter. To determine whether there are more delivery streams to list, check the value of <code>HasMoreDeliveryStreams</code> in the output. If there are more delivery streams to list, you can request them by specifying the name of the last delivery stream returned in the call in the <code>ExclusiveStartDeliveryStreamName</code> parameter of a subsequent call.</p>
    fn list_delivery_streams(
        &self,
        input: ListDeliveryStreamsInput,
    ) -> RusotoFuture<ListDeliveryStreamsOutput, ListDeliveryStreamsError>;

    /// <p>Lists the tags for the specified delivery stream. This operation has a limit of five transactions per second per account. </p>
    fn list_tags_for_delivery_stream(
        &self,
        input: ListTagsForDeliveryStreamInput,
    ) -> RusotoFuture<ListTagsForDeliveryStreamOutput, ListTagsForDeliveryStreamError>;

    /// <p>Writes a single data record into an Amazon Kinesis Data Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it can be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <code>PutRecord</code> operation returns a <code>RecordId</code>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <code>PutRecord</code> operation throws a <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it tries to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>
    fn put_record(&self, input: PutRecordInput) -> RusotoFuture<PutRecordOutput, PutRecordError>;

    /// <p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it could be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <b>FailedPutCount</b>, and an array of responses, <b>RequestResponses</b>. Each entry in the <b>RequestResponses</b> array provides additional information about the processed record. It directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <b>RequestResponses</b> includes both successfully and unsuccessfully processed records. Kinesis Data Firehose tries to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes a <b>RecordId</b> value, which is unique for the record. An unsuccessfully processed record includes <b>ErrorCode</b> and <b>ErrorMessage</b> values. <b>ErrorCode</b> reflects the type of error, and is one of the following values: <code>ServiceUnavailable</code> or <code>InternalFailure</code>. <b>ErrorMessage</b> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <b>FailedPutCount</b> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>
    fn put_record_batch(
        &self,
        input: PutRecordBatchInput,
    ) -> RusotoFuture<PutRecordBatchOutput, PutRecordBatchError>;

    /// <p>Adds or updates tags for the specified delivery stream. A tag is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p> Each delivery stream can have up to 50 tags. </p> <p> This operation has a limit of five transactions per second per account. </p>
    fn tag_delivery_stream(
        &self,
        input: TagDeliveryStreamInput,
    ) -> RusotoFuture<TagDeliveryStreamOutput, TagDeliveryStreamError>;

    /// <p>Removes tags from the specified delivery stream. Removed tags are deleted, and you can't recover them after this operation successfully completes.</p> <p>If you specify a tag that doesn't exist, the operation ignores it.</p> <p>This operation has a limit of five transactions per second per account. </p>
    fn untag_delivery_stream(
        &self,
        input: UntagDeliveryStreamInput,
    ) -> RusotoFuture<UntagDeliveryStreamOutput, UntagDeliveryStreamError>;

    /// <p>Updates the specified destination of the specified delivery stream.</p> <p>Use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Kinesis Data Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified, then the existing <code>EncryptionConfiguration</code> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Kinesis Data Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Kinesis Data Firehose uses <b>CurrentDeliveryStreamVersionId</b> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. Use the new version ID to set <b>CurrentDeliveryStreamVersionId</b> in the next call.</p>
    fn update_destination(
        &self,
        input: UpdateDestinationInput,
    ) -> RusotoFuture<UpdateDestinationOutput, UpdateDestinationError>;
}
/// A client for the Firehose API.
pub struct KinesisFirehoseClient {
    client: Client,
    region: region::Region,
}

impl KinesisFirehoseClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisFirehoseClient {
        KinesisFirehoseClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisFirehoseClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KinesisFirehoseClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl KinesisFirehose for KinesisFirehoseClient {
    /// <p>Creates a Kinesis Data Firehose delivery stream.</p> <p>By default, you can create up to 50 delivery streams per AWS Region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>A Kinesis Data Firehose delivery stream can be configured to receive records directly from providers using <a>PutRecord</a> or <a>PutRecordBatch</a>, or it can be configured to use an existing Kinesis stream as its source. To specify a Kinesis data stream as input, set the <code>DeliveryStreamType</code> parameter to <code>KinesisStreamAsSource</code>, and provide the Kinesis stream Amazon Resource Name (ARN) and role ARN in the <code>KinesisStreamSourceConfiguration</code> parameter.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon ES, Amazon Redshift, or Splunk. You must specify only one of the following destination configuration parameters: <b>ExtendedS3DestinationConfiguration</b>, <b>S3DestinationConfiguration</b>, <b>ElasticsearchDestinationConfiguration</b>, <b>RedshiftDestinationConfiguration</b>, or <b>SplunkDestinationConfiguration</b>.</p> <p>When you specify <b>S3DestinationConfiguration</b>, you can also provide the following optional values: <b>BufferingHints</b>, <b>EncryptionConfiguration</b>, and <b>CompressionFormat</b>. By default, if no <b>BufferingHints</b> value is provided, Kinesis Data Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. <b>BufferingHints</b> is a hint, so there are some cases where the service cannot adhere to these conditions strictly. For example, record boundaries might be such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location. Kinesis Data Firehose first delivers data to Amazon S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <b>RedshiftDestinationConfiguration.S3Configuration</b> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Kinesis Data Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Kinesis Data Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Kinesis Data Firehose principal to assume the role, and the role should have permissions that allow the service to deliver the data. For more information, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    fn create_delivery_stream(
        &self,
        input: CreateDeliveryStreamInput,
    ) -> RusotoFuture<CreateDeliveryStreamOutput, CreateDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.CreateDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes a delivery stream and its data.</p> <p>You can delete a delivery stream only if it is in <code>ACTIVE</code> or <code>DELETING</code> state, and not in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>While the delivery stream is <code>DELETING</code> state, the service might continue to accept the records, but it doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, you should first stop any applications that are sending records before deleting a delivery stream.</p>
    fn delete_delivery_stream(
        &self,
        input: DeleteDeliveryStreamInput,
    ) -> RusotoFuture<DeleteDeliveryStreamOutput, DeleteDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.DeleteDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Describes the specified delivery stream and gets the status. For example, after your delivery stream is created, call <code>DescribeDeliveryStream</code> to see whether the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it.</p>
    fn describe_delivery_stream(
        &self,
        input: DescribeDeliveryStreamInput,
    ) -> RusotoFuture<DescribeDeliveryStreamOutput, DescribeDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.DescribeDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists your delivery streams.</p> <p>The number of delivery streams might be too large to return using a single call to <code>ListDeliveryStreams</code>. You can limit the number of delivery streams returned, using the <b>Limit</b> parameter. To determine whether there are more delivery streams to list, check the value of <code>HasMoreDeliveryStreams</code> in the output. If there are more delivery streams to list, you can request them by specifying the name of the last delivery stream returned in the call in the <code>ExclusiveStartDeliveryStreamName</code> parameter of a subsequent call.</p>
    fn list_delivery_streams(
        &self,
        input: ListDeliveryStreamsInput,
    ) -> RusotoFuture<ListDeliveryStreamsOutput, ListDeliveryStreamsError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.ListDeliveryStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListDeliveryStreamsOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListDeliveryStreamsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Lists the tags for the specified delivery stream. This operation has a limit of five transactions per second per account. </p>
    fn list_tags_for_delivery_stream(
        &self,
        input: ListTagsForDeliveryStreamInput,
    ) -> RusotoFuture<ListTagsForDeliveryStreamOutput, ListTagsForDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "Firehose_20150804.ListTagsForDeliveryStream",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListTagsForDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsForDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Writes a single data record into an Amazon Kinesis Data Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it can be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <code>PutRecord</code> operation returns a <code>RecordId</code>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <code>PutRecord</code> operation throws a <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it tries to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>
    fn put_record(&self, input: PutRecordInput) -> RusotoFuture<PutRecordOutput, PutRecordError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.PutRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRecordOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutRecordError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits, see <a href="http://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it could be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <b>FailedPutCount</b>, and an array of responses, <b>RequestResponses</b>. Each entry in the <b>RequestResponses</b> array provides additional information about the processed record. It directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <b>RequestResponses</b> includes both successfully and unsuccessfully processed records. Kinesis Data Firehose tries to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records.</p> <p>A successfully processed record includes a <b>RecordId</b> value, which is unique for the record. An unsuccessfully processed record includes <b>ErrorCode</b> and <b>ErrorMessage</b> values. <b>ErrorCode</b> reflects the type of error, and is one of the following values: <code>ServiceUnavailable</code> or <code>InternalFailure</code>. <b>ErrorMessage</b> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <b>FailedPutCount</b> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <b>ServiceUnavailableException</b>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p>
    fn put_record_batch(
        &self,
        input: PutRecordBatchInput,
    ) -> RusotoFuture<PutRecordBatchOutput, PutRecordBatchError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.PutRecordBatch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutRecordBatchOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutRecordBatchError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Adds or updates tags for the specified delivery stream. A tag is a key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p> Each delivery stream can have up to 50 tags. </p> <p> This operation has a limit of five transactions per second per account. </p>
    fn tag_delivery_stream(
        &self,
        input: TagDeliveryStreamInput,
    ) -> RusotoFuture<TagDeliveryStreamOutput, TagDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.TagDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TagDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TagDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes tags from the specified delivery stream. Removed tags are deleted, and you can't recover them after this operation successfully completes.</p> <p>If you specify a tag that doesn't exist, the operation ignores it.</p> <p>This operation has a limit of five transactions per second per account. </p>
    fn untag_delivery_stream(
        &self,
        input: UntagDeliveryStreamInput,
    ) -> RusotoFuture<UntagDeliveryStreamOutput, UntagDeliveryStreamError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.UntagDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UntagDeliveryStreamOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UntagDeliveryStreamError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the specified destination of the specified delivery stream.</p> <p>Use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Kinesis Data Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified, then the existing <code>EncryptionConfiguration</code> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Kinesis Data Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Kinesis Data Firehose uses <b>CurrentDeliveryStreamVersionId</b> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. Use the new version ID to set <b>CurrentDeliveryStreamVersionId</b> in the next call.</p>
    fn update_destination(
        &self,
        input: UpdateDestinationInput,
    ) -> RusotoFuture<UpdateDestinationOutput, UpdateDestinationError> {
        let mut request = SignedRequest::new("POST", "firehose", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "Firehose_20150804.UpdateDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateDestinationOutput>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateDestinationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
