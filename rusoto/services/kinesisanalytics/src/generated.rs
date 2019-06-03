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

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::v2::{Dispatcher, Request, ServiceRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
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
#[cfg_attr(test, derive(Serialize))]
pub struct AddApplicationCloudWatchLoggingOptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationInputProcessingConfigurationRequest {
    /// <p>Name of the application to which you want to add the input processing configuration.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application to which you want to add the input processing configuration. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration to add the input processing configuration to. You can get a list of the input IDs for an application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    #[serde(rename = "InputId")]
    pub input_id: String,
    /// <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to add to the application.</p>
    #[serde(rename = "InputProcessingConfiguration")]
    pub input_processing_configuration: InputProcessingConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddApplicationInputProcessingConfigurationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationInputRequest {
    /// <p>Name of your existing Amazon Kinesis Analytics application to which you want to add the streaming source.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Current version of your Amazon Kinesis Analytics application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application version.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_Input.html">Input</a> to add.</p>
    #[serde(rename = "Input")]
    pub input: Input,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddApplicationInputResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationOutputRequest {
    /// <p>Name of the application to which you want to add the output configuration.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application to which you want to add the output configuration. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>An array of objects, each describing one output configuration. In the output configuration, you specify the name of an in-application stream, a destination (that is, an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), and record the formation to use when writing to the destination.</p>
    #[serde(rename = "Output")]
    pub output: Output,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddApplicationOutputResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddApplicationReferenceDataSourceRequest {
    /// <p>Name of an existing application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application for which you are adding the reference data source. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The reference data source can be an object in your Amazon S3 bucket. Amazon Kinesis Analytics reads the object and copies the data into the in-application table that is created. You provide an S3 bucket, object key name, and the resulting in-application table that is created. You must also provide an IAM role with the necessary permissions that Amazon Kinesis Analytics can assume to read the object from your S3 bucket on your behalf.</p>
    #[serde(rename = "ReferenceDataSource")]
    pub reference_data_source: ReferenceDataSource,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddApplicationReferenceDataSourceResponse {}

/// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Provides a description of the application, including the application Amazon Resource Name (ARN), status, latest version, and input and output configuration.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Describes the CloudWatch log streams that are configured to receive application messages. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>. </p>
    #[serde(rename = "CloudWatchLoggingOptionDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_option_descriptions: Option<Vec<CloudWatchLoggingOptionDescription>>,
    /// <p>Time stamp when the application version was created.</p>
    #[serde(rename = "CreateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<f64>,
    /// <p>Describes the application input configuration. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
    #[serde(rename = "InputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_descriptions: Option<Vec<InputDescription>>,
    /// <p>Time stamp when the application was last updated.</p>
    #[serde(rename = "LastUpdateTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_timestamp: Option<f64>,
    /// <p>Describes the application output configuration. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>. </p>
    #[serde(rename = "OutputDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_descriptions: Option<Vec<OutputDescription>>,
    /// <p>Describes reference data sources configured for the application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
    #[serde(rename = "ReferenceDataSourceDescriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_data_source_descriptions: Option<Vec<ReferenceDataSourceDescription>>,
}

/// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Provides application summary information, including the application Amazon Resource Name (ARN), name, and status.</p></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Provides additional mapping information when the record format uses delimiters, such as CSV. For example, the following sample records use CSV format, where the records use the <i>'\n'</i> as the row delimiter and a comma (",") as the column delimiter: </p> <p> <code>"name1", "address1"</code> </p> <p> <code>"name2", "address2"</code> </p>
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>One or more SQL statements that read input data, transform it, and generate output. For example, you can write a SQL statement that reads data from one in-application stream, generates a running average of the number of advertisement clicks by vendor, and insert resulting rows in another in-application stream using pumps. For more information about the typical pattern, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-app-code.html">Application Code</a>. </p> <p>You can provide such series of SQL statements, where output of one statement can be used as the input for the next statement. You store intermediate results by creating in-application streams and pumps.</p> <p>Note that the application code must create the streams with names specified in the <code>Outputs</code>. For example, if your <code>Outputs</code> defines output streams named <code>ExampleOutputStream1</code> and <code>ExampleOutputStream2</code>, then your application code must create these streams. </p>
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
    /// <p>Use this parameter to configure a CloudWatch log stream to monitor application configuration errors. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p>
    #[serde(rename = "CloudWatchLoggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logging_options: Option<Vec<CloudWatchLoggingOption>>,
    /// <p>Use this parameter to configure the application input.</p> <p>You can configure your application to receive input from a single streaming source. In this configuration, you map this streaming source to an in-application stream that is created. Your application code can then query the in-application stream like a table (you can think of it as a constantly updating table).</p> <p>For the streaming source, you provide its Amazon Resource Name (ARN) and format of data on the stream (for example, JSON, CSV, etc.). You also must provide an IAM role that Amazon Kinesis Analytics can assume to read this stream on your behalf.</p> <p>To create the in-application stream, you need to specify a schema to transform your data into a schematized version used in SQL. In the schema, you provide the necessary mapping of the data elements in the streaming source to record columns in the in-app stream.</p>
    #[serde(rename = "Inputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<Input>>,
    /// <p>You can configure application output to write data from any of the in-application streams to up to three destinations.</p> <p>These destinations can be Amazon Kinesis streams, Amazon Kinesis Firehose delivery streams, AWS Lambda destinations, or any combination of the three.</p> <p>In the configuration, you specify the in-application stream name, the destination stream or Lambda function Amazon Resource Name (ARN), and the format to use when writing data. You must also provide an IAM role that Amazon Kinesis Analytics can assume to write to the destination stream or Lambda function on your behalf.</p> <p>In the output configuration, you also provide the output stream or Lambda function ARN. For stream destinations, you provide the format of data in the stream (for example, JSON, CSV). You also must provide an IAM role that Amazon Kinesis Analytics can assume to write to the stream or Lambda function on your behalf.</p>
    #[serde(rename = "Outputs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<Output>>,
    /// <p>A list of one or more tags to assign to the application. A tag is a key-value pair that identifies an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>TBD</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The <code>CloudWatchLoggingOptionId</code> of the CloudWatch logging option to delete. You can get the <code>CloudWatchLoggingOptionId</code> by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation. </p>
    #[serde(rename = "CloudWatchLoggingOptionId")]
    pub cloud_watch_logging_option_id: String,
    /// <p>The version ID of the Kinesis Analytics application.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApplicationCloudWatchLoggingOptionResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationInputProcessingConfigurationRequest {
    /// <p>The Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>The version ID of the Kinesis Analytics application.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the input configuration from which to delete the input processing configuration. You can get a list of the input IDs for an application by using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    #[serde(rename = "InputId")]
    pub input_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApplicationInputProcessingConfigurationResponse {}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationOutputRequest {
    /// <p>Amazon Kinesis Analytics application name.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Amazon Kinesis Analytics application version. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned. </p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>The ID of the configuration to delete. Each output configuration that is added to the application, either when the application is created or later using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationOutput.html">AddApplicationOutput</a> operation, has a unique ID. You need to provide the ID to uniquely identify the output configuration that you want to delete from the application configuration. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the specific <code>OutputId</code>. </p>
    #[serde(rename = "OutputId")]
    pub output_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteApplicationOutputResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteApplicationReferenceDataSourceRequest {
    /// <p>Name of an existing application.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the current application version. If the version specified is not the current version, the <code>ConcurrentModificationException</code> is returned.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
    /// <p>ID of the reference data source. When you add a reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a>, Amazon Kinesis Analytics assigns an ID. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get the reference ID. </p>
    #[serde(rename = "ReferenceId")]
    pub reference_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeApplicationResponse {
    /// <p>Provides a description of the application, such as the application Amazon Resource Name (ARN), status, latest version, and input and output configuration details.</p>
    #[serde(rename = "ApplicationDetail")]
    pub application_detail: ApplicationDetail,
}

/// <p>Describes the data format when records are written to the destination. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DestinationSchema {
    /// <p>Specifies the format of the records on the output stream.</p>
    #[serde(rename = "RecordFormatType")]
    pub record_format_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DiscoverInputSchemaRequest {
    /// <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to use to preprocess the records before discovering the schema of the records.</p>
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
    /// <p>Specify this parameter to discover a schema from data in an Amazon S3 object.</p>
    #[serde(rename = "S3Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_configuration: Option<S3Configuration>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>When you configure the application input, you specify the streaming source, the in-application stream name that is created, and the mapping between the two. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Input {
    /// <p>Describes the number of in-application streams to create. </p> <p>Data from your source is routed to these in-application input streams.</p> <p> (see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>.</p>
    #[serde(rename = "InputParallelism")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_parallelism: Option<InputParallelism>,
    /// <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> for the input. An input processor transforms records as they are received from the stream, before the application's SQL code executes. Currently, the only input processing configuration available is <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputLambdaProcessor.html">InputLambdaProcessor</a>.</p>
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
    /// <p>Input source ID. You can get this ID by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p>
    #[serde(rename = "Id")]
    pub id: String,
    /// <p>Point at which you want the application to start processing records from the streaming source.</p>
    #[serde(rename = "InputStartingPositionConfiguration")]
    pub input_starting_position_configuration: InputStartingPositionConfiguration,
}

/// <p>Describes the application input configuration. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>An object that contains the Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a> function that is used to preprocess records in the stream, and the ARN of the IAM role that is used to access the AWS Lambda function. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputLambdaProcessor {
    /// <p><p>The ARN of the <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a> function that operates on records in the stream.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The ARN of the IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>An object that contains the Amazon Resource Name (ARN) of the <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a> function that is used to preprocess records in the stream, and the ARN of the IAM role that is used to access the AWS Lambda expression.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InputLambdaProcessorDescription {
    /// <p>The ARN of the <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a> function that is used to preprocess the records in the stream.</p>
    #[serde(rename = "ResourceARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
    /// <p>The ARN of the IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Represents an update to the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputLambdaProcessor.html">InputLambdaProcessor</a> that is used to preprocess the records in the stream.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputLambdaProcessorUpdate {
    /// <p><p>The Amazon Resource Name (ARN) of the new <a href="https://docs.aws.amazon.com/ambda/">AWS Lambda</a> function that is used to preprocess the records in the stream.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>The ARN of the new IAM role that is used to access the AWS Lambda function.</p>
    #[serde(rename = "RoleARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn_update: Option<String>,
}

/// <p>Describes the number of in-application streams to create for a given streaming source. For information about parallelism, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputParallelism {
    /// <p>Number of in-application streams to create. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p>
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

/// <p>Provides a description of a processor that is used to preprocess the records in the stream before being processed by your application code. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputProcessingConfiguration {
    /// <p>The <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputLambdaProcessor.html">InputLambdaProcessor</a> that is used to preprocess the records in the stream before being processed by your application code.</p>
    #[serde(rename = "InputLambdaProcessor")]
    pub input_lambda_processor: InputLambdaProcessor,
}

/// <p>Provides configuration information about an input processor. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InputProcessingConfigurationDescription {
    /// <p>Provides configuration information about the associated <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputLambdaProcessorDescription.html">InputLambdaProcessorDescription</a>.</p>
    #[serde(rename = "InputLambdaProcessorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_lambda_processor_description: Option<InputLambdaProcessorDescription>,
}

/// <p>Describes updates to an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InputProcessingConfigurationUpdate {
    /// <p>Provides update information for an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputLambdaProcessor.html">InputLambdaProcessor</a>.</p>
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
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to make sure that the role has the necessary permissions to access the stream.</p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p> Describes the Amazon Kinesis Firehose delivery stream that is configured as the streaming source in the application input configuration. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p> When updating an output configuration using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_UpdateApplication.html">UpdateApplication</a> operation, provides information about an Amazon Kinesis Firehose delivery stream configured as the destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct KinesisFirehoseOutputUpdate {
    /// <p>Amazon Resource Name (ARN) of the Amazon Kinesis Firehose delivery stream to write to.</p>
    #[serde(rename = "ResourceARNUpdate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn_update: Option<String>,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to access the stream on your behalf. You need to grant the necessary permissions to this role.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p> When updating an output configuration using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_UpdateApplication.html">UpdateApplication</a> operation, provides information about an Amazon Kinesis stream configured as the destination. </p>
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
    /// <p><p>Amazon Resource Name (ARN) of the destination Lambda function to write to.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>ARN of the IAM role that Amazon Kinesis Analytics can assume to write to the destination function on your behalf. You need to grant the necessary permissions to this role. </p>
    #[serde(rename = "RoleARN")]
    pub role_arn: String,
}

/// <p>For an application output, describes the AWS Lambda function configured as its destination. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>When updating an output configuration using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_UpdateApplication.html">UpdateApplication</a> operation, provides information about an AWS Lambda function configured as the destination.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LambdaOutputUpdate {
    /// <p><p>Amazon Resource Name (ARN) of the destination Lambda function.</p> <note> <p>To specify an earlier version of the Lambda function than the latest, include the Lambda function version in the Lambda function ARN. For more information about Lambda ARNs, see <a href="/general/latest/gr/aws-arns-and-namespaces.html#arn-syntax-lambda">Example ARNs: AWS Lambda</a> </p> </note></p>
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
#[cfg_attr(test, derive(Serialize))]
pub struct ListApplicationsResponse {
    /// <p>List of <code>ApplicationSummary</code> objects. </p>
    #[serde(rename = "ApplicationSummaries")]
    pub application_summaries: Vec<ApplicationSummary>,
    /// <p>Returns true if there are more applications to retrieve.</p>
    #[serde(rename = "HasMoreApplications")]
    pub has_more_applications: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the application for which to retrieve tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The key-value tags assigned to the application.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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

/// <p> Describes application output configuration in which you identify an in-application stream and a destination where you want the in-application stream data to be written. The destination can be an Amazon Kinesis stream or an Amazon Kinesis Firehose delivery stream. </p> <p/> <p>For limits on how many destinations an application can write and other limitations, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Output {
    /// <p>Describes the data format when records are written to the destination. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>Describes the data format when records are written to the destination. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Configuring Application Output</a>.</p>
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
    /// <p>Reference to the data element in the streaming input or the reference data source. This element is required if the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_RecordFormat.html#analytics-Type-RecordFormat-RecordFormatTypel">RecordFormatType</a> is <code>JSON</code>.</p>
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
    /// <p>Identifies the S3 bucket and object that contains the reference data. Also identifies the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf. An Amazon Kinesis Analytics application loads reference data only once. If the data changes, you call the <code>UpdateApplication</code> operation to trigger reloading of data into your application. </p>
    #[serde(rename = "S3ReferenceDataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_reference_data_source: Option<S3ReferenceDataSource>,
    /// <p>Name of the in-application table to create.</p>
    #[serde(rename = "TableName")]
    pub table_name: String,
}

/// <p>Describes the reference data source configured for an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReferenceDataSourceDescription {
    /// <p>ID of the reference data source. This is the ID that Amazon Kinesis Analytics assigns when you add the reference data source to your application using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation.</p>
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
    /// <p>ID of the reference data source being updated. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get this value.</p>
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

/// <p>Provides a description of an Amazon S3 data source, including the Amazon Resource Name (ARN) of the S3 bucket, the ARN of the IAM role that is used to access the bucket, and the name of the Amazon S3 object that contains the data.</p>
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

/// <p>Identifies the S3 bucket and object that contains the reference data. Also identifies the IAM role Amazon Kinesis Analytics can assume to read this object on your behalf.</p> <p>An Amazon Kinesis Analytics application loads reference data only once. If the data changes, you call the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_UpdateApplication.html">UpdateApplication</a> operation to trigger reloading of data into your application.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct StopApplicationResponse {}

/// <p>A key-value pair (the value is optional) that you can define and assign to AWS resources. If you specify a tag that already exists, the tag value is replaced with the value that you specify in the request. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50. For more information, see <a href="https://docs.aws.amazon.com/awsaccountbilling/latest/aboutv2/cost-alloc-tags.html">Using Cost Allocation Tags</a> in the <i>AWS Billing and Cost Management Guide</i>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The key of the key-value tag.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>The value of the key-value tag. The value is optional.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the application to assign the tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>The key-value tags to assign to the application.</p>
    #[serde(rename = "Tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the Kinesis Analytics application from which to remove the tags.</p>
    #[serde(rename = "ResourceARN")]
    pub resource_arn: String,
    /// <p>A list of keys of tags to remove from the specified application.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateApplicationRequest {
    /// <p>Name of the Amazon Kinesis Analytics application to update.</p>
    #[serde(rename = "ApplicationName")]
    pub application_name: String,
    /// <p>Describes application updates.</p>
    #[serde(rename = "ApplicationUpdate")]
    pub application_update: ApplicationUpdate,
    /// <p>The current application version ID. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to get this value.</p>
    #[serde(rename = "CurrentApplicationVersionId")]
    pub current_application_version_id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl AddApplicationCloudWatchLoggingOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationCloudWatchLoggingOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        AddApplicationCloudWatchLoggingOptionError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddApplicationCloudWatchLoggingOptionError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl AddApplicationInputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddApplicationInputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(AddApplicationInputError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AddApplicationInputError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(AddApplicationInputError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AddApplicationInputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddApplicationInputError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AddApplicationInputError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddApplicationInputError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl AddApplicationInputProcessingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationInputProcessingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        AddApplicationInputProcessingConfigurationError::UnsupportedOperation(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddApplicationInputProcessingConfigurationError::UnsupportedOperation(ref cause) => {
                cause
            }
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl AddApplicationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddApplicationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(AddApplicationOutputError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(AddApplicationOutputError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(AddApplicationOutputError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddApplicationOutputError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(AddApplicationOutputError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddApplicationOutputError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl AddApplicationReferenceDataSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AddApplicationReferenceDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        AddApplicationReferenceDataSourceError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            AddApplicationReferenceDataSourceError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>User-provided application code (query) is invalid. This can be a simple syntax error.</p>
    CodeValidation(String),
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Exceeded the number of applications allowed.</p>
    LimitExceeded(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(CreateApplicationError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(CreateApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidArgument(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateApplicationError::LimitExceeded(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateApplicationError::ResourceInUse(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(CreateApplicationError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            CreateApplicationError::ConcurrentModification(ref cause) => cause,
            CreateApplicationError::InvalidArgument(ref cause) => cause,
            CreateApplicationError::LimitExceeded(ref cause) => cause,
            CreateApplicationError::ResourceInUse(ref cause) => cause,
            CreateApplicationError::TooManyTags(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(DeleteApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationError::ResourceNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DeleteApplicationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteApplicationError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationCloudWatchLoggingOptionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationCloudWatchLoggingOptionError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteApplicationCloudWatchLoggingOptionError::UnsupportedOperation(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteApplicationCloudWatchLoggingOptionError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationInputProcessingConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationInputProcessingConfigurationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ConcurrentModification(
                            err.msg,
                        ),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::InvalidArgument(
                            err.msg,
                        ),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::ResourceNotFound(
                            err.msg,
                        ),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteApplicationInputProcessingConfigurationError::UnsupportedOperation(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteApplicationInputProcessingConfigurationError::UnsupportedOperation(ref cause) => {
                cause
            }
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationOutputError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationOutputError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationOutputError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::InvalidArgument(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteApplicationOutputError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteApplicationOutputError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteApplicationOutputError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DeleteApplicationReferenceDataSourceError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteApplicationReferenceDataSourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ConcurrentModification(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::InvalidArgument(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ResourceInUse(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::ResourceNotFound(err.msg),
                    )
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(
                        DeleteApplicationReferenceDataSourceError::UnsupportedOperation(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DeleteApplicationReferenceDataSourceError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeApplication
#[derive(Debug, PartialEq)]
pub enum DescribeApplicationError {
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl DescribeApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeApplicationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(DescribeApplicationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            DescribeApplicationError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by DiscoverInputSchema
#[derive(Debug, PartialEq)]
pub enum DiscoverInputSchemaError {
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Discovery failed to get a record from the streaming source because of the Amazon Kinesis Streams ProvisionedThroughputExceededException. For more information, see <a href="kinesis/latest/APIReference/API_GetRecords.html">GetRecords</a> in the Amazon Kinesis Streams API Reference.</p>
    ResourceProvisionedThroughputExceeded(String),
    /// <p>The service is unavailable. Back off and retry the operation. </p>
    ServiceUnavailable(String),
    /// <p>Data format is not valid. Amazon Kinesis Analytics is not able to detect schema for the given streaming source.</p>
    UnableToDetectSchema(String),
}

impl DiscoverInputSchemaError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DiscoverInputSchemaError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidArgumentException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::InvalidArgument(err.msg))
                }
                "ResourceProvisionedThroughputExceededException" => {
                    return RusotoError::Service(
                        DiscoverInputSchemaError::ResourceProvisionedThroughputExceeded(err.msg),
                    )
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "UnableToDetectSchemaException" => {
                    return RusotoError::Service(DiscoverInputSchemaError::UnableToDetectSchema(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
        }
    }
}
/// Errors returned by ListApplications
#[derive(Debug, PartialEq)]
pub enum ListApplicationsError {}

impl ListApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListApplicationsError {
    fn description(&self) -> &str {
        match *self {}
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(ListTagsForResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidArgument(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            ListTagsForResourceError::ConcurrentModification(ref cause) => cause,
            ListTagsForResourceError::InvalidArgument(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl StartApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InvalidApplicationConfigurationException" => {
                    return RusotoError::Service(
                        StartApplicationError::InvalidApplicationConfiguration(err.msg),
                    )
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(StartApplicationError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartApplicationError::ResourceNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(StartApplicationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            StartApplicationError::UnsupportedOperation(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl StopApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ResourceInUseException" => {
                    return RusotoError::Service(StopApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopApplicationError::ResourceNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(StopApplicationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            StopApplicationError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(TagResourceError::ConcurrentModification(err.msg))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(TagResourceError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(TagResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(TagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TagResourceError {
    fn description(&self) -> &str {
        match *self {
            TagResourceError::ConcurrentModification(ref cause) => cause,
            TagResourceError::InvalidArgument(ref cause) => cause,
            TagResourceError::ResourceInUse(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::TooManyTags(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>Exception thrown as a result of concurrent modification to an application. For example, two individuals attempting to edit the same application at the same time.</p>
    ConcurrentModification(String),
    /// <p>Specified input parameter value is invalid.</p>
    InvalidArgument(String),
    /// <p>Application is not available for this operation.</p>
    ResourceInUse(String),
    /// <p>Specified application can't be found.</p>
    ResourceNotFound(String),
    /// <p>Application created with too many tags, or too many tags added to an application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    TooManyTags(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UntagResourceError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UntagResourceError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UntagResourceError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "TooManyTagsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyTags(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UntagResourceError {
    fn description(&self) -> &str {
        match *self {
            UntagResourceError::ConcurrentModification(ref cause) => cause,
            UntagResourceError::InvalidArgument(ref cause) => cause,
            UntagResourceError::ResourceInUse(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::TooManyTags(ref cause) => cause,
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
    /// <p>The request was rejected because a specified parameter is not supported or a specified resource is not valid for this operation. </p>
    UnsupportedOperation(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "CodeValidationException" => {
                    return RusotoError::Service(UpdateApplicationError::CodeValidation(err.msg))
                }
                "ConcurrentModificationException" => {
                    return RusotoError::Service(UpdateApplicationError::ConcurrentModification(
                        err.msg,
                    ))
                }
                "InvalidArgumentException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidArgument(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateApplicationError::ResourceNotFound(err.msg))
                }
                "UnsupportedOperationException" => {
                    return RusotoError::Service(UpdateApplicationError::UnsupportedOperation(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
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
            UpdateApplicationError::UnsupportedOperation(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Kinesis Analytics API. Kinesis Analytics clients implement this trait.
pub trait KinesisAnalytics {
    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p></p>
    fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> Request<AddApplicationCloudWatchLoggingOptionRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_CreateApplication.html">CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p></p>
    fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> Request<AddApplicationInputRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to an application. An input processor preprocesses records on the input stream before the application&#39;s SQL code executes. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p></p>
    fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> Request<AddApplicationInputProcessingConfigurationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Understanding Application Output (Destination)</a>. </p> <p> Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p></p>
    fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> Request<AddApplicationOutputRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p></p>
    fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> Request<AddApplicationReferenceDataSourceRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to three destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to three destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html">Getting Started</a>. </p></p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Request<CreateApplicationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p></p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Request<DeleteApplicationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p></p>
    fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> Request<DeleteApplicationCloudWatchLoggingOptionRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> from an input.</p></p>
    fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> Request<DeleteApplicationInputProcessingConfigurationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p></p>
    fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> Request<DeleteApplicationOutputRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p></p>
    fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> Request<DeleteApplicationReferenceDataSourceRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_ListApplications.html">ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p></p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Request<DescribeApplicationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream) or S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p></p>
    fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> Request<DiscoverInputSchemaRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p></p>
    fn list_applications(&self, input: ListApplicationsRequest)
        -> Request<ListApplicationsRequest>;

    /// <p>Retrieves the list of key-value tags assigned to the application.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Request<ListTagsForResourceRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_StopApplication.html">StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p></p>
    fn start_application(&self, input: StartApplicationRequest)
        -> Request<StartApplicationRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p></p>
    fn stop_application(&self, input: StopApplicationRequest) -> Request<StopApplicationRequest>;

    /// <p>Adds one or more key-value tags to a Kinesis Analytics application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest>;

    /// <p>Removes one or more tags from a Kinesis Analytics application.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest>;

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p></p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Request<UpdateApplicationRequest>;
}
/// A client for the Kinesis Analytics API.
#[derive(Clone)]
pub struct KinesisAnalyticsClient {
    client: Client,
    region: region::Region,
}

impl KinesisAnalyticsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> KinesisAnalyticsClient {
        KinesisAnalyticsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> KinesisAnalyticsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        KinesisAnalyticsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }
}

impl KinesisAnalytics for KinesisAnalyticsClient {
    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds a CloudWatch log stream to monitor application configuration errors. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p></p>
    fn add_application_cloud_watch_logging_option(
        &self,
        input: AddApplicationCloudWatchLoggingOptionRequest,
    ) -> Request<AddApplicationCloudWatchLoggingOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p> Adds a streaming source to your Amazon Kinesis application. For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. </p> <p>You can add a streaming source either when you create an application or you can use this operation to add a streaming source after you create an application. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_CreateApplication.html">CreateApplication</a>.</p> <p>Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application version. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationInput</code> action.</p></p>
    fn add_application_input(
        &self,
        input: AddApplicationInputRequest,
    ) -> Request<AddApplicationInputRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> to an application. An input processor preprocesses records on the input stream before the application&#39;s SQL code executes. Currently, the only input processor available is <a href="https://docs.aws.amazon.com/lambda/">AWS Lambda</a>.</p></p>
    fn add_application_input_processing_configuration(
        &self,
        input: AddApplicationInputProcessingConfigurationRequest,
    ) -> Request<AddApplicationInputProcessingConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds an external destination to your Amazon Kinesis Analytics application.</p> <p>If you want Amazon Kinesis Analytics to deliver data from an in-application stream within your application to an external destination (such as an Amazon Kinesis stream, an Amazon Kinesis Firehose delivery stream, or an AWS Lambda function), you add the relevant configuration to your application using this operation. You can configure one or more outputs for your application. Each output configuration maps an in-application stream and an external destination.</p> <p> You can use one of the output configurations to deliver data from your in-application error stream to an external destination so that you can analyze the errors. For more information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-output.html">Understanding Application Output (Destination)</a>. </p> <p> Any configuration update, including adding a streaming source using this operation, results in a new version of the application. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the current application version.</p> <p>For the limits on the number of application inputs and outputs you can configure, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action.</p></p>
    fn add_application_output(
        &self,
        input: AddApplicationOutputRequest,
    ) -> Request<AddApplicationOutputRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Adds a reference data source to an existing application.</p> <p>Amazon Kinesis Analytics reads reference data (that is, an Amazon S3 object) and creates an in-application table within your application. In the request, you provide the source (S3 bucket name and object key name), name of the in-application table to create, and the necessary mapping information that describes how data in Amazon S3 object maps to columns in the resulting in-application table.</p> <p> For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. For the limits on data sources you can add to your application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/limits.html">Limits</a>. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:AddApplicationOutput</code> action. </p></p>
    fn add_application_reference_data_source(
        &self,
        input: AddApplicationReferenceDataSourceRequest,
    ) -> Request<AddApplicationReferenceDataSourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p> Creates an Amazon Kinesis Analytics application. You can configure each application with one streaming source as input, application code to process the input, and up to three destinations where you want Amazon Kinesis Analytics to write the output data from your application. For an overview, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works.html">How it Works</a>. </p> <p>In the input configuration, you map the streaming source to an in-application stream, which you can think of as a constantly updating table. In the mapping, you must provide a schema for the in-application stream and map each data column in the in-application stream to a data element in the streaming source.</p> <p>Your application code is one or more SQL statements that read input data, transform it, and generate output. Your application code can create one or more SQL artifacts like SQL streams or pumps.</p> <p>In the output configuration, you can configure the application to write data from in-application streams created in your applications to up to three destinations.</p> <p> To read data from your source stream or write data to destination streams, Amazon Kinesis Analytics needs your permissions. You grant these permissions by creating IAM roles. This operation requires permissions to perform the <code>kinesisanalytics:CreateApplication</code> action. </p> <p> For introductory exercises to create an Amazon Kinesis Analytics application, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/getting-started.html">Getting Started</a>. </p></p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Request<CreateApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes the specified application. Amazon Kinesis Analytics halts application execution and deletes the application, including any application artifacts (such as in-application streams, reference table, and application code).</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplication</code> action.</p></p>
    fn delete_application(
        &self,
        input: DeleteApplicationRequest,
    ) -> Request<DeleteApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes a CloudWatch log stream from an application. For more information about using CloudWatch log streams with Amazon Kinesis Analytics applications, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/cloudwatch-logs.html">Working with Amazon CloudWatch Logs</a>.</p></p>
    fn delete_application_cloud_watch_logging_option(
        &self,
        input: DeleteApplicationCloudWatchLoggingOptionRequest,
    ) -> Request<DeleteApplicationCloudWatchLoggingOptionRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes an <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_InputProcessingConfiguration.html">InputProcessingConfiguration</a> from an input.</p></p>
    fn delete_application_input_processing_configuration(
        &self,
        input: DeleteApplicationInputProcessingConfigurationRequest,
    ) -> Request<DeleteApplicationInputProcessingConfigurationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes output destination configuration from your application configuration. Amazon Kinesis Analytics will no longer write data from the corresponding in-application stream to the external output destination.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DeleteApplicationOutput</code> action.</p></p>
    fn delete_application_output(
        &self,
        input: DeleteApplicationOutputRequest,
    ) -> Request<DeleteApplicationOutputRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Deletes a reference data source configuration from the specified application configuration.</p> <p>If the application is running, Amazon Kinesis Analytics immediately removes the in-application table that you created using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_AddApplicationReferenceDataSource.html">AddApplicationReferenceDataSource</a> operation. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics.DeleteApplicationReferenceDataSource</code> action.</p></p>
    fn delete_application_reference_data_source(
        &self,
        input: DeleteApplicationReferenceDataSourceRequest,
    ) -> Request<DeleteApplicationReferenceDataSourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Returns information about a specific Amazon Kinesis Analytics application.</p> <p>If you want to retrieve a list of all applications in your account, use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_ListApplications.html">ListApplications</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:DescribeApplication</code> action. You can use <code>DescribeApplication</code> to get the current application versionId, which you need to call other operations such as <code>Update</code>. </p></p>
    fn describe_application(
        &self,
        input: DescribeApplicationRequest,
    ) -> Request<DescribeApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Infers a schema by evaluating sample records on the specified streaming source (Amazon Kinesis stream or Amazon Kinesis Firehose delivery stream) or S3 object. In the response, the operation returns the inferred schema and also the sample records that the operation used to infer the schema.</p> <p> You can use the inferred schema when configuring a streaming source for your application. For conceptual information, see <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/how-it-works-input.html">Configuring Application Input</a>. Note that when you create an application using the Amazon Kinesis Analytics console, the console uses this operation to infer a schema and show it in the console user interface. </p> <p> This operation requires permissions to perform the <code>kinesisanalytics:DiscoverInputSchema</code> action. </p></p>
    fn discover_input_schema(
        &self,
        input: DiscoverInputSchemaRequest,
    ) -> Request<DiscoverInputSchemaRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Returns a list of Amazon Kinesis Analytics applications in your account. For each application, the response includes the application name, Amazon Resource Name (ARN), and status. If the response returns the <code>HasMoreApplications</code> value as true, you can send another request by adding the <code>ExclusiveStartApplicationName</code> in the request body, and set the value of this to the last application name from the previous response. </p> <p>If you want detailed information about a specific application, use <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a>.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:ListApplications</code> action.</p></p>
    fn list_applications(
        &self,
        input: ListApplicationsRequest,
    ) -> Request<ListApplicationsRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Retrieves the list of key-value tags assigned to the application.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Request<ListTagsForResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Starts the specified Amazon Kinesis Analytics application. After creating an application, you must exclusively call this operation to start your application.</p> <p>After the application starts, it begins consuming the input data, processes it, and writes the output to the configured destination.</p> <p> The application status must be <code>READY</code> for you to start an application. You can get the application status in the console or using the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation.</p> <p>After you start the application, you can stop the application from processing the input by calling the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_StopApplication.html">StopApplication</a> operation.</p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StartApplication</code> action.</p></p>
    fn start_application(
        &self,
        input: StartApplicationRequest,
    ) -> Request<StartApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Stops the application from processing input data. You can stop an application only if it is in the running state. You can use the <a href="https://docs.aws.amazon.com/kinesisanalytics/latest/dev/API_DescribeApplication.html">DescribeApplication</a> operation to find the application state. After the application is stopped, Amazon Kinesis Analytics stops reading data from the input, the application stops processing data, and there is no output written to the destination. </p> <p>This operation requires permissions to perform the <code>kinesisanalytics:StopApplication</code> action.</p></p>
    fn stop_application(&self, input: StopApplicationRequest) -> Request<StopApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Adds one or more key-value tags to a Kinesis Analytics application. Note that the maximum number of application tags includes system tags. The maximum number of user-defined application tags is 50.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> Request<TagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p>Removes one or more tags from a Kinesis Analytics application.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> Request<UntagResourceRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }

    /// <p><note> <p>This documentation is for version 1 of the Amazon Kinesis Data Analytics API, which only supports SQL applications. Version 2 of the API supports SQL and Java applications. For more information about version 2, see <a href="/kinesisanalytics/latest/apiv2/Welcome.html">Amazon Kinesis Data Analytics API V2 Documentation</a>.</p> </note> <p>Updates an existing Amazon Kinesis Analytics application. Using this API, you can update application code, input configuration, and output configuration. </p> <p>Note that Amazon Kinesis Analytics updates the <code>CurrentApplicationVersionId</code> each time you update your application. </p> <p>This operation requires permission for the <code>kinesisanalytics:UpdateApplication</code> action.</p></p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Request<UpdateApplicationRequest> {
        Request::new(input, self.region.clone(), self.client.clone())
    }
}

impl ServiceRequest for AddApplicationCloudWatchLoggingOptionRequest {
    type Output = AddApplicationCloudWatchLoggingOptionResponse;
    type Error = AddApplicationCloudWatchLoggingOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddApplicationCloudWatchLoggingOptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationCloudWatchLoggingOptionError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for AddApplicationInputRequest {
    type Output = AddApplicationInputResponse;
    type Error = AddApplicationInputError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationInput",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddApplicationInputResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AddApplicationInputError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for AddApplicationInputProcessingConfigurationRequest {
    type Output = AddApplicationInputProcessingConfigurationResponse;
    type Error = AddApplicationInputProcessingConfigurationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddApplicationInputProcessingConfigurationResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationInputProcessingConfigurationError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for AddApplicationOutputRequest {
    type Output = AddApplicationOutputResponse;
    type Error = AddApplicationOutputError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationOutput",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddApplicationOutputResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(AddApplicationOutputError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for AddApplicationReferenceDataSourceRequest {
    type Output = AddApplicationReferenceDataSourceResponse;
    type Error = AddApplicationReferenceDataSourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.AddApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddApplicationReferenceDataSourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddApplicationReferenceDataSourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for CreateApplicationRequest {
    type Output = CreateApplicationResponse;
    type Error = CreateApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.CreateApplication",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateApplicationError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteApplicationRequest {
    type Output = DeleteApplicationResponse;
    type Error = DeleteApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplication",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteApplicationError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for DeleteApplicationCloudWatchLoggingOptionRequest {
    type Output = DeleteApplicationCloudWatchLoggingOptionResponse;
    type Error = DeleteApplicationCloudWatchLoggingOptionError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationCloudWatchLoggingOption",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApplicationCloudWatchLoggingOptionResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationCloudWatchLoggingOptionError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteApplicationInputProcessingConfigurationRequest {
    type Output = DeleteApplicationInputProcessingConfigurationResponse;
    type Error = DeleteApplicationInputProcessingConfigurationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationInputProcessingConfiguration",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
                            if response.status.is_success() {
                                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response).deserialize::<DeleteApplicationInputProcessingConfigurationResponse, _>()
                }))
                            } else {
                                Box::new(response.buffer().from_err().and_then(|response| {
                                    Err(DeleteApplicationInputProcessingConfigurationError::from_response(response))
                                }))
                            }
                        })
    }
}

impl ServiceRequest for DeleteApplicationOutputRequest {
    type Output = DeleteApplicationOutputResponse;
    type Error = DeleteApplicationOutputError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationOutput",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApplicationOutputResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationOutputError::from_response(response))
                }))
            }
        })
    }
}

impl ServiceRequest for DeleteApplicationReferenceDataSourceRequest {
    type Output = DeleteApplicationReferenceDataSourceResponse;
    type Error = DeleteApplicationReferenceDataSourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DeleteApplicationReferenceDataSource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DeleteApplicationReferenceDataSourceResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationReferenceDataSourceError::from_response(
                        response,
                    ))
                }))
            }
        })
    }
}

impl ServiceRequest for DescribeApplicationRequest {
    type Output = DescribeApplicationResponse;
    type Error = DescribeApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DescribeApplication",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeApplicationError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for DiscoverInputSchemaRequest {
    type Output = DiscoverInputSchemaResponse;
    type Error = DiscoverInputSchemaError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.DiscoverInputSchema",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DiscoverInputSchemaResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DiscoverInputSchemaError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for ListApplicationsRequest {
    type Output = ListApplicationsResponse;
    type Error = ListApplicationsError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.ListApplications");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListApplicationsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListApplicationsError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for ListTagsForResourceRequest {
    type Output = ListTagsForResourceResponse;
    type Error = ListTagsForResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.ListTagsForResource",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }
}

impl ServiceRequest for StartApplicationRequest {
    type Output = StartApplicationResponse;
    type Error = StartApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StartApplication");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartApplicationError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for StopApplicationRequest {
    type Output = StopApplicationResponse;
    type Error = StopApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.StopApplication");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StopApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopApplicationError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for TagResourceRequest {
    type Output = TagResourceResponse;
    type Error = TagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.TagResource");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UntagResourceRequest {
    type Output = UntagResourceResponse;
    type Error = UntagResourceError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "KinesisAnalytics_20150814.UntagResource");
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }
}

impl ServiceRequest for UpdateApplicationRequest {
    type Output = UpdateApplicationResponse;
    type Error = UpdateApplicationError;

    fn dispatch(
        self,
        region: &region::Region,
        dispatcher: &impl Dispatcher,
    ) -> RusotoFuture<Self::Output, Self::Error> {
        let mut request = SignedRequest::new("POST", "kinesisanalytics", region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "KinesisAnalytics_20150814.UpdateApplication",
        );
        let encoded = serde_json::to_string(&self).unwrap();
        request.set_payload(Some(encoded));

        dispatcher.dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<UpdateApplicationResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateApplicationError::from_response(response))),
                )
            }
        })
    }
}
