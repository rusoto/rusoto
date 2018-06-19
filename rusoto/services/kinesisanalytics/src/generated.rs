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
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationCloudWatchLoggingOptionRequest {
    /// <p>The Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Provides the CloudWatch log stream Amazon Resource Name (ARN) and the IAM role ARN. Note: To write application messages to CloudWatch, the IAM role that is used must have the <code>PutLogEvents</code> policy action enabled.</p>
    #[serde(rename = "CloudWatchLoggingOption")]
    pub cloud_watch_logging_option: CloudWatchLoggingOption,
    /// <p>The version ID of the Kinesis Analytics application.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddApplicationCloudWatchLoggingOptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationInputProcessingConfigurationRequest {
    /// <p>Name of the application to which you want to add the input processing configuration.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application to which you want to add the input processing configuration. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration to add the input processing configuration to. You can get a list of the input IDs for an application using the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "InputId")]
    pub input_id: String,
    /// <p>The <a>InputProcessingConfiguration</a> to add to the application.</p>
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: InputProcessingConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddApplicationInputProcessingConfigurationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationInputRequest {
    /// <p>Name of your existing Amazon Kinesis Analytics application to which you want to add the streaming source.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Current version of your Amazon Kinesis Analytics application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The <a>Input</a> to add.</p>
    #[serde(rename = "Input")]
    pub input: Input,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddApplicationInputResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationOutputRequest {
    /// <p>Name of the application to which you want to add the output configuration.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application to which you want to add the output configuration. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>An array of objects, each describing one output configuration. In the output configuration, you specify the name of an in-application stream, a destination (that is, an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an Amazon Lambda function), and record the formation to use when writing to the destination.</p>
    #[serde(rename = "Output")]
    pub output: Output,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddApplicationOutputResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationReferenceDataSourceRequest {
    /// <p>Name of an existing application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application for which you are adding the reference data source. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The reference data source can be an object in your Amazon S3 bucket. Amazon Kinesis Analytics reads the object and copies the data into the in-application table that is created. You provide an S3 bucket, object key name, and the resulting in-application table that is created. You must also provide an IAM role with the necessary permissions that Amazon Kinesis Analytics can assume to read the object from your S3 bucket on your behalf.</p>
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddApplicationReferenceDataSourceResponse {}

/// <p>Provides a description of the application, including the application Amazon Resource Name (ARN), status, latest version, and input and output configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApplicationDetail {
    /// <p>ARN of the application.</p>
    #[serde(rename = "ApplicationARN")]
    pub application_arn: String,
    /// <p>Returns the application code that you provided to perform data analysis on any of the in-application streams in your application.</p>
    #[serde(rename = "ApplicationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code: Option<String>,
    /// <p>Description of the application.</p>
    #[serde(rename = "ApplicationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    /// <p>Name of the application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Status of the application.</p>
    #[serde(rename = "ApplicationStatus")]
    pub application_status: String,
    /// <p>Provides the current application version.</p>
    #[serde(rename = "ApplicationVersionId")]
    pub application_version_id: i64,
    /// <p>Describes the CloudWatch log streams that are configured to receive application messages. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>. </p>
    #[serde(rename = "CloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    /// <p>Time stamp when the application version was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>Describes the application input configuration. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
    #[serde(rename = "InputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
    /// <p>Time stamp when the application was last updated.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>Describes the application output configuration. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>. </p>
    #[serde(rename = "OutputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
    /// <p>Describes reference data sources configured for the application. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
    #[serde(rename = "ReferenceDataSourceDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

/// <p>Provides application summary information, including the application Amazon Resource Name (ARN), name, and status.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ApplicationSummary {
    /// <p>ARN of the application.</p>
    #[serde(rename = "ApplicationARN")]
    pub application_arn: String,
    /// <p>Name of the application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Status of the application.</p>
    #[serde(rename = "ApplicationStatus")]
    pub application_status: String,
}

/// <p>Describes updates to apply to an existing Amazon Kinesis Analytics application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ApplicationUpdate {
    /// <p>Describes application code updates.</p>
    #[serde(rename = "ApplicationCodeUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code_update: Option<String>,
    /// <p>Describes application CloudWatch logging option updates.</p>
    #[serde(rename = "CloudWatchLoggingOptionUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_updates: Option<Vec<CloudWatchLoggingOptionUpdate>>,
    /// <p>Describes application input configuration updates.</p>
    #[serde(rename = "InputUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_updates: Option<Vec<InputUpdate>>,
    /// <p>Describes application output configuration updates.</p>
    #[serde(rename = "OutputUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_updates: Option<Vec<OutputUpdate>>,
    /// <p>Describes application reference data source updates.</p>
    #[serde(rename = "ReferenceDataSourceUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_updates: Option<Vec<ReferenceDataSourceUpdate>>,
}

/// <p>Provides additional mapping information when the record format uses delimiters, such as CSV. For example, the following sample records use CSV format, where the records use the <i>'\n'</i> as the row delimiter and a comma (",") as the column delimiter: </p> <p> <code>"name1", "address1" </code> </p> <p> <code>"name2, "address2"</code> </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CSVMappingParameters {
    /// <p>Column delimiter. For example, in a CSV format, a comma (",") is the typical column delimiter.</p>
    #[serde(rename = "RecordColumnDelimiter")]
    pub record_column_delimiter: String,
    /// <p>Row delimiter. For example, in a CSV format, <i>'\n'</i> is the typical row delimiter.</p>
    #[serde(rename = "RecordRowDelimiter")]
    pub record_row_delimiter: String,
}

/// <p>Provides a description of CloudWatch logging options, including the log stream Amazon Resource Name (ARN) and the role ARN.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CloudWatchLoggingOption {
    /// <p>ARN of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "LogStreamARN")]
    pub log_stream_arn: String,
    /// <p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role that is used must have the <code>PutLogEvents</code> policy action enabled.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Description of the CloudWatch logging option.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CloudWatchLoggingOptionDescription {
    /// <p>ID of the CloudWatch logging option description.</p>
    #[serde(rename = "CloudWatchLoggingOptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_id: Option<String>,
    /// <p>ARN of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "LogStreamARN")]
    pub log_stream_arn: String,
    /// <p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role used must have the <code>PutLogEvents</code> policy action enabled.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Describes CloudWatch logging option updates.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CloudWatchLoggingOptionUpdate {
    /// <p>ID of the CloudWatch logging option to update</p>
    #[serde(rename = "CloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    /// <p>ARN of the CloudWatch log to receive application messages.</p>
    #[serde(rename = "LogStreamARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_arn_update: Option<String>,
    /// <p>IAM ARN of the role to use to send application messages. Note: To write application messages to CloudWatch, the IAM role used must have the <code>PutLogEvents</code> policy action enabled.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>TBD</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateApplicationRequest {
    /// <p>One or more SQL statements that read input data, transform it, and generate output. For example, you can write a SQL statement that reads data from one in-application stream, generates a running average of the number of advertisement clicks by vendor, and insert resulting rows in another in-application stream using pumps. For more information about the typical pattern, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-app-code.html">Application Code</a>. </p> <p>You can provide such series of SQL statements, where output of one statement can be used as the input for the next statement. You store intermediate results by creating in-application streams and pumps.</p> <p>Note that the application code must create the streams with names specified in the <code>Outputs</code>. For example, if your <code>Outputs</code> defines output streams named <code>ExampleOutputStream1</code> and <code>ExampleOutputStream2</code>, then your application code must create these streams. </p>
    #[serde(rename = "ApplicationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_code: Option<String>,
    /// <p>Summary description of the application.</p>
    #[serde(rename = "ApplicationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_description: Option<String>,
    /// <p>Name of your Amazon Kinesis Analytics application (for example, <code>sample-app</code>).</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Use this parameter to configure a CloudWatch log stream to monitor application configuration errors. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<Vec<CloudWatchLoggingOption>>,
    /// <p>Use this parameter to configure the application input.</p> <p>You can configure your application to receive input from a single streaming source. In this configuration, you map this streaming source to an in-application stream that is created. Your application code can then query the in-application stream like a table (you can think of it as a constantly updating table).</p> <p>For the streaming source, you provide its Amazon Resource Name (ARN) and format of data on the stream (for example, JSON, CSV, etc.). You also must provide an IAM role that Amazon Kinesis Analytics can assume to read this stream on your behalf.</p> <p>To create the in-application stream, you need to specify a schema to transform your data into a schematized version used in SQL. In the schema, you provide the necessary mapping of the data elements in the streaming source to record columns in the in-app stream.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    /// <p>You can configure application output to write data from any of the in-application streams to up to three destinations.</p> <p>These destinations can be Amazon Kinesis streams, Amazon Kinesis Firehose delivery streams, Amazon Lambda destinations, or any combination of the three.</p> <p>In the configuration, you specify the in-application stream name, the destination stream or Lambda function Amazon Resource Name (ARN), and the format to use when writing data. You must also provide an IAM role that Amazon Kinesis Analytics can assume to write to the destination stream or Lambda function on your behalf.</p> <p>In the output configuration, you also provide the output stream or Lambda function ARN. For stream destinations, you provide the format of data in the stream (for example, JSON, CSV). You also must provide an IAM role that Amazon Kinesis Analytics can assume to write to the stream or Lambda function on your behalf.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
}

/// <p>TBD</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateApplicationResponse {
    /// <p>In response to your <code>CreateApplication</code> request, Amazon Kinesis Analytics returns a response with a summary of the application it created, including the application Amazon Resource Name (ARN), name, and status.</p>
    #[serde(rename = "ApplicationSummary")]
    pub application_summary: ApplicationSummary,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionRequest {
    /// <p>The Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <a>DescribeApplication</a> operation. </p>
    #[serde(rename = "CloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    /// <p>The version ID of the Kinesis Analytics application.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationCloudWatchLoggingOptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationInputProcessingConfigurationRequest {
    /// <p>The Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>The version ID of the Kinesis Analytics application.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "InputId")]
    pub input_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationInputProcessingConfigurationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationOutputRequest {
    /// <p>Amazon Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Amazon Kinesis Analytics application version. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the configuration to delete. Each output configuration that is added to the application, either when the application is created or later using the <a>AddApplicationOutput</a> operation, has a unique ID. You need to provide the ID to uniquely identify the output configuration that you want to delete from the application configuration. You can use the <a>DescribeApplication</a> operation to get the specific <code>OutputId</code>. </p>
    #[serde(rename = "OutputId")]
    pub output_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationOutputResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationReferenceDataSourceRequest {
    /// <p>Name of an existing application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application. You can use the <a>DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>ID of the reference data source. When you add a reference data source to your application using the <a>AddApplicationReferenceDataSource</a>, Amazon Kinesis Analytics assigns an ID. You can use the <a>DescribeApplication</a> operation to get the reference ID. </p>
    #[serde(rename = "ReferenceId")]
    pub reference_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationReferenceDataSourceResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationRequest {
    /// <p>Name of the Amazon Kinesis Analytics application to delete.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p> You can use the <code>DescribeApplication</code> operation to get this value. </p>
    #[serde(rename = "CreateTimestamp")]
    pub create_timestamp: f64,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeApplicationRequest {
    /// <p>Name of the application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeApplicationResponse {
    /// <p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p>
    #[serde(rename = "ApplicationDetail")]
    pub application_detail: ApplicationDetail,
}

/// <p>Describes the data format when records are written to the destination. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationSchema {
    /// <p>Specifies the format of the records on the output stream.</p>
    #[serde(rename = "RecordFormatType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DiscoverInputSchemaRequest {
    /// <p>The <a>InputProcessingConfiguration</a> to use to preprocess the records before discovering the schema of the records.</p>
    #[serde(rename = "InputProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    /// <p>Point at which you want Amazon Kinesis Analytics to start reading records from the specified streaming source discovery purposes.</p>
    #[serde(rename = "InputStartingPositionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    /// <p>Amazon Resource Name (ARN) of the streaming source.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>Specify this parameter to discover a schema from data in an S3 object.</p>
    #[serde(rename = "S3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DiscoverInputSchemaResponse {
    /// <p>Schema inferred from the streaming source. It identifies the format of the data in the streaming source and how each data element maps to corresponding columns in the in-application stream that you can create.</p>
    #[serde(rename = "InputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    /// <p>An array of elements, where each element corresponds to a row in a stream record (a stream record can have more than one row).</p>
    #[serde(rename = "ParsedInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parsed_input_records: Option<Vec<Vec<String>>>,
    /// <p>Stream data that was modified by the processor specified in the <code>InputProcessingConfiguration</code> parameter.</p>
    #[serde(rename = "ProcessedInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processed_input_records: Option<Vec<String>>,
    /// <p>Raw stream data that was sampled to infer the schema.</p>
    #[serde(rename = "RawInputRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_input_records: Option<Vec<String>>,
}

/// <p>When you configure the application input, you specify the streaming source, the in-application stream name that is created, and the mapping between the two. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Input {
    /// <p>Describes the number of in-application streams to create. </p> <p>Data from your source is routed to these in-application input streams.</p> <p> (see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>.</p>
    #[serde(rename = "InputParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    /// <p>The <a>InputProcessingConfiguration</a> for the input. An input processor transforms records as they are received from the stream, before the application's SQL code executes. Currently, the only input processing configuration available is <a>InputLambdaProcessor</a>.</p>
    #[serde(rename = "InputProcessingConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration: Option<InputProcessingConfiguration>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created.</p> <p>Also used to describe the format of the reference data source.</p>
    #[serde(rename = "InputSchema")]
    pub input_schema: SourceSchema,
    /// <p>If the streaming source is an Amazon Kinesis Firehose delivery stream, identifies the delivery stream's ARN and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p> <p>Note: Either <code>KinesisStreamsInput</code> or <code>KinesisFirehoseInput</code> is required.</p>
    #[serde(rename = "KinesisFirehoseInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input: Option<KinesisFirehoseInput>,
    /// <p>If the streaming source is an Amazon Kinesis stream, identifies the stream's Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p> <p>Note: Either <code>KinesisStreamsInput</code> or <code>KinesisFirehoseInput</code> is required.</p>
    #[serde(rename = "KinesisStreamsInput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input: Option<KinesisStreamsInput>,
    /// <p>Name prefix to use when creating an in-application stream. Suppose that you specify a prefix "MyInApplicationStream." Amazon Kinesis Analytics then creates one or more (as per the <code>InputParallelism</code> count you specified) in-application streams with names "MyInApplicationStream_001," "MyInApplicationStream_002," and so on. </p>
    #[serde(rename = "NamePrefix")]
    pub name_prefix: String,
}

/// <p>When you start your application, you provide this configuration, which identifies the input source and the point in the input source at which you want the application to start processing records.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputConfiguration {
    /// <p>Input source ID. You can get this ID by calling the <a>DescribeApplication</a> operation.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    #[serde(rename = "InputStartingPositionConfiguration")]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
}

/// <p>Describes the application input configuration. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InputDescription {
    /// <p>Returns the in-application stream names that are mapped to the stream source.</p>
    #[serde(rename = "InAppStreamNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_app_stream_names: Option<Vec<String>>,
    /// <p>Input ID associated with the application input. This is the ID that Amazon Kinesis Analytics assigns to each input configuration you add to your application. </p>
    #[serde(rename = "InputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_id: Option<String>,
    /// <p>Describes the configured parallelism (number of in-application streams mapped to the streaming source).</p>
    #[serde(rename = "InputParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    /// <p>The description of the preprocessor that executes on records in this input before the application's code is run.</p>
    #[serde(rename = "InputProcessingConfigurationDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_description: Option<InputProcessingConfigurationDescription>,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns in the in-application stream that is being created. </p>
    #[serde(rename = "InputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<SourceSchema>,
    /// <p>Point at which the application is configured to read from the input stream.</p>
    #[serde(rename = "InputStartingPositionConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position_configuration: Option<InputStartingPositionConfiguration>,
    /// <p>If an Amazon Kinesis Firehose delivery stream is configured as a streaming source, provides the delivery stream's ARN and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>
    #[serde(rename = "KinesisFirehoseInputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_description: Option<KinesisFirehoseInputDescription>,
    /// <p>If an Amazon Kinesis stream is configured as streaming source, provides Amazon Kinesis stream's Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>
    #[serde(rename = "KinesisStreamsInputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_description: Option<KinesisStreamsInputDescription>,
    /// <p>In-application name prefix.</p>
    #[serde(rename = "NamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
}

/// <p>An object that contains the Amazon Resource Name (ARN) of the <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a> function that is used to preprocess records in the stream, and the ARN of the IAM role that is used to access the AWS Lambda function. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputLambdaProcessor {
    /// <p>The ARN of the <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a> function that operates on records in the stream.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The ARN of the IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>An object that contains the Amazon Resource Name (ARN) of the <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a> function that is used to preprocess records in the stream, and the ARN of the IAM role that is used to access the AWS Lambda expression.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InputLambdaProcessorDescription {
    /// <p>The ARN of the <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a> function that is used to preprocess the records in the stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The ARN of the IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Represents an update to the <a>InputLambdaProcessor</a> that is used to preprocess the records in the stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputLambdaProcessorUpdate {
    /// <p>The Amazon Resource Name (ARN) of the new <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a> function that is used to preprocess the records in the stream.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>The ARN of the new IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>Describes the number of in-application streams to create for a given streaming source. For information about parallelism, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputParallelism {
    /// <p>Number of in-application streams to create. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// <p>Provides updates to the parallelism count.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputParallelismUpdate {
    /// <p>Number of in-application streams to create for the specified streaming source.</p>
    #[serde(rename = "CountUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_update: Option<i64>,
}

/// <p>Provides a description of a processor that is used to preprocess the records in the stream before being processed by your application code. Currently, the only input processor available is <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputProcessingConfiguration {
    /// <p>The <a>InputLambdaProcessor</a> that is used to preprocess the records in the stream before being processed by your application code.</p>
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: InputLambdaProcessor,
}

/// <p>Provides configuration information about an input processor. Currently, the only input processor available is <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InputProcessingConfigurationDescription {
    /// <p>Provides configuration information about the associated <a>InputLambdaProcessorDescription</a>.</p>
    #[serde(rename = "InputLambdaProcessorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_lambda_processor_description: Option<InputLambdaProcessorDescription>,
}

/// <p>Describes updates to an <a>InputProcessingConfiguration</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputProcessingConfigurationUpdate {
    /// <p>Provides update information for an <a>InputLambdaProcessor</a>.</p>
    #[serde(rename = "InputLambdaProcessorUpdate")]
    pub input_lambda_processor_update: InputLambdaProcessorUpdate,
}

/// <p>Describes updates for the application's input schema.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputSchemaUpdate {
    /// <p>A list of <code>RecordColumn</code> objects. Each object describes the mapping of the streaming source element to the corresponding column in the in-application stream. </p>
    #[serde(rename = "RecordColumnUpdates")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_column_updates: Option<Vec<RecordColumn>>,
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    #[serde(rename = "RecordEncodingUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding_update: Option<String>,
    /// <p>Specifies the format of the records on the streaming source.</p>
    #[serde(rename = "RecordFormatUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_format_update: Option<RecordFormat>,
}

/// <p>Describes the point at which the application reads from the streaming source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStartingPositionConfiguration {
    /// <p><p>The starting position on the stream.</p> <ul> <li> <p> <code>NOW</code> - Start reading just after the most recent record in the stream, start at the request time stamp that the customer issued.</p> </li> <li> <p> <code>TRIM<em>HORIZON</code> - Start reading at the last untrimmed record in the stream, which is the oldest record available in the stream. This option is not available for an Amazon Kinesis Firehose delivery stream.</p> </li> <li> <p> <code>LAST</em>STOPPED_POINT</code> - Resume reading from where the application last stopped reading.</p> </li> </ul></p>
    #[serde(rename = "InputStartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_starting_position: Option<String>,
}

/// <p>Describes updates to a specific input configuration (identified by the <code>InputId</code> of an application). </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputUpdate {
    /// <p>Input ID of the application input to be updated.</p>
    #[serde(rename = "InputId")]
    pub input_id: String,
    /// <p>Describes the parallelism updates (the number in-application streams Amazon Kinesis Analytics creates for the specific streaming source).</p>
    #[serde(rename = "InputParallelismUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism_update: Option<InputParallelismUpdate>,
    /// <p>Describes updates for an input processing configuration.</p>
    #[serde(rename = "InputProcessingConfigurationUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_processing_configuration_update: Option<InputProcessingConfigurationUpdate>,
    /// <p>Describes the data format on the streaming source, and how record elements on the streaming source map to columns of the in-application stream that is created.</p>
    #[serde(rename = "InputSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema_update: Option<InputSchemaUpdate>,
    /// <p>If an Amazon Kinesis Firehose delivery stream is the streaming source to be updated, provides an updated stream ARN and IAM role ARN.</p>
    #[serde(rename = "KinesisFirehoseInputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_input_update: Option<KinesisFirehoseInputUpdate>,
    /// <p>If an Amazon Kinesis stream is the streaming source to be updated, provides an updated stream Amazon Resource Name (ARN) and IAM role ARN.</p>
    #[serde(rename = "KinesisStreamsInputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_input_update: Option<KinesisStreamsInputUpdate>,
    /// <p>Name prefix for in-application streams that Amazon Kinesis Analytics creates for the specific streaming source.</p>
    #[serde(rename = "NamePrefixUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix_update: Option<String>,
}

/// <p>Provides additional mapping information when JSON is the record format on the streaming source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JSONMappingParameters {
    /// <p>Path to the top-level parent that contains the records.</p>
    #[serde(rename = "RecordRowPath")]
    pub record_row_path: String,
}

/// <p> Identifies an Amazon Kinesis Firehose delivery stream as the streaming source. You provide the delivery stream's Amazon Resource Name (ARN) and an IAM role ARN that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisFirehoseInput {
    /// <p>ARN of the input delivery stream.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to make sure the role has necessary permissions to access the stream.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> Describes the Amazon Kinesis Firehose delivery stream that is configured as the streaming source in the application input configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KinesisFirehoseInputDescription {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics assumes to access the stream.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When updating application input configuration, provides information about an Amazon Kinesis Firehose delivery stream as the streaming source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisFirehoseInputUpdate {
    /// <p>Amazon Resource Name (ARN) of the input Amazon Kinesis Firehose delivery stream to read.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant necessary permissions to this role.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>When configuring application output, identifies an Amazon Kinesis Firehose delivery stream as the destination. You provide the stream Amazon Resource Name (ARN) and an IAM role that enables Amazon Kinesis Analytics to write to the stream on your behalf.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisFirehoseOutput {
    /// <p>ARN of the destination Amazon Kinesis Firehose delivery stream to write to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> For an application output, describes the Amazon Kinesis Firehose delivery stream configured as its destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KinesisFirehoseOutputDescription {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p> When updating an output configuration using the <a>UpdateApplication</a> operation, provides information about an Amazon Kinesis Firehose delivery stream configured as the destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisFirehoseOutputUpdate {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream to write to.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant necessary permissions to this role.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p> Identifies an Amazon Kinesis stream as the streaming source. You provide the stream's Amazon Resource Name (ARN) and an IAM role ARN that enables Amazon Kinesis Analytics to access the stream on your behalf.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisStreamsInput {
    /// <p>ARN of the input Amazon Kinesis stream to read.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> Describes the Amazon Kinesis stream that is configured as the streaming source in the application input configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KinesisStreamsInputDescription {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When updating application input configuration, provides information about an Amazon Kinesis stream as the streaming source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisStreamsInputUpdate {
    /// <p>Amazon Resource Name (ARN) of the input Amazon Kinesis stream to read.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>When configuring application output, identifies an Amazon Kinesis stream as the destination. You provide the stream Amazon Resource Name (ARN) and also an IAM role ARN that Amazon Kinesis Analytics can use to write to the stream on your behalf.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisStreamsOutput {
    /// <p>ARN of the destination Amazon Kinesis stream to write to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination stream on your behalf. You need to grant the necessary permissions to this role.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> For an application output, describes the Amazon Kinesis stream configured as its destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct KinesisStreamsOutputDescription {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p> When updating an output configuration using the <a>UpdateApplication</a> operation, provides information about an Amazon Kinesis stream configured as the destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisStreamsOutputUpdate {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis stream where you want to write the output.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>When configuring application output, identifies an AWS Lambda function as the destination. You provide the function Amazon Resource Name (ARN) and also an IAM role ARN that Amazon Kinesis Analytics can use to write to the function on your behalf. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LambdaOutput {
    /// <p>Amazon Resource Name (ARN) of the destination Lambda function to write to.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination function on your behalf. You need to grant the necessary permissions to this role. </p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>For an application output, describes the AWS Lambda function configured as its destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct LambdaOutputDescription {
    /// <p>Amazon Resource Name (ARN) of the destination Lambda function.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination function.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>When updating an output configuration using the <a>UpdateApplication</a> operation, provides information about an AWS Lambda function configured as the destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LambdaOutputUpdate {
    /// <p>Amazon Resource Name (ARN) of the destination Lambda function.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination function on your behalf. You need to grant the necessary permissions to this role. </p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListApplicationsRequest {
    /// <p>Name of the application to start the list with. When using pagination to retrieve the list, you don't need to specify this parameter in the first request. However, in subsequent requests, you add the last application name from the previous response to get the next page of applications.</p>
    #[serde(rename = "ExclusiveStartApplicationName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_start_application_name: Option<String>,
    /// <p>Maximum number of applications to list.</p>
    #[serde(rename = "Limit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListApplicationsResponse {
    /// <p>List of <code>ApplicationSummary</code> objects. </p>
    #[serde(rename = "ApplicationSummaries")]
    pub application_summaries: Vec<ApplicationSummary>,
    /// <p>Returns true if there are more applications to retrieve.</p>
    #[serde(rename = "HasMoreApplications")]
    pub has_more_applications: bool,
}

/// <p>When configuring application input at the time of creating or updating an application, provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MappingParameters {
    /// <p>Provides additional mapping information when the record format uses delimiters (for example, CSV).</p>
    #[serde(rename = "CSVMappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv_mapping_parameters: Option<CSVMappingParameters>,
    /// <p>Provides additional mapping information when JSON is the record format on the streaming source.</p>
    #[serde(rename = "JSONMappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_mapping_parameters: Option<JSONMappingParameters>,
}

/// <p> Describes application output configuration in which you identify an in-application stream and a destination where you want the in-application stream data to be written. The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream. </p> <p/> <p>For limits on how many destinations an application can write and other limitations, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Output {
    /// <p>Describes the data format when records are written to the destination. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>.</p>
    #[serde(rename = "DestinationSchema")]
    pub destination_schema: DestinationSchema,
    /// <p>Identifies an Amazon Kinesis Firehose delivery stream as the destination.</p>
    #[serde(rename = "KinesisFirehoseOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output: Option<KinesisFirehoseOutput>,
    /// <p>Identifies an Amazon Kinesis stream as the destination.</p>
    #[serde(rename = "KinesisStreamsOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output: Option<KinesisStreamsOutput>,
    /// <p>Identifies an AWS Lambda function as the destination.</p>
    #[serde(rename = "LambdaOutput")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output: Option<LambdaOutput>,
    /// <p>Name of the in-application stream.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p>Describes the application output configuration, which includes the in-application stream name and the destination where the stream data is written. The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct OutputDescription {
    /// <p>Data format used for writing data to the destination.</p>
    #[serde(rename = "DestinationSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema: Option<DestinationSchema>,
    /// <p>Describes the Amazon Kinesis Firehose delivery stream configured as the destination where output is written.</p>
    #[serde(rename = "KinesisFirehoseOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_description: Option<KinesisFirehoseOutputDescription>,
    /// <p>Describes Amazon Kinesis stream configured as the destination where output is written.</p>
    #[serde(rename = "KinesisStreamsOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_description: Option<KinesisStreamsOutputDescription>,
    /// <p>Describes the AWS Lambda function configured as the destination where output is written.</p>
    #[serde(rename = "LambdaOutputDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_description: Option<LambdaOutputDescription>,
    /// <p>Name of the in-application stream configured as output.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier for the output configuration.</p>
    #[serde(rename = "OutputId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_id: Option<String>,
}

/// <p> Describes updates to the output configuration identified by the <code>OutputId</code>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct OutputUpdate {
    /// <p>Describes the data format when records are written to the destination. For more information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>.</p>
    #[serde(rename = "DestinationSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_schema_update: Option<DestinationSchema>,
    /// <p>Describes an Amazon Kinesis Firehose delivery stream as the destination for the output.</p>
    #[serde(rename = "KinesisFirehoseOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_firehose_output_update: Option<KinesisFirehoseOutputUpdate>,
    /// <p>Describes an Amazon Kinesis stream as the destination for the output.</p>
    #[serde(rename = "KinesisStreamsOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_streams_output_update: Option<KinesisStreamsOutputUpdate>,
    /// <p>Describes an AWS Lambda function as the destination for the output.</p>
    #[serde(rename = "LambdaOutputUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_output_update: Option<LambdaOutputUpdate>,
    /// <p>If you want to specify a different in-application stream for this output configuration, use this field to specify the new in-application stream name.</p>
    #[serde(rename = "NameUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_update: Option<String>,
    /// <p>Identifies the specific output configuration that you want to update.</p>
    #[serde(rename = "OutputId")]
    pub output_id: String,
}

/// <p>Describes the mapping of each data element in the streaming source to the corresponding column in the in-application stream.</p> <p>Also used to describe the format of the reference data source.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordColumn {
    /// <p>Reference to the data element in the streaming input of the reference data source.</p>
    #[serde(rename = "Mapping")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<String>,
    /// <p>Name of the column created in the in-application input stream or reference table.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Type of column created in the in-application input stream or reference table.</p>
    #[serde(rename = "SqlType")]
    pub sql_type: String,
}

/// <p> Describes the record format and relevant mapping information that should be applied to schematize the records on the stream. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordFormat {
    /// <p>When configuring application input at the time of creating or updating an application, provides additional mapping information specific to the record format (such as JSON, CSV, or record fields delimited by some delimiter) on the streaming source.</p>
    #[serde(rename = "MappingParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping_parameters: Option<MappingParameters>,
    /// <p>The type of record format.</p>
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,
}

/// <p>Describes the reference data source by providing the source information (S3 bucket name and object key name), the resulting in-application table name that is created, and the necessary schema to map the data elements in the Amazon S3 object to the in-application table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReferenceDataSource {
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[serde(rename = "ReferenceSchema")]
    pub reference_schema: SourceSchema,
    /// <p>Identifies the S3 bucket and object that contains the reference data. Also identifies the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf. An Amazon Kinesis Analytics application loads reference data only once. If the data changes, you call the <a>UpdateApplication</a> operation to trigger reloading of data into your application. </p>
    #[serde(rename = "S3ReferenceDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    /// <p>Name of the in-application table to create.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Describes the reference data source configured for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ReferenceDataSourceDescription {
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a>AddApplicationReferenceDataSource</a> operation.</p>
    #[serde(rename = "ReferenceId")]
    pub reference_id: String,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
    #[serde(rename = "ReferenceSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema: Option<SourceSchema>,
    /// <p>Provides the S3 bucket name, the object key name that contains the reference data. It also provides the Amazon Resource Name (ARN) of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application reference table.</p>
    #[serde(rename = "S3ReferenceDataSourceDescription")]
    pub s3_reference_data_source_description: S3ReferenceDataSourceDescription,
    /// <p>The in-application table name created by the specific reference data source configuration.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>When you update a reference data source configuration for an application, this object provides all the updated values (such as the source bucket name and object key name), the in-application table name that is created, and updated mapping information that maps the data in the Amazon S3 object to the in-application reference table that is created.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ReferenceDataSourceUpdate {
    /// <p>ID of the reference data source being updated. You can use the <a>DescribeApplication</a> operation to get this value.</p>
    #[serde(rename = "ReferenceId")]
    pub reference_id: String,
    /// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream. </p>
    #[serde(rename = "ReferenceSchemaUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_schema_update: Option<SourceSchema>,
    /// <p>Describes the S3 bucket name, object key name, and IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf and populate the in-application reference table.</p>
    #[serde(rename = "S3ReferenceDataSourceUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source_update: Option<S3ReferenceDataSourceUpdate>,
    /// <p>In-application table name that is created by this update.</p>
    #[serde(rename = "TableNameUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name_update: Option<String>,
}

/// <p>Provides a description of an Amazon S3 data source, including the Amazon Resource Name (ARN) of the S3 bucket, the ARN of the IAM role that is used to access the bucket, and the name of the S3 object that contains the data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3Configuration {
    /// <p>ARN of the S3 bucket that contains the data.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>The name of the object that contains the data.</p>
    #[serde(rename = "FileKey")]
    pub file_key: String,
    /// <p>IAM ARN of the role used to access the data.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>Identifies the S3 bucket and object that contains the reference data. Also identifies the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf.</p> <p>An Amazon Kinesis Analytics application loads reference data only once. If the data changes, you call the <a>UpdateApplication</a> operation to trigger reloading of data into your application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3ReferenceDataSource {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>Object key name containing reference data.</p>
    #[serde(rename = "FileKey")]
    pub file_key: String,
    /// <p>ARN of the IAM role that the service can assume to read data on your behalf. This role must have permission for the <code>s3:GetObject</code> action on the object and trust policy that allows Amazon Kinesis Analytics service principal to assume this role.</p>
    #[serde(rename = "ReferenceRoleARN")]
    pub reference_role_arn: String,
}

/// <p>Provides the bucket name and object key name that stores the reference data.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct S3ReferenceDataSourceDescription {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "BucketARN")]
    pub bucket_arn: String,
    /// <p>Amazon S3 object key name.</p>
    #[serde(rename = "FileKey")]
    pub file_key: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf to populate the in-application reference table.</p>
    #[serde(rename = "ReferenceRoleARN")]
    pub reference_role_arn: String,
}

/// <p>Describes the S3 bucket name, object key name, and IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object on your behalf and populate the in-application reference table.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct S3ReferenceDataSourceUpdate {
    /// <p>Amazon Resource Name (ARN) of the S3 bucket.</p>
    #[serde(rename = "BucketARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_arn_update: Option<String>,
    /// <p>Object key name.</p>
    #[serde(rename = "FileKeyUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_key_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to read the Amazon S3 object and populate the in-application.</p>
    #[serde(rename = "ReferenceRoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_role_arn_update: Option<String>,
}

/// <p>Describes the format of the data in the streaming source, and how each data element maps to corresponding columns created in the in-application stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceSchema {
    /// <p>A list of <code>RecordColumn</code> objects.</p>
    #[serde(rename = "RecordColumns")]
    pub record_columns: Vec<RecordColumn>,
    /// <p>Specifies the encoding of the records in the streaming source. For example, UTF-8.</p>
    #[serde(rename = "RecordEncoding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_encoding: Option<String>,
    /// <p>Specifies the format of the records on the streaming source.</p>
    #[serde(rename = "RecordFormat")]
    pub record_format: RecordFormat,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartApplicationRequest {
    /// <p>Name of the application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Identifies the specific input, by ID, that the application starts consuming. Amazon Kinesis Analytics starts reading the streaming source associated with the input. You can also specify where in the streaming source you want Amazon Kinesis Analytics to start reading.</p>
    #[serde(rename = "InputConfigurations")]
    pub input_configurations: Vec<InputConfiguration>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartApplicationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopApplicationRequest {
    /// <p>Name of the running application to stop.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopApplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationRequest {
    /// <p>Name of the Amazon Kinesis Analytics application to update.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Describes application updates.</p>
    #[serde(rename = "ApplicationUpdate")]
    pub application_update: ApplicationUpdate,
    /// <p>The current application version ID. You can use the <a>DescribeApplication</a> operation to get this value.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct UpdateApplicationResponse {}

/// Errors returned by AddApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum AddApplicationCloudWatchLoggingOptionError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl AddApplicationCloudWatchLoggingOptionError {
    pub fn from_body(body: &str) -> AddApplicationCloudWatchLoggingOptionError {
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
                        AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        AddApplicationCloudWatchLoggingOptionError::InvalidArgument(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        AddApplicationCloudWatchLoggingOptionError::ResourceInUse(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        AddApplicationCloudWatchLoggingOptionError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => AddApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: serde_json::error::Error) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: CredentialsError) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: HttpDispatchError) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationCloudWatchLoggingOptionError {
    fn from(err: io::Error) -> AddApplicationCloudWatchLoggingOptionError {
        AddApplicationCloudWatchLoggingOptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationCloudWatchLoggingOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationCloudWatchLoggingOptionError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::Validation(ref cause) => cause,
            AddApplicationCloudWatchLoggingOptionError::Credentials(ref err) => err.description(),
            AddApplicationCloudWatchLoggingOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationCloudWatchLoggingOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationInput
#[derive(Debug, PartialEq)]
pub enum AddApplicationInputError {
    /// <p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl AddApplicationInputError {
    pub fn from_body(body: &str) -> AddApplicationInputError {
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
                    "CodeValidationException" => {
                        AddApplicationInputError::CodeValidation(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        AddApplicationInputError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidArgumentException" => {
                        AddApplicationInputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        AddApplicationInputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationInputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddApplicationInputError::Validation(error_message.to_string())
                    }
                    _ => AddApplicationInputError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationInputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationInputError {
    fn from(err: serde_json::error::Error) -> AddApplicationInputError {
        AddApplicationInputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationInputError {
    fn from(err: CredentialsError) -> AddApplicationInputError {
        AddApplicationInputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationInputError {
    fn from(err: HttpDispatchError) -> AddApplicationInputError {
        AddApplicationInputError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationInputError {
    fn from(err: io::Error) -> AddApplicationInputError {
        AddApplicationInputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationInputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationInputError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationInputError::CodeValidation(ref cause) => cause,
            AddApplicationInputError::ConcurrentModification(ref cause) => cause,
            AddApplicationInputError::InvalidArgument(ref cause) => cause,
            AddApplicationInputError::ResourceInUse(ref cause) => cause,
            AddApplicationInputError::ResourceNotFound(ref cause) => cause,
            AddApplicationInputError::Validation(ref cause) => cause,
            AddApplicationInputError::Credentials(ref err) => err.description(),
            AddApplicationInputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationInputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationInputProcessingConfiguration
#[derive(Debug, PartialEq)]
pub enum AddApplicationInputProcessingConfigurationError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl AddApplicationInputProcessingConfigurationError {
    pub fn from_body(body: &str) -> AddApplicationInputProcessingConfigurationError {
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
                        AddApplicationInputProcessingConfigurationError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        AddApplicationInputProcessingConfigurationError::InvalidArgument(
                            String::from(error_message),
                        )
                    }
                    "ResourceInUseException" => {
                        AddApplicationInputProcessingConfigurationError::ResourceInUse(
                            String::from(error_message),
                        )
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationInputProcessingConfigurationError::ResourceNotFound(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        AddApplicationInputProcessingConfigurationError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => {
                        AddApplicationInputProcessingConfigurationError::Unknown(String::from(body))
                    }
                }
            }
            Err(_) => AddApplicationInputProcessingConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationInputProcessingConfigurationError {
    fn from(err: serde_json::error::Error) -> AddApplicationInputProcessingConfigurationError {
        AddApplicationInputProcessingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationInputProcessingConfigurationError {
    fn from(err: CredentialsError) -> AddApplicationInputProcessingConfigurationError {
        AddApplicationInputProcessingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationInputProcessingConfigurationError {
    fn from(err: HttpDispatchError) -> AddApplicationInputProcessingConfigurationError {
        AddApplicationInputProcessingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationInputProcessingConfigurationError {
    fn from(err: io::Error) -> AddApplicationInputProcessingConfigurationError {
        AddApplicationInputProcessingConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationInputProcessingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationInputProcessingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationInputProcessingConfigurationError::ConcurrentModification(ref cause) => {
                cause
            }
            AddApplicationInputProcessingConfigurationError::InvalidArgument(ref cause) => cause,
            AddApplicationInputProcessingConfigurationError::ResourceInUse(ref cause) => cause,
            AddApplicationInputProcessingConfigurationError::ResourceNotFound(ref cause) => cause,
            AddApplicationInputProcessingConfigurationError::Validation(ref cause) => cause,
            AddApplicationInputProcessingConfigurationError::Credentials(ref err) => {
                err.description()
            }
            AddApplicationInputProcessingConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationInputProcessingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationOutput
#[derive(Debug, PartialEq)]
pub enum AddApplicationOutputError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl AddApplicationOutputError {
    pub fn from_body(body: &str) -> AddApplicationOutputError {
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
                        AddApplicationOutputError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidArgumentException" => {
                        AddApplicationOutputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        AddApplicationOutputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationOutputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddApplicationOutputError::Validation(error_message.to_string())
                    }
                    _ => AddApplicationOutputError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationOutputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationOutputError {
    fn from(err: serde_json::error::Error) -> AddApplicationOutputError {
        AddApplicationOutputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationOutputError {
    fn from(err: CredentialsError) -> AddApplicationOutputError {
        AddApplicationOutputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationOutputError {
    fn from(err: HttpDispatchError) -> AddApplicationOutputError {
        AddApplicationOutputError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationOutputError {
    fn from(err: io::Error) -> AddApplicationOutputError {
        AddApplicationOutputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationOutputError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationOutputError::ConcurrentModification(ref cause) => cause,
            AddApplicationOutputError::InvalidArgument(ref cause) => cause,
            AddApplicationOutputError::ResourceInUse(ref cause) => cause,
            AddApplicationOutputError::ResourceNotFound(ref cause) => cause,
            AddApplicationOutputError::Validation(ref cause) => cause,
            AddApplicationOutputError::Credentials(ref err) => err.description(),
            AddApplicationOutputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationOutputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by AddApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum AddApplicationReferenceDataSourceError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl AddApplicationReferenceDataSourceError {
    pub fn from_body(body: &str) -> AddApplicationReferenceDataSourceError {
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
                        AddApplicationReferenceDataSourceError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        AddApplicationReferenceDataSourceError::InvalidArgument(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        AddApplicationReferenceDataSourceError::ResourceInUse(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        AddApplicationReferenceDataSourceError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => AddApplicationReferenceDataSourceError::Validation(
                        error_message.to_string(),
                    ),
                    _ => AddApplicationReferenceDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddApplicationReferenceDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddApplicationReferenceDataSourceError {
    fn from(err: serde_json::error::Error) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddApplicationReferenceDataSourceError {
    fn from(err: CredentialsError) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddApplicationReferenceDataSourceError {
    fn from(err: HttpDispatchError) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddApplicationReferenceDataSourceError {
    fn from(err: io::Error) -> AddApplicationReferenceDataSourceError {
        AddApplicationReferenceDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddApplicationReferenceDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddApplicationReferenceDataSourceError {
    fn description(&self) -> &str {
        match *self {
            AddApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => cause,
            AddApplicationReferenceDataSourceError::InvalidArgument(ref cause) => cause,
            AddApplicationReferenceDataSourceError::ResourceInUse(ref cause) => cause,
            AddApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => cause,
            AddApplicationReferenceDataSourceError::Validation(ref cause) => cause,
            AddApplicationReferenceDataSourceError::Credentials(ref err) => err.description(),
            AddApplicationReferenceDataSourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddApplicationReferenceDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Exceeded the number of applications allowed.</p>
    LimitExceeded(String),
    /// <p>Application is not available for this operation.</p>
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

impl CreateApplicationError {
    pub fn from_body(body: &str) -> CreateApplicationError {
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
                    "CodeValidationException" => {
                        CreateApplicationError::CodeValidation(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        CreateApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        CreateApplicationError::LimitExceeded(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        CreateApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateApplicationError::Validation(error_message.to_string())
                    }
                    _ => CreateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateApplicationError {
    fn from(err: serde_json::error::Error) -> CreateApplicationError {
        CreateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateApplicationError {
    fn from(err: CredentialsError) -> CreateApplicationError {
        CreateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateApplicationError {
    fn from(err: HttpDispatchError) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateApplicationError {
    fn from(err: io::Error) -> CreateApplicationError {
        CreateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateApplicationError {
    fn description(&self) -> &str {
        match *self {
            CreateApplicationError::CodeValidation(ref cause) => cause,
            CreateApplicationError::InvalidArgument(ref cause) => cause,
            CreateApplicationError::LimitExceeded(ref cause) => cause,
            CreateApplicationError::ResourceInUse(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplication
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl DeleteApplicationError {
    pub fn from_body(body: &str) -> DeleteApplicationError {
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
                        DeleteApplicationError::ConcurrentModification(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationError {
        DeleteApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationError {
    fn from(err: CredentialsError) -> DeleteApplicationError {
        DeleteApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationError {
    fn from(err: io::Error) -> DeleteApplicationError {
        DeleteApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationError::ResourceInUse(ref cause) => cause,
            DeleteApplicationError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationError::Validation(ref cause) => cause,
            DeleteApplicationError::Credentials(ref err) => err.description(),
            DeleteApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationCloudWatchLoggingOption
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationCloudWatchLoggingOptionError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl DeleteApplicationCloudWatchLoggingOptionError {
    pub fn from_body(body: &str) -> DeleteApplicationCloudWatchLoggingOptionError {
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
                        DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(
                            String::from(error_message),
                        )
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DeleteApplicationCloudWatchLoggingOptionError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DeleteApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationCloudWatchLoggingOptionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: CredentialsError) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: HttpDispatchError) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationCloudWatchLoggingOptionError {
    fn from(err: io::Error) -> DeleteApplicationCloudWatchLoggingOptionError {
        DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationCloudWatchLoggingOptionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationCloudWatchLoggingOptionError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(ref cause) => {
                cause
            }
            DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::Validation(ref cause) => cause,
            DeleteApplicationCloudWatchLoggingOptionError::Credentials(ref err) => {
                err.description()
            }
            DeleteApplicationCloudWatchLoggingOptionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationCloudWatchLoggingOptionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationInputProcessingConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationInputProcessingConfigurationError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl DeleteApplicationInputProcessingConfigurationError {
    pub fn from_body(body: &str) -> DeleteApplicationInputProcessingConfigurationError {
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
                        DeleteApplicationInputProcessingConfigurationError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        DeleteApplicationInputProcessingConfigurationError::InvalidArgument(
                            String::from(error_message),
                        )
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationInputProcessingConfigurationError::ResourceInUse(
                            String::from(error_message),
                        )
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationInputProcessingConfigurationError::ResourceNotFound(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DeleteApplicationInputProcessingConfigurationError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DeleteApplicationInputProcessingConfigurationError::Unknown(String::from(
                        body,
                    )),
                }
            }
            Err(_) => {
                DeleteApplicationInputProcessingConfigurationError::Unknown(String::from(body))
            }
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationInputProcessingConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationInputProcessingConfigurationError {
        DeleteApplicationInputProcessingConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationInputProcessingConfigurationError {
    fn from(err: CredentialsError) -> DeleteApplicationInputProcessingConfigurationError {
        DeleteApplicationInputProcessingConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationInputProcessingConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteApplicationInputProcessingConfigurationError {
        DeleteApplicationInputProcessingConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationInputProcessingConfigurationError {
    fn from(err: io::Error) -> DeleteApplicationInputProcessingConfigurationError {
        DeleteApplicationInputProcessingConfigurationError::HttpDispatch(HttpDispatchError::from(
            err,
        ))
    }
}
impl fmt::Display for DeleteApplicationInputProcessingConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationInputProcessingConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationInputProcessingConfigurationError::ConcurrentModification(
                ref cause,
            ) => cause,
            DeleteApplicationInputProcessingConfigurationError::InvalidArgument(ref cause) => cause,
            DeleteApplicationInputProcessingConfigurationError::ResourceInUse(ref cause) => cause,
            DeleteApplicationInputProcessingConfigurationError::ResourceNotFound(ref cause) => {
                cause
            }
            DeleteApplicationInputProcessingConfigurationError::Validation(ref cause) => cause,
            DeleteApplicationInputProcessingConfigurationError::Credentials(ref err) => {
                err.description()
            }
            DeleteApplicationInputProcessingConfigurationError::HttpDispatch(
                ref dispatch_error,
            ) => dispatch_error.description(),
            DeleteApplicationInputProcessingConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationOutput
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationOutputError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl DeleteApplicationOutputError {
    pub fn from_body(body: &str) -> DeleteApplicationOutputError {
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
                        DeleteApplicationOutputError::ConcurrentModification(String::from(
                            error_message,
                        ))
                    }
                    "InvalidArgumentException" => {
                        DeleteApplicationOutputError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationOutputError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationOutputError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationOutputError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationOutputError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationOutputError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationOutputError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationOutputError {
    fn from(err: CredentialsError) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationOutputError {
    fn from(err: HttpDispatchError) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationOutputError {
    fn from(err: io::Error) -> DeleteApplicationOutputError {
        DeleteApplicationOutputError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationOutputError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationOutputError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationOutputError::InvalidArgument(ref cause) => cause,
            DeleteApplicationOutputError::ResourceInUse(ref cause) => cause,
            DeleteApplicationOutputError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationOutputError::Validation(ref cause) => cause,
            DeleteApplicationOutputError::Credentials(ref err) => err.description(),
            DeleteApplicationOutputError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationOutputError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplicationReferenceDataSource
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationReferenceDataSourceError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl DeleteApplicationReferenceDataSourceError {
    pub fn from_body(body: &str) -> DeleteApplicationReferenceDataSourceError {
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
                        DeleteApplicationReferenceDataSourceError::ConcurrentModification(
                            String::from(error_message),
                        )
                    }
                    "InvalidArgumentException" => {
                        DeleteApplicationReferenceDataSourceError::InvalidArgument(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        DeleteApplicationReferenceDataSourceError::ResourceInUse(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteApplicationReferenceDataSourceError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => DeleteApplicationReferenceDataSourceError::Validation(
                        error_message.to_string(),
                    ),
                    _ => DeleteApplicationReferenceDataSourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationReferenceDataSourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationReferenceDataSourceError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationReferenceDataSourceError {
    fn from(err: CredentialsError) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationReferenceDataSourceError {
    fn from(err: HttpDispatchError) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationReferenceDataSourceError {
    fn from(err: io::Error) -> DeleteApplicationReferenceDataSourceError {
        DeleteApplicationReferenceDataSourceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationReferenceDataSourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationReferenceDataSourceError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationReferenceDataSourceError::ConcurrentModification(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::InvalidArgument(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::ResourceInUse(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::ResourceNotFound(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::Validation(ref cause) => cause,
            DeleteApplicationReferenceDataSourceError::Credentials(ref err) => err.description(),
            DeleteApplicationReferenceDataSourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationReferenceDataSourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    /// <p>Specified application can't be found.</p>
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

impl DescribeApplicationError {
    pub fn from_body(body: &str) -> DescribeApplicationError {
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
                        DescribeApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeApplicationError::Validation(error_message.to_string())
                    }
                    _ => DescribeApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeApplicationError {
    fn from(err: serde_json::error::Error) -> DescribeApplicationError {
        DescribeApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeApplicationError {
    fn from(err: CredentialsError) -> DescribeApplicationError {
        DescribeApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeApplicationError {
    fn from(err: HttpDispatchError) -> DescribeApplicationError {
        DescribeApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeApplicationError {
    fn from(err: io::Error) -> DescribeApplicationError {
        DescribeApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeApplicationError {
    fn description(&self) -> &str {
        match *self {
            DescribeApplicationError::ResourceNotFound(ref cause) => cause,
            DescribeApplicationError::Validation(ref cause) => cause,
            DescribeApplicationError::Credentials(ref err) => err.description(),
            DescribeApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DiscoverInputSchema
#[derive(Debug, PartialEq)]
pub enum DiscoverInputSchemaError {
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Discovery failed to get a record from the streaming source because of the Amazon Kinesis Streams ProvisionedThroughputExceededException. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetRecords.html">GetRecords</a> in the Amazon Kinesis Streams API Reference.</p>
    ResourceProvisionedThroughputExceeded(String),
    /// <p>The service is unavailable, back off and retry the operation. </p>
    ServiceUnavailable(String),
    /// <p>Data format is not valid, Amazon Kinesis Analytics is not able to detect schema for the given streaming source.</p>
    UnableToDetectSchema(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DiscoverInputSchemaError {
    pub fn from_body(body: &str) -> DiscoverInputSchemaError {
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
                        DiscoverInputSchemaError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceProvisionedThroughputExceededException" => {
                        DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(
                            String::from(error_message),
                        )
                    }
                    "ServiceUnavailableException" => {
                        DiscoverInputSchemaError::ServiceUnavailable(String::from(error_message))
                    }
                    "UnableToDetectSchemaException" => {
                        DiscoverInputSchemaError::UnableToDetectSchema(String::from(error_message))
                    }
                    "ValidationException" => {
                        DiscoverInputSchemaError::Validation(error_message.to_string())
                    }
                    _ => DiscoverInputSchemaError::Unknown(String::from(body)),
                }
            }
            Err(_) => DiscoverInputSchemaError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DiscoverInputSchemaError {
    fn from(err: serde_json::error::Error) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DiscoverInputSchemaError {
    fn from(err: CredentialsError) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DiscoverInputSchemaError {
    fn from(err: HttpDispatchError) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::HttpDispatch(err)
    }
}
impl From<io::Error> for DiscoverInputSchemaError {
    fn from(err: io::Error) -> DiscoverInputSchemaError {
        DiscoverInputSchemaError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DiscoverInputSchemaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DiscoverInputSchemaError {
    fn description(&self) -> &str {
        match *self {
            DiscoverInputSchemaError::InvalidArgument(ref cause) => cause,
            DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(ref cause) => cause,
            DiscoverInputSchemaError::ServiceUnavailable(ref cause) => cause,
            DiscoverInputSchemaError::UnableToDetectSchema(ref cause) => cause,
            DiscoverInputSchemaError::Validation(ref cause) => cause,
            DiscoverInputSchemaError::Credentials(ref err) => err.description(),
            DiscoverInputSchemaError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DiscoverInputSchemaError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListApplicationsError {
    pub fn from_body(body: &str) -> ListApplicationsError {
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
                        ListApplicationsError::Validation(error_message.to_string())
                    }
                    _ => ListApplicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListApplicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListApplicationsError {
    fn from(err: serde_json::error::Error) -> ListApplicationsError {
        ListApplicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListApplicationsError {
    fn from(err: CredentialsError) -> ListApplicationsError {
        ListApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListApplicationsError {
    fn from(err: HttpDispatchError) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListApplicationsError {
    fn from(err: io::Error) -> ListApplicationsError {
        ListApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {
            ListApplicationsError::Validation(ref cause) => cause,
            ListApplicationsError::Credentials(ref err) => err.description(),
            ListApplicationsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartApplication
#[derive(Debug, PartialEq)]
pub enum StartApplicationError {
    /// <p>User-provided application configuration is not valid.</p>
    InvalidApplicationConfiguration(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl StartApplicationError {
    pub fn from_body(body: &str) -> StartApplicationError {
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
                    "InvalidApplicationConfigurationException" => {
                        StartApplicationError::InvalidApplicationConfiguration(String::from(
                            error_message,
                        ))
                    }
                    "InvalidArgumentException" => {
                        StartApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        StartApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StartApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartApplicationError::Validation(error_message.to_string())
                    }
                    _ => StartApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartApplicationError {
    fn from(err: serde_json::error::Error) -> StartApplicationError {
        StartApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartApplicationError {
    fn from(err: CredentialsError) -> StartApplicationError {
        StartApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartApplicationError {
    fn from(err: HttpDispatchError) -> StartApplicationError {
        StartApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartApplicationError {
    fn from(err: io::Error) -> StartApplicationError {
        StartApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartApplicationError {
    fn description(&self) -> &str {
        match *self {
            StartApplicationError::InvalidApplicationConfiguration(ref cause) => cause,
            StartApplicationError::InvalidArgument(ref cause) => cause,
            StartApplicationError::ResourceInUse(ref cause) => cause,
            StartApplicationError::ResourceNotFound(ref cause) => cause,
            StartApplicationError::Validation(ref cause) => cause,
            StartApplicationError::Credentials(ref err) => err.description(),
            StartApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopApplication
#[derive(Debug, PartialEq)]
pub enum StopApplicationError {
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl StopApplicationError {
    pub fn from_body(body: &str) -> StopApplicationError {
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
                        StopApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        StopApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        StopApplicationError::Validation(error_message.to_string())
                    }
                    _ => StopApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopApplicationError {
    fn from(err: serde_json::error::Error) -> StopApplicationError {
        StopApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopApplicationError {
    fn from(err: CredentialsError) -> StopApplicationError {
        StopApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopApplicationError {
    fn from(err: HttpDispatchError) -> StopApplicationError {
        StopApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopApplicationError {
    fn from(err: io::Error) -> StopApplicationError {
        StopApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopApplicationError {
    fn description(&self) -> &str {
        match *self {
            StopApplicationError::ResourceInUse(ref cause) => cause,
            StopApplicationError::ResourceNotFound(ref cause) => cause,
            StopApplicationError::Validation(ref cause) => cause,
            StopApplicationError::Credentials(ref err) => err.description(),
            StopApplicationError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StopApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
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

impl UpdateApplicationError {
    pub fn from_body(body: &str) -> UpdateApplicationError {
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
                    "CodeValidationException" => {
                        UpdateApplicationError::CodeValidation(String::from(error_message))
                    }
                    "ConcurrentModificationException" => {
                        UpdateApplicationError::ConcurrentModification(String::from(error_message))
                    }
                    "InvalidArgumentException" => {
                        UpdateApplicationError::InvalidArgument(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateApplicationError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateApplicationError::ResourceNotFound(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateApplicationError::Validation(error_message.to_string())
                    }
                    _ => UpdateApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateApplicationError {
    fn from(err: serde_json::error::Error) -> UpdateApplicationError {
        UpdateApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateApplicationError {
    fn from(err: CredentialsError) -> UpdateApplicationError {
        UpdateApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateApplicationError {
    fn from(err: HttpDispatchError) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateApplicationError {
    fn from(err: io::Error) -> UpdateApplicationError {
        UpdateApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateApplicationError {
    fn description(&self) -> &str {
        match *self {
            UpdateApplicationError::CodeValidation(ref cause) => cause,
            UpdateApplicationError::ConcurrentModification(ref cause) => cause,
            UpdateApplicationError::InvalidArgument(ref cause) => cause,
            UpdateApplicationError::ResourceInUse(ref cause) => cause,
            UpdateApplicationError::ResourceNotFound(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Kinesis Analytics API. Kinesis Analytics clients implement this trait.
pub trait KinesisAnalytics {
    /// <p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> RusotoFuture<
        AddApplicationCloudWatchLoggingOptionResponse,
        AddApplicationCloudWatchLoggingOptionError,
    >;

    /// <p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p>
    fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> RusotoFuture<AddApplicationInputResponse, AddApplicationInputError>;

    /// <p>Adds an <a>InputProcessingConfiguration</a> to an application. An input processor preprocesses records on the input stream before the application's SQL code executes. Currently, the only input processor available is <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a>.</p>
    fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> RusotoFuture<
        AddApplicationInputProcessingConfigurationResponse,
        AddApplicationInputProcessingConfigurationError,
    >;

    /// <p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an Amazon Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Understanding Application Output (Destination)</a>. </p> <p> Note that any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p>
    fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> RusotoFuture<AddApplicationOutputResponse, AddApplicationOutputError>;

    /// <p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p>
    fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> RusotoFuture<
        AddApplicationReferenceDataSourceResponse,
        AddApplicationReferenceDataSourceError,
    >;

    /// <p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to three destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to three destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html">Getting Started</a>. </p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError>;

    /// <p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<DeleteApplicationResponse, DeleteApplicationError>;

    /// <p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> RusotoFuture<
        DeleteApplicationCloudWatchLoggingOptionResponse,
        DeleteApplicationCloudWatchLoggingOptionError,
    >;

    /// <p>Deletes an <a>InputProcessingConfiguration</a> from an input.</p>
    fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> RusotoFuture<
        DeleteApplicationInputProcessingConfigurationResponse,
        DeleteApplicationInputProcessingConfigurationError,
    >;

    /// <p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p>
    fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> RusotoFuture<DeleteApplicationOutputResponse, DeleteApplicationOutputError>;

    /// <p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p>
    fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> RusotoFuture<
        DeleteApplicationReferenceDataSourceResponse,
        DeleteApplicationReferenceDataSourceError,
    >;

    /// <p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> RusotoFuture<DescribeApplicationResponse, DescribeApplicationError>;

    /// <p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream) or S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p>
    fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> RusotoFuture<DiscoverInputSchemaResponse, DiscoverInputSchemaError>;

    /// <p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError>;

    /// <p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a>DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a>StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p>
    fn start_application(
        &self,
        input: StartApplicationRequest,
    ) -> RusotoFuture<StartApplicationResponse, StartApplicationError>;

    /// <p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a>DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p>
    fn stop_application(
        &self,
        input: StopApplicationRequest,
    ) -> RusotoFuture<StopApplicationResponse, StopApplicationError>;

    /// <p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the Kinesis Analytics API.
pub struct KinesisAnalyticsClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl KinesisAnalyticsClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> KinesisAnalyticsClient {
        KinesisAnalyticsClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> KinesisAnalyticsClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        KinesisAnalyticsClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> KinesisAnalytics for KinesisAnalyticsClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> RusotoFuture<
        AddApplicationCloudWatchLoggingOptionResponse,
        AddApplicationCloudWatchLoggingOptionError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddApplicationCloudWatchLoggingOptionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationCloudWatchLoggingOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a>CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p>
    fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> RusotoFuture<AddApplicationInputResponse, AddApplicationInputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationInput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddApplicationInputResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationInputError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds an <a>InputProcessingConfiguration</a> to an application. An input processor preprocesses records on the input stream before the application's SQL code executes. Currently, the only input processor available is <a href="https://aws.amazon.com/documentation/lambda/">AWS Lambda</a>.</p>
    fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> RusotoFuture<
        AddApplicationInputProcessingConfigurationResponse,
        AddApplicationInputProcessingConfigurationError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddApplicationInputProcessingConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationInputProcessingConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an Amazon Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Understanding Application Output (Destination)</a>. </p> <p> Note that any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a>DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p>
    fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> RusotoFuture<AddApplicationOutputResponse, AddApplicationOutputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddApplicationOutputResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationOutputError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p>
    fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> RusotoFuture<
        AddApplicationReferenceDataSourceResponse,
        AddApplicationReferenceDataSourceError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<AddApplicationReferenceDataSourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationReferenceDataSourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to three destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to three destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html">Getting Started</a>. </p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.CreateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> RusotoFuture<DeleteApplicationResponse, DeleteApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> RusotoFuture<
        DeleteApplicationCloudWatchLoggingOptionResponse,
        DeleteApplicationCloudWatchLoggingOptionError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteApplicationCloudWatchLoggingOptionResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationCloudWatchLoggingOptionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes an <a>InputProcessingConfiguration</a> from an input.</p>
    fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> RusotoFuture<
        DeleteApplicationInputProcessingConfigurationResponse,
        DeleteApplicationInputProcessingConfigurationError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteApplicationInputProcessingConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(
                        DeleteApplicationInputProcessingConfigurationError::from_body(
                            String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                        ),
                    )
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p>
    fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> RusotoFuture<DeleteApplicationOutputResponse, DeleteApplicationOutputError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationOutput",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteApplicationOutputResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationOutputError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a>AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p>
    fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> RusotoFuture<
        DeleteApplicationReferenceDataSourceResponse,
        DeleteApplicationReferenceDataSourceError,
    > {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteApplicationReferenceDataSourceResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationReferenceDataSourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a>ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> RusotoFuture<DescribeApplicationResponse, DescribeApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DescribeApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DescribeApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream) or S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href="http://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p>
    fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> RusotoFuture<DiscoverInputSchemaResponse, DiscoverInputSchemaError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DiscoverInputSchema",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DiscoverInputSchemaResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DiscoverInputSchemaError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a>DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> RusotoFuture<ListApplicationsResponse, ListApplicationsError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.ListApplications");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListApplicationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a>DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a>StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p>
    fn start_application(
        &self,
        input: StartApplicationRequest,
    ) -> RusotoFuture<StartApplicationResponse, StartApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StartApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a>DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p>
    fn stop_application(
        &self,
        input: StopApplicationRequest,
    ) -> RusotoFuture<StopApplicationResponse, StopApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StopApplication");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.UpdateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(UpdateApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }
}

#[cfg(test)]
mod protocol_tests {}
