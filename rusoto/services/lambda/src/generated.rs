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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
/// <p>Provides limits of code size and concurrency associated with the current account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccountLimit {
    /// <p>Size, in bytes, of code/dependencies that you can zip into a deployment package (uncompressed zip/jar size) for uploading. The default limit is 250 MB.</p>
    #[serde(rename = "CodeSizeUnzipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_unzipped: Option<i64>,
    /// <p>Size, in bytes, of a single zipped code/dependencies package you can upload for your Lambda function(.zip/.jar file). Try using Amazon S3 for uploading larger files. Default limit is 50 MB.</p>
    #[serde(rename = "CodeSizeZipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_zipped: Option<i64>,
    /// <p>Number of simultaneous executions of your function per region. For more information or to request a limit increase for concurrent executions, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Lambda Function Concurrent Executions</a>. The default limit is 1000.</p>
    #[serde(rename = "ConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_executions: Option<i64>,
    /// <p>Maximum size, in bytes, of a code package you can upload per region. The default size is 75 GB. </p>
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
    /// <p>The number of concurrent executions available to functions that do not have concurrency limits set. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "UnreservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreserved_concurrent_executions: Option<i64>,
}

/// <p>Provides code size usage and function count associated with the current account and region.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AccountUsage {
    /// <p>The number of your account's existing functions per region.</p>
    #[serde(rename = "FunctionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<i64>,
    /// <p>Total size, in bytes, of the account's deployment packages per region.</p>
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddPermissionRequest {
    /// <p>The AWS Lambda action you want to allow in this statement. Each Lambda action is a string starting with <code>lambda:</code> followed by the API name . For example, <code>lambda:CreateFunction</code>. You can use wildcard (<code>lambda:*</code>) to grant permission for all AWS Lambda actions. </p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>A unique token that must be supplied by the principal invoking the function. This is currently only used for Alexa Smart Home functions.</p>
    #[serde(rename = "EventSourceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<String>,
    /// <p>Name of the Lambda function whose resource policy you are updating by adding a new permission.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The principal who is getting this permission. It can be Amazon S3 service Principal (<code>s3.amazonaws.com</code>) if you want Amazon S3 to invoke the function, an AWS account ID if you are granting cross-account permission, or any valid AWS service principal such as <code>sns.amazonaws.com</code>. For example, you might want to allow a custom application in another AWS account to push events to AWS Lambda by invoking your function. </p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>You can use this optional query parameter to describe a qualified ARN using a function version or an alias name. The permission will then apply to the specific qualified ARN. For example, if you specify function version 2 as the qualifier, then permission applies only when request is made using qualified function ARN:</p> <p> <code>arn:aws:lambda:aws-region:acct-id:function:function-name:2</code> </p> <p>If you specify an alias name, for example <code>PROD</code>, then the permission is valid only for requests made using the alias ARN:</p> <p> <code>arn:aws:lambda:aws-region:acct-id:function:function-name:PROD</code> </p> <p>If the qualifier is not specified, the permission is valid only when requests is made using unqualified function ARN.</p> <p> <code>arn:aws:lambda:aws-region:acct-id:function:function-name</code> </p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>This parameter is used for S3 and SES. The AWS account ID (without a hyphen) of the source owner. For example, if the <code>SourceArn</code> identifies a bucket, then this is the bucket owner's account ID. You can use this additional condition to ensure the bucket you specify is owned by a specific account (it is possible the bucket owner deleted the bucket and some other AWS account created the bucket). You can also use this condition to specify all sources (that is, you don't specify the <code>SourceArn</code>) owned by a specific account. </p>
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
    /// <p><p>This is optional; however, when granting permission to invoke your function, you should specify this field with the Amazon Resource Name (ARN) as its value. This ensures that only events generated from the specified source can invoke the function.</p> <important> <p>If you add a permission without providing the source ARN, any AWS account that creates a mapping to your function ARN can send events to invoke your Lambda function.</p> </important></p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>A unique statement identifier.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AddPermissionResponse {
    /// <p>The permission statement you specified in the request. The response returns the same as a string using a backslash ("\") as an escape character in the JSON.</p>
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

/// <p>Provides configuration information about a Lambda function version alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AliasConfiguration {
    /// <p>Lambda function ARN that is qualified using the alias name as the suffix. For example, if you create an alias called <code>BETA</code> that points to a helloworld function version, the ARN is <code>arn:aws:lambda:aws-regions:acct-id:function:helloworld:BETA</code>.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>Alias description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Function version to which the alias points.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Alias name.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Represents the latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Specifies an additional function versions the alias points to, allowing you to dictate what percentage of traffic will invoke each version. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// <p>The parent object that implements what percentage of traffic will invoke each function version. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasRoutingConfiguration {
    /// <p>Set this value to dictate what percentage of traffic will invoke the updated function version. If set to an empty string, 100 percent of traffic will invoke <code>function-version</code>. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
    #[serde(rename = "AdditionalVersionWeights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_weights: Option<::std::collections::HashMap<String, f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Concurrency {
    /// <p>The number of concurrent executions reserved for this function. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>Description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the Lambda function for which you want to create an alias. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Lambda function version for which you are creating the alias.</p>
    #[serde(rename = "FunctionVersion")]
    pub function_version: String,
    /// <p>Name for the alias you are creating.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Specifies an additional version your alias can point to, allowing you to dictate what percentage of traffic will invoke each version. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEventSourceMappingRequest {
    /// <p>The largest number of records that AWS Lambda will retrieve from your event source at the time of invoking your function. Your function receives an event with all the retrieved records. The default for Amazon Kinesis and Amazon DynamoDB is 100 records. For SQS, the default is 1.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>Indicates whether AWS Lambda should begin polling the event source. By default, <code>Enabled</code> is true. </p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the event source. Any record added to this source could cause AWS Lambda to invoke your Lambda function, it depends on the <code>BatchSize</code>. AWS Lambda POSTs the event's records to your Lambda function as JSON.</p>
    #[serde(rename = "EventSourceArn")]
    pub event_source_arn: String,
    /// <p>The Lambda function to invoke when AWS Lambda detects an event on the stream.</p> <p> You can specify the function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). </p> <p> If you are using versioning, you can also provide a qualified function ARN (ARN that is qualified with function version or alias name as suffix). For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a> </p> <p>AWS Lambda also allows you to specify only the function name with the account ID qualifier (for example, <code>account-id:Thumbnail</code>). </p> <p>Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The position in the DynamoDB or Kinesis stream where AWS Lambda should start reading. For more information, see <a href="http://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetShardIterator.html#Kinesis-GetShardIterator-request-ShardIteratorType">GetShardIterator</a> in the <i>Amazon Kinesis API Reference Guide</i> or <a href="http://docs.aws.amazon.com/amazondynamodb/latest/APIReference/API_streams_GetShardIterator.html">GetShardIterator</a> in the <i>Amazon DynamoDB API Reference Guide</i>. The <code>AT_TIMESTAMP</code> value is supported only for <a href="http://docs.aws.amazon.com/streams/latest/dev/amazon-kinesis-streams.html">Kinesis streams</a>. </p>
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    /// <p>The timestamp of the data record from which to start reading. Used with <a href="http://docs.aws.amazon.com/kinesis/latest/APIReference/API_GetShardIterator.html#Kinesis-GetShardIterator-request-ShardIteratorType">shard iterator type</a> AT_TIMESTAMP. If a record with this exact timestamp does not exist, the iterator returned is for the next (later) record. If the timestamp is older than the current trim horizon, the iterator returned is for the oldest untrimmed data record (TRIM_HORIZON). Valid only for <a href="http://docs.aws.amazon.com/streams/latest/dev/amazon-kinesis-streams.html">Kinesis streams</a>. </p>
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionRequest {
    /// <p>The code for the Lambda function.</p>
    #[serde(rename = "Code")]
    pub code: FunctionCode,
    /// <p>The parent object that contains the target ARN (Amazon Resource Name) of an Amazon SQS queue or Amazon SNS topic. For more information, see <a>dlq</a>. </p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>A short, user-defined function description. Lambda does not use this value. Assign a meaningful description as you see fit.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// <p>The name you want to assign to the function you are uploading. The function names appear in the console and are returned in the <a>ListFunctions</a> API. Function names are used to specify functions to other AWS Lambda API operations, such as <a>Invoke</a>. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The function within your code that Lambda calls to begin execution. For Node.js, it is the <i>module-name</i>.<i>export</i> value in your function. For Java, it can be <code>package.class-name::handler</code> or <code>package.class-name</code>. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/java-programming-model-handler-types.html">Lambda Function Handler (Java)</a>. </p>
    #[serde(rename = "Handler")]
    pub handler: String,
    /// <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt your function's environment variables. If not provided, AWS Lambda will use a default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The amount of memory, in MB, your Lambda function is given. Lambda uses this memory size to infer the amount of CPU and memory allocated to your function. Your function use-case determines your CPU and memory requirements. For example, a database operation might need less memory compared to an image processing function. The default value is 128 MB. The value must be a multiple of 64 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>This boolean parameter can be used to request AWS Lambda to create the Lambda function and publish a version as an atomic operation.</p>
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Lambda assumes when it executes your function to access any other Amazon Web Services (AWS) resources. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/lambda-introduction.html">AWS Lambda: How it Works</a>. </p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p><p>The runtime environment for the Lambda function you are uploading.</p> <p>To use the Python runtime v3.6, set the value to &quot;python3.6&quot;. To use the Python runtime v2.7, set the value to &quot;python2.7&quot;. To use the Node.js runtime v6.10, set the value to &quot;nodejs6.10&quot;. To use the Node.js runtime v4.3, set the value to &quot;nodejs4.3&quot;. To use the .NET Core runtime v1.0, set the value to &quot;dotnetcore1.0&quot;. To use the .NET Core runtime v2.0, set the value to &quot;dotnetcore2.0&quot;.</p> <note> <p>Node v0.10.42 is currently marked as deprecated. You must migrate existing functions to the newer Node.js runtime versions available on AWS Lambda (nodejs4.3 or nodejs6.10) as soon as possible. Failure to do so will result in an invalid parameter error being returned. Note that you will have to follow this procedure for each region that contains functions written in the Node v0.10.42 runtime.</p> </note></p>
    #[serde(rename = "Runtime")]
    pub runtime: String,
    /// <p>The list of tags (key-value pairs) assigned to the new function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The function execution time at which Lambda should terminate the function. Because the execution time has cost implications, we recommend you set this value based on your expected execution time. The default is 3 seconds.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The parent object that contains your function's tracing settings.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    /// <p>If your Lambda function accesses resources in a VPC, you provide this parameter identifying the list of security group IDs and subnet IDs. These must belong to the same VPC. You must provide at least one security group and one subnet ID.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic you specify as your Dead Letter Queue (DLQ). For more information, see <a>dlq</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    /// <p>The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic you specify as your Dead Letter Queue (DLQ). <a>dlq</a>. For more information, see <a>dlq</a>. </p>
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The Lambda function name for which the alias is created. Deleting an alias does not delete the function version to which it is pointing. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Name of the alias to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventSourceMappingRequest {
    /// <p>The event source mapping ID.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionConcurrencyRequest {
    /// <p>The name of the function you are removing concurrent execution limits from. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionRequest {
    /// <p>The Lambda function to delete.</p> <p> You can specify the function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). If you are using versioning, you can also provide a qualified function ARN (ARN that is qualified with function version or alias name as suffix). AWS Lambda also allows you to specify only the function name with the account ID qualifier (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Using this optional parameter you can specify a function version (but not the <code>$LATEST</code> version) to direct AWS Lambda to delete a specific function version. If the function version has one or more aliases pointing to it, you will get an error because you cannot have aliases pointing to it. You can delete any function version but not the <code>$LATEST</code>, that is, you cannot specify <code>$LATEST</code> as the value of this parameter. The <code>$LATEST</code> version can be deleted only when you want to delete all the function versions and aliases.</p> <p>You can only specify a function version, not an alias name, using this parameter. You cannot delete a function version using its alias.</p> <p>If you don't specify this parameter, AWS Lambda will delete the function, including all of its versions and aliases.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p>The parent object that contains your environment's configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Environment {
    /// <p>The key-value pairs that represent your environment's configuration settings.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The parent object that contains error information associated with your configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentError {
    /// <p>The error code returned by the environment error object.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message returned by the environment error object.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The parent object returned that contains your environment's configuration settings or any error information associated with your configuration settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EnvironmentResponse {
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EnvironmentError>,
    /// <p>The key-value pairs returned that represent your environment's configuration settings or error information.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Describes mapping between an Amazon Kinesis or DynamoDB stream or an Amazon SQS queue and a Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct EventSourceMappingConfiguration {
    /// <p>The largest number of records that AWS Lambda will retrieve from your event source at the time of invoking your function. Your function receives an event with all the retrieved records.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis or DynamoDB stream or the SQS queue that is the source of events.</p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The Lambda function to invoke when AWS Lambda detects an event on the poll-based source.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The UTC time string indicating the last time the event mapping was updated.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The result of the last AWS Lambda invocation of your Lambda function.</p>
    #[serde(rename = "LastProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processing_result: Option<String>,
    /// <p>The state of the event source mapping. It can be <code>Creating</code>, <code>Enabled</code>, <code>Disabled</code>, <code>Enabling</code>, <code>Disabling</code>, <code>Updating</code>, or <code>Deleting</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason the event source mapping is in its current state. It is either user-requested or an AWS Lambda-initiated state transition.</p>
    #[serde(rename = "StateTransitionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_reason: Option<String>,
    /// <p>The AWS Lambda assigned opaque identifier for the mapping.</p>
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// <p>The code for the Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FunctionCode {
    /// <p>Amazon S3 bucket name where the .zip file containing your deployment package is stored. This bucket must reside in the same AWS region where you are creating the Lambda function.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 object (the deployment package) key name you want to upload.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>The Amazon S3 object (the deployment package) version you want to upload.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The contents of your zip file containing your deployment package. If you are using the web API directly, the contents of the zip file must be base64-encoded. If you are using the AWS SDKs or the AWS CLI, the SDKs or CLI will do the encoding for you. For more information about creating a .zip file, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role.html">Execution Permissions</a> in the <b>AWS Lambda Developer Guide</b>. </p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub zip_file: Option<Vec<u8>>,
}

/// <p>The object for the Lambda function location.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FunctionCodeLocation {
    /// <p>The presigned URL you can use to download the function's .zip file that you previously uploaded. The URL is valid for up to 10 minutes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The repository from which you can download the function.</p>
    #[serde(rename = "RepositoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
}

/// <p>A complex type that describes function metadata.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct FunctionConfiguration {
    /// <p>It is the SHA256 hash of your function deployment package.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The size, in bytes, of the function .zip file you uploaded.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    /// <p>The parent object that contains the target ARN (Amazon Resource Name) of an Amazon SQS queue or Amazon SNS topic. For more information, see <a>dlq</a>. </p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>The user-provided description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The parent object that contains your environment's configuration settings.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentResponse>,
    /// <p>The Amazon Resource Name (ARN) assigned to the function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The name of the function. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The function Lambda calls to begin executing your function.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt your function's environment variables. If empty, it means you are using the AWS Lambda default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The time stamp of the last time you updated the function. The time stamp is conveyed as a string complying with ISO-8601 in this way YYYY-MM-DDThh:mm:ssTZD (e.g., 1997-07-16T19:20:30+01:00). For more information, see <a href="https://www.w3.org/TR/NOTE-datetime">Date and Time Formats</a>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>Returns the ARN (Amazon Resource Name) of the master function.</p>
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    /// <p>The memory size, in MB, you configured for the function. Must be a multiple of 64 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>Represents the latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Lambda assumes when it executes your function to access any other Amazon Web Services (AWS) resources.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The runtime environment for the Lambda function.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The function execution time at which Lambda should terminate the function. Because the execution time has cost implications, we recommend you set this value based on your expected execution time. The default is 3 seconds.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The parent object that contains your function's tracing settings.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfigResponse>,
    /// <p>The version of the Lambda function.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>VPC configuration associated with your Lambda function.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetAccountSettingsResponse {
    #[serde(rename = "AccountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_limit: Option<AccountLimit>,
    #[serde(rename = "AccountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_usage: Option<AccountUsage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAliasRequest {
    /// <p>Function name for which the alias is created. An alias is a subresource that exists only in the context of an existing Lambda function so you must specify the function name. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Name of the alias for which you want to retrieve information.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEventSourceMappingRequest {
    /// <p>The AWS Lambda assigned ID of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionConfigurationRequest {
    /// <p>The name of the Lambda function for which you want to retrieve the configuration information.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Using this optional parameter you can specify a function version or an alias name. If you specify function version, the API uses qualified function ARN and returns information about the specific function version. If you specify an alias name, the API uses the alias ARN and returns information about the function version to which the alias points.</p> <p>If you don't specify this parameter, the API uses unqualified function ARN, and returns information about the <code>$LATEST</code> function version.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionRequest {
    /// <p>The Lambda function name.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Use this optional parameter to specify a function version or an alias name. If you specify function version, the API uses qualified function ARN for the request and returns information about the specific Lambda function version. If you specify an alias name, the API uses the alias ARN and returns information about the function version to which the alias points. If you don't provide this parameter, the API uses unqualified function ARN and returns information about the <code>$LATEST</code> version of the Lambda function. </p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p>This response contains the object for the Lambda function location (see <a>FunctionCodeLocation</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetFunctionResponse {
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FunctionCodeLocation>,
    /// <p>The concurrent execution limit set for this function. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "Concurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<FunctionConfiguration>,
    /// <p>Returns the list of tags associated with the function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPolicyRequest {
    /// <p>Function name whose resource policy you want to retrieve.</p> <p> You can specify the function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). If you are using versioning, you can also provide a qualified function ARN (ARN that is qualified with function version or alias name as suffix). AWS Lambda also allows you to specify only the function name with the account ID qualifier (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>You can specify this optional query parameter to specify a function version or an alias name in which case this API will return all permissions associated with the specific qualified ARN. If you don't provide this parameter, the API will return permissions that apply to the unqualified function ARN.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct GetPolicyResponse {
    /// <p>The resource policy associated with the specified function. The response returns the same as a string using a backslash ("\") as an escape character in the JSON.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>Represents the latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvocationRequest {
    /// <p>Using the <code>ClientContext</code> you can pass client-specific information to the Lambda function you are invoking. You can then process the client information in your Lambda function as you choose through the context variable. For an example of a <code>ClientContext</code> JSON, see <a href="http://docs.aws.amazon.com/mobileanalytics/latest/ug/PutEvents.html">PutEvents</a> in the <i>Amazon Mobile Analytics API Reference and User Guide</i>.</p> <p>The ClientContext JSON must be base64-encoded and has a maximum size of 3583 bytes.</p>
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    /// <p>The Lambda function name.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>By default, the <code>Invoke</code> API assumes <code>RequestResponse</code> invocation type. You can optionally request asynchronous execution by specifying <code>Event</code> as the <code>InvocationType</code>. You can also use this parameter to request AWS Lambda to not execute the function but do some verification, such as if the caller is authorized to invoke the function and if the inputs are valid. You request this by specifying <code>DryRun</code> as the <code>InvocationType</code>. This is useful in a cross-account scenario when you want to verify access to a function without running it. </p>
    #[serde(rename = "InvocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
    /// <p>You can set this optional parameter to <code>Tail</code> in the request only if you specify the <code>InvocationType</code> parameter with value <code>RequestResponse</code>. In this case, AWS Lambda returns the base64-encoded last 4 KB of log data produced by your Lambda function in the <code>x-amz-log-result</code> header. </p>
    #[serde(rename = "LogType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    /// <p>JSON that you want to provide to your Lambda function as input.</p>
    #[serde(rename = "Payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub payload: Option<Vec<u8>>,
    /// <p>You can use this optional parameter to specify a Lambda function version or alias name. If you specify a function version, the API uses the qualified function ARN to invoke a specific Lambda function. If you specify an alias name, the API uses the alias ARN to invoke the Lambda function version to which the alias points.</p> <p>If you don't provide this parameter, then the API uses unqualified function ARN which results in invocation of the <code>$LATEST</code> version.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p>Upon success, returns an empty response. Otherwise, throws an exception.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvocationResponse {
    /// <p>The function version that has been executed. This value is returned only if the invocation type is <code>RequestResponse</code>. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
    pub executed_version: Option<String>,
    /// <p>Indicates whether an error occurred while executing the Lambda function. If an error occurred this field will have one of two values; <code>Handled</code> or <code>Unhandled</code>. <code>Handled</code> errors are errors that are reported by the function while the <code>Unhandled</code> errors are those detected and reported by AWS Lambda. Unhandled errors include out of memory errors and function timeouts. For information about how to report an <code>Handled</code> error, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/programming-model.html">Programming Model</a>. </p>
    pub function_error: Option<String>,
    /// <p> It is the base64-encoded logs for the Lambda function invocation. This is present only if the invocation type is <code>RequestResponse</code> and the logs were requested. </p>
    pub log_result: Option<String>,
    /// <p> It is the JSON representation of the object returned by the Lambda function. This is present only if the invocation type is <code>RequestResponse</code>. </p> <p>In the event of a function error this field contains a message describing the error. For the <code>Handled</code> errors the Lambda function will report this message. For <code>Unhandled</code> errors AWS Lambda reports the message. </p>
    pub payload: Option<Vec<u8>>,
    /// <p>The HTTP status code will be in the 200 range for successful request. For the <code>RequestResponse</code> invocation type this status code will be 200. For the <code>Event</code> invocation type this status code will be 202. For the <code>DryRun</code> invocation type the status code will be 204. </p>
    pub status_code: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvokeAsyncRequest {
    /// <p>The Lambda function name. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>JSON that you want to provide to your Lambda function as input.</p>
    #[serde(rename = "InvokeArgs")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub invoke_args: Vec<u8>,
}

/// <p>Upon success, it returns empty response. Otherwise, throws an exception.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct InvokeAsyncResponse {
    /// <p>It will be 202 upon success.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAliasesRequest {
    /// <p>Lambda function name for which the alias is created. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>If you specify this optional parameter, the API returns only the aliases that are pointing to the specific Lambda function version, otherwise the API returns all of the aliases created for the Lambda function.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Optional string. An opaque pagination token returned from a previous <code>ListAliases</code> operation. If present, indicates where to continue the listing.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Optional integer. Specifies the maximum number of aliases to return in response. This parameter value must be greater than 0.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasConfiguration>>,
    /// <p>A string, present if there are more aliases.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEventSourceMappingsRequest {
    /// <p>The Amazon Resource Name (ARN) of the Amazon Kinesis or DynamoDB stream, or an SQS queue. (This parameter is optional.)</p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The name of the Lambda function.</p> <p> You can specify the function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). If you are using versioning, you can also provide a qualified function ARN (ARN that is qualified with function version or alias name as suffix). AWS Lambda also allows you to specify only the function name with the account ID qualifier (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>Optional string. An opaque pagination token returned from a previous <code>ListEventSourceMappings</code> operation. If present, specifies to continue the list from where the returning call left off. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Optional integer. Specifies the maximum number of event sources to return in response. This value must be greater than 0.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p>Contains a list of event sources (see <a>EventSourceMappingConfiguration</a>)</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListEventSourceMappingsResponse {
    /// <p>An array of <code>EventSourceMappingConfiguration</code> objects.</p>
    #[serde(rename = "EventSourceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_mappings: Option<Vec<EventSourceMappingConfiguration>>,
    /// <p>A string, present if there are more event source mappings.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionsRequest {
    /// <p>Optional string. If not specified, only the unqualified functions ARNs (Amazon Resource Names) will be returned.</p> <p>Valid value:</p> <p> <code>ALL</code>: Will return all versions, including <code>$LATEST</code> which will have fully qualified ARNs (Amazon Resource Names).</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Optional string. An opaque pagination token returned from a previous <code>ListFunctions</code> operation. If present, indicates where to continue the listing. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Optional string. If not specified, will return only regular function versions (i.e., non-replicated versions).</p> <p>Valid values are:</p> <p>The region from which the functions are replicated. For example, if you specify <code>us-east-1</code>, only functions replicated from that region will be returned.</p> <p> <code>ALL</code>: Will return all functions from any region. If specified, you also must specify a valid FunctionVersion parameter.</p>
    #[serde(rename = "MasterRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_region: Option<String>,
    /// <p>Optional integer. Specifies the maximum number of AWS Lambda functions to return in response. This parameter value must be greater than 0.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p>Contains a list of AWS Lambda function configurations (see <a>FunctionConfiguration</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListFunctionsResponse {
    /// <p>A list of Lambda functions.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<FunctionConfiguration>>,
    /// <p>A string, present if there are more functions.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The ARN (Amazon Resource Name) of the function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListTagsResponse {
    /// <p>The list of tags assigned to the function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVersionsByFunctionRequest {
    /// <p>Function name whose versions to list. You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p> Optional string. An opaque pagination token returned from a previous <code>ListVersionsByFunction</code> operation. If present, indicates where to continue the listing. </p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Optional integer. Specifies the maximum number of AWS Lambda function versions to return in response. This parameter value must be greater than 0.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ListVersionsByFunctionResponse {
    /// <p>A string, present if there are more function versions.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of Lambda function versions.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<FunctionConfiguration>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublishVersionRequest {
    /// <p>The SHA256 hash of the deployment package you want to publish. This provides validation on the code you are publishing. If you provide this parameter, the value must match the SHA256 of the $LATEST version for the publication to succeed. You can use the <b>DryRun</b> parameter of <a>UpdateFunctionCode</a> to verify the hash value that will be returned before publishing your new version.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The description for the version you are publishing. If not provided, AWS Lambda copies the description from the $LATEST version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Lambda function name. You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutFunctionConcurrencyRequest {
    /// <p>The name of the function you are setting concurrent execution limits on. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The concurrent execution limit reserved for this function. For more information, see <a>concurrent-executions</a>.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: i64,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemovePermissionRequest {
    /// <p>Lambda function whose resource policy you want to remove a permission from.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>You can specify this optional parameter to remove permission associated with a specific function version or function alias. If you don't specify this parameter, the API removes permission associated with the unqualified function ARN.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Statement ID of the permission to remove.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN (Amazon Resource Name) of the Lambda function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>The list of tags (key-value pairs) you are assigning to the Lambda function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The parent object that contains your function's tracing settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TracingConfig {
    /// <p>Can be either PassThrough or Active. If PassThrough, Lambda will only trace the request from an upstream service if it contains a tracing header with "sampled=1". If Active, Lambda will respect any tracing header it receives from an upstream service. If no tracing header is received, Lambda will call X-Ray for a tracing decision.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// <p>Parent object of the tracing information associated with your Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct TracingConfigResponse {
    /// <p>The tracing mode associated with your Lambda function.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN (Amazon Resource Name) of the function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>The list of tag keys to be deleted from the function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAliasRequest {
    /// <p>You can change the description of the alias using this parameter.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The function name for which the alias is created. Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Using this parameter you can change the Lambda function version to which the alias points.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>The alias name.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Specifies an additional version your alias can point to, allowing you to dictate what percentage of traffic will invoke each version. For more information, see <a>lambda-traffic-shifting-using-aliases</a>.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEventSourceMappingRequest {
    /// <p>The maximum number of stream records that can be sent to your Lambda function for a single invocation.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>Specifies whether AWS Lambda should actively poll the stream or not. If disabled, AWS Lambda will not poll the stream.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The Lambda function to which you want the stream records sent.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p> <p>If you are using versioning, you can also provide a qualified function ARN (ARN that is qualified with function version or alias name as suffix). For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a> </p> <p>Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 character in length.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The event source mapping identifier.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionCodeRequest {
    /// <p>This boolean parameter can be used to test your request to AWS Lambda to update the Lambda function and publish a version as an atomic operation. It will do all necessary computation and validation of your code but will not upload it or a publish a version. Each time this operation is invoked, the <code>CodeSha256</code> hash value of the provided code will also be computed and returned in the response.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The existing Lambda function name whose code you want to replace.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 characters in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>This boolean parameter can be used to request AWS Lambda to update the Lambda function and publish a version as an atomic operation.</p>
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Amazon S3 bucket name where the .zip file containing your deployment package is stored. This bucket must reside in the same AWS Region where you are creating the Lambda function.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 object (the deployment package) key name you want to upload.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>The Amazon S3 object (the deployment package) version you want to upload.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The contents of your zip file containing your deployment package. If you are using the web API directly, the contents of the zip file must be base64-encoded. If you are using the AWS SDKs or the AWS CLI, the SDKs or CLI will do the encoding for you. For more information about creating a .zip file, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role.html">Execution Permissions</a>. </p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default,
    )]
    pub zip_file: Option<Vec<u8>>,
}

/// <p><p/></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionConfigurationRequest {
    /// <p>The parent object that contains the target ARN (Amazon Resource Name) of an Amazon SQS queue or Amazon SNS topic. For more information, see <a>dlq</a>. </p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>A short user-defined function description. AWS Lambda does not use this value. Assign a meaningful description as you see fit.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The parent object that contains your environment's configuration settings.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// <p>The name of the Lambda function.</p> <p> You can specify a function name (for example, <code>Thumbnail</code>) or you can specify Amazon Resource Name (ARN) of the function (for example, <code>arn:aws:lambda:us-west-2:account-id:function:ThumbNail</code>). AWS Lambda also allows you to specify a partial ARN (for example, <code>account-id:Thumbnail</code>). Note that the length constraint applies only to the ARN. If you specify only the function name, it is limited to 64 character in length. </p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The function that Lambda calls to begin executing your function. For Node.js, it is the <code>module-name.export</code> value in your function. </p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt your function's environment variables. If you elect to use the AWS Lambda default service key, pass in an empty string ("") for this parameter.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The amount of memory, in MB, your Lambda function is given. AWS Lambda uses this memory size to infer the amount of CPU allocated to your function. Your function use-case determines your CPU and memory requirements. For example, a database operation might need less memory compared to an image processing function. The default value is 128 MB. The value must be a multiple of 64 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>An optional value you can use to ensure you are updating the latest update of the function version or alias. If the <code>RevisionID</code> you pass doesn't match the latest <code>RevisionId</code> of the function or alias, it will fail with an error message, advising you to retrieve the latest function version or alias <code>RevisionID</code> using either or .</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Lambda will assume when it executes your function.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p><p>The runtime environment for the Lambda function.</p> <p>To use the Python runtime v3.6, set the value to &quot;python3.6&quot;. To use the Python runtime v2.7, set the value to &quot;python2.7&quot;. To use the Node.js runtime v6.10, set the value to &quot;nodejs6.10&quot;. To use the Node.js runtime v4.3, set the value to &quot;nodejs4.3&quot;. To use the .NET Core runtime v1.0, set the value to &quot;dotnetcore1.0&quot;. To use the .NET Core runtime v2.0, set the value to &quot;dotnetcore2.0&quot;.</p> <note> <p>Node v0.10.42 is currently marked as deprecated. You must migrate existing functions to the newer Node.js runtime versions available on AWS Lambda (nodejs4.3 or nodejs6.10) as soon as possible. Failure to do so will result in an invalid parameter error being returned. Note that you will have to follow this procedure for each region that contains functions written in the Node v0.10.42 runtime.</p> </note></p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The function execution time at which AWS Lambda should terminate the function. Because the execution time has cost implications, we recommend you set this value based on your expected execution time. The default is 3 seconds.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The parent object that contains your function's tracing settings.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>If your Lambda function accesses resources in a VPC, you provide this parameter identifying the list of security group IDs and subnet IDs. These must belong to the same VPC. You must provide at least one security group and one subnet ID.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VpcConfig {
    /// <p>A list of one or more security groups IDs in your VPC.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of one or more subnet IDs in your VPC.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>VPC configuration associated with your Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct VpcConfigResponse {
    /// <p>A list of security group IDs associated with the Lambda function.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of subnet IDs associated with the Lambda function.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The VPC ID associated with you Lambda function.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// Errors returned by AddPermission
#[derive(Debug, PartialEq)]
pub enum AddPermissionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>Lambda function access policy is limited to 20 KB.</p>
    PolicyLengthExceeded(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AddPermissionError {
    pub fn from_body(body: &str) -> AddPermissionError {
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
                    "InvalidParameterValueException" => {
                        AddPermissionError::InvalidParameterValue(String::from(error_message))
                    }
                    "PolicyLengthExceededException" => {
                        AddPermissionError::PolicyLengthExceeded(String::from(error_message))
                    }
                    "PreconditionFailedException" => {
                        AddPermissionError::PreconditionFailed(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        AddPermissionError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        AddPermissionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => AddPermissionError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        AddPermissionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        AddPermissionError::Validation(error_message.to_string())
                    }
                    _ => AddPermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => AddPermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AddPermissionError {
    fn from(err: serde_json::error::Error) -> AddPermissionError {
        AddPermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AddPermissionError {
    fn from(err: CredentialsError) -> AddPermissionError {
        AddPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddPermissionError {
    fn from(err: HttpDispatchError) -> AddPermissionError {
        AddPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddPermissionError {
    fn from(err: io::Error) -> AddPermissionError {
        AddPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddPermissionError {
    fn description(&self) -> &str {
        match *self {
            AddPermissionError::InvalidParameterValue(ref cause) => cause,
            AddPermissionError::PolicyLengthExceeded(ref cause) => cause,
            AddPermissionError::PreconditionFailed(ref cause) => cause,
            AddPermissionError::ResourceConflict(ref cause) => cause,
            AddPermissionError::ResourceNotFound(ref cause) => cause,
            AddPermissionError::Service(ref cause) => cause,
            AddPermissionError::TooManyRequests(ref cause) => cause,
            AddPermissionError::Validation(ref cause) => cause,
            AddPermissionError::Credentials(ref err) => err.description(),
            AddPermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            AddPermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateAliasError {
    pub fn from_body(body: &str) -> CreateAliasError {
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
                    "InvalidParameterValueException" => {
                        CreateAliasError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        CreateAliasError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => CreateAliasError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateAliasError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateAliasError::Validation(error_message.to_string())
                    }
                    _ => CreateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAliasError {
    fn from(err: CredentialsError) -> CreateAliasError {
        CreateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAliasError {
    fn from(err: HttpDispatchError) -> CreateAliasError {
        CreateAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAliasError {
    fn from(err: io::Error) -> CreateAliasError {
        CreateAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAliasError {
    fn description(&self) -> &str {
        match *self {
            CreateAliasError::InvalidParameterValue(ref cause) => cause,
            CreateAliasError::ResourceConflict(ref cause) => cause,
            CreateAliasError::ResourceNotFound(ref cause) => cause,
            CreateAliasError::Service(ref cause) => cause,
            CreateAliasError::TooManyRequests(ref cause) => cause,
            CreateAliasError::Validation(ref cause) => cause,
            CreateAliasError::Credentials(ref err) => err.description(),
            CreateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum CreateEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateEventSourceMappingError {
    pub fn from_body(body: &str) -> CreateEventSourceMappingError {
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
                    "InvalidParameterValueException" => {
                        CreateEventSourceMappingError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceConflictException" => {
                        CreateEventSourceMappingError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateEventSourceMappingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        CreateEventSourceMappingError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        CreateEventSourceMappingError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateEventSourceMappingError::Validation(error_message.to_string())
                    }
                    _ => CreateEventSourceMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateEventSourceMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> CreateEventSourceMappingError {
        CreateEventSourceMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateEventSourceMappingError {
    fn from(err: CredentialsError) -> CreateEventSourceMappingError {
        CreateEventSourceMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateEventSourceMappingError {
    fn from(err: HttpDispatchError) -> CreateEventSourceMappingError {
        CreateEventSourceMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateEventSourceMappingError {
    fn from(err: io::Error) -> CreateEventSourceMappingError {
        CreateEventSourceMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateEventSourceMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateEventSourceMappingError {
    fn description(&self) -> &str {
        match *self {
            CreateEventSourceMappingError::InvalidParameterValue(ref cause) => cause,
            CreateEventSourceMappingError::ResourceConflict(ref cause) => cause,
            CreateEventSourceMappingError::ResourceNotFound(ref cause) => cause,
            CreateEventSourceMappingError::Service(ref cause) => cause,
            CreateEventSourceMappingError::TooManyRequests(ref cause) => cause,
            CreateEventSourceMappingError::Validation(ref cause) => cause,
            CreateEventSourceMappingError::Credentials(ref err) => err.description(),
            CreateEventSourceMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateEventSourceMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateFunction
#[derive(Debug, PartialEq)]
pub enum CreateFunctionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateFunctionError {
    pub fn from_body(body: &str) -> CreateFunctionError {
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
                    "CodeStorageExceededException" => {
                        CreateFunctionError::CodeStorageExceeded(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        CreateFunctionError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        CreateFunctionError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateFunctionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => CreateFunctionError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        CreateFunctionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        CreateFunctionError::Validation(error_message.to_string())
                    }
                    _ => CreateFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateFunctionError {
    fn from(err: serde_json::error::Error) -> CreateFunctionError {
        CreateFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateFunctionError {
    fn from(err: CredentialsError) -> CreateFunctionError {
        CreateFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateFunctionError {
    fn from(err: HttpDispatchError) -> CreateFunctionError {
        CreateFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateFunctionError {
    fn from(err: io::Error) -> CreateFunctionError {
        CreateFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateFunctionError {
    fn description(&self) -> &str {
        match *self {
            CreateFunctionError::CodeStorageExceeded(ref cause) => cause,
            CreateFunctionError::InvalidParameterValue(ref cause) => cause,
            CreateFunctionError::ResourceConflict(ref cause) => cause,
            CreateFunctionError::ResourceNotFound(ref cause) => cause,
            CreateFunctionError::Service(ref cause) => cause,
            CreateFunctionError::TooManyRequests(ref cause) => cause,
            CreateFunctionError::Validation(ref cause) => cause,
            CreateFunctionError::Credentials(ref err) => err.description(),
            CreateFunctionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteAliasError {
    pub fn from_body(body: &str) -> DeleteAliasError {
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
                    "InvalidParameterValueException" => {
                        DeleteAliasError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServiceException" => DeleteAliasError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteAliasError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteAliasError::Validation(error_message.to_string())
                    }
                    _ => DeleteAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteAliasError {
    fn from(err: serde_json::error::Error) -> DeleteAliasError {
        DeleteAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAliasError {
    fn from(err: CredentialsError) -> DeleteAliasError {
        DeleteAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAliasError {
    fn from(err: HttpDispatchError) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAliasError {
    fn from(err: io::Error) -> DeleteAliasError {
        DeleteAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAliasError {
    fn description(&self) -> &str {
        match *self {
            DeleteAliasError::InvalidParameterValue(ref cause) => cause,
            DeleteAliasError::Service(ref cause) => cause,
            DeleteAliasError::TooManyRequests(ref cause) => cause,
            DeleteAliasError::Validation(ref cause) => cause,
            DeleteAliasError::Credentials(ref err) => err.description(),
            DeleteAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum DeleteEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSoure Mapping in CREATING, or tried to delete a EventSoure mapping currently in the UPDATING state. </p>
    ResourceInUse(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteEventSourceMappingError {
    pub fn from_body(body: &str) -> DeleteEventSourceMappingError {
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
                    "InvalidParameterValueException" => {
                        DeleteEventSourceMappingError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceInUseException" => {
                        DeleteEventSourceMappingError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteEventSourceMappingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        DeleteEventSourceMappingError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteEventSourceMappingError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteEventSourceMappingError::Validation(error_message.to_string())
                    }
                    _ => DeleteEventSourceMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteEventSourceMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> DeleteEventSourceMappingError {
        DeleteEventSourceMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteEventSourceMappingError {
    fn from(err: CredentialsError) -> DeleteEventSourceMappingError {
        DeleteEventSourceMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteEventSourceMappingError {
    fn from(err: HttpDispatchError) -> DeleteEventSourceMappingError {
        DeleteEventSourceMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteEventSourceMappingError {
    fn from(err: io::Error) -> DeleteEventSourceMappingError {
        DeleteEventSourceMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteEventSourceMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteEventSourceMappingError {
    fn description(&self) -> &str {
        match *self {
            DeleteEventSourceMappingError::InvalidParameterValue(ref cause) => cause,
            DeleteEventSourceMappingError::ResourceInUse(ref cause) => cause,
            DeleteEventSourceMappingError::ResourceNotFound(ref cause) => cause,
            DeleteEventSourceMappingError::Service(ref cause) => cause,
            DeleteEventSourceMappingError::TooManyRequests(ref cause) => cause,
            DeleteEventSourceMappingError::Validation(ref cause) => cause,
            DeleteEventSourceMappingError::Credentials(ref err) => err.description(),
            DeleteEventSourceMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteEventSourceMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFunction
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFunctionError {
    pub fn from_body(body: &str) -> DeleteFunctionError {
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
                    "InvalidParameterValueException" => {
                        DeleteFunctionError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        DeleteFunctionError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteFunctionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => DeleteFunctionError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        DeleteFunctionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFunctionError::Validation(error_message.to_string())
                    }
                    _ => DeleteFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFunctionError {
    fn from(err: serde_json::error::Error) -> DeleteFunctionError {
        DeleteFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFunctionError {
    fn from(err: CredentialsError) -> DeleteFunctionError {
        DeleteFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFunctionError {
    fn from(err: HttpDispatchError) -> DeleteFunctionError {
        DeleteFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFunctionError {
    fn from(err: io::Error) -> DeleteFunctionError {
        DeleteFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFunctionError {
    fn description(&self) -> &str {
        match *self {
            DeleteFunctionError::InvalidParameterValue(ref cause) => cause,
            DeleteFunctionError::ResourceConflict(ref cause) => cause,
            DeleteFunctionError::ResourceNotFound(ref cause) => cause,
            DeleteFunctionError::Service(ref cause) => cause,
            DeleteFunctionError::TooManyRequests(ref cause) => cause,
            DeleteFunctionError::Validation(ref cause) => cause,
            DeleteFunctionError::Credentials(ref err) => err.description(),
            DeleteFunctionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteFunctionConcurrency
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionConcurrencyError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteFunctionConcurrencyError {
    pub fn from_body(body: &str) -> DeleteFunctionConcurrencyError {
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
                    "InvalidParameterValueException" => {
                        DeleteFunctionConcurrencyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DeleteFunctionConcurrencyError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        DeleteFunctionConcurrencyError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteFunctionConcurrencyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteFunctionConcurrencyError::Validation(error_message.to_string())
                    }
                    _ => DeleteFunctionConcurrencyError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteFunctionConcurrencyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteFunctionConcurrencyError {
    fn from(err: serde_json::error::Error) -> DeleteFunctionConcurrencyError {
        DeleteFunctionConcurrencyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteFunctionConcurrencyError {
    fn from(err: CredentialsError) -> DeleteFunctionConcurrencyError {
        DeleteFunctionConcurrencyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteFunctionConcurrencyError {
    fn from(err: HttpDispatchError) -> DeleteFunctionConcurrencyError {
        DeleteFunctionConcurrencyError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteFunctionConcurrencyError {
    fn from(err: io::Error) -> DeleteFunctionConcurrencyError {
        DeleteFunctionConcurrencyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteFunctionConcurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteFunctionConcurrencyError {
    fn description(&self) -> &str {
        match *self {
            DeleteFunctionConcurrencyError::InvalidParameterValue(ref cause) => cause,
            DeleteFunctionConcurrencyError::ResourceNotFound(ref cause) => cause,
            DeleteFunctionConcurrencyError::Service(ref cause) => cause,
            DeleteFunctionConcurrencyError::TooManyRequests(ref cause) => cause,
            DeleteFunctionConcurrencyError::Validation(ref cause) => cause,
            DeleteFunctionConcurrencyError::Credentials(ref err) => err.description(),
            DeleteFunctionConcurrencyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteFunctionConcurrencyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAccountSettingsError {
    pub fn from_body(body: &str) -> GetAccountSettingsError {
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
                    "ServiceException" => {
                        GetAccountSettingsError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetAccountSettingsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetAccountSettingsError::Validation(error_message.to_string())
                    }
                    _ => GetAccountSettingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAccountSettingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAccountSettingsError {
    fn from(err: serde_json::error::Error) -> GetAccountSettingsError {
        GetAccountSettingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAccountSettingsError {
    fn from(err: CredentialsError) -> GetAccountSettingsError {
        GetAccountSettingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAccountSettingsError {
    fn from(err: HttpDispatchError) -> GetAccountSettingsError {
        GetAccountSettingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAccountSettingsError {
    fn from(err: io::Error) -> GetAccountSettingsError {
        GetAccountSettingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAccountSettingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAccountSettingsError {
    fn description(&self) -> &str {
        match *self {
            GetAccountSettingsError::Service(ref cause) => cause,
            GetAccountSettingsError::TooManyRequests(ref cause) => cause,
            GetAccountSettingsError::Validation(ref cause) => cause,
            GetAccountSettingsError::Credentials(ref err) => err.description(),
            GetAccountSettingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAccountSettingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAlias
#[derive(Debug, PartialEq)]
pub enum GetAliasError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetAliasError {
    pub fn from_body(body: &str) -> GetAliasError {
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
                    "InvalidParameterValueException" => {
                        GetAliasError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => GetAliasError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetAliasError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetAliasError::Validation(error_message.to_string()),
                    _ => GetAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetAliasError {
    fn from(err: serde_json::error::Error) -> GetAliasError {
        GetAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAliasError {
    fn from(err: CredentialsError) -> GetAliasError {
        GetAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAliasError {
    fn from(err: HttpDispatchError) -> GetAliasError {
        GetAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAliasError {
    fn from(err: io::Error) -> GetAliasError {
        GetAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAliasError {
    fn description(&self) -> &str {
        match *self {
            GetAliasError::InvalidParameterValue(ref cause) => cause,
            GetAliasError::ResourceNotFound(ref cause) => cause,
            GetAliasError::Service(ref cause) => cause,
            GetAliasError::TooManyRequests(ref cause) => cause,
            GetAliasError::Validation(ref cause) => cause,
            GetAliasError::Credentials(ref err) => err.description(),
            GetAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum GetEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetEventSourceMappingError {
    pub fn from_body(body: &str) -> GetEventSourceMappingError {
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
                    "InvalidParameterValueException" => {
                        GetEventSourceMappingError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        GetEventSourceMappingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetEventSourceMappingError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetEventSourceMappingError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetEventSourceMappingError::Validation(error_message.to_string())
                    }
                    _ => GetEventSourceMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetEventSourceMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> GetEventSourceMappingError {
        GetEventSourceMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetEventSourceMappingError {
    fn from(err: CredentialsError) -> GetEventSourceMappingError {
        GetEventSourceMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetEventSourceMappingError {
    fn from(err: HttpDispatchError) -> GetEventSourceMappingError {
        GetEventSourceMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetEventSourceMappingError {
    fn from(err: io::Error) -> GetEventSourceMappingError {
        GetEventSourceMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetEventSourceMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetEventSourceMappingError {
    fn description(&self) -> &str {
        match *self {
            GetEventSourceMappingError::InvalidParameterValue(ref cause) => cause,
            GetEventSourceMappingError::ResourceNotFound(ref cause) => cause,
            GetEventSourceMappingError::Service(ref cause) => cause,
            GetEventSourceMappingError::TooManyRequests(ref cause) => cause,
            GetEventSourceMappingError::Validation(ref cause) => cause,
            GetEventSourceMappingError::Credentials(ref err) => err.description(),
            GetEventSourceMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetEventSourceMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunction
#[derive(Debug, PartialEq)]
pub enum GetFunctionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFunctionError {
    pub fn from_body(body: &str) -> GetFunctionError {
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
                    "InvalidParameterValueException" => {
                        GetFunctionError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetFunctionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => GetFunctionError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetFunctionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFunctionError::Validation(error_message.to_string())
                    }
                    _ => GetFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFunctionError {
    fn from(err: serde_json::error::Error) -> GetFunctionError {
        GetFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFunctionError {
    fn from(err: CredentialsError) -> GetFunctionError {
        GetFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFunctionError {
    fn from(err: HttpDispatchError) -> GetFunctionError {
        GetFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFunctionError {
    fn from(err: io::Error) -> GetFunctionError {
        GetFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionError::InvalidParameterValue(ref cause) => cause,
            GetFunctionError::ResourceNotFound(ref cause) => cause,
            GetFunctionError::Service(ref cause) => cause,
            GetFunctionError::TooManyRequests(ref cause) => cause,
            GetFunctionError::Validation(ref cause) => cause,
            GetFunctionError::Credentials(ref err) => err.description(),
            GetFunctionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetFunctionConfiguration
#[derive(Debug, PartialEq)]
pub enum GetFunctionConfigurationError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetFunctionConfigurationError {
    pub fn from_body(body: &str) -> GetFunctionConfigurationError {
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
                    "InvalidParameterValueException" => {
                        GetFunctionConfigurationError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        GetFunctionConfigurationError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        GetFunctionConfigurationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetFunctionConfigurationError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetFunctionConfigurationError::Validation(error_message.to_string())
                    }
                    _ => GetFunctionConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetFunctionConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetFunctionConfigurationError {
    fn from(err: serde_json::error::Error) -> GetFunctionConfigurationError {
        GetFunctionConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetFunctionConfigurationError {
    fn from(err: CredentialsError) -> GetFunctionConfigurationError {
        GetFunctionConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetFunctionConfigurationError {
    fn from(err: HttpDispatchError) -> GetFunctionConfigurationError {
        GetFunctionConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetFunctionConfigurationError {
    fn from(err: io::Error) -> GetFunctionConfigurationError {
        GetFunctionConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetFunctionConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetFunctionConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetFunctionConfigurationError::InvalidParameterValue(ref cause) => cause,
            GetFunctionConfigurationError::ResourceNotFound(ref cause) => cause,
            GetFunctionConfigurationError::Service(ref cause) => cause,
            GetFunctionConfigurationError::TooManyRequests(ref cause) => cause,
            GetFunctionConfigurationError::Validation(ref cause) => cause,
            GetFunctionConfigurationError::Credentials(ref err) => err.description(),
            GetFunctionConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetFunctionConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetPolicyError {
    pub fn from_body(body: &str) -> GetPolicyError {
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
                    "InvalidParameterValueException" => {
                        GetPolicyError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetPolicyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => GetPolicyError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        GetPolicyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => GetPolicyError::Validation(error_message.to_string()),
                    _ => GetPolicyError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetPolicyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetPolicyError {
    fn from(err: serde_json::error::Error) -> GetPolicyError {
        GetPolicyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetPolicyError {
    fn from(err: CredentialsError) -> GetPolicyError {
        GetPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetPolicyError {
    fn from(err: HttpDispatchError) -> GetPolicyError {
        GetPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetPolicyError {
    fn from(err: io::Error) -> GetPolicyError {
        GetPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetPolicyError::InvalidParameterValue(ref cause) => cause,
            GetPolicyError::ResourceNotFound(ref cause) => cause,
            GetPolicyError::Service(ref cause) => cause,
            GetPolicyError::TooManyRequests(ref cause) => cause,
            GetPolicyError::Validation(ref cause) => cause,
            GetPolicyError::Credentials(ref err) => err.description(),
            GetPolicyError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetPolicyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by Invoke
#[derive(Debug, PartialEq)]
pub enum InvokeError {
    /// <p><p/></p>
    EC2AccessDenied(String),
    /// <p>AWS Lambda was throttled by Amazon EC2 during Lambda function initialization using the execution role provided for the Lambda function.</p>
    EC2Throttled(String),
    /// <p>AWS Lambda received an unexpected EC2 client exception while setting up for the Lambda function.</p>
    EC2Unexpected(String),
    /// <p>AWS Lambda was not able to create an Elastic Network Interface (ENI) in the VPC, specified as part of Lambda function configuration, because the limit for network interfaces has been reached.</p>
    ENILimitReached(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The request body could not be parsed as JSON.</p>
    InvalidRequestContent(String),
    /// <p>The runtime or runtime version specified is not supported.</p>
    InvalidRuntime(String),
    /// <p>The Security Group ID provided in the Lambda function VPC configuration is invalid.</p>
    InvalidSecurityGroupID(String),
    /// <p>The Subnet ID provided in the Lambda function VPC configuration is invalid.</p>
    InvalidSubnetID(String),
    /// <p>AWS Lambda could not unzip the function zip file.</p>
    InvalidZipFile(String),
    /// <p>Lambda was unable to decrypt the environment variables because KMS access was denied. Check the Lambda function's KMS permissions.</p>
    KMSAccessDenied(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key used is disabled. Check the Lambda function's KMS key settings.</p>
    KMSDisabled(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key used is in an invalid state for Decrypt. Check the function's KMS key settings.</p>
    KMSInvalidState(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key was not found. Check the function's KMS key settings. </p>
    KMSNotFound(String),
    /// <p>The request payload exceeded the <code>Invoke</code> request body JSON input limit. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a>. </p>
    RequestTooLarge(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>AWS Lambda was not able to set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses.</p>
    SubnetIPAddressLimitReached(String),
    /// <p> </p>
    TooManyRequests(String),
    /// <p>The content type of the <code>Invoke</code> request body is not JSON.</p>
    UnsupportedMediaType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InvokeError {
    pub fn from_body(body: &str) -> InvokeError {
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
                    "EC2AccessDeniedException" => {
                        InvokeError::EC2AccessDenied(String::from(error_message))
                    }
                    "EC2ThrottledException" => {
                        InvokeError::EC2Throttled(String::from(error_message))
                    }
                    "EC2UnexpectedException" => {
                        InvokeError::EC2Unexpected(String::from(error_message))
                    }
                    "ENILimitReachedException" => {
                        InvokeError::ENILimitReached(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        InvokeError::InvalidParameterValue(String::from(error_message))
                    }
                    "InvalidRequestContentException" => {
                        InvokeError::InvalidRequestContent(String::from(error_message))
                    }
                    "InvalidRuntimeException" => {
                        InvokeError::InvalidRuntime(String::from(error_message))
                    }
                    "InvalidSecurityGroupIDException" => {
                        InvokeError::InvalidSecurityGroupID(String::from(error_message))
                    }
                    "InvalidSubnetIDException" => {
                        InvokeError::InvalidSubnetID(String::from(error_message))
                    }
                    "InvalidZipFileException" => {
                        InvokeError::InvalidZipFile(String::from(error_message))
                    }
                    "KMSAccessDeniedException" => {
                        InvokeError::KMSAccessDenied(String::from(error_message))
                    }
                    "KMSDisabledException" => InvokeError::KMSDisabled(String::from(error_message)),
                    "KMSInvalidStateException" => {
                        InvokeError::KMSInvalidState(String::from(error_message))
                    }
                    "KMSNotFoundException" => InvokeError::KMSNotFound(String::from(error_message)),
                    "RequestTooLargeException" => {
                        InvokeError::RequestTooLarge(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InvokeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => InvokeError::Service(String::from(error_message)),
                    "SubnetIPAddressLimitReachedException" => {
                        InvokeError::SubnetIPAddressLimitReached(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        InvokeError::TooManyRequests(String::from(error_message))
                    }
                    "UnsupportedMediaTypeException" => {
                        InvokeError::UnsupportedMediaType(String::from(error_message))
                    }
                    "ValidationException" => InvokeError::Validation(error_message.to_string()),
                    _ => InvokeError::Unknown(String::from(body)),
                }
            }
            Err(_) => InvokeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InvokeError {
    fn from(err: serde_json::error::Error) -> InvokeError {
        InvokeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InvokeError {
    fn from(err: CredentialsError) -> InvokeError {
        InvokeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InvokeError {
    fn from(err: HttpDispatchError) -> InvokeError {
        InvokeError::HttpDispatch(err)
    }
}
impl From<io::Error> for InvokeError {
    fn from(err: io::Error) -> InvokeError {
        InvokeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InvokeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvokeError {
    fn description(&self) -> &str {
        match *self {
            InvokeError::EC2AccessDenied(ref cause) => cause,
            InvokeError::EC2Throttled(ref cause) => cause,
            InvokeError::EC2Unexpected(ref cause) => cause,
            InvokeError::ENILimitReached(ref cause) => cause,
            InvokeError::InvalidParameterValue(ref cause) => cause,
            InvokeError::InvalidRequestContent(ref cause) => cause,
            InvokeError::InvalidRuntime(ref cause) => cause,
            InvokeError::InvalidSecurityGroupID(ref cause) => cause,
            InvokeError::InvalidSubnetID(ref cause) => cause,
            InvokeError::InvalidZipFile(ref cause) => cause,
            InvokeError::KMSAccessDenied(ref cause) => cause,
            InvokeError::KMSDisabled(ref cause) => cause,
            InvokeError::KMSInvalidState(ref cause) => cause,
            InvokeError::KMSNotFound(ref cause) => cause,
            InvokeError::RequestTooLarge(ref cause) => cause,
            InvokeError::ResourceNotFound(ref cause) => cause,
            InvokeError::Service(ref cause) => cause,
            InvokeError::SubnetIPAddressLimitReached(ref cause) => cause,
            InvokeError::TooManyRequests(ref cause) => cause,
            InvokeError::UnsupportedMediaType(ref cause) => cause,
            InvokeError::Validation(ref cause) => cause,
            InvokeError::Credentials(ref err) => err.description(),
            InvokeError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InvokeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by InvokeAsync
#[derive(Debug, PartialEq)]
pub enum InvokeAsyncError {
    /// <p>The request body could not be parsed as JSON.</p>
    InvalidRequestContent(String),
    /// <p>The runtime or runtime version specified is not supported.</p>
    InvalidRuntime(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl InvokeAsyncError {
    pub fn from_body(body: &str) -> InvokeAsyncError {
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
                    "InvalidRequestContentException" => {
                        InvokeAsyncError::InvalidRequestContent(String::from(error_message))
                    }
                    "InvalidRuntimeException" => {
                        InvokeAsyncError::InvalidRuntime(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        InvokeAsyncError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => InvokeAsyncError::Service(String::from(error_message)),
                    "ValidationException" => {
                        InvokeAsyncError::Validation(error_message.to_string())
                    }
                    _ => InvokeAsyncError::Unknown(String::from(body)),
                }
            }
            Err(_) => InvokeAsyncError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for InvokeAsyncError {
    fn from(err: serde_json::error::Error) -> InvokeAsyncError {
        InvokeAsyncError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for InvokeAsyncError {
    fn from(err: CredentialsError) -> InvokeAsyncError {
        InvokeAsyncError::Credentials(err)
    }
}
impl From<HttpDispatchError> for InvokeAsyncError {
    fn from(err: HttpDispatchError) -> InvokeAsyncError {
        InvokeAsyncError::HttpDispatch(err)
    }
}
impl From<io::Error> for InvokeAsyncError {
    fn from(err: io::Error) -> InvokeAsyncError {
        InvokeAsyncError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for InvokeAsyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for InvokeAsyncError {
    fn description(&self) -> &str {
        match *self {
            InvokeAsyncError::InvalidRequestContent(ref cause) => cause,
            InvokeAsyncError::InvalidRuntime(ref cause) => cause,
            InvokeAsyncError::ResourceNotFound(ref cause) => cause,
            InvokeAsyncError::Service(ref cause) => cause,
            InvokeAsyncError::Validation(ref cause) => cause,
            InvokeAsyncError::Credentials(ref err) => err.description(),
            InvokeAsyncError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            InvokeAsyncError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListAliasesError {
    pub fn from_body(body: &str) -> ListAliasesError {
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
                    "InvalidParameterValueException" => {
                        ListAliasesError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListAliasesError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => ListAliasesError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListAliasesError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListAliasesError::Validation(error_message.to_string())
                    }
                    _ => ListAliasesError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListAliasesError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListAliasesError {
    fn from(err: serde_json::error::Error) -> ListAliasesError {
        ListAliasesError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAliasesError {
    fn from(err: CredentialsError) -> ListAliasesError {
        ListAliasesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAliasesError {
    fn from(err: HttpDispatchError) -> ListAliasesError {
        ListAliasesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAliasesError {
    fn from(err: io::Error) -> ListAliasesError {
        ListAliasesError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAliasesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAliasesError {
    fn description(&self) -> &str {
        match *self {
            ListAliasesError::InvalidParameterValue(ref cause) => cause,
            ListAliasesError::ResourceNotFound(ref cause) => cause,
            ListAliasesError::Service(ref cause) => cause,
            ListAliasesError::TooManyRequests(ref cause) => cause,
            ListAliasesError::Validation(ref cause) => cause,
            ListAliasesError::Credentials(ref err) => err.description(),
            ListAliasesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAliasesError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListEventSourceMappings
#[derive(Debug, PartialEq)]
pub enum ListEventSourceMappingsError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListEventSourceMappingsError {
    pub fn from_body(body: &str) -> ListEventSourceMappingsError {
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
                    "InvalidParameterValueException" => {
                        ListEventSourceMappingsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListEventSourceMappingsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListEventSourceMappingsError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListEventSourceMappingsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListEventSourceMappingsError::Validation(error_message.to_string())
                    }
                    _ => ListEventSourceMappingsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListEventSourceMappingsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListEventSourceMappingsError {
    fn from(err: serde_json::error::Error) -> ListEventSourceMappingsError {
        ListEventSourceMappingsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListEventSourceMappingsError {
    fn from(err: CredentialsError) -> ListEventSourceMappingsError {
        ListEventSourceMappingsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListEventSourceMappingsError {
    fn from(err: HttpDispatchError) -> ListEventSourceMappingsError {
        ListEventSourceMappingsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListEventSourceMappingsError {
    fn from(err: io::Error) -> ListEventSourceMappingsError {
        ListEventSourceMappingsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListEventSourceMappingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEventSourceMappingsError {
    fn description(&self) -> &str {
        match *self {
            ListEventSourceMappingsError::InvalidParameterValue(ref cause) => cause,
            ListEventSourceMappingsError::ResourceNotFound(ref cause) => cause,
            ListEventSourceMappingsError::Service(ref cause) => cause,
            ListEventSourceMappingsError::TooManyRequests(ref cause) => cause,
            ListEventSourceMappingsError::Validation(ref cause) => cause,
            ListEventSourceMappingsError::Credentials(ref err) => err.description(),
            ListEventSourceMappingsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListEventSourceMappingsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFunctions
#[derive(Debug, PartialEq)]
pub enum ListFunctionsError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListFunctionsError {
    pub fn from_body(body: &str) -> ListFunctionsError {
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
                    "InvalidParameterValueException" => {
                        ListFunctionsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServiceException" => ListFunctionsError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListFunctionsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListFunctionsError::Validation(error_message.to_string())
                    }
                    _ => ListFunctionsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListFunctionsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListFunctionsError {
    fn from(err: serde_json::error::Error) -> ListFunctionsError {
        ListFunctionsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListFunctionsError {
    fn from(err: CredentialsError) -> ListFunctionsError {
        ListFunctionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListFunctionsError {
    fn from(err: HttpDispatchError) -> ListFunctionsError {
        ListFunctionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListFunctionsError {
    fn from(err: io::Error) -> ListFunctionsError {
        ListFunctionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListFunctionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFunctionsError {
    fn description(&self) -> &str {
        match *self {
            ListFunctionsError::InvalidParameterValue(ref cause) => cause,
            ListFunctionsError::Service(ref cause) => cause,
            ListFunctionsError::TooManyRequests(ref cause) => cause,
            ListFunctionsError::Validation(ref cause) => cause,
            ListFunctionsError::Credentials(ref err) => err.description(),
            ListFunctionsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListFunctionsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListTagsError {
    pub fn from_body(body: &str) -> ListTagsError {
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
                    "InvalidParameterValueException" => {
                        ListTagsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => ListTagsError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        ListTagsError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => ListTagsError::Validation(error_message.to_string()),
                    _ => ListTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsError {
    fn from(err: CredentialsError) -> ListTagsError {
        ListTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsError {
    fn from(err: HttpDispatchError) -> ListTagsError {
        ListTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsError {
    fn from(err: io::Error) -> ListTagsError {
        ListTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsError {
    fn description(&self) -> &str {
        match *self {
            ListTagsError::InvalidParameterValue(ref cause) => cause,
            ListTagsError::ResourceNotFound(ref cause) => cause,
            ListTagsError::Service(ref cause) => cause,
            ListTagsError::TooManyRequests(ref cause) => cause,
            ListTagsError::Validation(ref cause) => cause,
            ListTagsError::Credentials(ref err) => err.description(),
            ListTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListVersionsByFunction
#[derive(Debug, PartialEq)]
pub enum ListVersionsByFunctionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListVersionsByFunctionError {
    pub fn from_body(body: &str) -> ListVersionsByFunctionError {
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
                    "InvalidParameterValueException" => {
                        ListVersionsByFunctionError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        ListVersionsByFunctionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        ListVersionsByFunctionError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListVersionsByFunctionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListVersionsByFunctionError::Validation(error_message.to_string())
                    }
                    _ => ListVersionsByFunctionError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListVersionsByFunctionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListVersionsByFunctionError {
    fn from(err: serde_json::error::Error) -> ListVersionsByFunctionError {
        ListVersionsByFunctionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListVersionsByFunctionError {
    fn from(err: CredentialsError) -> ListVersionsByFunctionError {
        ListVersionsByFunctionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListVersionsByFunctionError {
    fn from(err: HttpDispatchError) -> ListVersionsByFunctionError {
        ListVersionsByFunctionError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListVersionsByFunctionError {
    fn from(err: io::Error) -> ListVersionsByFunctionError {
        ListVersionsByFunctionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListVersionsByFunctionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListVersionsByFunctionError {
    fn description(&self) -> &str {
        match *self {
            ListVersionsByFunctionError::InvalidParameterValue(ref cause) => cause,
            ListVersionsByFunctionError::ResourceNotFound(ref cause) => cause,
            ListVersionsByFunctionError::Service(ref cause) => cause,
            ListVersionsByFunctionError::TooManyRequests(ref cause) => cause,
            ListVersionsByFunctionError::Validation(ref cause) => cause,
            ListVersionsByFunctionError::Credentials(ref err) => err.description(),
            ListVersionsByFunctionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListVersionsByFunctionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PublishVersion
#[derive(Debug, PartialEq)]
pub enum PublishVersionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PublishVersionError {
    pub fn from_body(body: &str) -> PublishVersionError {
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
                    "CodeStorageExceededException" => {
                        PublishVersionError::CodeStorageExceeded(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        PublishVersionError::InvalidParameterValue(String::from(error_message))
                    }
                    "PreconditionFailedException" => {
                        PublishVersionError::PreconditionFailed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        PublishVersionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => PublishVersionError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        PublishVersionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        PublishVersionError::Validation(error_message.to_string())
                    }
                    _ => PublishVersionError::Unknown(String::from(body)),
                }
            }
            Err(_) => PublishVersionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PublishVersionError {
    fn from(err: serde_json::error::Error) -> PublishVersionError {
        PublishVersionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PublishVersionError {
    fn from(err: CredentialsError) -> PublishVersionError {
        PublishVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishVersionError {
    fn from(err: HttpDispatchError) -> PublishVersionError {
        PublishVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishVersionError {
    fn from(err: io::Error) -> PublishVersionError {
        PublishVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishVersionError {
    fn description(&self) -> &str {
        match *self {
            PublishVersionError::CodeStorageExceeded(ref cause) => cause,
            PublishVersionError::InvalidParameterValue(ref cause) => cause,
            PublishVersionError::PreconditionFailed(ref cause) => cause,
            PublishVersionError::ResourceNotFound(ref cause) => cause,
            PublishVersionError::Service(ref cause) => cause,
            PublishVersionError::TooManyRequests(ref cause) => cause,
            PublishVersionError::Validation(ref cause) => cause,
            PublishVersionError::Credentials(ref err) => err.description(),
            PublishVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            PublishVersionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by PutFunctionConcurrency
#[derive(Debug, PartialEq)]
pub enum PutFunctionConcurrencyError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl PutFunctionConcurrencyError {
    pub fn from_body(body: &str) -> PutFunctionConcurrencyError {
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
                    "InvalidParameterValueException" => {
                        PutFunctionConcurrencyError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        PutFunctionConcurrencyError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        PutFunctionConcurrencyError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        PutFunctionConcurrencyError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        PutFunctionConcurrencyError::Validation(error_message.to_string())
                    }
                    _ => PutFunctionConcurrencyError::Unknown(String::from(body)),
                }
            }
            Err(_) => PutFunctionConcurrencyError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for PutFunctionConcurrencyError {
    fn from(err: serde_json::error::Error) -> PutFunctionConcurrencyError {
        PutFunctionConcurrencyError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for PutFunctionConcurrencyError {
    fn from(err: CredentialsError) -> PutFunctionConcurrencyError {
        PutFunctionConcurrencyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutFunctionConcurrencyError {
    fn from(err: HttpDispatchError) -> PutFunctionConcurrencyError {
        PutFunctionConcurrencyError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutFunctionConcurrencyError {
    fn from(err: io::Error) -> PutFunctionConcurrencyError {
        PutFunctionConcurrencyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutFunctionConcurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutFunctionConcurrencyError {
    fn description(&self) -> &str {
        match *self {
            PutFunctionConcurrencyError::InvalidParameterValue(ref cause) => cause,
            PutFunctionConcurrencyError::ResourceNotFound(ref cause) => cause,
            PutFunctionConcurrencyError::Service(ref cause) => cause,
            PutFunctionConcurrencyError::TooManyRequests(ref cause) => cause,
            PutFunctionConcurrencyError::Validation(ref cause) => cause,
            PutFunctionConcurrencyError::Credentials(ref err) => err.description(),
            PutFunctionConcurrencyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutFunctionConcurrencyError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl RemovePermissionError {
    pub fn from_body(body: &str) -> RemovePermissionError {
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
                    "InvalidParameterValueException" => {
                        RemovePermissionError::InvalidParameterValue(String::from(error_message))
                    }
                    "PreconditionFailedException" => {
                        RemovePermissionError::PreconditionFailed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RemovePermissionError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        RemovePermissionError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        RemovePermissionError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        RemovePermissionError::Validation(error_message.to_string())
                    }
                    _ => RemovePermissionError::Unknown(String::from(body)),
                }
            }
            Err(_) => RemovePermissionError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RemovePermissionError {
    fn from(err: serde_json::error::Error) -> RemovePermissionError {
        RemovePermissionError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for RemovePermissionError {
    fn from(err: CredentialsError) -> RemovePermissionError {
        RemovePermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemovePermissionError {
    fn from(err: HttpDispatchError) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemovePermissionError {
    fn from(err: io::Error) -> RemovePermissionError {
        RemovePermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemovePermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemovePermissionError {
    fn description(&self) -> &str {
        match *self {
            RemovePermissionError::InvalidParameterValue(ref cause) => cause,
            RemovePermissionError::PreconditionFailed(ref cause) => cause,
            RemovePermissionError::ResourceNotFound(ref cause) => cause,
            RemovePermissionError::Service(ref cause) => cause,
            RemovePermissionError::TooManyRequests(ref cause) => cause,
            RemovePermissionError::Validation(ref cause) => cause,
            RemovePermissionError::Credentials(ref err) => err.description(),
            RemovePermissionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RemovePermissionError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl TagResourceError {
    pub fn from_body(body: &str) -> TagResourceError {
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
                    "InvalidParameterValueException" => {
                        TagResourceError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        TagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => TagResourceError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        TagResourceError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        TagResourceError::Validation(error_message.to_string())
                    }
                    _ => TagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => TagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for TagResourceError {
    fn from(err: CredentialsError) -> TagResourceError {
        TagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TagResourceError {
    fn from(err: HttpDispatchError) -> TagResourceError {
        TagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for TagResourceError {
    fn from(err: io::Error) -> TagResourceError {
        TagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            TagResourceError::InvalidParameterValue(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::Service(ref cause) => cause,
            TagResourceError::TooManyRequests(ref cause) => cause,
            TagResourceError::Validation(ref cause) => cause,
            TagResourceError::Credentials(ref err) => err.description(),
            TagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UntagResourceError {
    pub fn from_body(body: &str) -> UntagResourceError {
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
                    "InvalidParameterValueException" => {
                        UntagResourceError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UntagResourceError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => UntagResourceError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UntagResourceError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UntagResourceError::Validation(error_message.to_string())
                    }
                    _ => UntagResourceError::Unknown(String::from(body)),
                }
            }
            Err(_) => UntagResourceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UntagResourceError {
    fn from(err: CredentialsError) -> UntagResourceError {
        UntagResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UntagResourceError {
    fn from(err: HttpDispatchError) -> UntagResourceError {
        UntagResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for UntagResourceError {
    fn from(err: io::Error) -> UntagResourceError {
        UntagResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            UntagResourceError::InvalidParameterValue(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::Service(ref cause) => cause,
            UntagResourceError::TooManyRequests(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateAliasError {
    pub fn from_body(body: &str) -> UpdateAliasError {
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
                    "InvalidParameterValueException" => {
                        UpdateAliasError::InvalidParameterValue(String::from(error_message))
                    }
                    "PreconditionFailedException" => {
                        UpdateAliasError::PreconditionFailed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateAliasError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => UpdateAliasError::Service(String::from(error_message)),
                    "TooManyRequestsException" => {
                        UpdateAliasError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateAliasError::Validation(error_message.to_string())
                    }
                    _ => UpdateAliasError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateAliasError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateAliasError {
    fn from(err: serde_json::error::Error) -> UpdateAliasError {
        UpdateAliasError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAliasError {
    fn from(err: CredentialsError) -> UpdateAliasError {
        UpdateAliasError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAliasError {
    fn from(err: HttpDispatchError) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAliasError {
    fn from(err: io::Error) -> UpdateAliasError {
        UpdateAliasError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAliasError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAliasError {
    fn description(&self) -> &str {
        match *self {
            UpdateAliasError::InvalidParameterValue(ref cause) => cause,
            UpdateAliasError::PreconditionFailed(ref cause) => cause,
            UpdateAliasError::ResourceNotFound(ref cause) => cause,
            UpdateAliasError::Service(ref cause) => cause,
            UpdateAliasError::TooManyRequests(ref cause) => cause,
            UpdateAliasError::Validation(ref cause) => cause,
            UpdateAliasError::Credentials(ref err) => err.description(),
            UpdateAliasError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAliasError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum UpdateEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSoure Mapping in CREATING, or tried to delete a EventSoure mapping currently in the UPDATING state. </p>
    ResourceInUse(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateEventSourceMappingError {
    pub fn from_body(body: &str) -> UpdateEventSourceMappingError {
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
                    "InvalidParameterValueException" => {
                        UpdateEventSourceMappingError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceConflictException" => {
                        UpdateEventSourceMappingError::ResourceConflict(String::from(error_message))
                    }
                    "ResourceInUseException" => {
                        UpdateEventSourceMappingError::ResourceInUse(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateEventSourceMappingError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        UpdateEventSourceMappingError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateEventSourceMappingError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateEventSourceMappingError::Validation(error_message.to_string())
                    }
                    _ => UpdateEventSourceMappingError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateEventSourceMappingError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> UpdateEventSourceMappingError {
        UpdateEventSourceMappingError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateEventSourceMappingError {
    fn from(err: CredentialsError) -> UpdateEventSourceMappingError {
        UpdateEventSourceMappingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateEventSourceMappingError {
    fn from(err: HttpDispatchError) -> UpdateEventSourceMappingError {
        UpdateEventSourceMappingError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateEventSourceMappingError {
    fn from(err: io::Error) -> UpdateEventSourceMappingError {
        UpdateEventSourceMappingError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateEventSourceMappingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateEventSourceMappingError {
    fn description(&self) -> &str {
        match *self {
            UpdateEventSourceMappingError::InvalidParameterValue(ref cause) => cause,
            UpdateEventSourceMappingError::ResourceConflict(ref cause) => cause,
            UpdateEventSourceMappingError::ResourceInUse(ref cause) => cause,
            UpdateEventSourceMappingError::ResourceNotFound(ref cause) => cause,
            UpdateEventSourceMappingError::Service(ref cause) => cause,
            UpdateEventSourceMappingError::TooManyRequests(ref cause) => cause,
            UpdateEventSourceMappingError::Validation(ref cause) => cause,
            UpdateEventSourceMappingError::Credentials(ref err) => err.description(),
            UpdateEventSourceMappingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateEventSourceMappingError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFunctionCode
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionCodeError {
    /// <p>You have exceeded your maximum total code size per account. <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateFunctionCodeError {
    pub fn from_body(body: &str) -> UpdateFunctionCodeError {
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
                    "CodeStorageExceededException" => {
                        UpdateFunctionCodeError::CodeStorageExceeded(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        UpdateFunctionCodeError::InvalidParameterValue(String::from(error_message))
                    }
                    "PreconditionFailedException" => {
                        UpdateFunctionCodeError::PreconditionFailed(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateFunctionCodeError::ResourceNotFound(String::from(error_message))
                    }
                    "ServiceException" => {
                        UpdateFunctionCodeError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateFunctionCodeError::TooManyRequests(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateFunctionCodeError::Validation(error_message.to_string())
                    }
                    _ => UpdateFunctionCodeError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFunctionCodeError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFunctionCodeError {
    fn from(err: serde_json::error::Error) -> UpdateFunctionCodeError {
        UpdateFunctionCodeError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFunctionCodeError {
    fn from(err: CredentialsError) -> UpdateFunctionCodeError {
        UpdateFunctionCodeError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFunctionCodeError {
    fn from(err: HttpDispatchError) -> UpdateFunctionCodeError {
        UpdateFunctionCodeError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFunctionCodeError {
    fn from(err: io::Error) -> UpdateFunctionCodeError {
        UpdateFunctionCodeError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFunctionCodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFunctionCodeError {
    fn description(&self) -> &str {
        match *self {
            UpdateFunctionCodeError::CodeStorageExceeded(ref cause) => cause,
            UpdateFunctionCodeError::InvalidParameterValue(ref cause) => cause,
            UpdateFunctionCodeError::PreconditionFailed(ref cause) => cause,
            UpdateFunctionCodeError::ResourceNotFound(ref cause) => cause,
            UpdateFunctionCodeError::Service(ref cause) => cause,
            UpdateFunctionCodeError::TooManyRequests(ref cause) => cause,
            UpdateFunctionCodeError::Validation(ref cause) => cause,
            UpdateFunctionCodeError::Credentials(ref err) => err.description(),
            UpdateFunctionCodeError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFunctionCodeError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateFunctionConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionConfigurationError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p> </p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl UpdateFunctionConfigurationError {
    pub fn from_body(body: &str) -> UpdateFunctionConfigurationError {
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
                    "InvalidParameterValueException" => {
                        UpdateFunctionConfigurationError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "PreconditionFailedException" => {
                        UpdateFunctionConfigurationError::PreconditionFailed(String::from(
                            error_message,
                        ))
                    }
                    "ResourceConflictException" => {
                        UpdateFunctionConfigurationError::ResourceConflict(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        UpdateFunctionConfigurationError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServiceException" => {
                        UpdateFunctionConfigurationError::Service(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateFunctionConfigurationError::TooManyRequests(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        UpdateFunctionConfigurationError::Validation(error_message.to_string())
                    }
                    _ => UpdateFunctionConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateFunctionConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateFunctionConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateFunctionConfigurationError {
        UpdateFunctionConfigurationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateFunctionConfigurationError {
    fn from(err: CredentialsError) -> UpdateFunctionConfigurationError {
        UpdateFunctionConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateFunctionConfigurationError {
    fn from(err: HttpDispatchError) -> UpdateFunctionConfigurationError {
        UpdateFunctionConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateFunctionConfigurationError {
    fn from(err: io::Error) -> UpdateFunctionConfigurationError {
        UpdateFunctionConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateFunctionConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateFunctionConfigurationError {
    fn description(&self) -> &str {
        match *self {
            UpdateFunctionConfigurationError::InvalidParameterValue(ref cause) => cause,
            UpdateFunctionConfigurationError::PreconditionFailed(ref cause) => cause,
            UpdateFunctionConfigurationError::ResourceConflict(ref cause) => cause,
            UpdateFunctionConfigurationError::ResourceNotFound(ref cause) => cause,
            UpdateFunctionConfigurationError::Service(ref cause) => cause,
            UpdateFunctionConfigurationError::TooManyRequests(ref cause) => cause,
            UpdateFunctionConfigurationError::Validation(ref cause) => cause,
            UpdateFunctionConfigurationError::Credentials(ref err) => err.description(),
            UpdateFunctionConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateFunctionConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Lambda API. AWS Lambda clients implement this trait.
pub trait Lambda {
    /// <p>Adds a permission to the resource policy associated with the specified AWS Lambda function. You use resource policies to grant permissions to event sources that use <i>push</i> model. In a <i>push</i> model, event sources (such as Amazon S3 and custom applications) invoke your Lambda function. Each permission you add to the resource policy allows an event source, permission to invoke the Lambda function. </p> <p>For information about the push model, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/lambda-introduction.html">Lambda Functions</a>. </p> <p>If you are using versioning, the permissions you add are specific to the Lambda function version or alias you specify in the <code>AddPermission</code> request via the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:AddPermission</code> action.</p>
    fn add_permission(
        &self,
        input: AddPermissionRequest,
    ) -> RusotoFuture<AddPermissionResponse, AddPermissionError>;

    /// <p>Creates an alias that points to the specified Lambda function version. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>Alias names are unique for a given function. This requires permission for the lambda:CreateAlias action.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, CreateAliasError>;

    /// <p>Identifies a poll-based event source for a Lambda function. It can be either an Amazon Kinesis or DynamoDB stream, or an Amazon SQS queue. AWS Lambda invokes the specified function when records are posted to the event source.</p> <p>This association between a poll-based source and a Lambda function is called the event source mapping.</p> <p>You provide mapping information (for example, which stream or SQS queue to read from and which Lambda function to invoke) in the request body.</p> <p>Amazon Kinesis or DynamoDB stream event sources can be associated with multiple AWS Lambda functions and a given Lambda function can be associated with multiple AWS event sources. For Amazon SQS, you can configure multiple queues as event sources for a single Lambda function, but an SQS queue can be mapped only to a single Lambda function.</p> <p>If you are using versioning, you can specify a specific function version or an alias via the function name parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:CreateEventSourceMapping</code> action.</p>
    fn create_event_source_mapping(
        &self,
        input: CreateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, CreateEventSourceMappingError>;

    /// <p>Creates a new Lambda function. The function metadata is created from the request parameters, and the code for the function is provided by a .zip file in the request body. If the function name already exists, the operation will fail. Note that the function name is case-sensitive.</p> <p> If you are using versioning, you can also publish a version of the Lambda function you are creating using the <code>Publish</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:CreateFunction</code> action.</p>
    fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> RusotoFuture<FunctionConfiguration, CreateFunctionError>;

    /// <p>Deletes the specified Lambda function alias. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:DeleteAlias action.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError>;

    /// <p>Removes an event source mapping. This means AWS Lambda will no longer invoke the function for events in the associated source.</p> <p>This operation requires permission for the <code>lambda:DeleteEventSourceMapping</code> action.</p>
    fn delete_event_source_mapping(
        &self,
        input: DeleteEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, DeleteEventSourceMappingError>;

    /// <p>Deletes the specified Lambda function code and configuration.</p> <p>If you are using the versioning feature and you don't specify a function version in your <code>DeleteFunction</code> request, AWS Lambda will delete the function, including all its versions, and any aliases pointing to the function versions. To delete a specific function version, you must provide the function version via the <code>Qualifier</code> parameter. For information about function versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>When you delete a function the associated resource policy is also deleted. You will need to delete the event source mappings explicitly.</p> <p>This operation requires permission for the <code>lambda:DeleteFunction</code> action.</p>
    fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> RusotoFuture<(), DeleteFunctionError>;

    /// <p>Removes concurrent execution limits from this function. For more information, see <a>concurrent-executions</a>.</p>
    fn delete_function_concurrency(
        &self,
        input: DeleteFunctionConcurrencyRequest,
    ) -> RusotoFuture<(), DeleteFunctionConcurrencyError>;

    /// <p>Returns a customer's account settings.</p> <p>You can use this operation to retrieve Lambda limits information, such as code size and concurrency limits. For more information about limits, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">AWS Lambda Limits</a>. You can also retrieve resource usage statistics, such as code storage usage and function count.</p>
    fn get_account_settings(
        &self,
    ) -> RusotoFuture<GetAccountSettingsResponse, GetAccountSettingsError>;

    /// <p>Returns the specified alias information such as the alias ARN, description, and function version it is pointing to. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the <code>lambda:GetAlias</code> action.</p>
    fn get_alias(&self, input: GetAliasRequest) -> RusotoFuture<AliasConfiguration, GetAliasError>;

    /// <p>Returns configuration information for the specified event source mapping (see <a>CreateEventSourceMapping</a>).</p> <p>This operation requires permission for the <code>lambda:GetEventSourceMapping</code> action.</p>
    fn get_event_source_mapping(
        &self,
        input: GetEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, GetEventSourceMappingError>;

    /// <p>Returns the configuration information of the Lambda function and a presigned URL link to the .zip file you uploaded with <a>CreateFunction</a> so you can download the .zip file. Note that the URL is valid for up to 10 minutes. The configuration information is the same information you provided as parameters when uploading the function.</p> <p>Using the optional <code>Qualifier</code> parameter, you can specify a specific function version for which you want this information. If you don't specify this parameter, the API uses unqualified function ARN which return information about the <code>$LATEST</code> version of the Lambda function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>.</p> <p>This operation requires permission for the <code>lambda:GetFunction</code> action.</p>
    fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> RusotoFuture<GetFunctionResponse, GetFunctionError>;

    /// <p>Returns the configuration information of the Lambda function. This the same information you provided as parameters when uploading the function by using <a>CreateFunction</a>.</p> <p>If you are using the versioning feature, you can retrieve this information for a specific function version by using the optional <code>Qualifier</code> parameter and specifying the function version or alias that points to it. If you don't provide it, the API returns information about the $LATEST version of the function. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>.</p> <p>This operation requires permission for the <code>lambda:GetFunctionConfiguration</code> operation.</p>
    fn get_function_configuration(
        &self,
        input: GetFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, GetFunctionConfigurationError>;

    /// <p>Returns the resource policy associated with the specified Lambda function.</p> <p> If you are using the versioning feature, you can get the resource policy associated with the specific Lambda function version or alias by specifying the version or alias name using the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>You need permission for the <code>lambda:GetPolicy action.</code> </p>
    fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> RusotoFuture<GetPolicyResponse, GetPolicyError>;

    /// <p><p>Invokes a specific Lambda function. For an example, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/with-dynamodb-create-function.html#with-dbb-invoke-manually">Create the Lambda Function and Test It Manually</a>. </p> <p>If you are using the versioning feature, you can invoke the specific function version by providing function version or alias name that is pointing to the function version using the <code>Qualifier</code> parameter in the request. If you don&#39;t provide the <code>Qualifier</code> parameter, the <code>$LATEST</code> version of the Lambda function is invoked. Invocations occur at least once in response to an event and functions must be idempotent to handle this. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p> <note> <p>The <code>TooManyRequestsException</code> noted below will return the following: <code>ConcurrentInvocationLimitExceeded</code> will be returned if you have no functions with reserved concurrency and have exceeded your account concurrent limit or if a function without reserved concurrency exceeds the account&#39;s unreserved concurrency limit. <code>ReservedFunctionConcurrentInvocationLimitExceeded</code> will be returned when a function with reserved concurrency exceeds its configured concurrency limit. </p> </note></p>
    fn invoke(&self, input: InvocationRequest) -> RusotoFuture<InvocationResponse, InvokeError>;

    /// <p><important> <p>This API is deprecated. We recommend you use <code>Invoke</code> API (see <a>Invoke</a>).</p> </important> <p>Submits an invocation request to AWS Lambda. Upon receiving the request, Lambda executes the specified function asynchronously. To see the logs generated by the Lambda function execution, see the CloudWatch Logs console.</p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p></p>
    fn invoke_async(
        &self,
        input: InvokeAsyncRequest,
    ) -> RusotoFuture<InvokeAsyncResponse, InvokeAsyncError>;

    /// <p>Returns list of aliases created for a Lambda function. For each alias, the response includes information such as the alias ARN, description, alias name, and the function version to which it points. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:ListAliases action.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Returns a list of event source mappings you created using the <code>CreateEventSourceMapping</code> (see <a>CreateEventSourceMapping</a>). </p> <p>For each mapping, the API returns configuration information. You can optionally specify filters to retrieve specific event source mappings.</p> <p>If you are using the versioning feature, you can get list of event source mappings for a specific Lambda function version or an alias as described in the <code>FunctionName</code> parameter. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:ListEventSourceMappings</code> action.</p>
    fn list_event_source_mappings(
        &self,
        input: ListEventSourceMappingsRequest,
    ) -> RusotoFuture<ListEventSourceMappingsResponse, ListEventSourceMappingsError>;

    /// <p>Returns a list of your Lambda functions. For each function, the response includes the function configuration information. You must use <a>GetFunction</a> to retrieve the code for your function.</p> <p>This operation requires permission for the <code>lambda:ListFunctions</code> action.</p> <p>If you are using the versioning feature, you can list all of your functions or only <code>$LATEST</code> versions. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> RusotoFuture<ListFunctionsResponse, ListFunctionsError>;

    /// <p>Returns a list of tags assigned to a function when supplied the function ARN (Amazon Resource Name). For more information on Tagging, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>List all versions of a function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn list_versions_by_function(
        &self,
        input: ListVersionsByFunctionRequest,
    ) -> RusotoFuture<ListVersionsByFunctionResponse, ListVersionsByFunctionError>;

    /// <p>Publishes a version of your function from the current snapshot of $LATEST. That is, AWS Lambda takes a snapshot of the function code and configuration information from $LATEST and publishes a new version. The code and configuration cannot be modified after publication. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn publish_version(
        &self,
        input: PublishVersionRequest,
    ) -> RusotoFuture<FunctionConfiguration, PublishVersionError>;

    /// <p>Sets a limit on the number of concurrent executions available to this function. It is a subset of your account's total concurrent execution limit per region. Note that Lambda automatically reserves a buffer of 100 concurrent executions for functions without any reserved concurrency limit. This means if your account limit is 1000, you have a total of 900 available to allocate to individual functions. For more information, see <a>concurrent-executions</a>.</p>
    fn put_function_concurrency(
        &self,
        input: PutFunctionConcurrencyRequest,
    ) -> RusotoFuture<Concurrency, PutFunctionConcurrencyError>;

    /// <p>You can remove individual permissions from an resource policy associated with a Lambda function by providing a statement ID that you provided when you added the permission.</p> <p>If you are using versioning, the permissions you remove are specific to the Lambda function version or alias you specify in the <code>AddPermission</code> request via the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>Note that removal of a permission will cause an active event source to lose permission to the function.</p> <p>You need permission for the <code>lambda:RemovePermission</code> action.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p>Creates a list of tags (key-value pairs) on the Lambda function. Requires the Lambda function ARN (Amazon Resource Name). If a key is specified without a value, Lambda creates a tag with the specified key and a value of null. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>. </p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes tags from a Lambda function. Requires the function ARN (Amazon Resource Name). For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>. </p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Using this API you can update the function version to which the alias points and the alias description. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:UpdateAlias action.</p>
    fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, UpdateAliasError>;

    /// <p>You can update an event source mapping. This is useful if you want to change the parameters of the existing mapping without losing your position in the stream. You can change which function will receive the stream records, but to change the stream itself, you must create a new mapping.</p> <p>If you are using the versioning feature, you can update the event source mapping to map to a specific Lambda function version or alias as described in the <code>FunctionName</code> parameter. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>If you disable the event source mapping, AWS Lambda stops polling. If you enable again, it will resume polling from the time it had stopped polling, so you don't lose processing of any records. However, if you delete event source mapping and create it again, it will reset.</p> <p>This operation requires permission for the <code>lambda:UpdateEventSourceMapping</code> action.</p>
    fn update_event_source_mapping(
        &self,
        input: UpdateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, UpdateEventSourceMappingError>;

    /// <p>Updates the code for the specified Lambda function. This operation must only be used on an existing Lambda function and cannot be used to update the function configuration.</p> <p>If you are using the versioning feature, note this API will always update the $LATEST version of your Lambda function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:UpdateFunctionCode</code> action.</p>
    fn update_function_code(
        &self,
        input: UpdateFunctionCodeRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionCodeError>;

    /// <p>Updates the configuration parameters for the specified Lambda function by using the values provided in the request. You provide only the parameters you want to change. This operation must only be used on an existing Lambda function and cannot be used to update the function's code.</p> <p>If you are using the versioning feature, note this API will always update the $LATEST version of your Lambda function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:UpdateFunctionConfiguration</code> action.</p>
    fn update_function_configuration(
        &self,
        input: UpdateFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionConfigurationError>;
}
/// A client for the AWS Lambda API.
pub struct LambdaClient {
    client: Client,
    region: region::Region,
}

impl LambdaClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> LambdaClient {
        LambdaClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LambdaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        LambdaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Lambda for LambdaClient {
    /// <p>Adds a permission to the resource policy associated with the specified AWS Lambda function. You use resource policies to grant permissions to event sources that use <i>push</i> model. In a <i>push</i> model, event sources (such as Amazon S3 and custom applications) invoke your Lambda function. Each permission you add to the resource policy allows an event source, permission to invoke the Lambda function. </p> <p>For information about the push model, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/lambda-introduction.html">Lambda Functions</a>. </p> <p>If you are using versioning, the permissions you add are specific to the Lambda function version or alias you specify in the <code>AddPermission</code> request via the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:AddPermission</code> action.</p>
    fn add_permission(
        &self,
        input: AddPermissionRequest,
    ) -> RusotoFuture<AddPermissionResponse, AddPermissionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/policy",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AddPermissionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddPermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates an alias that points to the specified Lambda function version. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>Alias names are unique for a given function. This requires permission for the lambda:CreateAlias action.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, CreateAliasError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Identifies a poll-based event source for a Lambda function. It can be either an Amazon Kinesis or DynamoDB stream, or an Amazon SQS queue. AWS Lambda invokes the specified function when records are posted to the event source.</p> <p>This association between a poll-based source and a Lambda function is called the event source mapping.</p> <p>You provide mapping information (for example, which stream or SQS queue to read from and which Lambda function to invoke) in the request body.</p> <p>Amazon Kinesis or DynamoDB stream event sources can be associated with multiple AWS Lambda functions and a given Lambda function can be associated with multiple AWS event sources. For Amazon SQS, you can configure multiple queues as event sources for a single Lambda function, but an SQS queue can be mapped only to a single Lambda function.</p> <p>If you are using versioning, you can specify a specific function version or an alias via the function name parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:CreateEventSourceMapping</code> action.</p>
    fn create_event_source_mapping(
        &self,
        input: CreateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, CreateEventSourceMappingError> {
        let request_uri = "/2015-03-31/event-source-mappings/";

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EventSourceMappingConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateEventSourceMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a new Lambda function. The function metadata is created from the request parameters, and the code for the function is provided by a .zip file in the request body. If the function name already exists, the operation will fail. Note that the function name is case-sensitive.</p> <p> If you are using versioning, you can also publish a version of the Lambda function you are creating using the <code>Publish</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:CreateFunction</code> action.</p>
    fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> RusotoFuture<FunctionConfiguration, CreateFunctionError> {
        let request_uri = "/2015-03-31/functions";

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateFunctionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified Lambda function alias. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:DeleteAlias action.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes an event source mapping. This means AWS Lambda will no longer invoke the function for events in the associated source.</p> <p>This operation requires permission for the <code>lambda:DeleteEventSourceMapping</code> action.</p>
    fn delete_event_source_mapping(
        &self,
        input: DeleteEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, DeleteEventSourceMappingError> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EventSourceMappingConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteEventSourceMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified Lambda function code and configuration.</p> <p>If you are using the versioning feature and you don't specify a function version in your <code>DeleteFunction</code> request, AWS Lambda will delete the function, including all its versions, and any aliases pointing to the function versions. To delete a specific function version, you must provide the function version via the <code>Qualifier</code> parameter. For information about function versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>When you delete a function the associated resource policy is also deleted. You will need to delete the event source mappings explicitly.</p> <p>This operation requires permission for the <code>lambda:DeleteFunction</code> action.</p>
    fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> RusotoFuture<(), DeleteFunctionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFunctionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes concurrent execution limits from this function. For more information, see <a>concurrent-executions</a>.</p>
    fn delete_function_concurrency(
        &self,
        input: DeleteFunctionConcurrencyRequest,
    ) -> RusotoFuture<(), DeleteFunctionConcurrencyError> {
        let request_uri = format!(
            "/2017-10-31/functions/{function_name}/concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteFunctionConcurrencyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a customer's account settings.</p> <p>You can use this operation to retrieve Lambda limits information, such as code size and concurrency limits. For more information about limits, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/limits.html">AWS Lambda Limits</a>. You can also retrieve resource usage statistics, such as code storage usage and function count.</p>
    fn get_account_settings(
        &self,
    ) -> RusotoFuture<GetAccountSettingsResponse, GetAccountSettingsError> {
        let request_uri = "/2016-08-19/account-settings/";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetAccountSettingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAccountSettingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the specified alias information such as the alias ARN, description, and function version it is pointing to. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the <code>lambda:GetAlias</code> action.</p>
    fn get_alias(&self, input: GetAliasRequest) -> RusotoFuture<AliasConfiguration, GetAliasError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns configuration information for the specified event source mapping (see <a>CreateEventSourceMapping</a>).</p> <p>This operation requires permission for the <code>lambda:GetEventSourceMapping</code> action.</p>
    fn get_event_source_mapping(
        &self,
        input: GetEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, GetEventSourceMappingError> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EventSourceMappingConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetEventSourceMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the configuration information of the Lambda function and a presigned URL link to the .zip file you uploaded with <a>CreateFunction</a> so you can download the .zip file. Note that the URL is valid for up to 10 minutes. The configuration information is the same information you provided as parameters when uploading the function.</p> <p>Using the optional <code>Qualifier</code> parameter, you can specify a specific function version for which you want this information. If you don't specify this parameter, the API uses unqualified function ARN which return information about the <code>$LATEST</code> version of the Lambda function. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>.</p> <p>This operation requires permission for the <code>lambda:GetFunction</code> action.</p>
    fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> RusotoFuture<GetFunctionResponse, GetFunctionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the configuration information of the Lambda function. This the same information you provided as parameters when uploading the function by using <a>CreateFunction</a>.</p> <p>If you are using the versioning feature, you can retrieve this information for a specific function version by using the optional <code>Qualifier</code> parameter and specifying the function version or alias that points to it. If you don't provide it, the API returns information about the $LATEST version of the function. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>.</p> <p>This operation requires permission for the <code>lambda:GetFunctionConfiguration</code> operation.</p>
    fn get_function_configuration(
        &self,
        input: GetFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, GetFunctionConfigurationError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/configuration",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns the resource policy associated with the specified Lambda function.</p> <p> If you are using the versioning feature, you can get the resource policy associated with the specific Lambda function version or alias by specifying the version or alias name using the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>You need permission for the <code>lambda:GetPolicy action.</code> </p>
    fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> RusotoFuture<GetPolicyResponse, GetPolicyError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/policy",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetPolicyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><p>Invokes a specific Lambda function. For an example, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/with-dynamodb-create-function.html#with-dbb-invoke-manually">Create the Lambda Function and Test It Manually</a>. </p> <p>If you are using the versioning feature, you can invoke the specific function version by providing function version or alias name that is pointing to the function version using the <code>Qualifier</code> parameter in the request. If you don&#39;t provide the <code>Qualifier</code> parameter, the <code>$LATEST</code> version of the Lambda function is invoked. Invocations occur at least once in response to an event and functions must be idempotent to handle this. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p> <note> <p>The <code>TooManyRequestsException</code> noted below will return the following: <code>ConcurrentInvocationLimitExceeded</code> will be returned if you have no functions with reserved concurrency and have exceeded your account concurrent limit or if a function without reserved concurrency exceeds the account&#39;s unreserved concurrency limit. <code>ReservedFunctionConcurrentInvocationLimitExceeded</code> will be returned when a function with reserved concurrency exceeds its configured concurrency limit. </p> </note></p>
    fn invoke(&self, input: InvocationRequest) -> RusotoFuture<InvocationResponse, InvokeError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/invocations",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = if let Some(ref payload) = input.payload {
            Some(payload.to_owned())
        } else {
            None
        };
        request.set_payload(encoded);

        if let Some(ref client_context) = input.client_context {
            request.add_header("X-Amz-Client-Context", &client_context.to_string());
        }

        if let Some(ref invocation_type) = input.invocation_type {
            request.add_header("X-Amz-Invocation-Type", &invocation_type.to_string());
        }

        if let Some(ref log_type) = input.log_type {
            request.add_header("X-Amz-Log-Type", &log_type.to_string());
        }
        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut result = InvocationResponse::default();
                    result.payload = Some(response.body);

                    if let Some(executed_version) = response.headers.get("X-Amz-Executed-Version") {
                        let value = executed_version.to_owned();
                        result.executed_version = Some(value)
                    };
                    if let Some(function_error) = response.headers.get("X-Amz-Function-Error") {
                        let value = function_error.to_owned();
                        result.function_error = Some(value)
                    };
                    if let Some(log_result) = response.headers.get("X-Amz-Log-Result") {
                        let value = log_result.to_owned();
                        result.log_result = Some(value)
                    };
                    result.status_code = Some(response.status.as_u16() as i64);
                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InvokeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p><important> <p>This API is deprecated. We recommend you use <code>Invoke</code> API (see <a>Invoke</a>).</p> </important> <p>Submits an invocation request to AWS Lambda. Upon receiving the request, Lambda executes the specified function asynchronously. To see the logs generated by the Lambda function execution, see the CloudWatch Logs console.</p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p></p>
    fn invoke_async(
        &self,
        input: InvokeAsyncRequest,
    ) -> RusotoFuture<InvokeAsyncResponse, InvokeAsyncError> {
        let request_uri = format!(
            "/2014-11-13/functions/{function_name}/invoke-async/",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.invoke_args.to_owned());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<InvokeAsyncResponse>(&body).unwrap();

                    result.status = Some(response.status.as_u16() as i64);
                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(InvokeAsyncError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns list of aliases created for a Lambda function. For each alias, the response includes information such as the alias ARN, description, alias name, and the function version to which it points. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:ListAliases action.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.function_version {
            params.put("FunctionVersion", x);
        }
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListAliasesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAliasesError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of event source mappings you created using the <code>CreateEventSourceMapping</code> (see <a>CreateEventSourceMapping</a>). </p> <p>For each mapping, the API returns configuration information. You can optionally specify filters to retrieve specific event source mappings.</p> <p>If you are using the versioning feature, you can get list of event source mappings for a specific Lambda function version or an alias as described in the <code>FunctionName</code> parameter. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:ListEventSourceMappings</code> action.</p>
    fn list_event_source_mappings(
        &self,
        input: ListEventSourceMappingsRequest,
    ) -> RusotoFuture<ListEventSourceMappingsResponse, ListEventSourceMappingsError> {
        let request_uri = "/2015-03-31/event-source-mappings/";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.event_source_arn {
            params.put("EventSourceArn", x);
        }
        if let Some(ref x) = input.function_name {
            params.put("FunctionName", x);
        }
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListEventSourceMappingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListEventSourceMappingsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of your Lambda functions. For each function, the response includes the function configuration information. You must use <a>GetFunction</a> to retrieve the code for your function.</p> <p>This operation requires permission for the <code>lambda:ListFunctions</code> action.</p> <p>If you are using the versioning feature, you can list all of your functions or only <code>$LATEST</code> versions. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> RusotoFuture<ListFunctionsResponse, ListFunctionsError> {
        let request_uri = "/2015-03-31/functions/";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.function_version {
            params.put("FunctionVersion", x);
        }
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.master_region {
            params.put("MasterRegion", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFunctionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListFunctionsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Returns a list of tags assigned to a function when supplied the function ARN (Amazon Resource Name). For more information on Tagging, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListTagsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>List all versions of a function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn list_versions_by_function(
        &self,
        input: ListVersionsByFunctionRequest,
    ) -> RusotoFuture<ListVersionsByFunctionResponse, ListVersionsByFunctionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/versions",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.marker {
            params.put("Marker", x);
        }
        if let Some(ref x) = input.max_items {
            params.put("MaxItems", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListVersionsByFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListVersionsByFunctionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Publishes a version of your function from the current snapshot of $LATEST. That is, AWS Lambda takes a snapshot of the function code and configuration information from $LATEST and publishes a new version. The code and configuration cannot be modified after publication. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p>
    fn publish_version(
        &self,
        input: PublishVersionRequest,
    ) -> RusotoFuture<FunctionConfiguration, PublishVersionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/versions",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PublishVersionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Sets a limit on the number of concurrent executions available to this function. It is a subset of your account's total concurrent execution limit per region. Note that Lambda automatically reserves a buffer of 100 concurrent executions for functions without any reserved concurrency limit. This means if your account limit is 1000, you have a total of 900 available to allocate to individual functions. For more information, see <a>concurrent-executions</a>.</p>
    fn put_function_concurrency(
        &self,
        input: PutFunctionConcurrencyRequest,
    ) -> RusotoFuture<Concurrency, PutFunctionConcurrencyError> {
        let request_uri = format!(
            "/2017-10-31/functions/{function_name}/concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Concurrency>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutFunctionConcurrencyError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>You can remove individual permissions from an resource policy associated with a Lambda function by providing a statement ID that you provided when you added the permission.</p> <p>If you are using versioning, the permissions you remove are specific to the Lambda function version or alias you specify in the <code>AddPermission</code> request via the <code>Qualifier</code> parameter. For more information about versioning, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>Note that removal of a permission will cause an active event source to lose permission to the function.</p> <p>You need permission for the <code>lambda:RemovePermission</code> action.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/policy/{statement_id}",
            function_name = input.function_name,
            statement_id = input.statement_id
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        if let Some(ref x) = input.revision_id {
            params.put("RevisionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemovePermissionError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Creates a list of tags (key-value pairs) on the Lambda function. Requires the Lambda function ARN (Amazon Resource Name). If a key is specified without a value, Lambda creates a tag with the specified key and a value of null. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>. </p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(TagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Removes tags from a Lambda function. Requires the function ARN (Amazon Resource Name). For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/tagging.html">Tagging Lambda Functions</a> in the <b>AWS Lambda Developer Guide</b>. </p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UntagResourceError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Using this API you can update the function version to which the alias points and the alias description. For more information, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/aliases-intro.html">Introduction to AWS Lambda Aliases</a>.</p> <p>This requires permission for the lambda:UpdateAlias action.</p>
    fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, UpdateAliasError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateAliasError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>You can update an event source mapping. This is useful if you want to change the parameters of the existing mapping without losing your position in the stream. You can change which function will receive the stream records, but to change the stream itself, you must create a new mapping.</p> <p>If you are using the versioning feature, you can update the event source mapping to map to a specific Lambda function version or alias as described in the <code>FunctionName</code> parameter. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>If you disable the event source mapping, AWS Lambda stops polling. If you enable again, it will resume polling from the time it had stopped polling, so you don't lose processing of any records. However, if you delete event source mapping and create it again, it will reset.</p> <p>This operation requires permission for the <code>lambda:UpdateEventSourceMapping</code> action.</p>
    fn update_event_source_mapping(
        &self,
        input: UpdateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, UpdateEventSourceMappingError> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 202 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EventSourceMappingConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateEventSourceMappingError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the code for the specified Lambda function. This operation must only be used on an existing Lambda function and cannot be used to update the function configuration.</p> <p>If you are using the versioning feature, note this API will always update the $LATEST version of your Lambda function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:UpdateFunctionCode</code> action.</p>
    fn update_function_code(
        &self,
        input: UpdateFunctionCodeRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionCodeError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/code",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFunctionCodeError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }

    /// <p>Updates the configuration parameters for the specified Lambda function by using the values provided in the request. You provide only the parameters you want to change. This operation must only be used on an existing Lambda function and cannot be used to update the function's code.</p> <p>If you are using the versioning feature, note this API will always update the $LATEST version of your Lambda function. For information about the versioning feature, see <a href="http://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">AWS Lambda Function Versioning and Aliases</a>. </p> <p>This operation requires permission for the <code>lambda:UpdateFunctionConfiguration</code> action.</p>
    fn update_function_configuration(
        &self,
        input: UpdateFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionConfigurationError> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/configuration",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFunctionConfigurationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
