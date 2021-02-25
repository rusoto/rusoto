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

use rusoto_core::proto;
use rusoto_core::request::HttpResponse;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

impl KinesisFirehoseClient {
    fn new_signed_request(&self, http_method: &str, request_uri: &str) -> SignedRequest {
        let mut request = SignedRequest::new(http_method, "firehose", &self.region, request_uri);

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
/// <p>Describes hints for the buffering to perform before delivering data to the destination. These options are treated as hints, and therefore Kinesis Data Firehose might choose to use different values when it is optimal. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct BufferingHints {
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300. This parameter is optional but if you specify a value for it, you must also specify a value for <code>SizeInMBs</code>, and vice versa.</p>
    #[serde(rename = "IntervalInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    /// <p>Buffer incoming data to the specified size, in MiBs, before delivering it to the destination. The default value is 5. This parameter is optional but if you specify a value for it, you must also specify a value for <code>IntervalInSeconds</code>, and vice versa.</p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MiB/sec, the value should be 10 MiB or higher.</p>
    #[serde(rename = "SizeInMBs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

/// <p>Describes the Amazon CloudWatch logging options for your delivery stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownCompressionFormat {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum CompressionFormat {
    Gzip,
    HadoopSnappy,
    Snappy,
    Uncompressed,
    Zip,
    #[doc(hidden)]
    UnknownVariant(UnknownCompressionFormat),
}

impl Default for CompressionFormat {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for CompressionFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for CompressionFormat {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for CompressionFormat {
    fn into(self) -> String {
        match self {
            CompressionFormat::Gzip => "GZIP".to_string(),
            CompressionFormat::HadoopSnappy => "HADOOP_SNAPPY".to_string(),
            CompressionFormat::Snappy => "Snappy".to_string(),
            CompressionFormat::Uncompressed => "UNCOMPRESSED".to_string(),
            CompressionFormat::Zip => "ZIP".to_string(),
            CompressionFormat::UnknownVariant(UnknownCompressionFormat { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a CompressionFormat {
    fn into(self) -> &'a str {
        match self {
            CompressionFormat::Gzip => &"GZIP",
            CompressionFormat::HadoopSnappy => &"HADOOP_SNAPPY",
            CompressionFormat::Snappy => &"Snappy",
            CompressionFormat::Uncompressed => &"UNCOMPRESSED",
            CompressionFormat::Zip => &"ZIP",
            CompressionFormat::UnknownVariant(UnknownCompressionFormat { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for CompressionFormat {
    fn from(name: &str) -> Self {
        match name {
            "GZIP" => CompressionFormat::Gzip,
            "HADOOP_SNAPPY" => CompressionFormat::HadoopSnappy,
            "Snappy" => CompressionFormat::Snappy,
            "UNCOMPRESSED" => CompressionFormat::Uncompressed,
            "ZIP" => CompressionFormat::Zip,
            _ => CompressionFormat::UnknownVariant(UnknownCompressionFormat {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for CompressionFormat {
    fn from(name: String) -> Self {
        match &*name {
            "GZIP" => CompressionFormat::Gzip,
            "HADOOP_SNAPPY" => CompressionFormat::HadoopSnappy,
            "Snappy" => CompressionFormat::Snappy,
            "UNCOMPRESSED" => CompressionFormat::Uncompressed,
            "ZIP" => CompressionFormat::Zip,
            _ => CompressionFormat::UnknownVariant(UnknownCompressionFormat { name }),
        }
    }
}

impl ::std::str::FromStr for CompressionFormat {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for CompressionFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for CompressionFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownContentEncoding {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ContentEncoding {
    Gzip,
    None,
    #[doc(hidden)]
    UnknownVariant(UnknownContentEncoding),
}

impl Default for ContentEncoding {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ContentEncoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ContentEncoding {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ContentEncoding {
    fn into(self) -> String {
        match self {
            ContentEncoding::Gzip => "GZIP".to_string(),
            ContentEncoding::None => "NONE".to_string(),
            ContentEncoding::UnknownVariant(UnknownContentEncoding { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ContentEncoding {
    fn into(self) -> &'a str {
        match self {
            ContentEncoding::Gzip => &"GZIP",
            ContentEncoding::None => &"NONE",
            ContentEncoding::UnknownVariant(UnknownContentEncoding { name: original }) => original,
        }
    }
}

impl From<&str> for ContentEncoding {
    fn from(name: &str) -> Self {
        match name {
            "GZIP" => ContentEncoding::Gzip,
            "NONE" => ContentEncoding::None,
            _ => ContentEncoding::UnknownVariant(UnknownContentEncoding {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ContentEncoding {
    fn from(name: String) -> Self {
        match &*name {
            "GZIP" => ContentEncoding::Gzip,
            "NONE" => ContentEncoding::None,
            _ => ContentEncoding::UnknownVariant(UnknownContentEncoding { name }),
        }
    }
}

impl ::std::str::FromStr for ContentEncoding {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ContentEncoding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ContentEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a <code>COPY</code> command for Amazon Redshift.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CopyCommand {
    /// <p>Optional parameters to use with the Amazon Redshift <code>COPY</code> command. For more information, see the "Optional Parameters" section of <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY.html">Amazon Redshift COPY command</a>. Some possible examples that would apply to Kinesis Data Firehose are as follows:</p> <p> <code>delimiter '\t' lzop;</code> - fields are delimited with "\t" (TAB character) and compressed using lzop.</p> <p> <code>delimiter '|'</code> - fields are delimited with "|" (this is the default delimiter).</p> <p> <code>delimiter '|' escape</code> - the delimiter should be escaped.</p> <p> <code>fixedwidth 'venueid:3,venuename:25,venuecity:12,venuestate:2,venueseats:6'</code> - fields are fixed width in the source, with each width specified after every column in the table.</p> <p> <code>JSON 's3://mybucket/jsonpaths.txt'</code> - data is in JSON format, and the path specified is the format of the data.</p> <p>For more examples, see <a href="https://docs.aws.amazon.com/redshift/latest/dg/r_COPY_command_examples.html">Amazon Redshift COPY command examples</a>.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateDeliveryStreamInput {
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration_input:
        Option<DeliveryStreamEncryptionConfigurationInput>,
    /// <p>The name of the delivery stream. This name must be unique per AWS account in the same AWS Region. If the delivery streams are in different accounts or different Regions, you can have multiple delivery streams with the same name.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p><p>The delivery stream type. This parameter can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul></p>
    #[serde(rename = "DeliveryStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<DeliveryStreamType>,
    /// <p>The destination in Amazon ES. You can specify only one destination.</p>
    #[serde(rename = "ElasticsearchDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_destination_configuration: Option<ElasticsearchDestinationConfiguration>,
    /// <p>The destination in Amazon S3. You can specify only one destination.</p>
    #[serde(rename = "ExtendedS3DestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extended_s3_destination_configuration: Option<ExtendedS3DestinationConfiguration>,
    /// <p>Enables configuring Kinesis Firehose to deliver data to any HTTP endpoint destination. You can specify only one destination.</p>
    #[serde(rename = "HttpEndpointDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_configuration: Option<HttpEndpointDestinationConfiguration>,
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
    /// <p>A set of tags to assign to the delivery stream. A tag is a key-value pair that you can define and assign to AWS resources. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the AWS Billing and Cost Management User Guide.</p> <p>You can specify up to 50 tags when creating a delivery stream.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateDeliveryStreamOutput {
    /// <p>The ARN of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_arn: Option<String>,
}

/// <p>Specifies that you want Kinesis Data Firehose to convert data from the JSON format to the Parquet or ORC format before writing it to Amazon S3. Kinesis Data Firehose uses the serializer and deserializer that you specify, in addition to the column information from the AWS Glue table, to deserialize your input data from JSON and then serialize it to the Parquet or ORC format. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/record-format-conversion.html">Kinesis Data Firehose Record Format Conversion</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DataFormatConversionConfiguration {
    /// <p>Defaults to <code>true</code>. Set it to <code>false</code> if you want to disable format conversion while preserving the configuration details.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>Specifies the deserializer that you want Kinesis Data Firehose to use to convert the format of your data from JSON. This parameter is required if <code>Enabled</code> is set to true.</p>
    #[serde(rename = "InputFormatConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_format_configuration: Option<InputFormatConfiguration>,
    /// <p>Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data to the Parquet or ORC format. This parameter is required if <code>Enabled</code> is set to true.</p>
    #[serde(rename = "OutputFormatConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format_configuration: Option<OutputFormatConfiguration>,
    /// <p>Specifies the AWS Glue Data Catalog table that contains the column information. This parameter is required if <code>Enabled</code> is set to true.</p>
    #[serde(rename = "SchemaConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_configuration: Option<SchemaConfiguration>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteDeliveryStreamInput {
    /// <p>Set this to true if you want to delete the delivery stream even if Kinesis Data Firehose is unable to retire the grant for the CMK. Kinesis Data Firehose might be unable to retire the grant due to a customer error, such as when the CMK or the grant are in an invalid state. If you force deletion, you can then use the <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_RevokeGrant.html">RevokeGrant</a> operation to revoke the grant you gave to Kinesis Data Firehose. If a failure to retire the grant happens due to an AWS KMS issue, Kinesis Data Firehose keeps retrying the delete operation.</p> <p>The default value is false.</p>
    #[serde(rename = "AllowForceDelete")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_force_delete: Option<bool>,
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDeliveryStreamOutput {}

/// <p>Contains information about a delivery stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeliveryStreamDescription {
    /// <p>The date and time that the delivery stream was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>The Amazon Resource Name (ARN) of the delivery stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "DeliveryStreamARN")]
    pub delivery_stream_arn: String,
    /// <p>Indicates the server-side encryption (SSE) status for the delivery stream.</p>
    #[serde(rename = "DeliveryStreamEncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration: Option<DeliveryStreamEncryptionConfiguration>,
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The status of the delivery stream. If the status of a delivery stream is <code>CREATING_FAILED</code>, this status doesn't change, and you can't invoke <code>CreateDeliveryStream</code> again on it. However, you can invoke the <a>DeleteDeliveryStream</a> operation to delete it.</p>
    #[serde(rename = "DeliveryStreamStatus")]
    pub delivery_stream_status: DeliveryStreamStatus,
    /// <p><p>The delivery stream type. This can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul></p>
    #[serde(rename = "DeliveryStreamType")]
    pub delivery_stream_type: DeliveryStreamType,
    /// <p>The destinations.</p>
    #[serde(rename = "Destinations")]
    pub destinations: Vec<DestinationDescription>,
    /// <p>Provides details in case one of the following operations fails due to an error related to KMS: <a>CreateDeliveryStream</a>, <a>DeleteDeliveryStream</a>, <a>StartDeliveryStreamEncryption</a>, <a>StopDeliveryStreamEncryption</a>.</p>
    #[serde(rename = "FailureDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
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

/// <p>Contains information about the server-side encryption (SSE) status for the delivery stream, the type customer master key (CMK) in use, if any, and the ARN of the CMK. You can get <code>DeliveryStreamEncryptionConfiguration</code> by invoking the <a>DescribeDeliveryStream</a> operation. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeliveryStreamEncryptionConfiguration {
    /// <p>Provides details in case one of the following operations fails due to an error related to KMS: <a>CreateDeliveryStream</a>, <a>DeleteDeliveryStream</a>, <a>StartDeliveryStreamEncryption</a>, <a>StopDeliveryStreamEncryption</a>.</p>
    #[serde(rename = "FailureDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_description: Option<FailureDescription>,
    /// <p>If <code>KeyType</code> is <code>CUSTOMER_MANAGED_CMK</code>, this field contains the ARN of the customer managed CMK. If <code>KeyType</code> is <code>AWS_OWNED_CMK</code>, <code>DeliveryStreamEncryptionConfiguration</code> doesn't contain a value for <code>KeyARN</code>.</p>
    #[serde(rename = "KeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    /// <p>Indicates the type of customer master key (CMK) that is used for encryption. The default setting is <code>AWS_OWNED_CMK</code>. For more information about CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys (CMKs)</a>.</p>
    #[serde(rename = "KeyType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_type: Option<KeyType>,
    /// <p>This is the server-side encryption (SSE) status for the delivery stream. For a full description of the different values of this status, see <a>StartDeliveryStreamEncryption</a> and <a>StopDeliveryStreamEncryption</a>. If this status is <code>ENABLING_FAILED</code> or <code>DISABLING_FAILED</code>, it is the status of the most recent attempt to enable or disable SSE, respectively.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeliveryStreamEncryptionStatus>,
}

/// <p>Specifies the type and Amazon Resource Name (ARN) of the CMK to use for Server-Side Encryption (SSE). </p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeliveryStreamEncryptionConfigurationInput {
    /// <p>If you set <code>KeyType</code> to <code>CUSTOMER_MANAGED_CMK</code>, you must specify the Amazon Resource Name (ARN) of the CMK. If you set <code>KeyType</code> to <code>AWS_OWNED_CMK</code>, Kinesis Data Firehose uses a service-account CMK.</p>
    #[serde(rename = "KeyARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_arn: Option<String>,
    /// <p><p>Indicates the type of customer master key (CMK) to use for encryption. The default setting is <code>AWS<em>OWNED</em>CMK</code>. For more information about CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/concepts.html#master_keys">Customer Master Keys (CMKs)</a>. When you invoke <a>CreateDeliveryStream</a> or <a>StartDeliveryStreamEncryption</a> with <code>KeyType</code> set to CUSTOMER<em>MANAGED</em>CMK, Kinesis Data Firehose invokes the Amazon KMS operation <a href="https://docs.aws.amazon.com/kms/latest/APIReference/API_CreateGrant.html">CreateGrant</a> to create a grant that allows the Kinesis Data Firehose service to use the customer managed CMK to perform encryption and decryption. Kinesis Data Firehose manages that grant. </p> <p>When you invoke <a>StartDeliveryStreamEncryption</a> to change the CMK for a delivery stream that is encrypted with a customer managed CMK, Kinesis Data Firehose schedules the grant it had on the old CMK for retirement.</p> <p>You can use a CMK of type CUSTOMER<em>MANAGED</em>CMK to encrypt up to 500 delivery streams. If a <a>CreateDeliveryStream</a> or <a>StartDeliveryStreamEncryption</a> operation exceeds this limit, Kinesis Data Firehose throws a <code>LimitExceededException</code>. </p> <important> <p>To encrypt your delivery stream, use symmetric CMKs. Kinesis Data Firehose doesn&#39;t support asymmetric CMKs. For information about symmetric and asymmetric CMKs, see <a href="https://docs.aws.amazon.com/kms/latest/developerguide/symm-asymm-concepts.html">About Symmetric and Asymmetric CMKs</a> in the AWS Key Management Service developer guide.</p> </important></p>
    #[serde(rename = "KeyType")]
    pub key_type: KeyType,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeliveryStreamEncryptionStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeliveryStreamEncryptionStatus {
    Disabled,
    Disabling,
    DisablingFailed,
    Enabled,
    Enabling,
    EnablingFailed,
    #[doc(hidden)]
    UnknownVariant(UnknownDeliveryStreamEncryptionStatus),
}

impl Default for DeliveryStreamEncryptionStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeliveryStreamEncryptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeliveryStreamEncryptionStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeliveryStreamEncryptionStatus {
    fn into(self) -> String {
        match self {
            DeliveryStreamEncryptionStatus::Disabled => "DISABLED".to_string(),
            DeliveryStreamEncryptionStatus::Disabling => "DISABLING".to_string(),
            DeliveryStreamEncryptionStatus::DisablingFailed => "DISABLING_FAILED".to_string(),
            DeliveryStreamEncryptionStatus::Enabled => "ENABLED".to_string(),
            DeliveryStreamEncryptionStatus::Enabling => "ENABLING".to_string(),
            DeliveryStreamEncryptionStatus::EnablingFailed => "ENABLING_FAILED".to_string(),
            DeliveryStreamEncryptionStatus::UnknownVariant(
                UnknownDeliveryStreamEncryptionStatus { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DeliveryStreamEncryptionStatus {
    fn into(self) -> &'a str {
        match self {
            DeliveryStreamEncryptionStatus::Disabled => &"DISABLED",
            DeliveryStreamEncryptionStatus::Disabling => &"DISABLING",
            DeliveryStreamEncryptionStatus::DisablingFailed => &"DISABLING_FAILED",
            DeliveryStreamEncryptionStatus::Enabled => &"ENABLED",
            DeliveryStreamEncryptionStatus::Enabling => &"ENABLING",
            DeliveryStreamEncryptionStatus::EnablingFailed => &"ENABLING_FAILED",
            DeliveryStreamEncryptionStatus::UnknownVariant(
                UnknownDeliveryStreamEncryptionStatus { name: original },
            ) => original,
        }
    }
}

impl From<&str> for DeliveryStreamEncryptionStatus {
    fn from(name: &str) -> Self {
        match name {
            "DISABLED" => DeliveryStreamEncryptionStatus::Disabled,
            "DISABLING" => DeliveryStreamEncryptionStatus::Disabling,
            "DISABLING_FAILED" => DeliveryStreamEncryptionStatus::DisablingFailed,
            "ENABLED" => DeliveryStreamEncryptionStatus::Enabled,
            "ENABLING" => DeliveryStreamEncryptionStatus::Enabling,
            "ENABLING_FAILED" => DeliveryStreamEncryptionStatus::EnablingFailed,
            _ => DeliveryStreamEncryptionStatus::UnknownVariant(
                UnknownDeliveryStreamEncryptionStatus {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for DeliveryStreamEncryptionStatus {
    fn from(name: String) -> Self {
        match &*name {
            "DISABLED" => DeliveryStreamEncryptionStatus::Disabled,
            "DISABLING" => DeliveryStreamEncryptionStatus::Disabling,
            "DISABLING_FAILED" => DeliveryStreamEncryptionStatus::DisablingFailed,
            "ENABLED" => DeliveryStreamEncryptionStatus::Enabled,
            "ENABLING" => DeliveryStreamEncryptionStatus::Enabling,
            "ENABLING_FAILED" => DeliveryStreamEncryptionStatus::EnablingFailed,
            _ => DeliveryStreamEncryptionStatus::UnknownVariant(
                UnknownDeliveryStreamEncryptionStatus { name },
            ),
        }
    }
}

impl ::std::str::FromStr for DeliveryStreamEncryptionStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DeliveryStreamEncryptionStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeliveryStreamEncryptionStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeliveryStreamFailureType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeliveryStreamFailureType {
    CreateEniFailed,
    CreateKmsGrantFailed,
    DeleteEniFailed,
    DisabledKmsKey,
    EniAccessDenied,
    InvalidKmsKey,
    KmsAccessDenied,
    KmsKeyNotFound,
    KmsOptInRequired,
    RetireKmsGrantFailed,
    SecurityGroupAccessDenied,
    SecurityGroupNotFound,
    SubnetAccessDenied,
    SubnetNotFound,
    UnknownError,
    #[doc(hidden)]
    UnknownVariant(UnknownDeliveryStreamFailureType),
}

impl Default for DeliveryStreamFailureType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeliveryStreamFailureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeliveryStreamFailureType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeliveryStreamFailureType {
    fn into(self) -> String {
        match self {
            DeliveryStreamFailureType::CreateEniFailed => "CREATE_ENI_FAILED".to_string(),
            DeliveryStreamFailureType::CreateKmsGrantFailed => {
                "CREATE_KMS_GRANT_FAILED".to_string()
            }
            DeliveryStreamFailureType::DeleteEniFailed => "DELETE_ENI_FAILED".to_string(),
            DeliveryStreamFailureType::DisabledKmsKey => "DISABLED_KMS_KEY".to_string(),
            DeliveryStreamFailureType::EniAccessDenied => "ENI_ACCESS_DENIED".to_string(),
            DeliveryStreamFailureType::InvalidKmsKey => "INVALID_KMS_KEY".to_string(),
            DeliveryStreamFailureType::KmsAccessDenied => "KMS_ACCESS_DENIED".to_string(),
            DeliveryStreamFailureType::KmsKeyNotFound => "KMS_KEY_NOT_FOUND".to_string(),
            DeliveryStreamFailureType::KmsOptInRequired => "KMS_OPT_IN_REQUIRED".to_string(),
            DeliveryStreamFailureType::RetireKmsGrantFailed => {
                "RETIRE_KMS_GRANT_FAILED".to_string()
            }
            DeliveryStreamFailureType::SecurityGroupAccessDenied => {
                "SECURITY_GROUP_ACCESS_DENIED".to_string()
            }
            DeliveryStreamFailureType::SecurityGroupNotFound => {
                "SECURITY_GROUP_NOT_FOUND".to_string()
            }
            DeliveryStreamFailureType::SubnetAccessDenied => "SUBNET_ACCESS_DENIED".to_string(),
            DeliveryStreamFailureType::SubnetNotFound => "SUBNET_NOT_FOUND".to_string(),
            DeliveryStreamFailureType::UnknownError => "UNKNOWN_ERROR".to_string(),
            DeliveryStreamFailureType::UnknownVariant(UnknownDeliveryStreamFailureType {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DeliveryStreamFailureType {
    fn into(self) -> &'a str {
        match self {
            DeliveryStreamFailureType::CreateEniFailed => &"CREATE_ENI_FAILED",
            DeliveryStreamFailureType::CreateKmsGrantFailed => &"CREATE_KMS_GRANT_FAILED",
            DeliveryStreamFailureType::DeleteEniFailed => &"DELETE_ENI_FAILED",
            DeliveryStreamFailureType::DisabledKmsKey => &"DISABLED_KMS_KEY",
            DeliveryStreamFailureType::EniAccessDenied => &"ENI_ACCESS_DENIED",
            DeliveryStreamFailureType::InvalidKmsKey => &"INVALID_KMS_KEY",
            DeliveryStreamFailureType::KmsAccessDenied => &"KMS_ACCESS_DENIED",
            DeliveryStreamFailureType::KmsKeyNotFound => &"KMS_KEY_NOT_FOUND",
            DeliveryStreamFailureType::KmsOptInRequired => &"KMS_OPT_IN_REQUIRED",
            DeliveryStreamFailureType::RetireKmsGrantFailed => &"RETIRE_KMS_GRANT_FAILED",
            DeliveryStreamFailureType::SecurityGroupAccessDenied => &"SECURITY_GROUP_ACCESS_DENIED",
            DeliveryStreamFailureType::SecurityGroupNotFound => &"SECURITY_GROUP_NOT_FOUND",
            DeliveryStreamFailureType::SubnetAccessDenied => &"SUBNET_ACCESS_DENIED",
            DeliveryStreamFailureType::SubnetNotFound => &"SUBNET_NOT_FOUND",
            DeliveryStreamFailureType::UnknownError => &"UNKNOWN_ERROR",
            DeliveryStreamFailureType::UnknownVariant(UnknownDeliveryStreamFailureType {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DeliveryStreamFailureType {
    fn from(name: &str) -> Self {
        match name {
            "CREATE_ENI_FAILED" => DeliveryStreamFailureType::CreateEniFailed,
            "CREATE_KMS_GRANT_FAILED" => DeliveryStreamFailureType::CreateKmsGrantFailed,
            "DELETE_ENI_FAILED" => DeliveryStreamFailureType::DeleteEniFailed,
            "DISABLED_KMS_KEY" => DeliveryStreamFailureType::DisabledKmsKey,
            "ENI_ACCESS_DENIED" => DeliveryStreamFailureType::EniAccessDenied,
            "INVALID_KMS_KEY" => DeliveryStreamFailureType::InvalidKmsKey,
            "KMS_ACCESS_DENIED" => DeliveryStreamFailureType::KmsAccessDenied,
            "KMS_KEY_NOT_FOUND" => DeliveryStreamFailureType::KmsKeyNotFound,
            "KMS_OPT_IN_REQUIRED" => DeliveryStreamFailureType::KmsOptInRequired,
            "RETIRE_KMS_GRANT_FAILED" => DeliveryStreamFailureType::RetireKmsGrantFailed,
            "SECURITY_GROUP_ACCESS_DENIED" => DeliveryStreamFailureType::SecurityGroupAccessDenied,
            "SECURITY_GROUP_NOT_FOUND" => DeliveryStreamFailureType::SecurityGroupNotFound,
            "SUBNET_ACCESS_DENIED" => DeliveryStreamFailureType::SubnetAccessDenied,
            "SUBNET_NOT_FOUND" => DeliveryStreamFailureType::SubnetNotFound,
            "UNKNOWN_ERROR" => DeliveryStreamFailureType::UnknownError,
            _ => DeliveryStreamFailureType::UnknownVariant(UnknownDeliveryStreamFailureType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DeliveryStreamFailureType {
    fn from(name: String) -> Self {
        match &*name {
            "CREATE_ENI_FAILED" => DeliveryStreamFailureType::CreateEniFailed,
            "CREATE_KMS_GRANT_FAILED" => DeliveryStreamFailureType::CreateKmsGrantFailed,
            "DELETE_ENI_FAILED" => DeliveryStreamFailureType::DeleteEniFailed,
            "DISABLED_KMS_KEY" => DeliveryStreamFailureType::DisabledKmsKey,
            "ENI_ACCESS_DENIED" => DeliveryStreamFailureType::EniAccessDenied,
            "INVALID_KMS_KEY" => DeliveryStreamFailureType::InvalidKmsKey,
            "KMS_ACCESS_DENIED" => DeliveryStreamFailureType::KmsAccessDenied,
            "KMS_KEY_NOT_FOUND" => DeliveryStreamFailureType::KmsKeyNotFound,
            "KMS_OPT_IN_REQUIRED" => DeliveryStreamFailureType::KmsOptInRequired,
            "RETIRE_KMS_GRANT_FAILED" => DeliveryStreamFailureType::RetireKmsGrantFailed,
            "SECURITY_GROUP_ACCESS_DENIED" => DeliveryStreamFailureType::SecurityGroupAccessDenied,
            "SECURITY_GROUP_NOT_FOUND" => DeliveryStreamFailureType::SecurityGroupNotFound,
            "SUBNET_ACCESS_DENIED" => DeliveryStreamFailureType::SubnetAccessDenied,
            "SUBNET_NOT_FOUND" => DeliveryStreamFailureType::SubnetNotFound,
            "UNKNOWN_ERROR" => DeliveryStreamFailureType::UnknownError,
            _ => {
                DeliveryStreamFailureType::UnknownVariant(UnknownDeliveryStreamFailureType { name })
            }
        }
    }
}

impl ::std::str::FromStr for DeliveryStreamFailureType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DeliveryStreamFailureType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeliveryStreamFailureType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeliveryStreamStatus {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeliveryStreamStatus {
    Active,
    Creating,
    CreatingFailed,
    Deleting,
    DeletingFailed,
    #[doc(hidden)]
    UnknownVariant(UnknownDeliveryStreamStatus),
}

impl Default for DeliveryStreamStatus {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeliveryStreamStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeliveryStreamStatus {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeliveryStreamStatus {
    fn into(self) -> String {
        match self {
            DeliveryStreamStatus::Active => "ACTIVE".to_string(),
            DeliveryStreamStatus::Creating => "CREATING".to_string(),
            DeliveryStreamStatus::CreatingFailed => "CREATING_FAILED".to_string(),
            DeliveryStreamStatus::Deleting => "DELETING".to_string(),
            DeliveryStreamStatus::DeletingFailed => "DELETING_FAILED".to_string(),
            DeliveryStreamStatus::UnknownVariant(UnknownDeliveryStreamStatus {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a DeliveryStreamStatus {
    fn into(self) -> &'a str {
        match self {
            DeliveryStreamStatus::Active => &"ACTIVE",
            DeliveryStreamStatus::Creating => &"CREATING",
            DeliveryStreamStatus::CreatingFailed => &"CREATING_FAILED",
            DeliveryStreamStatus::Deleting => &"DELETING",
            DeliveryStreamStatus::DeletingFailed => &"DELETING_FAILED",
            DeliveryStreamStatus::UnknownVariant(UnknownDeliveryStreamStatus {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for DeliveryStreamStatus {
    fn from(name: &str) -> Self {
        match name {
            "ACTIVE" => DeliveryStreamStatus::Active,
            "CREATING" => DeliveryStreamStatus::Creating,
            "CREATING_FAILED" => DeliveryStreamStatus::CreatingFailed,
            "DELETING" => DeliveryStreamStatus::Deleting,
            "DELETING_FAILED" => DeliveryStreamStatus::DeletingFailed,
            _ => DeliveryStreamStatus::UnknownVariant(UnknownDeliveryStreamStatus {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DeliveryStreamStatus {
    fn from(name: String) -> Self {
        match &*name {
            "ACTIVE" => DeliveryStreamStatus::Active,
            "CREATING" => DeliveryStreamStatus::Creating,
            "CREATING_FAILED" => DeliveryStreamStatus::CreatingFailed,
            "DELETING" => DeliveryStreamStatus::Deleting,
            "DELETING_FAILED" => DeliveryStreamStatus::DeletingFailed,
            _ => DeliveryStreamStatus::UnknownVariant(UnknownDeliveryStreamStatus { name }),
        }
    }
}

impl ::std::str::FromStr for DeliveryStreamStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

#[cfg(any(test, feature = "serialize_structs"))]
impl Serialize for DeliveryStreamStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeliveryStreamStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownDeliveryStreamType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum DeliveryStreamType {
    DirectPut,
    KinesisStreamAsSource,
    #[doc(hidden)]
    UnknownVariant(UnknownDeliveryStreamType),
}

impl Default for DeliveryStreamType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for DeliveryStreamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for DeliveryStreamType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for DeliveryStreamType {
    fn into(self) -> String {
        match self {
            DeliveryStreamType::DirectPut => "DirectPut".to_string(),
            DeliveryStreamType::KinesisStreamAsSource => "KinesisStreamAsSource".to_string(),
            DeliveryStreamType::UnknownVariant(UnknownDeliveryStreamType { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a DeliveryStreamType {
    fn into(self) -> &'a str {
        match self {
            DeliveryStreamType::DirectPut => &"DirectPut",
            DeliveryStreamType::KinesisStreamAsSource => &"KinesisStreamAsSource",
            DeliveryStreamType::UnknownVariant(UnknownDeliveryStreamType { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for DeliveryStreamType {
    fn from(name: &str) -> Self {
        match name {
            "DirectPut" => DeliveryStreamType::DirectPut,
            "KinesisStreamAsSource" => DeliveryStreamType::KinesisStreamAsSource,
            _ => DeliveryStreamType::UnknownVariant(UnknownDeliveryStreamType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for DeliveryStreamType {
    fn from(name: String) -> Self {
        match &*name {
            "DirectPut" => DeliveryStreamType::DirectPut,
            "KinesisStreamAsSource" => DeliveryStreamType::KinesisStreamAsSource,
            _ => DeliveryStreamType::UnknownVariant(UnknownDeliveryStreamType { name }),
        }
    }
}

impl ::std::str::FromStr for DeliveryStreamType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for DeliveryStreamType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for DeliveryStreamType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDeliveryStreamOutput {
    /// <p>Information about the delivery stream.</p>
    #[serde(rename = "DeliveryStreamDescription")]
    pub delivery_stream_description: DeliveryStreamDescription,
}

/// <p>The deserializer you want Kinesis Data Firehose to use for converting the input data from JSON. Kinesis Data Firehose then serializes the data to its final format using the <a>Serializer</a>. Kinesis Data Firehose supports two types of deserializers: the <a href="https://cwiki.apache.org/confluence/display/Hive/LanguageManual+DDL#LanguageManualDDL-JSON">Apache Hive JSON SerDe</a> and the <a href="https://github.com/rcongiu/Hive-JSON-Serde">OpenX JSON SerDe</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Describes the specified HTTP endpoint destination.</p>
    #[serde(rename = "HttpEndpointDestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_description: Option<HttpEndpointDestinationDescription>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ElasticsearchDestinationConfiguration {
    /// <p>The buffering options. If no value is specified, the default values for <code>ElasticsearchBufferingHints</code> are used.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The Amazon CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The endpoint to use when communicating with the cluster. Specify either this <code>ClusterEndpoint</code> or the <code>DomainARN</code> field.</p>
    #[serde(rename = "ClusterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    /// <p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the role specified in <b>RoleARN</b>. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> <p>Specify either <code>ClusterEndpoint</code> or <code>DomainARN</code>.</p>
    #[serde(rename = "DomainARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The Elasticsearch index name.</p>
    #[serde(rename = "IndexName")]
    pub index_name: String,
    /// <p>The Elasticsearch index rotation period. Index rotation appends a timestamp to the <code>IndexName</code> to facilitate the expiration of old data. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation">Index Rotation for the Amazon ES Destination</a>. The default value is <code>OneDay</code>.</p>
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<ElasticsearchIndexRotationPeriod>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES. The default value is 300 (5 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>Defines how documents should be delivered to Amazon S3. When it is set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any documents that could not be indexed to the configured Amazon S3 destination, with <code>elasticsearch-failed/</code> appended to the key prefix. When set to <code>AllDocuments</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents with <code>elasticsearch-failed/</code> appended to the prefix. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-s3-backup">Amazon S3 Backup for the Amazon ES Destination</a>. Default value is <code>FailedDocumentsOnly</code>.</p> <p>You can't change this backup mode after you create the delivery stream. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<ElasticsearchS3BackupMode>,
    /// <p>The configuration for the backup Amazon S3 location.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    /// <p>The Elasticsearch type name. For Elasticsearch 6.x, there can be only one type per index. If you try to specify a new type for an existing index that already has another type, Kinesis Data Firehose returns an error during run time.</p> <p>For Elasticsearch 7.x, don't specify a <code>TypeName</code>.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// <p>The details of the VPC of the Amazon ES destination.</p>
    #[serde(rename = "VpcConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<VpcConfiguration>,
}

/// <p>The destination description in Amazon ES.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ElasticsearchDestinationDescription {
    /// <p>The buffering options.</p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The Amazon CloudWatch logging options.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The endpoint to use when communicating with the cluster. Kinesis Data Firehose uses either this <code>ClusterEndpoint</code> or the <code>DomainARN</code> field to send data to Amazon ES.</p>
    #[serde(rename = "ClusterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    /// <p>The ARN of the Amazon ES domain. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> <p>Kinesis Data Firehose uses either <code>ClusterEndpoint</code> or <code>DomainARN</code> to send data to Amazon ES.</p>
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
    pub index_rotation_period: Option<ElasticsearchIndexRotationPeriod>,
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
    pub s3_backup_mode: Option<ElasticsearchS3BackupMode>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
    /// <p>The Elasticsearch type name. This applies to Elasticsearch 6.x and lower versions. For Elasticsearch 7.x, there's no value for <code>TypeName</code>.</p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    /// <p>The details of the VPC of the Amazon ES destination.</p>
    #[serde(rename = "VpcConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration_description: Option<VpcConfigurationDescription>,
}

/// <p>Describes an update for a destination in Amazon ES.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ElasticsearchDestinationUpdate {
    /// <p>The buffering options. If no value is specified, <code>ElasticsearchBufferingHints</code> object default values are used. </p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<ElasticsearchBufferingHints>,
    /// <p>The CloudWatch logging options for your delivery stream.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The endpoint to use when communicating with the cluster. Specify either this <code>ClusterEndpoint</code> or the <code>DomainARN</code> field.</p>
    #[serde(rename = "ClusterEndpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_endpoint: Option<String>,
    /// <p>The ARN of the Amazon ES domain. The IAM role must have permissions for <code>DescribeElasticsearchDomain</code>, <code>DescribeElasticsearchDomains</code>, and <code>DescribeElasticsearchDomainConfig</code> after assuming the IAM role specified in <code>RoleARN</code>. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p> <p>Specify either <code>ClusterEndpoint</code> or <code>DomainARN</code>.</p>
    #[serde(rename = "DomainARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    /// <p>The Elasticsearch index name.</p>
    #[serde(rename = "IndexName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    /// <p>The Elasticsearch index rotation period. Index rotation appends a timestamp to <code>IndexName</code> to facilitate the expiration of old data. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/basic-deliver.html#es-index-rotation">Index Rotation for the Amazon ES Destination</a>. Default value is <code>OneDay</code>.</p>
    #[serde(rename = "IndexRotationPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_rotation_period: Option<ElasticsearchIndexRotationPeriod>,
    /// <p>The data processing configuration.</p>
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES. The default value is 300 (5 minutes).</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<ElasticsearchRetryOptions>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role to be assumed by Kinesis Data Firehose for calling the Amazon ES Configuration API and for indexing documents. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> and <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
    /// <p>The Elasticsearch type name. For Elasticsearch 6.x, there can be only one type per index. If you try to specify a new type for an existing index that already has another type, Kinesis Data Firehose returns an error during runtime.</p> <p>If you upgrade Elasticsearch from 6.x to 7.x and don’t update your delivery stream, Kinesis Data Firehose still delivers data to Elasticsearch with the old index name and type name. If you want to update your delivery stream with a new index name, provide an empty string for <code>TypeName</code>. </p>
    #[serde(rename = "TypeName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownElasticsearchIndexRotationPeriod {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ElasticsearchIndexRotationPeriod {
    NoRotation,
    OneDay,
    OneHour,
    OneMonth,
    OneWeek,
    #[doc(hidden)]
    UnknownVariant(UnknownElasticsearchIndexRotationPeriod),
}

impl Default for ElasticsearchIndexRotationPeriod {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ElasticsearchIndexRotationPeriod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ElasticsearchIndexRotationPeriod {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ElasticsearchIndexRotationPeriod {
    fn into(self) -> String {
        match self {
            ElasticsearchIndexRotationPeriod::NoRotation => "NoRotation".to_string(),
            ElasticsearchIndexRotationPeriod::OneDay => "OneDay".to_string(),
            ElasticsearchIndexRotationPeriod::OneHour => "OneHour".to_string(),
            ElasticsearchIndexRotationPeriod::OneMonth => "OneMonth".to_string(),
            ElasticsearchIndexRotationPeriod::OneWeek => "OneWeek".to_string(),
            ElasticsearchIndexRotationPeriod::UnknownVariant(
                UnknownElasticsearchIndexRotationPeriod { name: original },
            ) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ElasticsearchIndexRotationPeriod {
    fn into(self) -> &'a str {
        match self {
            ElasticsearchIndexRotationPeriod::NoRotation => &"NoRotation",
            ElasticsearchIndexRotationPeriod::OneDay => &"OneDay",
            ElasticsearchIndexRotationPeriod::OneHour => &"OneHour",
            ElasticsearchIndexRotationPeriod::OneMonth => &"OneMonth",
            ElasticsearchIndexRotationPeriod::OneWeek => &"OneWeek",
            ElasticsearchIndexRotationPeriod::UnknownVariant(
                UnknownElasticsearchIndexRotationPeriod { name: original },
            ) => original,
        }
    }
}

impl From<&str> for ElasticsearchIndexRotationPeriod {
    fn from(name: &str) -> Self {
        match name {
            "NoRotation" => ElasticsearchIndexRotationPeriod::NoRotation,
            "OneDay" => ElasticsearchIndexRotationPeriod::OneDay,
            "OneHour" => ElasticsearchIndexRotationPeriod::OneHour,
            "OneMonth" => ElasticsearchIndexRotationPeriod::OneMonth,
            "OneWeek" => ElasticsearchIndexRotationPeriod::OneWeek,
            _ => ElasticsearchIndexRotationPeriod::UnknownVariant(
                UnknownElasticsearchIndexRotationPeriod {
                    name: name.to_owned(),
                },
            ),
        }
    }
}

impl From<String> for ElasticsearchIndexRotationPeriod {
    fn from(name: String) -> Self {
        match &*name {
            "NoRotation" => ElasticsearchIndexRotationPeriod::NoRotation,
            "OneDay" => ElasticsearchIndexRotationPeriod::OneDay,
            "OneHour" => ElasticsearchIndexRotationPeriod::OneHour,
            "OneMonth" => ElasticsearchIndexRotationPeriod::OneMonth,
            "OneWeek" => ElasticsearchIndexRotationPeriod::OneWeek,
            _ => ElasticsearchIndexRotationPeriod::UnknownVariant(
                UnknownElasticsearchIndexRotationPeriod { name },
            ),
        }
    }
}

impl ::std::str::FromStr for ElasticsearchIndexRotationPeriod {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ElasticsearchIndexRotationPeriod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ElasticsearchIndexRotationPeriod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to Amazon ES.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ElasticsearchRetryOptions {
    /// <p>After an initial failure to deliver to Amazon ES, the total amount of time during which Kinesis Data Firehose retries delivery (including the first attempt). After this time has elapsed, the failed documents are written to Amazon S3. Default value is 300 seconds (5 minutes). A value of 0 (zero) results in no retries.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownElasticsearchS3BackupMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ElasticsearchS3BackupMode {
    AllDocuments,
    FailedDocumentsOnly,
    #[doc(hidden)]
    UnknownVariant(UnknownElasticsearchS3BackupMode),
}

impl Default for ElasticsearchS3BackupMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ElasticsearchS3BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ElasticsearchS3BackupMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ElasticsearchS3BackupMode {
    fn into(self) -> String {
        match self {
            ElasticsearchS3BackupMode::AllDocuments => "AllDocuments".to_string(),
            ElasticsearchS3BackupMode::FailedDocumentsOnly => "FailedDocumentsOnly".to_string(),
            ElasticsearchS3BackupMode::UnknownVariant(UnknownElasticsearchS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ElasticsearchS3BackupMode {
    fn into(self) -> &'a str {
        match self {
            ElasticsearchS3BackupMode::AllDocuments => &"AllDocuments",
            ElasticsearchS3BackupMode::FailedDocumentsOnly => &"FailedDocumentsOnly",
            ElasticsearchS3BackupMode::UnknownVariant(UnknownElasticsearchS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ElasticsearchS3BackupMode {
    fn from(name: &str) -> Self {
        match name {
            "AllDocuments" => ElasticsearchS3BackupMode::AllDocuments,
            "FailedDocumentsOnly" => ElasticsearchS3BackupMode::FailedDocumentsOnly,
            _ => ElasticsearchS3BackupMode::UnknownVariant(UnknownElasticsearchS3BackupMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ElasticsearchS3BackupMode {
    fn from(name: String) -> Self {
        match &*name {
            "AllDocuments" => ElasticsearchS3BackupMode::AllDocuments,
            "FailedDocumentsOnly" => ElasticsearchS3BackupMode::FailedDocumentsOnly,
            _ => {
                ElasticsearchS3BackupMode::UnknownVariant(UnknownElasticsearchS3BackupMode { name })
            }
        }
    }
}

impl ::std::str::FromStr for ElasticsearchS3BackupMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ElasticsearchS3BackupMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ElasticsearchS3BackupMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the encryption for a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EncryptionConfiguration {
    /// <p>The encryption key.</p>
    #[serde(rename = "KMSEncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_encryption_config: Option<KMSEncryptionConfig>,
    /// <p>Specifically override existing encryption information to ensure that no encryption is used.</p>
    #[serde(rename = "NoEncryptionConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_encryption_config: Option<NoEncryptionConfig>,
}

/// <p>Describes the configuration of a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub compression_format: Option<CompressionFormat>,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
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
    /// <p>The Amazon S3 backup mode. After you create a delivery stream, you can update it to enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the delivery stream to disable it. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<S3BackupMode>,
}

/// <p>Describes a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    pub compression_format: CompressionFormat,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
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
    pub s3_backup_mode: Option<S3BackupMode>,
}

/// <p>Describes an update for a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub compression_format: Option<CompressionFormat>,
    /// <p>The serializer, deserializer, and schema for converting data from the JSON format to the Parquet or ORC format before writing it to Amazon S3.</p>
    #[serde(rename = "DataFormatConversionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format_conversion_configuration: Option<DataFormatConversionConfiguration>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
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
    /// <p>You can update a delivery stream to enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the delivery stream to disable it. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<S3BackupMode>,
    /// <p>The Amazon S3 destination for backup.</p>
    #[serde(rename = "S3BackupUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_update: Option<S3DestinationUpdate>,
}

/// <p>Provides details in case one of the following operations fails due to an error related to KMS: <a>CreateDeliveryStream</a>, <a>DeleteDeliveryStream</a>, <a>StartDeliveryStreamEncryption</a>, <a>StopDeliveryStreamEncryption</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FailureDescription {
    /// <p>A message providing details about the error that caused the failure.</p>
    #[serde(rename = "Details")]
    pub details: String,
    /// <p>The type of error that caused the failure.</p>
    #[serde(rename = "Type")]
    pub type_: DeliveryStreamFailureType,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownHECEndpointType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum HECEndpointType {
    Event,
    Raw,
    #[doc(hidden)]
    UnknownVariant(UnknownHECEndpointType),
}

impl Default for HECEndpointType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for HECEndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for HECEndpointType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for HECEndpointType {
    fn into(self) -> String {
        match self {
            HECEndpointType::Event => "Event".to_string(),
            HECEndpointType::Raw => "Raw".to_string(),
            HECEndpointType::UnknownVariant(UnknownHECEndpointType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a HECEndpointType {
    fn into(self) -> &'a str {
        match self {
            HECEndpointType::Event => &"Event",
            HECEndpointType::Raw => &"Raw",
            HECEndpointType::UnknownVariant(UnknownHECEndpointType { name: original }) => original,
        }
    }
}

impl From<&str> for HECEndpointType {
    fn from(name: &str) -> Self {
        match name {
            "Event" => HECEndpointType::Event,
            "Raw" => HECEndpointType::Raw,
            _ => HECEndpointType::UnknownVariant(UnknownHECEndpointType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for HECEndpointType {
    fn from(name: String) -> Self {
        match &*name {
            "Event" => HECEndpointType::Event,
            "Raw" => HECEndpointType::Raw,
            _ => HECEndpointType::UnknownVariant(UnknownHECEndpointType { name }),
        }
    }
}

impl ::std::str::FromStr for HECEndpointType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for HECEndpointType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for HECEndpointType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The native Hive / HCatalog JsonSerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the OpenX SerDe.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HiveJsonSerDe {
    /// <p>Indicates how you want Kinesis Data Firehose to parse the date and timestamps that may be present in your input data JSON. To specify these format strings, follow the pattern syntax of JodaTime's DateTimeFormat format strings. For more information, see <a href="https://www.joda.org/joda-time/apidocs/org/joda/time/format/DateTimeFormat.html">Class DateTimeFormat</a>. You can also use the special value <code>millis</code> to parse timestamps in epoch milliseconds. If you don't specify a format, Kinesis Data Firehose uses <code>java.sql.Timestamp::valueOf</code> by default.</p>
    #[serde(rename = "TimestampFormats")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_formats: Option<Vec<String>>,
}

/// <p>Describes the buffering options that can be applied before data is delivered to the HTTP endpoint destination. Kinesis Data Firehose treats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpEndpointBufferingHints {
    /// <p>Buffer incoming data for the specified period of time, in seconds, before delivering it to the destination. The default value is 300 (5 minutes). </p>
    #[serde(rename = "IntervalInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_in_seconds: Option<i64>,
    /// <p>Buffer incoming data to the specified size, in MBs, before delivering it to the destination. The default value is 5. </p> <p>We recommend setting this parameter to a value greater than the amount of data you typically ingest into the delivery stream in 10 seconds. For example, if you typically ingest data at 1 MB/sec, the value should be 10 MB or higher. </p>
    #[serde(rename = "SizeInMBs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_m_bs: Option<i64>,
}

/// <p>Describes the metadata that's delivered to the specified HTTP endpoint destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpEndpointCommonAttribute {
    /// <p>The name of the HTTP endpoint common attribute.</p>
    #[serde(rename = "AttributeName")]
    pub attribute_name: String,
    /// <p>The value of the HTTP endpoint common attribute.</p>
    #[serde(rename = "AttributeValue")]
    pub attribute_value: String,
}

/// <p>Describes the configuration of the HTTP endpoint to which Kinesis Firehose delivers data.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HttpEndpointConfiguration {
    /// <p>The access key required for Kinesis Firehose to authenticate with the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "AccessKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_key: Option<String>,
    /// <p>The name of the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL of the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "Url")]
    pub url: String,
}

/// <p>Describes the HTTP endpoint selected as the destination. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HttpEndpointDescription {
    /// <p>The name of the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The URL of the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "Url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// <p>Describes the configuration of the HTTP endpoint destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HttpEndpointDestinationConfiguration {
    /// <p>The buffering options that can be used before data is delivered to the specified destination. Kinesis Data Firehose treats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if you specify a value for one of them, you must also provide a value for the other. </p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The configuration of the HTTP endpoint selected as the destination.</p>
    #[serde(rename = "EndpointConfiguration")]
    pub endpoint_configuration: HttpEndpointConfiguration,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The configuration of the requeste sent to the HTTP endpoint specified as the destination.</p>
    #[serde(rename = "RequestConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    /// <p>Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    /// <p>Kinesis Data Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Data Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Kinesis Data Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<HttpEndpointS3BackupMode>,
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
}

/// <p>Describes the HTTP endpoint destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct HttpEndpointDestinationDescription {
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Kinesis Data Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other. </p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>The configuration of the specified HTTP endpoint destination.</p>
    #[serde(rename = "EndpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<HttpEndpointDescription>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The configuration of request sent to the HTTP endpoint specified as the destination.</p>
    #[serde(rename = "RequestConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    /// <p>Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    /// <p>Kinesis Data Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Kinesis Data Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<HttpEndpointS3BackupMode>,
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
}

/// <p>Updates the specified HTTP endpoint destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct HttpEndpointDestinationUpdate {
    /// <p>Describes buffering options that can be applied to the data before it is delivered to the HTTPS endpoint destination. Kinesis Data Firehose teats these options as hints, and it might choose to use more optimal values. The <code>SizeInMBs</code> and <code>IntervalInSeconds</code> parameters are optional. However, if specify a value for one of them, you must also provide a value for the other. </p>
    #[serde(rename = "BufferingHints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffering_hints: Option<HttpEndpointBufferingHints>,
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<CloudWatchLoggingOptions>,
    /// <p>Describes the configuration of the HTTP endpoint destination.</p>
    #[serde(rename = "EndpointConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_configuration: Option<HttpEndpointConfiguration>,
    #[serde(rename = "ProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    /// <p>The configuration of the request sent to the HTTP endpoint specified as the destination.</p>
    #[serde(rename = "RequestConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_configuration: Option<HttpEndpointRequestConfiguration>,
    /// <p>Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
    #[serde(rename = "RetryOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_options: Option<HttpEndpointRetryOptions>,
    /// <p>Kinesis Data Firehose uses this IAM role for all the permissions that the delivery stream needs.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Describes the S3 bucket backup options for the data that Kinesis Firehose delivers to the HTTP endpoint destination. You can back up all documents (<code>AllData</code>) or only the documents that Kinesis Data Firehose could not deliver to the specified HTTP endpoint destination (<code>FailedDataOnly</code>).</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<HttpEndpointS3BackupMode>,
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
}

/// <p>The configuration of the HTTP endpoint request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpEndpointRequestConfiguration {
    /// <p>Describes the metadata sent to the HTTP endpoint destination.</p>
    #[serde(rename = "CommonAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_attributes: Option<Vec<HttpEndpointCommonAttribute>>,
    /// <p>Kinesis Data Firehose uses the content encoding to compress the body of a request before sending the request to the destination. For more information, see <a href="https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Encoding">Content-Encoding</a> in MDN Web Docs, the official Mozilla documentation.</p>
    #[serde(rename = "ContentEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_encoding: Option<ContentEncoding>,
}

/// <p>Describes the retry behavior in case Kinesis Data Firehose is unable to deliver data to the specified HTTP endpoint destination, or if it doesn't receive a valid acknowledgment of receipt from the specified HTTP endpoint destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct HttpEndpointRetryOptions {
    /// <p>The total amount of time that Kinesis Data Firehose spends on retries. This duration starts after the initial attempt to send data to the custom destination via HTTPS endpoint fails. It doesn't include the periods during which Kinesis Data Firehose waits for acknowledgment from the specified destination after each attempt. </p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownHttpEndpointS3BackupMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum HttpEndpointS3BackupMode {
    AllData,
    FailedDataOnly,
    #[doc(hidden)]
    UnknownVariant(UnknownHttpEndpointS3BackupMode),
}

impl Default for HttpEndpointS3BackupMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for HttpEndpointS3BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for HttpEndpointS3BackupMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for HttpEndpointS3BackupMode {
    fn into(self) -> String {
        match self {
            HttpEndpointS3BackupMode::AllData => "AllData".to_string(),
            HttpEndpointS3BackupMode::FailedDataOnly => "FailedDataOnly".to_string(),
            HttpEndpointS3BackupMode::UnknownVariant(UnknownHttpEndpointS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a HttpEndpointS3BackupMode {
    fn into(self) -> &'a str {
        match self {
            HttpEndpointS3BackupMode::AllData => &"AllData",
            HttpEndpointS3BackupMode::FailedDataOnly => &"FailedDataOnly",
            HttpEndpointS3BackupMode::UnknownVariant(UnknownHttpEndpointS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for HttpEndpointS3BackupMode {
    fn from(name: &str) -> Self {
        match name {
            "AllData" => HttpEndpointS3BackupMode::AllData,
            "FailedDataOnly" => HttpEndpointS3BackupMode::FailedDataOnly,
            _ => HttpEndpointS3BackupMode::UnknownVariant(UnknownHttpEndpointS3BackupMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for HttpEndpointS3BackupMode {
    fn from(name: String) -> Self {
        match &*name {
            "AllData" => HttpEndpointS3BackupMode::AllData,
            "FailedDataOnly" => HttpEndpointS3BackupMode::FailedDataOnly,
            _ => HttpEndpointS3BackupMode::UnknownVariant(UnknownHttpEndpointS3BackupMode { name }),
        }
    }
}

impl ::std::str::FromStr for HttpEndpointS3BackupMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for HttpEndpointS3BackupMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for HttpEndpointS3BackupMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Specifies the deserializer you want to use to convert the format of the input data. This parameter is required if <code>Enabled</code> is set to true.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InputFormatConfiguration {
    /// <p>Specifies which deserializer to use. You can choose either the Apache Hive JSON SerDe or the OpenX JSON SerDe. If both are non-null, the server rejects the request.</p>
    #[serde(rename = "Deserializer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deserializer: Option<Deserializer>,
}

/// <p>Describes an encryption key for a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct KMSEncryptionConfig {
    /// <p>The Amazon Resource Name (ARN) of the encryption key. Must belong to the same AWS Region as the destination Amazon S3 bucket. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "AWSKMSKeyARN")]
    pub awskms_key_arn: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownKeyType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum KeyType {
    AwsOwnedCmk,
    CustomerManagedCmk,
    #[doc(hidden)]
    UnknownVariant(UnknownKeyType),
}

impl Default for KeyType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for KeyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for KeyType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for KeyType {
    fn into(self) -> String {
        match self {
            KeyType::AwsOwnedCmk => "AWS_OWNED_CMK".to_string(),
            KeyType::CustomerManagedCmk => "CUSTOMER_MANAGED_CMK".to_string(),
            KeyType::UnknownVariant(UnknownKeyType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a KeyType {
    fn into(self) -> &'a str {
        match self {
            KeyType::AwsOwnedCmk => &"AWS_OWNED_CMK",
            KeyType::CustomerManagedCmk => &"CUSTOMER_MANAGED_CMK",
            KeyType::UnknownVariant(UnknownKeyType { name: original }) => original,
        }
    }
}

impl From<&str> for KeyType {
    fn from(name: &str) -> Self {
        match name {
            "AWS_OWNED_CMK" => KeyType::AwsOwnedCmk,
            "CUSTOMER_MANAGED_CMK" => KeyType::CustomerManagedCmk,
            _ => KeyType::UnknownVariant(UnknownKeyType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for KeyType {
    fn from(name: String) -> Self {
        match &*name {
            "AWS_OWNED_CMK" => KeyType::AwsOwnedCmk,
            "CUSTOMER_MANAGED_CMK" => KeyType::CustomerManagedCmk,
            _ => KeyType::UnknownVariant(UnknownKeyType { name }),
        }
    }
}

impl ::std::str::FromStr for KeyType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for KeyType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for KeyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The stream and role Amazon Resource Names (ARNs) for a Kinesis data stream used as the source for a delivery stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct KinesisStreamSourceConfiguration {
    /// <p>The ARN of the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-kinesis-streams">Amazon Kinesis Data Streams ARN Format</a>.</p>
    #[serde(rename = "KinesisStreamARN")]
    pub kinesis_stream_arn: String,
    /// <p>The ARN of the role that provides access to the source Kinesis data stream. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-iam">AWS Identity and Access Management (IAM) ARN Format</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Details about a Kinesis data stream used as the source for a Kinesis Data Firehose delivery stream.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct KinesisStreamSourceDescription {
    /// <p>Kinesis Data Firehose starts retrieving records from the Kinesis data stream starting with this timestamp.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListDeliveryStreamsInput {
    /// <p>The delivery stream type. This can be one of the following values:</p> <ul> <li> <p> <code>DirectPut</code>: Provider applications access the delivery stream directly.</p> </li> <li> <p> <code>KinesisStreamAsSource</code>: The delivery stream uses a Kinesis data stream as a source.</p> </li> </ul> <p>This parameter is optional. If this parameter is omitted, delivery streams of all types are returned.</p>
    #[serde(rename = "DeliveryStreamType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_type: Option<DeliveryStreamType>,
    /// <p>The list of delivery streams returned by this call to <code>ListDeliveryStreams</code> will start with the delivery stream whose name comes alphabetically immediately after the name you specify in <code>ExclusiveStartDeliveryStreamName</code>.</p>
    #[serde(rename = "ExclusiveStartDeliveryStreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_delivery_stream_name: Option<String>,
    /// <p>The maximum number of delivery streams to list. The default value is 10.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListDeliveryStreamsOutput {
    /// <p>The names of the delivery streams.</p>
    #[serde(rename = "DeliveryStreamNames")]
    pub delivery_stream_names: Vec<String>,
    /// <p>Indicates whether there are more delivery streams available to list.</p>
    #[serde(rename = "HasMoreDeliveryStreams")]
    pub has_more_delivery_streams: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForDeliveryStreamOutput {
    /// <p>If this is <code>true</code> in the response, more tags are available. To list the remaining tags, set <code>ExclusiveStartTagKey</code> to the key of the last tag returned and call <code>ListTagsForDeliveryStream</code> again.</p>
    #[serde(rename = "HasMoreTags")]
    pub has_more_tags: bool,
    /// <p>A list of tags associated with <code>DeliveryStreamName</code>, starting with the first tag after <code>ExclusiveStartTagKey</code> and up to the specified <code>Limit</code>.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownNoEncryptionConfig {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum NoEncryptionConfig {
    NoEncryption,
    #[doc(hidden)]
    UnknownVariant(UnknownNoEncryptionConfig),
}

impl Default for NoEncryptionConfig {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for NoEncryptionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for NoEncryptionConfig {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for NoEncryptionConfig {
    fn into(self) -> String {
        match self {
            NoEncryptionConfig::NoEncryption => "NoEncryption".to_string(),
            NoEncryptionConfig::UnknownVariant(UnknownNoEncryptionConfig { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a NoEncryptionConfig {
    fn into(self) -> &'a str {
        match self {
            NoEncryptionConfig::NoEncryption => &"NoEncryption",
            NoEncryptionConfig::UnknownVariant(UnknownNoEncryptionConfig { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for NoEncryptionConfig {
    fn from(name: &str) -> Self {
        match name {
            "NoEncryption" => NoEncryptionConfig::NoEncryption,
            _ => NoEncryptionConfig::UnknownVariant(UnknownNoEncryptionConfig {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for NoEncryptionConfig {
    fn from(name: String) -> Self {
        match &*name {
            "NoEncryption" => NoEncryptionConfig::NoEncryption,
            _ => NoEncryptionConfig::UnknownVariant(UnknownNoEncryptionConfig { name }),
        }
    }
}

impl ::std::str::FromStr for NoEncryptionConfig {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for NoEncryptionConfig {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for NoEncryptionConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>The OpenX SerDe. Used by Kinesis Data Firehose for deserializing data, which means converting it from the JSON format in preparation for serializing it to the Parquet or ORC format. This is one of two deserializers you can choose, depending on which one offers the functionality you need. The other option is the native Hive / HCatalog JsonSerDe.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrcCompression {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OrcCompression {
    None,
    Snappy,
    Zlib,
    #[doc(hidden)]
    UnknownVariant(UnknownOrcCompression),
}

impl Default for OrcCompression {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OrcCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OrcCompression {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OrcCompression {
    fn into(self) -> String {
        match self {
            OrcCompression::None => "NONE".to_string(),
            OrcCompression::Snappy => "SNAPPY".to_string(),
            OrcCompression::Zlib => "ZLIB".to_string(),
            OrcCompression::UnknownVariant(UnknownOrcCompression { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a OrcCompression {
    fn into(self) -> &'a str {
        match self {
            OrcCompression::None => &"NONE",
            OrcCompression::Snappy => &"SNAPPY",
            OrcCompression::Zlib => &"ZLIB",
            OrcCompression::UnknownVariant(UnknownOrcCompression { name: original }) => original,
        }
    }
}

impl From<&str> for OrcCompression {
    fn from(name: &str) -> Self {
        match name {
            "NONE" => OrcCompression::None,
            "SNAPPY" => OrcCompression::Snappy,
            "ZLIB" => OrcCompression::Zlib,
            _ => OrcCompression::UnknownVariant(UnknownOrcCompression {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OrcCompression {
    fn from(name: String) -> Self {
        match &*name {
            "NONE" => OrcCompression::None,
            "SNAPPY" => OrcCompression::Snappy,
            "ZLIB" => OrcCompression::Zlib,
            _ => OrcCompression::UnknownVariant(UnknownOrcCompression { name }),
        }
    }
}

impl ::std::str::FromStr for OrcCompression {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for OrcCompression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OrcCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownOrcFormatVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum OrcFormatVersion {
    V011,
    V012,
    #[doc(hidden)]
    UnknownVariant(UnknownOrcFormatVersion),
}

impl Default for OrcFormatVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for OrcFormatVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for OrcFormatVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for OrcFormatVersion {
    fn into(self) -> String {
        match self {
            OrcFormatVersion::V011 => "V0_11".to_string(),
            OrcFormatVersion::V012 => "V0_12".to_string(),
            OrcFormatVersion::UnknownVariant(UnknownOrcFormatVersion { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a OrcFormatVersion {
    fn into(self) -> &'a str {
        match self {
            OrcFormatVersion::V011 => &"V0_11",
            OrcFormatVersion::V012 => &"V0_12",
            OrcFormatVersion::UnknownVariant(UnknownOrcFormatVersion { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for OrcFormatVersion {
    fn from(name: &str) -> Self {
        match name {
            "V0_11" => OrcFormatVersion::V011,
            "V0_12" => OrcFormatVersion::V012,
            _ => OrcFormatVersion::UnknownVariant(UnknownOrcFormatVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for OrcFormatVersion {
    fn from(name: String) -> Self {
        match &*name {
            "V0_11" => OrcFormatVersion::V011,
            "V0_12" => OrcFormatVersion::V012,
            _ => OrcFormatVersion::UnknownVariant(UnknownOrcFormatVersion { name }),
        }
    }
}

impl ::std::str::FromStr for OrcFormatVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for OrcFormatVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for OrcFormatVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A serializer to use for converting data to the ORC format before storing it in Amazon S3. For more information, see <a href="https://orc.apache.org/docs/">Apache ORC</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
    pub compression: Option<OrcCompression>,
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
    pub format_version: Option<OrcFormatVersion>,
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

/// <p>Specifies the serializer that you want Kinesis Data Firehose to use to convert the format of your data before it writes it to Amazon S3. This parameter is required if <code>Enabled</code> is set to true.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OutputFormatConfiguration {
    /// <p>Specifies which serializer to use. You can choose either the ORC SerDe or the Parquet SerDe. If both are non-null, the server rejects the request.</p>
    #[serde(rename = "Serializer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializer: Option<Serializer>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownParquetCompression {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ParquetCompression {
    Gzip,
    Snappy,
    Uncompressed,
    #[doc(hidden)]
    UnknownVariant(UnknownParquetCompression),
}

impl Default for ParquetCompression {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ParquetCompression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ParquetCompression {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ParquetCompression {
    fn into(self) -> String {
        match self {
            ParquetCompression::Gzip => "GZIP".to_string(),
            ParquetCompression::Snappy => "SNAPPY".to_string(),
            ParquetCompression::Uncompressed => "UNCOMPRESSED".to_string(),
            ParquetCompression::UnknownVariant(UnknownParquetCompression { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a ParquetCompression {
    fn into(self) -> &'a str {
        match self {
            ParquetCompression::Gzip => &"GZIP",
            ParquetCompression::Snappy => &"SNAPPY",
            ParquetCompression::Uncompressed => &"UNCOMPRESSED",
            ParquetCompression::UnknownVariant(UnknownParquetCompression { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for ParquetCompression {
    fn from(name: &str) -> Self {
        match name {
            "GZIP" => ParquetCompression::Gzip,
            "SNAPPY" => ParquetCompression::Snappy,
            "UNCOMPRESSED" => ParquetCompression::Uncompressed,
            _ => ParquetCompression::UnknownVariant(UnknownParquetCompression {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ParquetCompression {
    fn from(name: String) -> Self {
        match &*name {
            "GZIP" => ParquetCompression::Gzip,
            "SNAPPY" => ParquetCompression::Snappy,
            "UNCOMPRESSED" => ParquetCompression::Uncompressed,
            _ => ParquetCompression::UnknownVariant(UnknownParquetCompression { name }),
        }
    }
}

impl ::std::str::FromStr for ParquetCompression {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ParquetCompression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ParquetCompression {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>A serializer to use for converting data to the Parquet format before storing it in Amazon S3. For more information, see <a href="https://parquet.apache.org/documentation/latest/">Apache Parquet</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ParquetSerDe {
    /// <p>The Hadoop Distributed File System (HDFS) block size. This is useful if you intend to copy the data from Amazon S3 to HDFS before querying. The default is 256 MiB and the minimum is 64 MiB. Kinesis Data Firehose uses this value for padding calculations.</p>
    #[serde(rename = "BlockSizeBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size_bytes: Option<i64>,
    /// <p>The compression code to use over data blocks. The possible values are <code>UNCOMPRESSED</code>, <code>SNAPPY</code>, and <code>GZIP</code>, with the default being <code>SNAPPY</code>. Use <code>SNAPPY</code> for higher decompression speed. Use <code>GZIP</code> if the compression ratio is more important than speed.</p>
    #[serde(rename = "Compression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compression: Option<ParquetCompression>,
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
    pub writer_version: Option<ParquetWriterVersion>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownParquetWriterVersion {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ParquetWriterVersion {
    V1,
    V2,
    #[doc(hidden)]
    UnknownVariant(UnknownParquetWriterVersion),
}

impl Default for ParquetWriterVersion {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ParquetWriterVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ParquetWriterVersion {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ParquetWriterVersion {
    fn into(self) -> String {
        match self {
            ParquetWriterVersion::V1 => "V1".to_string(),
            ParquetWriterVersion::V2 => "V2".to_string(),
            ParquetWriterVersion::UnknownVariant(UnknownParquetWriterVersion {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ParquetWriterVersion {
    fn into(self) -> &'a str {
        match self {
            ParquetWriterVersion::V1 => &"V1",
            ParquetWriterVersion::V2 => &"V2",
            ParquetWriterVersion::UnknownVariant(UnknownParquetWriterVersion {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ParquetWriterVersion {
    fn from(name: &str) -> Self {
        match name {
            "V1" => ParquetWriterVersion::V1,
            "V2" => ParquetWriterVersion::V2,
            _ => ParquetWriterVersion::UnknownVariant(UnknownParquetWriterVersion {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ParquetWriterVersion {
    fn from(name: String) -> Self {
        match &*name {
            "V1" => ParquetWriterVersion::V1,
            "V2" => ParquetWriterVersion::V2,
            _ => ParquetWriterVersion::UnknownVariant(UnknownParquetWriterVersion { name }),
        }
    }
}

impl ::std::str::FromStr for ParquetWriterVersion {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ParquetWriterVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ParquetWriterVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes a data processing configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Processor {
    /// <p>The processor parameters.</p>
    #[serde(rename = "Parameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ProcessorParameter>>,
    /// <p>The type of processor.</p>
    #[serde(rename = "Type")]
    pub type_: ProcessorType,
}

/// <p>Describes the processor parameter.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ProcessorParameter {
    /// <p>The name of the parameter.</p>
    #[serde(rename = "ParameterName")]
    pub parameter_name: ProcessorParameterName,
    /// <p>The parameter value.</p>
    #[serde(rename = "ParameterValue")]
    pub parameter_value: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownProcessorParameterName {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ProcessorParameterName {
    BufferIntervalInSeconds,
    BufferSizeInMBs,
    LambdaArn,
    NumberOfRetries,
    RoleArn,
    #[doc(hidden)]
    UnknownVariant(UnknownProcessorParameterName),
}

impl Default for ProcessorParameterName {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ProcessorParameterName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ProcessorParameterName {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ProcessorParameterName {
    fn into(self) -> String {
        match self {
            ProcessorParameterName::BufferIntervalInSeconds => {
                "BufferIntervalInSeconds".to_string()
            }
            ProcessorParameterName::BufferSizeInMBs => "BufferSizeInMBs".to_string(),
            ProcessorParameterName::LambdaArn => "LambdaArn".to_string(),
            ProcessorParameterName::NumberOfRetries => "NumberOfRetries".to_string(),
            ProcessorParameterName::RoleArn => "RoleArn".to_string(),
            ProcessorParameterName::UnknownVariant(UnknownProcessorParameterName {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ProcessorParameterName {
    fn into(self) -> &'a str {
        match self {
            ProcessorParameterName::BufferIntervalInSeconds => &"BufferIntervalInSeconds",
            ProcessorParameterName::BufferSizeInMBs => &"BufferSizeInMBs",
            ProcessorParameterName::LambdaArn => &"LambdaArn",
            ProcessorParameterName::NumberOfRetries => &"NumberOfRetries",
            ProcessorParameterName::RoleArn => &"RoleArn",
            ProcessorParameterName::UnknownVariant(UnknownProcessorParameterName {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for ProcessorParameterName {
    fn from(name: &str) -> Self {
        match name {
            "BufferIntervalInSeconds" => ProcessorParameterName::BufferIntervalInSeconds,
            "BufferSizeInMBs" => ProcessorParameterName::BufferSizeInMBs,
            "LambdaArn" => ProcessorParameterName::LambdaArn,
            "NumberOfRetries" => ProcessorParameterName::NumberOfRetries,
            "RoleArn" => ProcessorParameterName::RoleArn,
            _ => ProcessorParameterName::UnknownVariant(UnknownProcessorParameterName {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ProcessorParameterName {
    fn from(name: String) -> Self {
        match &*name {
            "BufferIntervalInSeconds" => ProcessorParameterName::BufferIntervalInSeconds,
            "BufferSizeInMBs" => ProcessorParameterName::BufferSizeInMBs,
            "LambdaArn" => ProcessorParameterName::LambdaArn,
            "NumberOfRetries" => ProcessorParameterName::NumberOfRetries,
            "RoleArn" => ProcessorParameterName::RoleArn,
            _ => ProcessorParameterName::UnknownVariant(UnknownProcessorParameterName { name }),
        }
    }
}

impl ::std::str::FromStr for ProcessorParameterName {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ProcessorParameterName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ProcessorParameterName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownProcessorType {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum ProcessorType {
    Lambda,
    #[doc(hidden)]
    UnknownVariant(UnknownProcessorType),
}

impl Default for ProcessorType {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for ProcessorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for ProcessorType {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for ProcessorType {
    fn into(self) -> String {
        match self {
            ProcessorType::Lambda => "Lambda".to_string(),
            ProcessorType::UnknownVariant(UnknownProcessorType { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a ProcessorType {
    fn into(self) -> &'a str {
        match self {
            ProcessorType::Lambda => &"Lambda",
            ProcessorType::UnknownVariant(UnknownProcessorType { name: original }) => original,
        }
    }
}

impl From<&str> for ProcessorType {
    fn from(name: &str) -> Self {
        match name {
            "Lambda" => ProcessorType::Lambda,
            _ => ProcessorType::UnknownVariant(UnknownProcessorType {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for ProcessorType {
    fn from(name: String) -> Self {
        match &*name {
            "Lambda" => ProcessorType::Lambda,
            _ => ProcessorType::UnknownVariant(UnknownProcessorType { name }),
        }
    }
}

impl ::std::str::FromStr for ProcessorType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for ProcessorType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for ProcessorType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecordBatchInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>One or more records.</p>
    #[serde(rename = "Records")]
    pub records: Vec<Record>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecordBatchOutput {
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The number of records that might have failed processing. This number might be greater than 0 even if the <a>PutRecordBatch</a> call succeeds. Check <code>FailedPutCount</code> to determine whether there are records that you need to resend.</p>
    #[serde(rename = "FailedPutCount")]
    pub failed_put_count: i64,
    /// <p>The results array. For each record, the index of the response element is the same as the index used in the request array.</p>
    #[serde(rename = "RequestResponses")]
    pub request_responses: Vec<PutRecordBatchResponseEntry>,
}

/// <p>Contains the result for an individual record from a <a>PutRecordBatch</a> request. If the record is successfully added to your delivery stream, it receives a record ID. If the record fails to be added to your delivery stream, the result includes an error code and an error message.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutRecordInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>The record.</p>
    #[serde(rename = "Record")]
    pub record: Record,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutRecordOutput {
    /// <p>Indicates whether server-side encryption (SSE) was enabled during this operation.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The ID of the record.</p>
    #[serde(rename = "RecordId")]
    pub record_id: String,
}

/// <p>The unit of data in a delivery stream.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Record {
    /// <p>The data blob, which is base64-encoded when the blob is serialized. The maximum size of the data blob, before base64-encoding, is 1,000 KiB.</p>
    #[serde(rename = "Data")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub data: bytes::Bytes,
}

/// <p>Describes the configuration of a destination in Amazon Redshift.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The Amazon S3 backup mode. After you create a delivery stream, you can update it to enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the delivery stream to disable it. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<RedshiftS3BackupMode>,
    /// <p>The configuration for the intermediate Amazon S3 location from which Amazon Redshift obtains data. Restrictions are described in the topic for <a>CreateDeliveryStream</a>.</p> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes a destination in Amazon Redshift.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    pub s3_backup_mode: Option<RedshiftS3BackupMode>,
    /// <p>The Amazon S3 destination.</p>
    #[serde(rename = "S3DestinationDescription")]
    pub s3_destination_description: S3DestinationDescription,
    /// <p>The name of the user.</p>
    #[serde(rename = "Username")]
    pub username: String,
}

/// <p>Describes an update for a destination in Amazon Redshift.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>You can update a delivery stream to enable Amazon S3 backup if it is disabled. If backup is enabled, you can't update the delivery stream to disable it. </p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<RedshiftS3BackupMode>,
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct RedshiftRetryOptions {
    /// <p>The length of time during which Kinesis Data Firehose retries delivery after a failure, starting from the initial request and including the first attempt. The default value is 3600 seconds (60 minutes). Kinesis Data Firehose does not retry if the value of <code>DurationInSeconds</code> is 0 (zero) or if the first delivery attempt takes longer than the current value.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownRedshiftS3BackupMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum RedshiftS3BackupMode {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownRedshiftS3BackupMode),
}

impl Default for RedshiftS3BackupMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for RedshiftS3BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for RedshiftS3BackupMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for RedshiftS3BackupMode {
    fn into(self) -> String {
        match self {
            RedshiftS3BackupMode::Disabled => "Disabled".to_string(),
            RedshiftS3BackupMode::Enabled => "Enabled".to_string(),
            RedshiftS3BackupMode::UnknownVariant(UnknownRedshiftS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a RedshiftS3BackupMode {
    fn into(self) -> &'a str {
        match self {
            RedshiftS3BackupMode::Disabled => &"Disabled",
            RedshiftS3BackupMode::Enabled => &"Enabled",
            RedshiftS3BackupMode::UnknownVariant(UnknownRedshiftS3BackupMode {
                name: original,
            }) => original,
        }
    }
}

impl From<&str> for RedshiftS3BackupMode {
    fn from(name: &str) -> Self {
        match name {
            "Disabled" => RedshiftS3BackupMode::Disabled,
            "Enabled" => RedshiftS3BackupMode::Enabled,
            _ => RedshiftS3BackupMode::UnknownVariant(UnknownRedshiftS3BackupMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for RedshiftS3BackupMode {
    fn from(name: String) -> Self {
        match &*name {
            "Disabled" => RedshiftS3BackupMode::Disabled,
            "Enabled" => RedshiftS3BackupMode::Enabled,
            _ => RedshiftS3BackupMode::UnknownVariant(UnknownRedshiftS3BackupMode { name }),
        }
    }
}

impl ::std::str::FromStr for RedshiftS3BackupMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for RedshiftS3BackupMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for RedshiftS3BackupMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownS3BackupMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum S3BackupMode {
    Disabled,
    Enabled,
    #[doc(hidden)]
    UnknownVariant(UnknownS3BackupMode),
}

impl Default for S3BackupMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for S3BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for S3BackupMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for S3BackupMode {
    fn into(self) -> String {
        match self {
            S3BackupMode::Disabled => "Disabled".to_string(),
            S3BackupMode::Enabled => "Enabled".to_string(),
            S3BackupMode::UnknownVariant(UnknownS3BackupMode { name: original }) => original,
        }
    }
}

impl<'a> Into<&'a str> for &'a S3BackupMode {
    fn into(self) -> &'a str {
        match self {
            S3BackupMode::Disabled => &"Disabled",
            S3BackupMode::Enabled => &"Enabled",
            S3BackupMode::UnknownVariant(UnknownS3BackupMode { name: original }) => original,
        }
    }
}

impl From<&str> for S3BackupMode {
    fn from(name: &str) -> Self {
        match name {
            "Disabled" => S3BackupMode::Disabled,
            "Enabled" => S3BackupMode::Enabled,
            _ => S3BackupMode::UnknownVariant(UnknownS3BackupMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for S3BackupMode {
    fn from(name: String) -> Self {
        match &*name {
            "Disabled" => S3BackupMode::Disabled,
            "Enabled" => S3BackupMode::Enabled,
            _ => S3BackupMode::UnknownVariant(UnknownS3BackupMode { name }),
        }
    }
}

impl ::std::str::FromStr for S3BackupMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for S3BackupMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for S3BackupMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

/// <p>Describes the configuration of a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub compression_format: Option<CompressionFormat>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Describes a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    pub compression_format: CompressionFormat,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: EncryptionConfiguration,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Describes an update for a destination in Amazon S3.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub compression_format: Option<CompressionFormat>,
    /// <p>The encryption configuration. If no value is specified, the default is no encryption.</p>
    #[serde(rename = "EncryptionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_configuration: Option<EncryptionConfiguration>,
    /// <p>A prefix that Kinesis Data Firehose evaluates and adds to failed records before writing them to S3. This prefix appears immediately following the bucket name. For information about how to specify this prefix, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "ErrorOutputPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_output_prefix: Option<String>,
    /// <p>The "YYYY/MM/DD/HH" time format prefix is automatically used for delivered Amazon S3 files. You can also specify a custom prefix, as described in <a href="https://docs.aws.amazon.com/firehose/latest/dev/s3-prefixes.html">Custom Prefixes for Amazon S3 Objects</a>.</p>
    #[serde(rename = "Prefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS credentials. For more information, see <a href="https://docs.aws.amazon.com/general/latest/gr/aws-arns-and-namespaces.html">Amazon Resource Names (ARNs) and AWS Service Namespaces</a>.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Specifies the schema to which you want Kinesis Data Firehose to configure your data before it writes it to Amazon S3. This parameter is required if <code>Enabled</code> is set to true.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SourceDescription {
    /// <p>The <a>KinesisStreamSourceDescription</a> value for the source Kinesis data stream.</p>
    #[serde(rename = "KinesisStreamSourceDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_stream_source_description: Option<KinesisStreamSourceDescription>,
}

/// <p>Describes the configuration of a destination in Splunk.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub hec_endpoint_type: HECEndpointType,
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
    /// <p>Defines how documents should be delivered to Amazon S3. When set to <code>FailedEventsOnly</code>, Kinesis Data Firehose writes any data that could not be indexed to the configured Amazon S3 destination. When set to <code>AllEvents</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents to Amazon S3. The default value is <code>FailedEventsOnly</code>.</p> <p>You can update this backup mode from <code>FailedEventsOnly</code> to <code>AllEvents</code>. You can't update it from <code>AllEvents</code> to <code>FailedEventsOnly</code>.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<SplunkS3BackupMode>,
    /// <p>The configuration for the backup Amazon S3 location.</p>
    #[serde(rename = "S3Configuration")]
    pub s3_configuration: S3DestinationConfiguration,
}

/// <p>Describes a destination in Splunk.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    pub hec_endpoint_type: Option<HECEndpointType>,
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
    pub s3_backup_mode: Option<SplunkS3BackupMode>,
    /// <p>The Amazon S3 destination.&gt;</p>
    #[serde(rename = "S3DestinationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_destination_description: Option<S3DestinationDescription>,
}

/// <p>Describes an update for a destination in Splunk.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub hec_endpoint_type: Option<HECEndpointType>,
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
    /// <p>Specifies how you want Kinesis Data Firehose to back up documents to Amazon S3. When set to <code>FailedDocumentsOnly</code>, Kinesis Data Firehose writes any data that could not be indexed to the configured Amazon S3 destination. When set to <code>AllEvents</code>, Kinesis Data Firehose delivers all incoming records to Amazon S3, and also writes failed documents to Amazon S3. The default value is <code>FailedEventsOnly</code>.</p> <p>You can update this backup mode from <code>FailedEventsOnly</code> to <code>AllEvents</code>. You can't update it from <code>AllEvents</code> to <code>FailedEventsOnly</code>.</p>
    #[serde(rename = "S3BackupMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_backup_mode: Option<SplunkS3BackupMode>,
    /// <p>Your update to the configuration of the backup Amazon S3 location.</p>
    #[serde(rename = "S3Update")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_update: Option<S3DestinationUpdate>,
}

/// <p>Configures retry behavior in case Kinesis Data Firehose is unable to deliver documents to Splunk, or if it doesn't receive an acknowledgment from Splunk.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SplunkRetryOptions {
    /// <p>The total amount of time that Kinesis Data Firehose spends on retries. This duration starts after the initial attempt to send data to Splunk fails. It doesn't include the periods during which Kinesis Data Firehose waits for acknowledgment from Splunk after each attempt.</p>
    #[serde(rename = "DurationInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_seconds: Option<i64>,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnknownSplunkS3BackupMode {
    name: String,
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub enum SplunkS3BackupMode {
    AllEvents,
    FailedEventsOnly,
    #[doc(hidden)]
    UnknownVariant(UnknownSplunkS3BackupMode),
}

impl Default for SplunkS3BackupMode {
    fn default() -> Self {
        "".into()
    }
}

impl fmt::Display for SplunkS3BackupMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.into())
    }
}

impl rusoto_core::param::ToParam for SplunkS3BackupMode {
    fn to_param(&self) -> String {
        self.to_string()
    }
}

impl Into<String> for SplunkS3BackupMode {
    fn into(self) -> String {
        match self {
            SplunkS3BackupMode::AllEvents => "AllEvents".to_string(),
            SplunkS3BackupMode::FailedEventsOnly => "FailedEventsOnly".to_string(),
            SplunkS3BackupMode::UnknownVariant(UnknownSplunkS3BackupMode { name: original }) => {
                original
            }
        }
    }
}

impl<'a> Into<&'a str> for &'a SplunkS3BackupMode {
    fn into(self) -> &'a str {
        match self {
            SplunkS3BackupMode::AllEvents => &"AllEvents",
            SplunkS3BackupMode::FailedEventsOnly => &"FailedEventsOnly",
            SplunkS3BackupMode::UnknownVariant(UnknownSplunkS3BackupMode { name: original }) => {
                original
            }
        }
    }
}

impl From<&str> for SplunkS3BackupMode {
    fn from(name: &str) -> Self {
        match name {
            "AllEvents" => SplunkS3BackupMode::AllEvents,
            "FailedEventsOnly" => SplunkS3BackupMode::FailedEventsOnly,
            _ => SplunkS3BackupMode::UnknownVariant(UnknownSplunkS3BackupMode {
                name: name.to_owned(),
            }),
        }
    }
}

impl From<String> for SplunkS3BackupMode {
    fn from(name: String) -> Self {
        match &*name {
            "AllEvents" => SplunkS3BackupMode::AllEvents,
            "FailedEventsOnly" => SplunkS3BackupMode::FailedEventsOnly,
            _ => SplunkS3BackupMode::UnknownVariant(UnknownSplunkS3BackupMode { name }),
        }
    }
}

impl ::std::str::FromStr for SplunkS3BackupMode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.into())
    }
}

impl Serialize for SplunkS3BackupMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.into())
    }
}

impl<'de> Deserialize<'de> for SplunkS3BackupMode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDeliveryStreamEncryptionInput {
    /// <p>Used to specify the type and Amazon Resource Name (ARN) of the KMS key needed for Server-Side Encryption (SSE).</p>
    #[serde(rename = "DeliveryStreamEncryptionConfigurationInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_stream_encryption_configuration_input:
        Option<DeliveryStreamEncryptionConfigurationInput>,
    /// <p>The name of the delivery stream for which you want to enable server-side encryption (SSE).</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDeliveryStreamEncryptionOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDeliveryStreamEncryptionInput {
    /// <p>The name of the delivery stream for which you want to disable server-side encryption (SSE).</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopDeliveryStreamEncryptionOutput {}

/// <p>Metadata that you can assign to a delivery stream, consisting of a key-value pair.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>A unique identifier for the tag. Maximum length: 128 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An optional string, which you can use to describe or define the tag. Maximum length: 256 characters. Valid characters: Unicode letters, digits, white space, _ . / = + - % @</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagDeliveryStreamInput {
    /// <p>The name of the delivery stream to which you want to add the tags.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>A set of key-value pairs to use to create the tags.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagDeliveryStreamOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagDeliveryStreamInput {
    /// <p>The name of the delivery stream.</p>
    #[serde(rename = "DeliveryStreamName")]
    pub delivery_stream_name: String,
    /// <p>A list of tag keys. Each corresponding tag is removed from the delivery stream.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagDeliveryStreamOutput {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateDestinationInput {
    /// <p>Obtain this value from the <code>VersionId</code> result of <a>DeliveryStreamDescription</a>. This value is required, and helps the service perform conditional operations. For example, if there is an interleaving update and this value is null, then the update destination fails. After the update is successful, the <code>VersionId</code> value is updated. The service then performs a merge of the old configuration with the new configuration.</p>
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
    /// <p>Describes an update to the specified HTTP endpoint destination.</p>
    #[serde(rename = "HttpEndpointDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_endpoint_destination_update: Option<HttpEndpointDestinationUpdate>,
    /// <p>Describes an update for a destination in Amazon Redshift.</p>
    #[serde(rename = "RedshiftDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_destination_update: Option<RedshiftDestinationUpdate>,
    /// <p>Describes an update for a destination in Splunk.</p>
    #[serde(rename = "SplunkDestinationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splunk_destination_update: Option<SplunkDestinationUpdate>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateDestinationOutput {}

/// <p>The details of the VPC of the Amazon ES destination.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct VpcConfiguration {
    /// <p>The ARN of the IAM role that you want the delivery stream to use to create endpoints in the destination VPC. You can use your existing Kinesis Data Firehose delivery role or you can specify a new role. In either case, make sure that the role trusts the Kinesis Data Firehose service principal and that it grants the following permissions:</p> <ul> <li> <p> <code>ec2:DescribeVpcs</code> </p> </li> <li> <p> <code>ec2:DescribeVpcAttribute</code> </p> </li> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeSecurityGroups</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterfacePermission</code> </p> </li> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul> <p>If you revoke these permissions after you create the delivery stream, Kinesis Data Firehose can't scale out by creating more ENIs when necessary. You might therefore see a degradation in performance.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The IDs of the security groups that you want Kinesis Data Firehose to use when it creates ENIs in the VPC of the Amazon ES destination. You can use the same security group that the Amazon ES domain uses or different ones. If you specify different security groups here, ensure that they allow outbound HTTPS traffic to the Amazon ES domain's security group. Also ensure that the Amazon ES domain's security group allows HTTPS traffic from the security groups specified here. If you use the same security group for both your delivery stream and the Amazon ES domain, make sure the security group inbound rule allows HTTPS traffic. For more information about security group rules, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html#SecurityGroupRules">Security group rules</a> in the Amazon VPC documentation.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The IDs of the subnets that you want Kinesis Data Firehose to use to create ENIs in the VPC of the Amazon ES destination. Make sure that the routing tables and inbound and outbound rules allow traffic to flow from the subnets whose IDs are specified here to the subnets that have the destination Amazon ES endpoints. Kinesis Data Firehose creates at least one ENI in each of the subnets that are specified here. Do not delete or modify these ENIs.</p> <p>The number of ENIs that Kinesis Data Firehose creates in the subnets specified here scales up and down automatically based on throughput. To enable Kinesis Data Firehose to scale up the number of ENIs to match throughput, ensure that you have sufficient quota. To help you calculate the quota you need, assume that Kinesis Data Firehose can create up to three ENIs for this delivery stream for each of the subnets specified here. For more information about ENI quota, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/amazon-vpc-limits.html#vpc-limits-enis">Network Interfaces </a> in the Amazon VPC Quotas topic.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
}

/// <p>The details of the VPC of the Amazon ES destination.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VpcConfigurationDescription {
    /// <p>The ARN of the IAM role that the delivery stream uses to create endpoints in the destination VPC. You can use your existing Kinesis Data Firehose delivery role or you can specify a new role. In either case, make sure that the role trusts the Kinesis Data Firehose service principal and that it grants the following permissions:</p> <ul> <li> <p> <code>ec2:DescribeVpcs</code> </p> </li> <li> <p> <code>ec2:DescribeVpcAttribute</code> </p> </li> <li> <p> <code>ec2:DescribeSubnets</code> </p> </li> <li> <p> <code>ec2:DescribeSecurityGroups</code> </p> </li> <li> <p> <code>ec2:DescribeNetworkInterfaces</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterface</code> </p> </li> <li> <p> <code>ec2:CreateNetworkInterfacePermission</code> </p> </li> <li> <p> <code>ec2:DeleteNetworkInterface</code> </p> </li> </ul> <p>If you revoke these permissions after you create the delivery stream, Kinesis Data Firehose can't scale out by creating more ENIs when necessary. You might therefore see a degradation in performance.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
    /// <p>The IDs of the security groups that Kinesis Data Firehose uses when it creates ENIs in the VPC of the Amazon ES destination. You can use the same security group that the Amazon ES domain uses or different ones. If you specify different security groups, ensure that they allow outbound HTTPS traffic to the Amazon ES domain's security group. Also ensure that the Amazon ES domain's security group allows HTTPS traffic from the security groups specified here. If you use the same security group for both your delivery stream and the Amazon ES domain, make sure the security group inbound rule allows HTTPS traffic. For more information about security group rules, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_SecurityGroups.html#SecurityGroupRules">Security group rules</a> in the Amazon VPC documentation.</p>
    #[serde(rename = "SecurityGroupIds")]
    pub security_group_ids: Vec<String>,
    /// <p>The IDs of the subnets that Kinesis Data Firehose uses to create ENIs in the VPC of the Amazon ES destination. Make sure that the routing tables and inbound and outbound rules allow traffic to flow from the subnets whose IDs are specified here to the subnets that have the destination Amazon ES endpoints. Kinesis Data Firehose creates at least one ENI in each of the subnets that are specified here. Do not delete or modify these ENIs.</p> <p>The number of ENIs that Kinesis Data Firehose creates in the subnets specified here scales up and down automatically based on throughput. To enable Kinesis Data Firehose to scale up the number of ENIs to match throughput, ensure that you have sufficient quota. To help you calculate the quota you need, assume that Kinesis Data Firehose can create up to three ENIs for this delivery stream for each of the subnets specified here. For more information about ENI quota, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/amazon-vpc-limits.html#vpc-limits-enis">Network Interfaces </a> in the Amazon VPC Quotas topic.</p>
    #[serde(rename = "SubnetIds")]
    pub subnet_ids: Vec<String>,
    /// <p>The ID of the Amazon ES destination's VPC.</p>
    #[serde(rename = "VpcId")]
    pub vpc_id: String,
}

/// Errors returned by CreateDeliveryStream
#[derive(Debug, PartialEq)]
pub enum CreateDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>Kinesis Data Firehose throws this exception when an attempt to put records or to start or stop delivery stream encryption fails. This happens when the KMS service throws one of the following exception types: <code>AccessDeniedException</code>, <code>InvalidStateException</code>, <code>DisabledException</code>, or <code>NotFoundException</code>.</p>
    InvalidKMSResource(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
}

impl CreateDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateDeliveryStreamError::InvalidArgument(
                        err.msg,
                    ))
                }
                "InvalidKMSResourceException" => {
                    return RusotoError::Service(CreateDeliveryStreamError::InvalidKMSResource(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDeliveryStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateDeliveryStreamError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateDeliveryStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            CreateDeliveryStreamError::InvalidKMSResource(ref cause) => write!(f, "{}", cause),
            CreateDeliveryStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateDeliveryStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateDeliveryStreamError {}
/// Errors returned by DeleteDeliveryStream
#[derive(Debug, PartialEq)]
pub enum DeleteDeliveryStreamError {
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl DeleteDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteDeliveryStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDeliveryStreamError::ResourceNotFound(
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
impl fmt::Display for DeleteDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDeliveryStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteDeliveryStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDeliveryStreamError {}
/// Errors returned by DescribeDeliveryStream
#[derive(Debug, PartialEq)]
pub enum DescribeDeliveryStreamError {
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl DescribeDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDeliveryStreamError::ResourceNotFound(
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
impl fmt::Display for DescribeDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDeliveryStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDeliveryStreamError {}
/// Errors returned by ListDeliveryStreams
#[derive(Debug, PartialEq)]
pub enum ListDeliveryStreamsError {}

impl ListDeliveryStreamsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDeliveryStreamsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListDeliveryStreamsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListDeliveryStreamsError {}
/// Errors returned by ListTagsForDeliveryStream
#[derive(Debug, PartialEq)]
pub enum ListTagsForDeliveryStreamError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForDeliveryStreamError::InvalidArgument(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTagsForDeliveryStreamError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForDeliveryStreamError::ResourceNotFound(
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
impl fmt::Display for ListTagsForDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForDeliveryStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            ListTagsForDeliveryStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            ListTagsForDeliveryStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForDeliveryStreamError {}
/// Errors returned by PutRecord
#[derive(Debug, PartialEq)]
pub enum PutRecordError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>Kinesis Data Firehose throws this exception when an attempt to put records or to start or stop delivery stream encryption fails. This happens when the KMS service throws one of the following exception types: <code>AccessDeniedException</code>, <code>InvalidStateException</code>, <code>DisabledException</code>, or <code>NotFoundException</code>.</p>
    InvalidKMSResource(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p>
    ServiceUnavailable(String),
}

impl PutRecordError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRecordError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(PutRecordError::InvalidArgument(err.msg))
                }
                "InvalidKMSResourceException" => {
                    return RusotoError::Service(PutRecordError::InvalidKMSResource(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRecordError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutRecordError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRecordError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRecordError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            PutRecordError::InvalidKMSResource(ref cause) => write!(f, "{}", cause),
            PutRecordError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutRecordError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRecordError {}
/// Errors returned by PutRecordBatch
#[derive(Debug, PartialEq)]
pub enum PutRecordBatchError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>Kinesis Data Firehose throws this exception when an attempt to put records or to start or stop delivery stream encryption fails. This happens when the KMS service throws one of the following exception types: <code>AccessDeniedException</code>, <code>InvalidStateException</code>, <code>DisabledException</code>, or <code>NotFoundException</code>.</p>
    InvalidKMSResource(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is unavailable. Back off and retry the operation. If you continue to see the exception, throughput limits for the delivery stream may have been exceeded. For more information about limits and how to request an increase, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>.</p>
    ServiceUnavailable(String),
}

impl PutRecordBatchError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutRecordBatchError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(PutRecordBatchError::InvalidArgument(err.msg))
                }
                "InvalidKMSResourceException" => {
                    return RusotoError::Service(PutRecordBatchError::InvalidKMSResource(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutRecordBatchError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutRecordBatchError::ServiceUnavailable(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutRecordBatchError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutRecordBatchError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            PutRecordBatchError::InvalidKMSResource(ref cause) => write!(f, "{}", cause),
            PutRecordBatchError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutRecordBatchError::ServiceUnavailable(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutRecordBatchError {}
/// Errors returned by StartDeliveryStreamEncryption
#[derive(Debug, PartialEq)]
pub enum StartDeliveryStreamEncryptionError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>Kinesis Data Firehose throws this exception when an attempt to put records or to start or stop delivery stream encryption fails. This happens when the KMS service throws one of the following exception types: <code>AccessDeniedException</code>, <code>InvalidStateException</code>, <code>DisabledException</code>, or <code>NotFoundException</code>.</p>
    InvalidKMSResource(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl StartDeliveryStreamEncryptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDeliveryStreamEncryptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        StartDeliveryStreamEncryptionError::InvalidArgument(err.msg),
                    )
                }
                "InvalidKMSResourceException" => {
                    return RusotoError::Service(
                        StartDeliveryStreamEncryptionError::InvalidKMSResource(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartDeliveryStreamEncryptionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartDeliveryStreamEncryptionError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StartDeliveryStreamEncryptionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDeliveryStreamEncryptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDeliveryStreamEncryptionError::InvalidArgument(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDeliveryStreamEncryptionError::InvalidKMSResource(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDeliveryStreamEncryptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StartDeliveryStreamEncryptionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartDeliveryStreamEncryptionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartDeliveryStreamEncryptionError {}
/// Errors returned by StopDeliveryStreamEncryption
#[derive(Debug, PartialEq)]
pub enum StopDeliveryStreamEncryptionError {
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>You have already reached the limit for a requested resource.</p>
    LimitExceeded(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl StopDeliveryStreamEncryptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopDeliveryStreamEncryptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        StopDeliveryStreamEncryptionError::InvalidArgument(err.msg),
                    )
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StopDeliveryStreamEncryptionError::LimitExceeded(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopDeliveryStreamEncryptionError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        StopDeliveryStreamEncryptionError::ResourceNotFound(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDeliveryStreamEncryptionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDeliveryStreamEncryptionError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            StopDeliveryStreamEncryptionError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            StopDeliveryStreamEncryptionError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopDeliveryStreamEncryptionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StopDeliveryStreamEncryptionError {}
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
}

impl TagDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagDeliveryStreamError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagDeliveryStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(TagDeliveryStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagDeliveryStreamError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagDeliveryStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            TagDeliveryStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            TagDeliveryStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            TagDeliveryStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagDeliveryStreamError {}
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
}

impl UntagDeliveryStreamError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagDeliveryStreamError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagDeliveryStreamError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UntagDeliveryStreamError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UntagDeliveryStreamError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagDeliveryStreamError::ResourceNotFound(
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
impl fmt::Display for UntagDeliveryStreamError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagDeliveryStreamError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UntagDeliveryStreamError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UntagDeliveryStreamError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UntagDeliveryStreamError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagDeliveryStreamError {}
/// Errors returned by UpdateDestination
#[derive(Debug, PartialEq)]
pub enum UpdateDestinationError {
    /// <p>Another modification has already happened. Fetch <code>VersionId</code> again and use it to update the destination.</p>
    ConcurrentModification(String),
    /// <p>The specified input parameter has a value that is not valid.</p>
    InvalidArgument(String),
    /// <p>The resource is already in use and not available for this operation.</p>
    ResourceInUse(String),
    /// <p>The specified resource could not be found.</p>
    ResourceNotFound(String),
}

impl UpdateDestinationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDestinationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateDestinationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateDestinationError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateDestinationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDestinationError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateDestinationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateDestinationError::ConcurrentModification(ref cause) => write!(f, "{}", cause),
            UpdateDestinationError::InvalidArgument(ref cause) => write!(f, "{}", cause),
            UpdateDestinationError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateDestinationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateDestinationError {}
/// Trait representing the capabilities of the Firehose API. Firehose clients implement this trait.
#[async_trait]
pub trait KinesisFirehose {
    /// <p>Creates a Kinesis Data Firehose delivery stream.</p> <p>By default, you can create up to 50 delivery streams per AWS Region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. If the delivery stream creation fails, the status transitions to <code>CREATING_FAILED</code>. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>If the status of a delivery stream is <code>CREATING_FAILED</code>, this status doesn't change, and you can't invoke <code>CreateDeliveryStream</code> again on it. However, you can invoke the <a>DeleteDeliveryStream</a> operation to delete it.</p> <p>A Kinesis Data Firehose delivery stream can be configured to receive records directly from providers using <a>PutRecord</a> or <a>PutRecordBatch</a>, or it can be configured to use an existing Kinesis stream as its source. To specify a Kinesis data stream as input, set the <code>DeliveryStreamType</code> parameter to <code>KinesisStreamAsSource</code>, and provide the Kinesis stream Amazon Resource Name (ARN) and role ARN in the <code>KinesisStreamSourceConfiguration</code> parameter.</p> <p>To create a delivery stream with server-side encryption (SSE) enabled, include <a>DeliveryStreamEncryptionConfigurationInput</a> in your request. This is optional. You can also invoke <a>StartDeliveryStreamEncryption</a> to turn on SSE for an existing delivery stream that doesn't have SSE enabled.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon ES, Amazon Redshift, or Splunk. You must specify only one of the following destination configuration parameters: <code>ExtendedS3DestinationConfiguration</code>, <code>S3DestinationConfiguration</code>, <code>ElasticsearchDestinationConfiguration</code>, <code>RedshiftDestinationConfiguration</code>, or <code>SplunkDestinationConfiguration</code>.</p> <p>When you specify <code>S3DestinationConfiguration</code>, you can also provide the following optional values: BufferingHints, <code>EncryptionConfiguration</code>, and <code>CompressionFormat</code>. By default, if no <code>BufferingHints</code> value is provided, Kinesis Data Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. <code>BufferingHints</code> is a hint, so there are some cases where the service cannot adhere to these conditions strictly. For example, record boundaries might be such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location. Kinesis Data Firehose first delivers data to Amazon S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <code>RedshiftDestinationConfiguration.S3Configuration</code> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Kinesis Data Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Kinesis Data Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Kinesis Data Firehose principal to assume the role, and the role should have permissions that allow the service to deliver the data. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    async fn create_delivery_stream(
        &self,
        input: CreateDeliveryStreamInput,
    ) -> Result<CreateDeliveryStreamOutput, RusotoError<CreateDeliveryStreamError>>;

    /// <p>Deletes a delivery stream and its data.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>. You can delete a delivery stream only if it is in one of the following states: <code>ACTIVE</code>, <code>DELETING</code>, <code>CREATING_FAILED</code>, or <code>DELETING_FAILED</code>. You can't delete a delivery stream that is in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>While the delivery stream is in the <code>DELETING</code> state, the service might continue to accept records, but it doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, first stop any applications that are sending records before you delete a delivery stream.</p>
    async fn delete_delivery_stream(
        &self,
        input: DeleteDeliveryStreamInput,
    ) -> Result<DeleteDeliveryStreamOutput, RusotoError<DeleteDeliveryStreamError>>;

    /// <p>Describes the specified delivery stream and its status. For example, after your delivery stream is created, call <code>DescribeDeliveryStream</code> to see whether the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it. </p> <p>If the status of a delivery stream is <code>CREATING_FAILED</code>, this status doesn't change, and you can't invoke <a>CreateDeliveryStream</a> again on it. However, you can invoke the <a>DeleteDeliveryStream</a> operation to delete it. If the status is <code>DELETING_FAILED</code>, you can force deletion by invoking <a>DeleteDeliveryStream</a> again but with <a>DeleteDeliveryStreamInput$AllowForceDelete</a> set to true.</p>
    async fn describe_delivery_stream(
        &self,
        input: DescribeDeliveryStreamInput,
    ) -> Result<DescribeDeliveryStreamOutput, RusotoError<DescribeDeliveryStreamError>>;

    /// <p>Lists your delivery streams in alphabetical order of their names.</p> <p>The number of delivery streams might be too large to return using a single call to <code>ListDeliveryStreams</code>. You can limit the number of delivery streams returned, using the <code>Limit</code> parameter. To determine whether there are more delivery streams to list, check the value of <code>HasMoreDeliveryStreams</code> in the output. If there are more delivery streams to list, you can request them by calling this operation again and setting the <code>ExclusiveStartDeliveryStreamName</code> parameter to the name of the last delivery stream returned in the last call.</p>
    async fn list_delivery_streams(
        &self,
        input: ListDeliveryStreamsInput,
    ) -> Result<ListDeliveryStreamsOutput, RusotoError<ListDeliveryStreamsError>>;

    /// <p>Lists the tags for the specified delivery stream. This operation has a limit of five transactions per second per account. </p>
    async fn list_tags_for_delivery_stream(
        &self,
        input: ListTagsForDeliveryStreamInput,
    ) -> Result<ListTagsForDeliveryStreamOutput, RusotoError<ListTagsForDeliveryStreamError>>;

    /// <p><p>Writes a single data record into an Amazon Kinesis Data Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it can be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <code>PutRecord</code> operation returns a <code>RecordId</code>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <code>PutRecord</code> operation throws a <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it tries to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p> <important> <p>Don&#39;t concatenate two or more base64 strings to form the data fields of your records. Instead, concatenate the raw data, then perform base64 encoding.</p> </important></p>
    async fn put_record(
        &self,
        input: PutRecordInput,
    ) -> Result<PutRecordOutput, RusotoError<PutRecordError>>;

    /// <p><p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>For information about service quota, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Quota</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it could be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <code>FailedPutCount</code>, and an array of responses, <code>RequestResponses</code>. Even if the <a>PutRecordBatch</a> call succeeds, the value of <code>FailedPutCount</code> may be greater than 0, indicating that there are records for which the operation didn&#39;t succeed. Each entry in the <code>RequestResponses</code> array provides additional information about the processed record. It directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <code>RequestResponses</code> includes both successfully and unsuccessfully processed records. Kinesis Data Firehose tries to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records. </p> <p>A successfully processed record includes a <code>RecordId</code> value, which is unique for the record. An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error, and is one of the following values: <code>ServiceUnavailableException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <code>FailedPutCount</code> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p> <important> <p>Don&#39;t concatenate two or more base64 strings to form the data fields of your records. Instead, concatenate the raw data, then perform base64 encoding.</p> </important></p>
    async fn put_record_batch(
        &self,
        input: PutRecordBatchInput,
    ) -> Result<PutRecordBatchOutput, RusotoError<PutRecordBatchError>>;

    /// <p>Enables server-side encryption (SSE) for the delivery stream. </p> <p>This operation is asynchronous. It returns immediately. When you invoke it, Kinesis Data Firehose first sets the encryption status of the stream to <code>ENABLING</code>, and then to <code>ENABLED</code>. The encryption status of a delivery stream is the <code>Status</code> property in <a>DeliveryStreamEncryptionConfiguration</a>. If the operation fails, the encryption status changes to <code>ENABLING_FAILED</code>. You can continue to read and write data to your delivery stream while the encryption status is <code>ENABLING</code>, but the data is not encrypted. It can take up to 5 seconds after the encryption status changes to <code>ENABLED</code> before all records written to the delivery stream are encrypted. To find out whether a record or a batch of records was encrypted, check the response elements <a>PutRecordOutput$Encrypted</a> and <a>PutRecordBatchOutput$Encrypted</a>, respectively.</p> <p>To check the encryption status of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>Even if encryption is currently enabled for a delivery stream, you can still invoke this operation on it to change the ARN of the CMK or both its type and ARN. If you invoke this method to change the CMK, and the old CMK is of type <code>CUSTOMER_MANAGED_CMK</code>, Kinesis Data Firehose schedules the grant it had on the old CMK for retirement. If the new CMK is of type <code>CUSTOMER_MANAGED_CMK</code>, Kinesis Data Firehose creates a grant that enables it to use the new CMK to encrypt and decrypt data and to manage the grant.</p> <p>If a delivery stream already has encryption enabled and then you invoke this operation to change the ARN of the CMK or both its type and ARN and you get <code>ENABLING_FAILED</code>, this only means that the attempt to change the CMK failed. In this case, encryption remains enabled with the old CMK.</p> <p>If the encryption status of your delivery stream is <code>ENABLING_FAILED</code>, you can invoke this operation again with a valid CMK. The CMK must be enabled and the key policy mustn't explicitly deny the permission for Kinesis Data Firehose to invoke KMS encrypt and decrypt operations.</p> <p>You can enable SSE for a delivery stream only if it's a delivery stream that uses <code>DirectPut</code> as its source. </p> <p>The <code>StartDeliveryStreamEncryption</code> and <code>StopDeliveryStreamEncryption</code> operations have a combined limit of 25 calls per delivery stream per 24 hours. For example, you reach the limit if you call <code>StartDeliveryStreamEncryption</code> 13 times and <code>StopDeliveryStreamEncryption</code> 12 times for the same delivery stream in a 24-hour period.</p>
    async fn start_delivery_stream_encryption(
        &self,
        input: StartDeliveryStreamEncryptionInput,
    ) -> Result<StartDeliveryStreamEncryptionOutput, RusotoError<StartDeliveryStreamEncryptionError>>;

    /// <p>Disables server-side encryption (SSE) for the delivery stream. </p> <p>This operation is asynchronous. It returns immediately. When you invoke it, Kinesis Data Firehose first sets the encryption status of the stream to <code>DISABLING</code>, and then to <code>DISABLED</code>. You can continue to read and write data to your stream while its status is <code>DISABLING</code>. It can take up to 5 seconds after the encryption status changes to <code>DISABLED</code> before all records written to the delivery stream are no longer subject to encryption. To find out whether a record or a batch of records was encrypted, check the response elements <a>PutRecordOutput$Encrypted</a> and <a>PutRecordBatchOutput$Encrypted</a>, respectively.</p> <p>To check the encryption state of a delivery stream, use <a>DescribeDeliveryStream</a>. </p> <p>If SSE is enabled using a customer managed CMK and then you invoke <code>StopDeliveryStreamEncryption</code>, Kinesis Data Firehose schedules the related KMS grant for retirement and then retires it after it ensures that it is finished delivering records to the destination.</p> <p>The <code>StartDeliveryStreamEncryption</code> and <code>StopDeliveryStreamEncryption</code> operations have a combined limit of 25 calls per delivery stream per 24 hours. For example, you reach the limit if you call <code>StartDeliveryStreamEncryption</code> 13 times and <code>StopDeliveryStreamEncryption</code> 12 times for the same delivery stream in a 24-hour period.</p>
    async fn stop_delivery_stream_encryption(
        &self,
        input: StopDeliveryStreamEncryptionInput,
    ) -> Result<StopDeliveryStreamEncryptionOutput, RusotoError<StopDeliveryStreamEncryptionError>>;

    /// <p>Adds or updates tags for the specified delivery stream. A tag is a key-value pair that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>Each delivery stream can have up to 50 tags. </p> <p>This operation has a limit of five transactions per second per account. </p>
    async fn tag_delivery_stream(
        &self,
        input: TagDeliveryStreamInput,
    ) -> Result<TagDeliveryStreamOutput, RusotoError<TagDeliveryStreamError>>;

    /// <p>Removes tags from the specified delivery stream. Removed tags are deleted, and you can't recover them after this operation successfully completes.</p> <p>If you specify a tag that doesn't exist, the operation ignores it.</p> <p>This operation has a limit of five transactions per second per account. </p>
    async fn untag_delivery_stream(
        &self,
        input: UntagDeliveryStreamInput,
    ) -> Result<UntagDeliveryStreamOutput, RusotoError<UntagDeliveryStreamError>>;

    /// <p>Updates the specified destination of the specified delivery stream.</p> <p>Use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Kinesis Data Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified, then the existing <code>EncryptionConfiguration</code> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Kinesis Data Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Kinesis Data Firehose uses <code>CurrentDeliveryStreamVersionId</code> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. Use the new version ID to set <code>CurrentDeliveryStreamVersionId</code> in the next call.</p>
    async fn update_destination(
        &self,
        input: UpdateDestinationInput,
    ) -> Result<UpdateDestinationOutput, RusotoError<UpdateDestinationError>>;
}
/// A client for the Firehose API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisFirehoseClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        KinesisFirehoseClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> KinesisFirehoseClient {
        KinesisFirehoseClient { client, region }
    }
}

#[async_trait]
impl KinesisFirehose for KinesisFirehoseClient {
    /// <p>Creates a Kinesis Data Firehose delivery stream.</p> <p>By default, you can create up to 50 delivery streams per AWS Region.</p> <p>This is an asynchronous operation that immediately returns. The initial status of the delivery stream is <code>CREATING</code>. After the delivery stream is created, its status is <code>ACTIVE</code> and it now accepts data. If the delivery stream creation fails, the status transitions to <code>CREATING_FAILED</code>. Attempts to send data to a delivery stream that is not in the <code>ACTIVE</code> state cause an exception. To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>If the status of a delivery stream is <code>CREATING_FAILED</code>, this status doesn't change, and you can't invoke <code>CreateDeliveryStream</code> again on it. However, you can invoke the <a>DeleteDeliveryStream</a> operation to delete it.</p> <p>A Kinesis Data Firehose delivery stream can be configured to receive records directly from providers using <a>PutRecord</a> or <a>PutRecordBatch</a>, or it can be configured to use an existing Kinesis stream as its source. To specify a Kinesis data stream as input, set the <code>DeliveryStreamType</code> parameter to <code>KinesisStreamAsSource</code>, and provide the Kinesis stream Amazon Resource Name (ARN) and role ARN in the <code>KinesisStreamSourceConfiguration</code> parameter.</p> <p>To create a delivery stream with server-side encryption (SSE) enabled, include <a>DeliveryStreamEncryptionConfigurationInput</a> in your request. This is optional. You can also invoke <a>StartDeliveryStreamEncryption</a> to turn on SSE for an existing delivery stream that doesn't have SSE enabled.</p> <p>A delivery stream is configured with a single destination: Amazon S3, Amazon ES, Amazon Redshift, or Splunk. You must specify only one of the following destination configuration parameters: <code>ExtendedS3DestinationConfiguration</code>, <code>S3DestinationConfiguration</code>, <code>ElasticsearchDestinationConfiguration</code>, <code>RedshiftDestinationConfiguration</code>, or <code>SplunkDestinationConfiguration</code>.</p> <p>When you specify <code>S3DestinationConfiguration</code>, you can also provide the following optional values: BufferingHints, <code>EncryptionConfiguration</code>, and <code>CompressionFormat</code>. By default, if no <code>BufferingHints</code> value is provided, Kinesis Data Firehose buffers data up to 5 MB or for 5 minutes, whichever condition is satisfied first. <code>BufferingHints</code> is a hint, so there are some cases where the service cannot adhere to these conditions strictly. For example, record boundaries might be such that the size is a little over or under the configured buffering size. By default, no encryption is performed. We strongly recommend that you enable encryption to ensure secure data storage in Amazon S3.</p> <p>A few notes about Amazon Redshift as a destination:</p> <ul> <li> <p>An Amazon Redshift destination requires an S3 bucket as intermediate location. Kinesis Data Firehose first delivers data to Amazon S3 and then uses <code>COPY</code> syntax to load data into an Amazon Redshift table. This is specified in the <code>RedshiftDestinationConfiguration.S3Configuration</code> parameter.</p> </li> <li> <p>The compression formats <code>SNAPPY</code> or <code>ZIP</code> cannot be specified in <code>RedshiftDestinationConfiguration.S3Configuration</code> because the Amazon Redshift <code>COPY</code> operation that reads from the S3 bucket doesn't support these compression formats.</p> </li> <li> <p>We strongly recommend that you use the user name and password you provide exclusively with Kinesis Data Firehose, and that the permissions for the account are restricted for Amazon Redshift <code>INSERT</code> permissions.</p> </li> </ul> <p>Kinesis Data Firehose assumes the IAM role that is configured as part of the destination. The role should allow the Kinesis Data Firehose principal to assume the role, and the role should have permissions that allow the service to deliver the data. For more information, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/controlling-access.html#using-iam-s3">Grant Kinesis Data Firehose Access to an Amazon S3 Destination</a> in the <i>Amazon Kinesis Data Firehose Developer Guide</i>.</p>
    async fn create_delivery_stream(
        &self,
        input: CreateDeliveryStreamInput,
    ) -> Result<CreateDeliveryStreamOutput, RusotoError<CreateDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.CreateDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, CreateDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<CreateDeliveryStreamOutput, _>()
    }

    /// <p>Deletes a delivery stream and its data.</p> <p>To check the state of a delivery stream, use <a>DescribeDeliveryStream</a>. You can delete a delivery stream only if it is in one of the following states: <code>ACTIVE</code>, <code>DELETING</code>, <code>CREATING_FAILED</code>, or <code>DELETING_FAILED</code>. You can't delete a delivery stream that is in the <code>CREATING</code> state. While the deletion request is in process, the delivery stream is in the <code>DELETING</code> state.</p> <p>While the delivery stream is in the <code>DELETING</code> state, the service might continue to accept records, but it doesn't make any guarantees with respect to delivering the data. Therefore, as a best practice, first stop any applications that are sending records before you delete a delivery stream.</p>
    async fn delete_delivery_stream(
        &self,
        input: DeleteDeliveryStreamInput,
    ) -> Result<DeleteDeliveryStreamOutput, RusotoError<DeleteDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.DeleteDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DeleteDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<DeleteDeliveryStreamOutput, _>()
    }

    /// <p>Describes the specified delivery stream and its status. For example, after your delivery stream is created, call <code>DescribeDeliveryStream</code> to see whether the delivery stream is <code>ACTIVE</code> and therefore ready for data to be sent to it. </p> <p>If the status of a delivery stream is <code>CREATING_FAILED</code>, this status doesn't change, and you can't invoke <a>CreateDeliveryStream</a> again on it. However, you can invoke the <a>DeleteDeliveryStream</a> operation to delete it. If the status is <code>DELETING_FAILED</code>, you can force deletion by invoking <a>DeleteDeliveryStream</a> again but with <a>DeleteDeliveryStreamInput$AllowForceDelete</a> set to true.</p>
    async fn describe_delivery_stream(
        &self,
        input: DescribeDeliveryStreamInput,
    ) -> Result<DescribeDeliveryStreamOutput, RusotoError<DescribeDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.DescribeDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, DescribeDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<DescribeDeliveryStreamOutput, _>()
    }

    /// <p>Lists your delivery streams in alphabetical order of their names.</p> <p>The number of delivery streams might be too large to return using a single call to <code>ListDeliveryStreams</code>. You can limit the number of delivery streams returned, using the <code>Limit</code> parameter. To determine whether there are more delivery streams to list, check the value of <code>HasMoreDeliveryStreams</code> in the output. If there are more delivery streams to list, you can request them by calling this operation again and setting the <code>ExclusiveStartDeliveryStreamName</code> parameter to the name of the last delivery stream returned in the last call.</p>
    async fn list_delivery_streams(
        &self,
        input: ListDeliveryStreamsInput,
    ) -> Result<ListDeliveryStreamsOutput, RusotoError<ListDeliveryStreamsError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.ListDeliveryStreams");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListDeliveryStreamsError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<ListDeliveryStreamsOutput, _>()
    }

    /// <p>Lists the tags for the specified delivery stream. This operation has a limit of five transactions per second per account. </p>
    async fn list_tags_for_delivery_stream(
        &self,
        input: ListTagsForDeliveryStreamInput,
    ) -> Result<ListTagsForDeliveryStreamOutput, RusotoError<ListTagsForDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Firehose_20150804.ListTagsForDeliveryStream",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, ListTagsForDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<ListTagsForDeliveryStreamOutput, _>()
    }

    /// <p><p>Writes a single data record into an Amazon Kinesis Data Firehose delivery stream. To write multiple data records into a delivery stream, use <a>PutRecordBatch</a>. Applications using these operations are referred to as producers.</p> <p>By default, each delivery stream can take in up to 2,000 transactions per second, 5,000 records per second, or 5 MB per second. If you use <a>PutRecord</a> and <a>PutRecordBatch</a>, the limits are an aggregate across these two operations for each delivery stream. For more information about limits and how to request an increase, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Limits</a>. </p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it can be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <code>PutRecord</code> operation returns a <code>RecordId</code>, which is a unique string assigned to each record. Producer applications can use this ID for purposes such as auditability and investigation.</p> <p>If the <code>PutRecord</code> operation throws a <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream. </p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it tries to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p> <important> <p>Don&#39;t concatenate two or more base64 strings to form the data fields of your records. Instead, concatenate the raw data, then perform base64 encoding.</p> </important></p>
    async fn put_record(
        &self,
        input: PutRecordInput,
    ) -> Result<PutRecordOutput, RusotoError<PutRecordError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.PutRecord");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutRecordError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutRecordOutput, _>()
    }

    /// <p><p>Writes multiple data records into a delivery stream in a single call, which can achieve higher throughput per producer than when writing single records. To write single data records into a delivery stream, use <a>PutRecord</a>. Applications using these operations are referred to as producers.</p> <p>For information about service quota, see <a href="https://docs.aws.amazon.com/firehose/latest/dev/limits.html">Amazon Kinesis Data Firehose Quota</a>.</p> <p>Each <a>PutRecordBatch</a> request supports up to 500 records. Each record in the request can be as large as 1,000 KB (before 64-bit encoding), up to a limit of 4 MB for the entire request. These limits cannot be changed.</p> <p>You must specify the name of the delivery stream and the data record when using <a>PutRecord</a>. The data record consists of a data blob that can be up to 1,000 KB in size, and any kind of data. For example, it could be a segment from a log file, geographic location data, website clickstream data, and so on.</p> <p>Kinesis Data Firehose buffers records before delivering them to the destination. To disambiguate the data blobs at the destination, a common solution is to use delimiters in the data, such as a newline (<code>\n</code>) or some other character unique within the data. This allows the consumer application to parse individual data items when reading the data from the destination.</p> <p>The <a>PutRecordBatch</a> response includes a count of failed records, <code>FailedPutCount</code>, and an array of responses, <code>RequestResponses</code>. Even if the <a>PutRecordBatch</a> call succeeds, the value of <code>FailedPutCount</code> may be greater than 0, indicating that there are records for which the operation didn&#39;t succeed. Each entry in the <code>RequestResponses</code> array provides additional information about the processed record. It directly correlates with a record in the request array using the same ordering, from the top to the bottom. The response array always includes the same number of records as the request array. <code>RequestResponses</code> includes both successfully and unsuccessfully processed records. Kinesis Data Firehose tries to process all records in each <a>PutRecordBatch</a> request. A single record failure does not stop the processing of subsequent records. </p> <p>A successfully processed record includes a <code>RecordId</code> value, which is unique for the record. An unsuccessfully processed record includes <code>ErrorCode</code> and <code>ErrorMessage</code> values. <code>ErrorCode</code> reflects the type of error, and is one of the following values: <code>ServiceUnavailableException</code> or <code>InternalFailure</code>. <code>ErrorMessage</code> provides more detailed information about the error.</p> <p>If there is an internal server error or a timeout, the write might have completed or it might have failed. If <code>FailedPutCount</code> is greater than 0, retry the request, resending only those records that might have failed processing. This minimizes the possible duplicate records and also reduces the total bytes sent (and corresponding charges). We recommend that you handle any duplicates at the destination.</p> <p>If <a>PutRecordBatch</a> throws <code>ServiceUnavailableException</code>, back off and retry. If the exception persists, it is possible that the throughput limits have been exceeded for the delivery stream.</p> <p>Data records sent to Kinesis Data Firehose are stored for 24 hours from the time they are added to a delivery stream as it attempts to send the records to the destination. If the destination is unreachable for more than 24 hours, the data is no longer available.</p> <important> <p>Don&#39;t concatenate two or more base64 strings to form the data fields of your records. Instead, concatenate the raw data, then perform base64 encoding.</p> </important></p>
    async fn put_record_batch(
        &self,
        input: PutRecordBatchInput,
    ) -> Result<PutRecordBatchOutput, RusotoError<PutRecordBatchError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.PutRecordBatch");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, PutRecordBatchError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<PutRecordBatchOutput, _>()
    }

    /// <p>Enables server-side encryption (SSE) for the delivery stream. </p> <p>This operation is asynchronous. It returns immediately. When you invoke it, Kinesis Data Firehose first sets the encryption status of the stream to <code>ENABLING</code>, and then to <code>ENABLED</code>. The encryption status of a delivery stream is the <code>Status</code> property in <a>DeliveryStreamEncryptionConfiguration</a>. If the operation fails, the encryption status changes to <code>ENABLING_FAILED</code>. You can continue to read and write data to your delivery stream while the encryption status is <code>ENABLING</code>, but the data is not encrypted. It can take up to 5 seconds after the encryption status changes to <code>ENABLED</code> before all records written to the delivery stream are encrypted. To find out whether a record or a batch of records was encrypted, check the response elements <a>PutRecordOutput$Encrypted</a> and <a>PutRecordBatchOutput$Encrypted</a>, respectively.</p> <p>To check the encryption status of a delivery stream, use <a>DescribeDeliveryStream</a>.</p> <p>Even if encryption is currently enabled for a delivery stream, you can still invoke this operation on it to change the ARN of the CMK or both its type and ARN. If you invoke this method to change the CMK, and the old CMK is of type <code>CUSTOMER_MANAGED_CMK</code>, Kinesis Data Firehose schedules the grant it had on the old CMK for retirement. If the new CMK is of type <code>CUSTOMER_MANAGED_CMK</code>, Kinesis Data Firehose creates a grant that enables it to use the new CMK to encrypt and decrypt data and to manage the grant.</p> <p>If a delivery stream already has encryption enabled and then you invoke this operation to change the ARN of the CMK or both its type and ARN and you get <code>ENABLING_FAILED</code>, this only means that the attempt to change the CMK failed. In this case, encryption remains enabled with the old CMK.</p> <p>If the encryption status of your delivery stream is <code>ENABLING_FAILED</code>, you can invoke this operation again with a valid CMK. The CMK must be enabled and the key policy mustn't explicitly deny the permission for Kinesis Data Firehose to invoke KMS encrypt and decrypt operations.</p> <p>You can enable SSE for a delivery stream only if it's a delivery stream that uses <code>DirectPut</code> as its source. </p> <p>The <code>StartDeliveryStreamEncryption</code> and <code>StopDeliveryStreamEncryption</code> operations have a combined limit of 25 calls per delivery stream per 24 hours. For example, you reach the limit if you call <code>StartDeliveryStreamEncryption</code> 13 times and <code>StopDeliveryStreamEncryption</code> 12 times for the same delivery stream in a 24-hour period.</p>
    async fn start_delivery_stream_encryption(
        &self,
        input: StartDeliveryStreamEncryptionInput,
    ) -> Result<StartDeliveryStreamEncryptionOutput, RusotoError<StartDeliveryStreamEncryptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Firehose_20150804.StartDeliveryStreamEncryption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StartDeliveryStreamEncryptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StartDeliveryStreamEncryptionOutput, _>()
    }

    /// <p>Disables server-side encryption (SSE) for the delivery stream. </p> <p>This operation is asynchronous. It returns immediately. When you invoke it, Kinesis Data Firehose first sets the encryption status of the stream to <code>DISABLING</code>, and then to <code>DISABLED</code>. You can continue to read and write data to your stream while its status is <code>DISABLING</code>. It can take up to 5 seconds after the encryption status changes to <code>DISABLED</code> before all records written to the delivery stream are no longer subject to encryption. To find out whether a record or a batch of records was encrypted, check the response elements <a>PutRecordOutput$Encrypted</a> and <a>PutRecordBatchOutput$Encrypted</a>, respectively.</p> <p>To check the encryption state of a delivery stream, use <a>DescribeDeliveryStream</a>. </p> <p>If SSE is enabled using a customer managed CMK and then you invoke <code>StopDeliveryStreamEncryption</code>, Kinesis Data Firehose schedules the related KMS grant for retirement and then retires it after it ensures that it is finished delivering records to the destination.</p> <p>The <code>StartDeliveryStreamEncryption</code> and <code>StopDeliveryStreamEncryption</code> operations have a combined limit of 25 calls per delivery stream per 24 hours. For example, you reach the limit if you call <code>StartDeliveryStreamEncryption</code> 13 times and <code>StopDeliveryStreamEncryption</code> 12 times for the same delivery stream in a 24-hour period.</p>
    async fn stop_delivery_stream_encryption(
        &self,
        input: StopDeliveryStreamEncryptionInput,
    ) -> Result<StopDeliveryStreamEncryptionOutput, RusotoError<StopDeliveryStreamEncryptionError>>
    {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header(
            "x-amz-target",
            "Firehose_20150804.StopDeliveryStreamEncryption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, StopDeliveryStreamEncryptionError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response)
            .deserialize::<StopDeliveryStreamEncryptionOutput, _>()
    }

    /// <p>Adds or updates tags for the specified delivery stream. A tag is a key-value pair that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Tags are metadata. For example, you can add friendly names and descriptions or other types of information that can help you distinguish the delivery stream. For more information about tags, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management User Guide</i>. </p> <p>Each delivery stream can have up to 50 tags. </p> <p>This operation has a limit of five transactions per second per account. </p>
    async fn tag_delivery_stream(
        &self,
        input: TagDeliveryStreamInput,
    ) -> Result<TagDeliveryStreamOutput, RusotoError<TagDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.TagDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, TagDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<TagDeliveryStreamOutput, _>()
    }

    /// <p>Removes tags from the specified delivery stream. Removed tags are deleted, and you can't recover them after this operation successfully completes.</p> <p>If you specify a tag that doesn't exist, the operation ignores it.</p> <p>This operation has a limit of five transactions per second per account. </p>
    async fn untag_delivery_stream(
        &self,
        input: UntagDeliveryStreamInput,
    ) -> Result<UntagDeliveryStreamOutput, RusotoError<UntagDeliveryStreamError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.UntagDeliveryStream");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UntagDeliveryStreamError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UntagDeliveryStreamOutput, _>()
    }

    /// <p>Updates the specified destination of the specified delivery stream.</p> <p>Use this operation to change the destination type (for example, to replace the Amazon S3 destination with Amazon Redshift) or change the parameters associated with a destination (for example, to change the bucket name of the Amazon S3 destination). The update might not occur immediately. The target delivery stream remains active while the configurations are updated, so data writes to the delivery stream can continue during this process. The updated configurations are usually effective within a few minutes.</p> <p>Switching between Amazon ES and other services is not supported. For an Amazon ES destination, you can only update to another Amazon ES destination.</p> <p>If the destination type is the same, Kinesis Data Firehose merges the configuration parameters specified with the destination configuration that already exists on the delivery stream. If any of the parameters are not specified in the call, the existing values are retained. For example, in the Amazon S3 destination, if <a>EncryptionConfiguration</a> is not specified, then the existing <code>EncryptionConfiguration</code> is maintained on the destination.</p> <p>If the destination type is not the same, for example, changing the destination from Amazon S3 to Amazon Redshift, Kinesis Data Firehose does not merge any parameters. In this case, all parameters must be specified.</p> <p>Kinesis Data Firehose uses <code>CurrentDeliveryStreamVersionId</code> to avoid race conditions and conflicting merges. This is a required field, and the service updates the configuration only if the existing configuration has a version ID that matches. After the update is applied successfully, the version ID is updated, and can be retrieved using <a>DescribeDeliveryStream</a>. Use the new version ID to set <code>CurrentDeliveryStreamVersionId</code> in the next call.</p>
    async fn update_destination(
        &self,
        input: UpdateDestinationInput,
    ) -> Result<UpdateDestinationOutput, RusotoError<UpdateDestinationError>> {
        let mut request = self.new_signed_request("POST", "/");
        request.add_header("x-amz-target", "Firehose_20150804.UpdateDestination");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let response = self
            .sign_and_dispatch(request, UpdateDestinationError::from_response)
            .await?;
        let mut response = response;
        let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
        proto::json::ResponsePayload::new(&response).deserialize::<UpdateDestinationOutput, _>()
    }
}
