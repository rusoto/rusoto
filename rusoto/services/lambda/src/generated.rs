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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoFuture};

use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};
use rusoto_core::request::HttpDispatchError;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Limits that are related to concurrency and code storage. All file and storage sizes are in bytes.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AccountLimit {
    /// <p>The maximum size of your function's code and layers when they're extracted.</p>
    #[serde(rename = "CodeSizeUnzipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_unzipped: Option<i64>,
    /// <p>The maximum size of a deployment package when it's uploaded directly to AWS Lambda. Use Amazon S3 for larger files.</p>
    #[serde(rename = "CodeSizeZipped")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size_zipped: Option<i64>,
    /// <p>The maximum number of simultaneous function executions.</p>
    #[serde(rename = "ConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrent_executions: Option<i64>,
    /// <p>The amount of storage space that you can use for all deployment packages and layer archives.</p>
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
    /// <p>The maximum number of simultaneous function executions, minus the capacity that's reserved for individual functions with <a>PutFunctionConcurrency</a>.</p>
    #[serde(rename = "UnreservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unreserved_concurrent_executions: Option<i64>,
}

/// <p>The number of functions and amount of storage in use.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AccountUsage {
    /// <p>The number of Lambda functions.</p>
    #[serde(rename = "FunctionCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_count: Option<i64>,
    /// <p>The amount of storage space, in bytes, that's being used by deployment packages and layer archives.</p>
    #[serde(rename = "TotalCodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_code_size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddLayerVersionPermissionRequest {
    /// <p>The API action that grants access to the layer. For example, <code>lambda:GetLayerVersion</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>With the principal set to <code>*</code>, grant permission to all accounts in the specified organization.</p>
    #[serde(rename = "OrganizationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// <p>An account ID, or <code>*</code> to grant permission to all AWS accounts.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>An identifier that distinguishes the policy from others on the same layer version.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddLayerVersionPermissionResponse {
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The permission statement.</p>
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddPermissionRequest {
    /// <p>The action that the principal can use on the function. For example, <code>lambda:InvokeFunction</code> or <code>lambda:GetFunction</code>.</p>
    #[serde(rename = "Action")]
    pub action: String,
    /// <p>For Alexa Smart Home functions, a token that must be supplied by the invoker.</p>
    #[serde(rename = "EventSourceToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_token: Option<String>,
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The AWS service or account that invokes the function. If you specify a service, use <code>SourceArn</code> or <code>SourceAccount</code> to limit who can invoke the function through that service.</p>
    #[serde(rename = "Principal")]
    pub principal: String,
    /// <p>Specify a version or alias to add permissions to a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    /// <p>Only update the policy if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>For AWS services, the ID of the account that owns the resource. Use this instead of <code>SourceArn</code> to grant permission to resources that are owned by another account (for example, all of an account's Amazon S3 buckets). Or use it together with <code>SourceArn</code> to ensure that the resource is owned by the specified account. For example, an Amazon S3 bucket could be deleted by its owner and recreated by another account.</p>
    #[serde(rename = "SourceAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_account: Option<String>,
    /// <p>For AWS services, the ARN of the AWS resource that invokes the function. For example, an Amazon S3 bucket or Amazon SNS topic.</p>
    #[serde(rename = "SourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    /// <p>A statement identifier that differentiates the statement from others in the same policy.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddPermissionResponse {
    /// <p>The permission statement that's added to the function policy.</p>
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

/// <p>Provides configuration information about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AliasConfiguration {
    /// <p>The Amazon Resource Name (ARN) of the alias.</p>
    #[serde(rename = "AliasArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_arn: Option<String>,
    /// <p>A description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The function version that the alias invokes.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A unique identifier that changes when you update the alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">traffic-shifting</a> configuration of a Lambda function alias.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AliasRoutingConfiguration {
    /// <p>The name of the second alias, and the percentage of traffic that's routed to it.</p>
    #[serde(rename = "AdditionalVersionWeights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_weights: Option<::std::collections::HashMap<String, f64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Concurrency {
    /// <p>The number of concurrent executions that are reserved for this function. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAliasRequest {
    /// <p>A description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The function version that the alias invokes.</p>
    #[serde(rename = "FunctionVersion")]
    pub function_version: String,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateEventSourceMappingRequest {
    /// <p><p>The maximum number of items to retrieve in a single batch.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - Default 100. Max 1,000.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - Default 10. Max 10.</p> </li> </ul></p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>Disables the event source mapping to pause polling and invocation.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p><p>The Amazon Resource Name (ARN) of the event source.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - The ARN of the data stream or a stream consumer.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - The ARN of the stream.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - The ARN of the queue.</p> </li> </ul></p>
    #[serde(rename = "EventSourceArn")]
    pub event_source_arn: String,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Version or Alias ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The position in a stream from which to start reading. Required for Amazon Kinesis and Amazon DynamoDB Streams sources. <code>AT_TIMESTAMP</code> is only supported for Amazon Kinesis streams.</p>
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    /// <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateFunctionRequest {
    /// <p>The code for the function.</p>
    #[serde(rename = "Code")]
    pub code: FunctionCode,
    /// <p>A dead letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">Dead Letter Queues</a>.</p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>A description of the function.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Environment variables that are accessible from function code during execution.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the method within your code that Lambda calls to execute your function. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/programming-model-v2.html">Programming Model</a>.</p>
    #[serde(rename = "Handler")]
    pub handler: String,
    /// <p>The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">function layers</a> to add to the function's execution environment. Specify each layer by its ARN, including the version.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// <p>The amount of memory that your function has access to. Increasing the function's memory also increases its CPU allocation. The default value is 128 MB. The value must be a multiple of 64 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>Set to true to publish the first version of the function during creation.</p>
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the function's execution role.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime</a>.</p>
    #[serde(rename = "Runtime")]
    pub runtime: String,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> to apply to the function.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The amount of time that Lambda allows a function to run before stopping it. The default is 3 seconds. The maximum allowed value is 900 seconds.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>Set <code>Mode</code> to <code>Active</code> to sample and trace a subset of incoming requests with AWS X-Ray.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    /// <p>For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can only access resources and the internet through that VPC. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/vpc.html">VPC Settings</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">dead letter queue</a> for failed asynchronous invocations.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeadLetterConfig {
    /// <p>The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic.</p>
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAliasRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteEventSourceMappingRequest {
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionConcurrencyRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteFunctionRequest {
    /// <p>The name of the Lambda function or version.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version to delete. You can't delete a version that's referenced by an alias.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteLayerVersionRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

/// <p>A function's environment variable settings.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Environment {
    /// <p>Environment variable key-value pairs.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error messages for environment variables that couldn't be applied.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnvironmentError {
    /// <p>The error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>The results of a configuration update that applied environment variables.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EnvironmentResponse {
    /// <p>Error messages for environment variables that couldn't be applied.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EnvironmentError>,
    /// <p>Environment variable key-value pairs.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A mapping between an AWS resource and an AWS Lambda function. See <a>CreateEventSourceMapping</a> for details.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventSourceMappingConfiguration {
    /// <p>The maximum number of items to retrieve in a single batch.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the event source.</p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The ARN of the Lambda function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The date that the event source mapping was last updated.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The result of the last AWS Lambda invocation of your Lambda function.</p>
    #[serde(rename = "LastProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processing_result: Option<String>,
    /// <p>The state of the event source mapping. It can be one of the following: <code>Creating</code>, <code>Enabling</code>, <code>Enabled</code>, <code>Disabling</code>, <code>Disabled</code>, <code>Updating</code>, or <code>Deleting</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The cause of the last state change, either <code>User initiated</code> or <code>Lambda initiated</code>.</p>
    #[serde(rename = "StateTransitionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_reason: Option<String>,
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// <p>The code for the Lambda function. You can specify either an object in Amazon S3, or upload a deployment package directly.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FunctionCode {
    /// <p>An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key of the deployment package.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>For versioned objects, the version of the deployment package object to use.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The base64-encoded contents of the deployment package. AWS SDK and AWS CLI clients handle the encoding for you.</p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Vec<u8>>,
}

/// <p>Details about a function's deployment package.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FunctionCodeLocation {
    /// <p>A presigned URL that you can use to download the deployment package.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The service that's hosting the file.</p>
    #[serde(rename = "RepositoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
}

/// <p>Details about a function's configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FunctionConfiguration {
    /// <p>The SHA256 hash of the function's deployment package.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The size of the function's deployment package, in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    /// <p>The function's dead letter queue.</p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>The function's description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The function's environment variables.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<EnvironmentResponse>,
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The name of the function.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The function that Lambda calls to begin executing your function.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The KMS key that's used to encrypt the function's environment variables. This key is only returned if you've configured a customer-managed CMK.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The date and time that the function was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html"> layers</a>.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
    /// <p>For Lambda@Edge functions, the ARN of the master function.</p>
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    /// <p>The memory that's allocated to the function.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>The latest updated revision of the function or alias.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The function's execution role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The runtime environment for the Lambda function.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The amount of time that Lambda allows a function to run before stopping it.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The function's AWS X-Ray tracing configuration.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfigResponse>,
    /// <p>The version of the Lambda function.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The function's networking configuration.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfigResponse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAccountSettingsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAccountSettingsResponse {
    /// <p>Limits that are related to concurrency and code storage.</p>
    #[serde(rename = "AccountLimit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_limit: Option<AccountLimit>,
    /// <p>The number of functions and amount of storage in use.</p>
    #[serde(rename = "AccountUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_usage: Option<AccountUsage>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAliasRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetEventSourceMappingRequest {
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionConfigurationRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get details about a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetFunctionRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get details about a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetFunctionResponse {
    /// <p>The deployment package of the function or version.</p>
    #[serde(rename = "Code")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<FunctionCodeLocation>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">reserved concurrency</a>.</p>
    #[serde(rename = "Concurrency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub concurrency: Option<Concurrency>,
    /// <p>The configuration of the function or version.</p>
    #[serde(rename = "Configuration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<FunctionConfiguration>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLayerVersionPolicyRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLayerVersionPolicyResponse {
    /// <p>The policy document.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetLayerVersionRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetLayerVersionResponse {
    /// <p>The layer's compatible runtimes.</p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>Details about the layer version.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    /// <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The description of the version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the layer.</p>
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    /// <p>The ARN of the layer version.</p>
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    /// <p>The layer's software license.</p>
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    /// <p>The version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetPolicyRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetPolicyResponse {
    /// <p>The resource-based policy.</p>
    #[serde(rename = "Policy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// <p>A unique identifier for the current revision of the policy.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvocationRequest {
    /// <p>Up to 3583 bytes of base64-encoded data about the invoking client to pass to the function in the context object.</p>
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p><p>Choose from the following options.</p> <ul> <li> <p> <code>RequestResponse</code> (default) - Invoke the function synchronously. Keep the connection open until the function returns a response or times out. The API response includes the function response and additional data.</p> </li> <li> <p> <code>Event</code> - Invoke the function asynchronously. Send events that fail multiple times to the function&#39;s dead-letter queue (if it&#39;s configured). The API response only includes a status code.</p> </li> <li> <p> <code>DryRun</code> - Validate parameter values and verify that the user or role has permission to invoke the function.</p> </li> </ul></p>
    #[serde(rename = "InvocationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_type: Option<String>,
    /// <p>Set to <code>Tail</code> to include the execution log in the response.</p>
    #[serde(rename = "LogType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_type: Option<String>,
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    #[serde(rename = "Payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Vec<u8>>,
    /// <p>Specify a version or alias to invoke a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct InvocationResponse {
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub executed_version: Option<String>,
    /// <p><p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p> <ul> <li> <p> <code>Handled</code> - The runtime caught an error thrown by the function and formatted it into a JSON document.</p> </li> <li> <p> <code>Unhandled</code> - The runtime didn&#39;t handle the error. For example, the function ran out of memory or timed out.</p> </li> </ul></p>
    pub function_error: Option<String>,
    /// <p>The last 4 KB of the execution log, which is base64 encoded.</p>
    pub log_result: Option<String>,
    /// <p>The response from the function, or an error object.</p>
    pub payload: Option<Vec<u8>>,
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub status_code: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct InvokeAsyncRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The JSON that you want to provide to your Lambda function as input.</p>
    #[serde(rename = "InvokeArgs")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub invoke_args: Vec<u8>,
}

/// <p>A success response (<code>202 Accepted</code>) indicates that the request is queued for invocation. </p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InvokeAsyncResponse {
    /// <p>The status code.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>An <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Layer {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
}

/// <p>A ZIP archive that contains the contents of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. You can specify either an Amazon S3 location, or upload a layer archive directly.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LayerVersionContentInput {
    /// <p>The Amazon S3 bucket of the layer archive.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key of the layer archive.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>For versioned objects, the version of the layer archive object to use.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The base64-encoded contents of the layer archive. AWS SDK and AWS CLI clients handle the encoding for you.</p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Vec<u8>>,
}

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LayerVersionContentOutput {
    /// <p>The SHA-256 hash of the layer archive.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    /// <p>A link to the layer archive in Amazon S3 that is valid for 10 minutes.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LayerVersionsListItem {
    /// <p>The layer's compatible runtimes.</p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>The date that the version was created, in ISO 8601 format. For example, <code>2018-11-27T15:10:45.123+0000</code>.</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The description of the version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the layer version.</p>
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    /// <p>The layer's open-source license.</p>
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    /// <p>The version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

/// <p>Details about an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LayersListItem {
    /// <p>The newest version of the layer.</p>
    #[serde(rename = "LatestMatchingVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_matching_version: Option<LayerVersionsListItem>,
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    /// <p>The name of the layer.</p>
    #[serde(rename = "LayerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAliasesRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a function version to only list aliases that invoke that version.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Limit the number of aliases returned.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAliasesResponse {
    /// <p>A list of aliases.</p>
    #[serde(rename = "Aliases")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<AliasConfiguration>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEventSourceMappingsRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the event source.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - The ARN of the data stream or a stream consumer.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - The ARN of the stream.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - The ARN of the queue.</p> </li> </ul></p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Version or Alias ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>A pagination token returned by a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of event source mappings to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListEventSourceMappingsResponse {
    /// <p>A list of event source mappings.</p>
    #[serde(rename = "EventSourceMappings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_mappings: Option<Vec<EventSourceMappingConfiguration>>,
    /// <p>A pagination token that's returned when the response doesn't contain all event source mappings.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFunctionsRequest {
    /// <p>Set to <code>ALL</code> to include entries for all published versions of each function.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>For Lambda@Edge functions, the AWS Region of the master function. For example, <code>us-east-2</code> or <code>ALL</code>. If specified, you must set <code>FunctionVersion</code> to <code>ALL</code>.</p>
    #[serde(rename = "MasterRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_region: Option<String>,
    /// <p>Specify a value between 1 and 50 to limit the number of functions in the response.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

/// <p>A list of Lambda functions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFunctionsResponse {
    /// <p>A list of Lambda functions.</p>
    #[serde(rename = "Functions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<FunctionConfiguration>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLayerVersionsRequest {
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    #[serde(rename = "CompatibleRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<String>,
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>A pagination token returned by a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of versions to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLayerVersionsResponse {
    /// <p>A list of versions.</p>
    #[serde(rename = "LayerVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_versions: Option<Vec<LayerVersionsListItem>>,
    /// <p>A pagination token returned when the response doesn't contain all versions.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListLayersRequest {
    /// <p>A runtime identifier. For example, <code>go1.x</code>.</p>
    #[serde(rename = "CompatibleRuntime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtime: Option<String>,
    /// <p>A pagination token returned by a previous call.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of layers to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListLayersResponse {
    /// <p>A list of function layers.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<LayersListItem>>,
    /// <p>A pagination token returned when the response doesn't contain all layers.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>The function's tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListVersionsByFunctionRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Limit the number of versions that are returned.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListVersionsByFunctionResponse {
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of Lambda function versions.</p>
    #[serde(rename = "Versions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<FunctionConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublishLayerVersionRequest {
    /// <p>A list of compatible <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">function runtimes</a>. Used for filtering with <a>ListLayers</a> and <a>ListLayerVersions</a>.</p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>The function layer archive.</p>
    #[serde(rename = "Content")]
    pub content: LayerVersionContentInput,
    /// <p>The description of the version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p><p>The layer&#39;s software license. It can be any of the following:</p> <ul> <li> <p>An <a href="https://spdx.org/licenses/">SPDX license identifier</a>. For example, <code>MIT</code>.</p> </li> <li> <p>The URL of a license hosted on the internet. For example, <code>https://opensource.org/licenses/MIT</code>.</p> </li> <li> <p>The full text of the license.</p> </li> </ul></p>
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PublishLayerVersionResponse {
    /// <p>The layer's compatible runtimes.</p>
    #[serde(rename = "CompatibleRuntimes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_runtimes: Option<Vec<String>>,
    /// <p>Details about the layer version.</p>
    #[serde(rename = "Content")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<LayerVersionContentOutput>,
    /// <p>The date that the layer version was created, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    /// <p>The description of the version.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ARN of the layer.</p>
    #[serde(rename = "LayerArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_arn: Option<String>,
    /// <p>The ARN of the layer version.</p>
    #[serde(rename = "LayerVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layer_version_arn: Option<String>,
    /// <p>The layer's software license.</p>
    #[serde(rename = "LicenseInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_info: Option<String>,
    /// <p>The version number.</p>
    #[serde(rename = "Version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PublishVersionRequest {
    /// <p>Only publish a version if the hash value matches the value that's specified. Use this option to avoid publishing a version if the function code has changed since you last updated it. You can get the hash for the version that you uploaded from the output of <a>UpdateFunctionCode</a>.</p>
    #[serde(rename = "CodeSha256")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_sha_256: Option<String>,
    /// <p>A description for the version to override the description in the function configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Only update the function if the revision ID matches the ID that's specified. Use this option to avoid publishing a version if the function configuration has changed since you last updated it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutFunctionConcurrencyRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The number of simultaneous executions to reserve for the function.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveLayerVersionPermissionRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>Only update the policy if the revision ID matches the ID specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The identifier that was specified when the statement was added.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemovePermissionRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to remove permissions from a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
    /// <p>Only update the policy if the revision ID matches the ID that's specified. Use this option to avoid modifying a policy that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>Statement ID of the permission to remove.</p>
    #[serde(rename = "StatementId")]
    pub statement_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A list of tags to apply to the function.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The function's AWS X-Ray tracing configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TracingConfig {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// <p>The function's AWS X-Ray tracing configuration.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TracingConfigResponse {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A list of tag keys to remove from the function.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAliasRequest {
    /// <p>A description of the alias.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The function version that the alias invokes.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>Only update the alias if the revision ID matches the ID that's specified. Use this option to avoid modifying an alias that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-traffic-shifting-using-aliases.html">routing configuration</a> of the alias.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateEventSourceMappingRequest {
    /// <p><p>The maximum number of items to retrieve in a single batch.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - Default 100. Max 1,000.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - Default 10. Max 10.</p> </li> </ul></p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>Disables the event source mapping to pause polling and invocation.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Version or Alias ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionCodeRequest {
    /// <p>Set to true to validate the request parameters and access permissions without modifying the function code.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Set to true to publish a new version of the function after updating the code. This has the same effect as calling <a>PublishVersion</a> separately.</p>
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// <p>Only update the function if the revision ID matches the ID that's specified. Use this option to avoid modifying a function that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>An Amazon S3 bucket in the same AWS Region as your function. The bucket can be in a different AWS account.</p>
    #[serde(rename = "S3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,
    /// <p>The Amazon S3 key of the deployment package.</p>
    #[serde(rename = "S3Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
    /// <p>For versioned objects, the version of the deployment package object to use.</p>
    #[serde(rename = "S3ObjectVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_object_version: Option<String>,
    /// <p>The base64-encoded contents of the deployment package. AWS SDK and AWS CLI clients handle the encoding for you.</p>
    #[serde(rename = "ZipFile")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip_file: Option<Vec<u8>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateFunctionConfigurationRequest {
    /// <p>A dead letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">Dead Letter Queues</a>.</p>
    #[serde(rename = "DeadLetterConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    /// <p>A description of the function.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Environment variables that are accessible from function code during execution.</p>
    #[serde(rename = "Environment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the method within your code that Lambda calls to execute your function. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/programming-model-v2.html">Programming Model</a>.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">function layers</a> to add to the function's execution environment. Specify each layer by its ARN, including the version.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// <p>The amount of memory that your function has access to. Increasing the function's memory also increases its CPU allocation. The default value is 128 MB. The value must be a multiple of 64 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>Only update the function if the revision ID matches the ID that's specified. Use this option to avoid modifying a function that has changed since you last read it.</p>
    #[serde(rename = "RevisionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the function's execution role.</p>
    #[serde(rename = "Role")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime</a>.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
    /// <p>The amount of time that Lambda allows a function to run before stopping it. The default is 3 seconds. The maximum allowed value is 900 seconds.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>Set <code>Mode</code> to <code>Active</code> to sample and trace a subset of incoming requests with AWS X-Ray.</p>
    #[serde(rename = "TracingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracing_config: Option<TracingConfig>,
    /// <p>For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can only access resources and the internet through that VPC. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/vpc.html">VPC Settings</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The VPC security groups and subnets that are attached to a Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct VpcConfig {
    /// <p>A list of VPC security groups IDs.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

/// <p>The VPC security groups and subnets that are attached to a Lambda function.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct VpcConfigResponse {
    /// <p>A list of VPC security groups IDs.</p>
    #[serde(rename = "SecurityGroupIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    /// <p>A list of VPC subnet IDs.</p>
    #[serde(rename = "SubnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The ID of the VPC.</p>
    #[serde(rename = "VpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

/// Errors returned by AddLayerVersionPermission
#[derive(Debug, PartialEq)]
pub enum AddLayerVersionPermissionError {
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddLayerVersionPermissionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> AddLayerVersionPermissionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return AddLayerVersionPermissionError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "PolicyLengthExceededException" => {
                    return AddLayerVersionPermissionError::PolicyLengthExceeded(String::from(
                        error_message,
                    ));
                }
                "PreconditionFailedException" => {
                    return AddLayerVersionPermissionError::PreconditionFailed(String::from(
                        error_message,
                    ));
                }
                "ResourceConflictException" => {
                    return AddLayerVersionPermissionError::ResourceConflict(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return AddLayerVersionPermissionError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return AddLayerVersionPermissionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return AddLayerVersionPermissionError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return AddLayerVersionPermissionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AddLayerVersionPermissionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AddLayerVersionPermissionError {
    fn from(err: serde_json::error::Error) -> AddLayerVersionPermissionError {
        AddLayerVersionPermissionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for AddLayerVersionPermissionError {
    fn from(err: CredentialsError) -> AddLayerVersionPermissionError {
        AddLayerVersionPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AddLayerVersionPermissionError {
    fn from(err: HttpDispatchError) -> AddLayerVersionPermissionError {
        AddLayerVersionPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for AddLayerVersionPermissionError {
    fn from(err: io::Error) -> AddLayerVersionPermissionError {
        AddLayerVersionPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AddLayerVersionPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddLayerVersionPermissionError {
    fn description(&self) -> &str {
        match *self {
            AddLayerVersionPermissionError::InvalidParameterValue(ref cause) => cause,
            AddLayerVersionPermissionError::PolicyLengthExceeded(ref cause) => cause,
            AddLayerVersionPermissionError::PreconditionFailed(ref cause) => cause,
            AddLayerVersionPermissionError::ResourceConflict(ref cause) => cause,
            AddLayerVersionPermissionError::ResourceNotFound(ref cause) => cause,
            AddLayerVersionPermissionError::Service(ref cause) => cause,
            AddLayerVersionPermissionError::TooManyRequests(ref cause) => cause,
            AddLayerVersionPermissionError::Validation(ref cause) => cause,
            AddLayerVersionPermissionError::Credentials(ref err) => err.description(),
            AddLayerVersionPermissionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AddLayerVersionPermissionError::ParseError(ref cause) => cause,
            AddLayerVersionPermissionError::Unknown(_) => "unknown error",
        }
    }
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl AddPermissionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> AddPermissionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return AddPermissionError::InvalidParameterValue(String::from(error_message));
                }
                "PolicyLengthExceededException" => {
                    return AddPermissionError::PolicyLengthExceeded(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return AddPermissionError::PreconditionFailed(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return AddPermissionError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return AddPermissionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return AddPermissionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return AddPermissionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return AddPermissionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return AddPermissionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for AddPermissionError {
    fn from(err: serde_json::error::Error) -> AddPermissionError {
        AddPermissionError::ParseError(err.description().to_string())
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
            AddPermissionError::ParseError(ref cause) => cause,
            AddPermissionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateAliasError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return CreateAliasError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return CreateAliasError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return CreateAliasError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return CreateAliasError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return CreateAliasError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAliasError {
    fn from(err: serde_json::error::Error) -> CreateAliasError {
        CreateAliasError::ParseError(err.description().to_string())
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
            CreateAliasError::ParseError(ref cause) => cause,
            CreateAliasError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateEventSourceMappingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateEventSourceMappingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return CreateEventSourceMappingError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceConflictException" => {
                    return CreateEventSourceMappingError::ResourceConflict(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return CreateEventSourceMappingError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return CreateEventSourceMappingError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateEventSourceMappingError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateEventSourceMappingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateEventSourceMappingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> CreateEventSourceMappingError {
        CreateEventSourceMappingError::ParseError(err.description().to_string())
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
            CreateEventSourceMappingError::ParseError(ref cause) => cause,
            CreateEventSourceMappingError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateFunction
#[derive(Debug, PartialEq)]
pub enum CreateFunctionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists.</p>
    ResourceConflict(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl CreateFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateFunctionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CodeStorageExceededException" => {
                    return CreateFunctionError::CodeStorageExceeded(String::from(error_message));
                }
                "InvalidParameterValueException" => {
                    return CreateFunctionError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return CreateFunctionError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return CreateFunctionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return CreateFunctionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return CreateFunctionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateFunctionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateFunctionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateFunctionError {
    fn from(err: serde_json::error::Error) -> CreateFunctionError {
        CreateFunctionError::ParseError(err.description().to_string())
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
            CreateFunctionError::ParseError(ref cause) => cause,
            CreateFunctionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteAliasError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return DeleteAliasError::InvalidParameterValue(String::from(error_message));
                }
                "ServiceException" => return DeleteAliasError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return DeleteAliasError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAliasError {
    fn from(err: serde_json::error::Error) -> DeleteAliasError {
        DeleteAliasError::ParseError(err.description().to_string())
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
            DeleteAliasError::ParseError(ref cause) => cause,
            DeleteAliasError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum DeleteEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSource Mapping in CREATING, or tried to delete a EventSource mapping currently in the UPDATING state. </p>
    ResourceInUse(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteEventSourceMappingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteEventSourceMappingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return DeleteEventSourceMappingError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceInUseException" => {
                    return DeleteEventSourceMappingError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteEventSourceMappingError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return DeleteEventSourceMappingError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteEventSourceMappingError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteEventSourceMappingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteEventSourceMappingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> DeleteEventSourceMappingError {
        DeleteEventSourceMappingError::ParseError(err.description().to_string())
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
            DeleteEventSourceMappingError::ParseError(ref cause) => cause,
            DeleteEventSourceMappingError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteFunctionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return DeleteFunctionError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceConflictException" => {
                    return DeleteFunctionError::ResourceConflict(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteFunctionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return DeleteFunctionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteFunctionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteFunctionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteFunctionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteFunctionError {
    fn from(err: serde_json::error::Error) -> DeleteFunctionError {
        DeleteFunctionError::ParseError(err.description().to_string())
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
            DeleteFunctionError::ParseError(ref cause) => cause,
            DeleteFunctionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteFunctionConcurrencyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteFunctionConcurrencyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return DeleteFunctionConcurrencyError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return DeleteFunctionConcurrencyError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return DeleteFunctionConcurrencyError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteFunctionConcurrencyError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteFunctionConcurrencyError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteFunctionConcurrencyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteFunctionConcurrencyError {
    fn from(err: serde_json::error::Error) -> DeleteFunctionConcurrencyError {
        DeleteFunctionConcurrencyError::ParseError(err.description().to_string())
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
            DeleteFunctionConcurrencyError::ParseError(ref cause) => cause,
            DeleteFunctionConcurrencyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteLayerVersion
#[derive(Debug, PartialEq)]
pub enum DeleteLayerVersionError {
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl DeleteLayerVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteLayerVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ServiceException" => {
                    return DeleteLayerVersionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return DeleteLayerVersionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteLayerVersionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteLayerVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteLayerVersionError {
    fn from(err: serde_json::error::Error) -> DeleteLayerVersionError {
        DeleteLayerVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteLayerVersionError {
    fn from(err: CredentialsError) -> DeleteLayerVersionError {
        DeleteLayerVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteLayerVersionError {
    fn from(err: HttpDispatchError) -> DeleteLayerVersionError {
        DeleteLayerVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteLayerVersionError {
    fn from(err: io::Error) -> DeleteLayerVersionError {
        DeleteLayerVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteLayerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteLayerVersionError {
    fn description(&self) -> &str {
        match *self {
            DeleteLayerVersionError::Service(ref cause) => cause,
            DeleteLayerVersionError::TooManyRequests(ref cause) => cause,
            DeleteLayerVersionError::Validation(ref cause) => cause,
            DeleteLayerVersionError::Credentials(ref err) => err.description(),
            DeleteLayerVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteLayerVersionError::ParseError(ref cause) => cause,
            DeleteLayerVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAccountSettingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAccountSettingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "ServiceException" => {
                    return GetAccountSettingsError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetAccountSettingsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetAccountSettingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetAccountSettingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAccountSettingsError {
    fn from(err: serde_json::error::Error) -> GetAccountSettingsError {
        GetAccountSettingsError::ParseError(err.description().to_string())
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
            GetAccountSettingsError::ParseError(ref cause) => cause,
            GetAccountSettingsError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetAliasError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetAliasError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetAliasError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return GetAliasError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetAliasError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAliasError {
    fn from(err: serde_json::error::Error) -> GetAliasError {
        GetAliasError::ParseError(err.description().to_string())
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
            GetAliasError::ParseError(ref cause) => cause,
            GetAliasError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetEventSourceMappingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetEventSourceMappingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetEventSourceMappingError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return GetEventSourceMappingError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return GetEventSourceMappingError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetEventSourceMappingError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetEventSourceMappingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetEventSourceMappingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> GetEventSourceMappingError {
        GetEventSourceMappingError::ParseError(err.description().to_string())
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
            GetEventSourceMappingError::ParseError(ref cause) => cause,
            GetEventSourceMappingError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFunctionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetFunctionError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetFunctionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return GetFunctionError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetFunctionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetFunctionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetFunctionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFunctionError {
    fn from(err: serde_json::error::Error) -> GetFunctionError {
        GetFunctionError::ParseError(err.description().to_string())
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
            GetFunctionError::ParseError(ref cause) => cause,
            GetFunctionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetFunctionConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetFunctionConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetFunctionConfigurationError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return GetFunctionConfigurationError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return GetFunctionConfigurationError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetFunctionConfigurationError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetFunctionConfigurationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetFunctionConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetFunctionConfigurationError {
    fn from(err: serde_json::error::Error) -> GetFunctionConfigurationError {
        GetFunctionConfigurationError::ParseError(err.description().to_string())
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
            GetFunctionConfigurationError::ParseError(ref cause) => cause,
            GetFunctionConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetLayerVersion
#[derive(Debug, PartialEq)]
pub enum GetLayerVersionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetLayerVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetLayerVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetLayerVersionError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetLayerVersionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return GetLayerVersionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetLayerVersionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetLayerVersionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetLayerVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetLayerVersionError {
    fn from(err: serde_json::error::Error) -> GetLayerVersionError {
        GetLayerVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLayerVersionError {
    fn from(err: CredentialsError) -> GetLayerVersionError {
        GetLayerVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLayerVersionError {
    fn from(err: HttpDispatchError) -> GetLayerVersionError {
        GetLayerVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLayerVersionError {
    fn from(err: io::Error) -> GetLayerVersionError {
        GetLayerVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLayerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLayerVersionError {
    fn description(&self) -> &str {
        match *self {
            GetLayerVersionError::InvalidParameterValue(ref cause) => cause,
            GetLayerVersionError::ResourceNotFound(ref cause) => cause,
            GetLayerVersionError::Service(ref cause) => cause,
            GetLayerVersionError::TooManyRequests(ref cause) => cause,
            GetLayerVersionError::Validation(ref cause) => cause,
            GetLayerVersionError::Credentials(ref err) => err.description(),
            GetLayerVersionError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetLayerVersionError::ParseError(ref cause) => cause,
            GetLayerVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetLayerVersionPolicy
#[derive(Debug, PartialEq)]
pub enum GetLayerVersionPolicyError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetLayerVersionPolicyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetLayerVersionPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetLayerVersionPolicyError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return GetLayerVersionPolicyError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return GetLayerVersionPolicyError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return GetLayerVersionPolicyError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetLayerVersionPolicyError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetLayerVersionPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetLayerVersionPolicyError {
    fn from(err: serde_json::error::Error) -> GetLayerVersionPolicyError {
        GetLayerVersionPolicyError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetLayerVersionPolicyError {
    fn from(err: CredentialsError) -> GetLayerVersionPolicyError {
        GetLayerVersionPolicyError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetLayerVersionPolicyError {
    fn from(err: HttpDispatchError) -> GetLayerVersionPolicyError {
        GetLayerVersionPolicyError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetLayerVersionPolicyError {
    fn from(err: io::Error) -> GetLayerVersionPolicyError {
        GetLayerVersionPolicyError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetLayerVersionPolicyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetLayerVersionPolicyError {
    fn description(&self) -> &str {
        match *self {
            GetLayerVersionPolicyError::InvalidParameterValue(ref cause) => cause,
            GetLayerVersionPolicyError::ResourceNotFound(ref cause) => cause,
            GetLayerVersionPolicyError::Service(ref cause) => cause,
            GetLayerVersionPolicyError::TooManyRequests(ref cause) => cause,
            GetLayerVersionPolicyError::Validation(ref cause) => cause,
            GetLayerVersionPolicyError::Credentials(ref err) => err.description(),
            GetLayerVersionPolicyError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetLayerVersionPolicyError::ParseError(ref cause) => cause,
            GetLayerVersionPolicyError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl GetPolicyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetPolicyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return GetPolicyError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetPolicyError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return GetPolicyError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return GetPolicyError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return GetPolicyError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetPolicyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetPolicyError {
    fn from(err: serde_json::error::Error) -> GetPolicyError {
        GetPolicyError::ParseError(err.description().to_string())
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
            GetPolicyError::ParseError(ref cause) => cause,
            GetPolicyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by Invoke
#[derive(Debug, PartialEq)]
pub enum InvokeError {
    /// <p>Need additional permissions to configure VPC settings.</p>
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
    /// <p>AWS Lambda could not unzip the deployment package.</p>
    InvalidZipFile(String),
    /// <p>Lambda was unable to decrypt the environment variables because KMS access was denied. Check the Lambda function's KMS permissions.</p>
    KMSAccessDenied(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key used is disabled. Check the Lambda function's KMS key settings.</p>
    KMSDisabled(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key used is in an invalid state for Decrypt. Check the function's KMS key settings.</p>
    KMSInvalidState(String),
    /// <p>Lambda was unable to decrypt the environment variables because the KMS key was not found. Check the function's KMS key settings. </p>
    KMSNotFound(String),
    /// <p>The request payload exceeded the <code>Invoke</code> request body JSON input limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a>. </p>
    RequestTooLarge(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>AWS Lambda was not able to set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses.</p>
    SubnetIPAddressLimitReached(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// <p>The content type of the <code>Invoke</code> request body is not JSON.</p>
    UnsupportedMediaType(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl InvokeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InvokeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "EC2AccessDeniedException" => {
                    return InvokeError::EC2AccessDenied(String::from(error_message));
                }
                "EC2ThrottledException" => {
                    return InvokeError::EC2Throttled(String::from(error_message));
                }
                "EC2UnexpectedException" => {
                    return InvokeError::EC2Unexpected(String::from(error_message));
                }
                "ENILimitReachedException" => {
                    return InvokeError::ENILimitReached(String::from(error_message));
                }
                "InvalidParameterValueException" => {
                    return InvokeError::InvalidParameterValue(String::from(error_message));
                }
                "InvalidRequestContentException" => {
                    return InvokeError::InvalidRequestContent(String::from(error_message));
                }
                "InvalidRuntimeException" => {
                    return InvokeError::InvalidRuntime(String::from(error_message));
                }
                "InvalidSecurityGroupIDException" => {
                    return InvokeError::InvalidSecurityGroupID(String::from(error_message));
                }
                "InvalidSubnetIDException" => {
                    return InvokeError::InvalidSubnetID(String::from(error_message));
                }
                "InvalidZipFileException" => {
                    return InvokeError::InvalidZipFile(String::from(error_message));
                }
                "KMSAccessDeniedException" => {
                    return InvokeError::KMSAccessDenied(String::from(error_message));
                }
                "KMSDisabledException" => {
                    return InvokeError::KMSDisabled(String::from(error_message));
                }
                "KMSInvalidStateException" => {
                    return InvokeError::KMSInvalidState(String::from(error_message));
                }
                "KMSNotFoundException" => {
                    return InvokeError::KMSNotFound(String::from(error_message));
                }
                "RequestTooLargeException" => {
                    return InvokeError::RequestTooLarge(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return InvokeError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return InvokeError::Service(String::from(error_message)),
                "SubnetIPAddressLimitReachedException" => {
                    return InvokeError::SubnetIPAddressLimitReached(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return InvokeError::TooManyRequests(String::from(error_message));
                }
                "UnsupportedMediaTypeException" => {
                    return InvokeError::UnsupportedMediaType(String::from(error_message));
                }
                "ValidationException" => return InvokeError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return InvokeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InvokeError {
    fn from(err: serde_json::error::Error) -> InvokeError {
        InvokeError::ParseError(err.description().to_string())
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
            InvokeError::ParseError(ref cause) => cause,
            InvokeError::Unknown(_) => "unknown error",
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
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl InvokeAsyncError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> InvokeAsyncError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidRequestContentException" => {
                    return InvokeAsyncError::InvalidRequestContent(String::from(error_message));
                }
                "InvalidRuntimeException" => {
                    return InvokeAsyncError::InvalidRuntime(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return InvokeAsyncError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return InvokeAsyncError::Service(String::from(error_message)),
                "ValidationException" => {
                    return InvokeAsyncError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return InvokeAsyncError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for InvokeAsyncError {
    fn from(err: serde_json::error::Error) -> InvokeAsyncError {
        InvokeAsyncError::ParseError(err.description().to_string())
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
            InvokeAsyncError::ParseError(ref cause) => cause,
            InvokeAsyncError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListAliasesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListAliasesError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListAliasesError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListAliasesError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return ListAliasesError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return ListAliasesError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListAliasesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListAliasesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAliasesError {
    fn from(err: serde_json::error::Error) -> ListAliasesError {
        ListAliasesError::ParseError(err.description().to_string())
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
            ListAliasesError::ParseError(ref cause) => cause,
            ListAliasesError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListEventSourceMappingsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListEventSourceMappingsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListEventSourceMappingsError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return ListEventSourceMappingsError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return ListEventSourceMappingsError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListEventSourceMappingsError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ListEventSourceMappingsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListEventSourceMappingsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListEventSourceMappingsError {
    fn from(err: serde_json::error::Error) -> ListEventSourceMappingsError {
        ListEventSourceMappingsError::ParseError(err.description().to_string())
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
            ListEventSourceMappingsError::ParseError(ref cause) => cause,
            ListEventSourceMappingsError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListFunctionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListFunctionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListFunctionsError::InvalidParameterValue(String::from(error_message));
                }
                "ServiceException" => {
                    return ListFunctionsError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListFunctionsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListFunctionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListFunctionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListFunctionsError {
    fn from(err: serde_json::error::Error) -> ListFunctionsError {
        ListFunctionsError::ParseError(err.description().to_string())
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
            ListFunctionsError::ParseError(ref cause) => cause,
            ListFunctionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListLayerVersions
#[derive(Debug, PartialEq)]
pub enum ListLayerVersionsError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListLayerVersionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListLayerVersionsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListLayerVersionsError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return ListLayerVersionsError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return ListLayerVersionsError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListLayerVersionsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListLayerVersionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListLayerVersionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListLayerVersionsError {
    fn from(err: serde_json::error::Error) -> ListLayerVersionsError {
        ListLayerVersionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLayerVersionsError {
    fn from(err: CredentialsError) -> ListLayerVersionsError {
        ListLayerVersionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLayerVersionsError {
    fn from(err: HttpDispatchError) -> ListLayerVersionsError {
        ListLayerVersionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListLayerVersionsError {
    fn from(err: io::Error) -> ListLayerVersionsError {
        ListLayerVersionsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListLayerVersionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLayerVersionsError {
    fn description(&self) -> &str {
        match *self {
            ListLayerVersionsError::InvalidParameterValue(ref cause) => cause,
            ListLayerVersionsError::ResourceNotFound(ref cause) => cause,
            ListLayerVersionsError::Service(ref cause) => cause,
            ListLayerVersionsError::TooManyRequests(ref cause) => cause,
            ListLayerVersionsError::Validation(ref cause) => cause,
            ListLayerVersionsError::Credentials(ref err) => err.description(),
            ListLayerVersionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListLayerVersionsError::ParseError(ref cause) => cause,
            ListLayerVersionsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListLayers
#[derive(Debug, PartialEq)]
pub enum ListLayersError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListLayersError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListLayersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListLayersError::InvalidParameterValue(String::from(error_message));
                }
                "ServiceException" => return ListLayersError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return ListLayersError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListLayersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListLayersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListLayersError {
    fn from(err: serde_json::error::Error) -> ListLayersError {
        ListLayersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListLayersError {
    fn from(err: CredentialsError) -> ListLayersError {
        ListLayersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListLayersError {
    fn from(err: HttpDispatchError) -> ListLayersError {
        ListLayersError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListLayersError {
    fn from(err: io::Error) -> ListLayersError {
        ListLayersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListLayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListLayersError {
    fn description(&self) -> &str {
        match *self {
            ListLayersError::InvalidParameterValue(ref cause) => cause,
            ListLayersError::Service(ref cause) => cause,
            ListLayersError::TooManyRequests(ref cause) => cause,
            ListLayersError::Validation(ref cause) => cause,
            ListLayersError::Credentials(ref err) => err.description(),
            ListLayersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListLayersError::ParseError(ref cause) => cause,
            ListLayersError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListTagsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListTagsError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListTagsError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return ListTagsError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return ListTagsError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListTagsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListTagsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsError {
    fn from(err: serde_json::error::Error) -> ListTagsError {
        ListTagsError::ParseError(err.description().to_string())
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
            ListTagsError::ParseError(ref cause) => cause,
            ListTagsError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl ListVersionsByFunctionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListVersionsByFunctionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return ListVersionsByFunctionError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return ListVersionsByFunctionError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return ListVersionsByFunctionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return ListVersionsByFunctionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return ListVersionsByFunctionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListVersionsByFunctionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListVersionsByFunctionError {
    fn from(err: serde_json::error::Error) -> ListVersionsByFunctionError {
        ListVersionsByFunctionError::ParseError(err.description().to_string())
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
            ListVersionsByFunctionError::ParseError(ref cause) => cause,
            ListVersionsByFunctionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PublishLayerVersion
#[derive(Debug, PartialEq)]
pub enum PublishLayerVersionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PublishLayerVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PublishLayerVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CodeStorageExceededException" => {
                    return PublishLayerVersionError::CodeStorageExceeded(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterValueException" => {
                    return PublishLayerVersionError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return PublishLayerVersionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return PublishLayerVersionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return PublishLayerVersionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return PublishLayerVersionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PublishLayerVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PublishLayerVersionError {
    fn from(err: serde_json::error::Error) -> PublishLayerVersionError {
        PublishLayerVersionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PublishLayerVersionError {
    fn from(err: CredentialsError) -> PublishLayerVersionError {
        PublishLayerVersionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PublishLayerVersionError {
    fn from(err: HttpDispatchError) -> PublishLayerVersionError {
        PublishLayerVersionError::HttpDispatch(err)
    }
}
impl From<io::Error> for PublishLayerVersionError {
    fn from(err: io::Error) -> PublishLayerVersionError {
        PublishLayerVersionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PublishLayerVersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PublishLayerVersionError {
    fn description(&self) -> &str {
        match *self {
            PublishLayerVersionError::CodeStorageExceeded(ref cause) => cause,
            PublishLayerVersionError::InvalidParameterValue(ref cause) => cause,
            PublishLayerVersionError::ResourceNotFound(ref cause) => cause,
            PublishLayerVersionError::Service(ref cause) => cause,
            PublishLayerVersionError::TooManyRequests(ref cause) => cause,
            PublishLayerVersionError::Validation(ref cause) => cause,
            PublishLayerVersionError::Credentials(ref err) => err.description(),
            PublishLayerVersionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PublishLayerVersionError::ParseError(ref cause) => cause,
            PublishLayerVersionError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PublishVersion
#[derive(Debug, PartialEq)]
pub enum PublishVersionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PublishVersionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PublishVersionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CodeStorageExceededException" => {
                    return PublishVersionError::CodeStorageExceeded(String::from(error_message));
                }
                "InvalidParameterValueException" => {
                    return PublishVersionError::InvalidParameterValue(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return PublishVersionError::PreconditionFailed(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return PublishVersionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return PublishVersionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return PublishVersionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return PublishVersionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PublishVersionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PublishVersionError {
    fn from(err: serde_json::error::Error) -> PublishVersionError {
        PublishVersionError::ParseError(err.description().to_string())
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
            PublishVersionError::ParseError(ref cause) => cause,
            PublishVersionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl PutFunctionConcurrencyError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutFunctionConcurrencyError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return PutFunctionConcurrencyError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return PutFunctionConcurrencyError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return PutFunctionConcurrencyError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return PutFunctionConcurrencyError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return PutFunctionConcurrencyError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutFunctionConcurrencyError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutFunctionConcurrencyError {
    fn from(err: serde_json::error::Error) -> PutFunctionConcurrencyError {
        PutFunctionConcurrencyError::ParseError(err.description().to_string())
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
            PutFunctionConcurrencyError::ParseError(ref cause) => cause,
            PutFunctionConcurrencyError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RemoveLayerVersionPermission
#[derive(Debug, PartialEq)]
pub enum RemoveLayerVersionPermissionError {
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemoveLayerVersionPermissionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RemoveLayerVersionPermissionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return RemoveLayerVersionPermissionError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "PreconditionFailedException" => {
                    return RemoveLayerVersionPermissionError::PreconditionFailed(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return RemoveLayerVersionPermissionError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return RemoveLayerVersionPermissionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return RemoveLayerVersionPermissionError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return RemoveLayerVersionPermissionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RemoveLayerVersionPermissionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RemoveLayerVersionPermissionError {
    fn from(err: serde_json::error::Error) -> RemoveLayerVersionPermissionError {
        RemoveLayerVersionPermissionError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RemoveLayerVersionPermissionError {
    fn from(err: CredentialsError) -> RemoveLayerVersionPermissionError {
        RemoveLayerVersionPermissionError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RemoveLayerVersionPermissionError {
    fn from(err: HttpDispatchError) -> RemoveLayerVersionPermissionError {
        RemoveLayerVersionPermissionError::HttpDispatch(err)
    }
}
impl From<io::Error> for RemoveLayerVersionPermissionError {
    fn from(err: io::Error) -> RemoveLayerVersionPermissionError {
        RemoveLayerVersionPermissionError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RemoveLayerVersionPermissionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveLayerVersionPermissionError {
    fn description(&self) -> &str {
        match *self {
            RemoveLayerVersionPermissionError::InvalidParameterValue(ref cause) => cause,
            RemoveLayerVersionPermissionError::PreconditionFailed(ref cause) => cause,
            RemoveLayerVersionPermissionError::ResourceNotFound(ref cause) => cause,
            RemoveLayerVersionPermissionError::Service(ref cause) => cause,
            RemoveLayerVersionPermissionError::TooManyRequests(ref cause) => cause,
            RemoveLayerVersionPermissionError::Validation(ref cause) => cause,
            RemoveLayerVersionPermissionError::Credentials(ref err) => err.description(),
            RemoveLayerVersionPermissionError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RemoveLayerVersionPermissionError::ParseError(ref cause) => cause,
            RemoveLayerVersionPermissionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl RemovePermissionError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RemovePermissionError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return RemovePermissionError::InvalidParameterValue(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return RemovePermissionError::PreconditionFailed(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return RemovePermissionError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return RemovePermissionError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return RemovePermissionError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return RemovePermissionError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RemovePermissionError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RemovePermissionError {
    fn from(err: serde_json::error::Error) -> RemovePermissionError {
        RemovePermissionError::ParseError(err.description().to_string())
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
            RemovePermissionError::ParseError(ref cause) => cause,
            RemovePermissionError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl TagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> TagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return TagResourceError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return TagResourceError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return TagResourceError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return TagResourceError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return TagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TagResourceError {
    fn from(err: serde_json::error::Error) -> TagResourceError {
        TagResourceError::ParseError(err.description().to_string())
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
            TagResourceError::ParseError(ref cause) => cause,
            TagResourceError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UntagResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UntagResourceError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return UntagResourceError::InvalidParameterValue(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UntagResourceError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return UntagResourceError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UntagResourceError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UntagResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UntagResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UntagResourceError {
    fn from(err: serde_json::error::Error) -> UntagResourceError {
        UntagResourceError::ParseError(err.description().to_string())
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
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateAliasError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateAliasError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return UpdateAliasError::InvalidParameterValue(String::from(error_message));
                }
                "PreconditionFailedException" => {
                    return UpdateAliasError::PreconditionFailed(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateAliasError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => return UpdateAliasError::Service(String::from(error_message)),
                "TooManyRequestsException" => {
                    return UpdateAliasError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateAliasError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateAliasError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateAliasError {
    fn from(err: serde_json::error::Error) -> UpdateAliasError {
        UpdateAliasError::ParseError(err.description().to_string())
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
            UpdateAliasError::ParseError(ref cause) => cause,
            UpdateAliasError::Unknown(_) => "unknown error",
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
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSource Mapping in CREATING, or tried to delete a EventSource mapping currently in the UPDATING state. </p>
    ResourceInUse(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateEventSourceMappingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateEventSourceMappingError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return UpdateEventSourceMappingError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "ResourceConflictException" => {
                    return UpdateEventSourceMappingError::ResourceConflict(String::from(
                        error_message,
                    ));
                }
                "ResourceInUseException" => {
                    return UpdateEventSourceMappingError::ResourceInUse(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateEventSourceMappingError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return UpdateEventSourceMappingError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateEventSourceMappingError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateEventSourceMappingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateEventSourceMappingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateEventSourceMappingError {
    fn from(err: serde_json::error::Error) -> UpdateEventSourceMappingError {
        UpdateEventSourceMappingError::ParseError(err.description().to_string())
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
            UpdateEventSourceMappingError::ParseError(ref cause) => cause,
            UpdateEventSourceMappingError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateFunctionCode
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionCodeError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Limits</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid. For example, if you provided an IAM role for AWS Lambda to assume in the <code>CreateFunction</code> or the <code>UpdateFunctionConfiguration</code> API, that AWS Lambda is unable to assume you will get this exception.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource (for example, a Lambda function or access policy statement) specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateFunctionCodeError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateFunctionCodeError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "CodeStorageExceededException" => {
                    return UpdateFunctionCodeError::CodeStorageExceeded(String::from(error_message));
                }
                "InvalidParameterValueException" => {
                    return UpdateFunctionCodeError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "PreconditionFailedException" => {
                    return UpdateFunctionCodeError::PreconditionFailed(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateFunctionCodeError::ResourceNotFound(String::from(error_message));
                }
                "ServiceException" => {
                    return UpdateFunctionCodeError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateFunctionCodeError::TooManyRequests(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateFunctionCodeError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateFunctionCodeError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateFunctionCodeError {
    fn from(err: serde_json::error::Error) -> UpdateFunctionCodeError {
        UpdateFunctionCodeError::ParseError(err.description().to_string())
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
            UpdateFunctionCodeError::ParseError(ref cause) => cause,
            UpdateFunctionCodeError::Unknown(_) => "unknown error",
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
    /// <p>Request throughput limit exceeded.</p>
    TooManyRequests(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An error occurred parsing the response payload.
    ParseError(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(BufferedHttpResponse),
}

impl UpdateFunctionConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateFunctionConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let error_type = match res.headers.get("x-amzn-errortype") {
                Some(raw_error_type) => raw_error_type
                    .split(':')
                    .next()
                    .unwrap_or_else(|| "Unknown"),
                _ => json
                    .get("code")
                    .or_else(|| json.get("Code"))
                    .and_then(|c| c.as_str())
                    .unwrap_or_else(|| "Unknown"),
            };

            // message can come in either "message" or "Message"
            // see boto BaseJSONParser impl for parsing message
            // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L595-L598
            let error_message = json
                .get("message")
                .or_else(|| json.get("Message"))
                .and_then(|m| m.as_str())
                .unwrap_or("");

            match error_type {
                "InvalidParameterValueException" => {
                    return UpdateFunctionConfigurationError::InvalidParameterValue(String::from(
                        error_message,
                    ));
                }
                "PreconditionFailedException" => {
                    return UpdateFunctionConfigurationError::PreconditionFailed(String::from(
                        error_message,
                    ));
                }
                "ResourceConflictException" => {
                    return UpdateFunctionConfigurationError::ResourceConflict(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return UpdateFunctionConfigurationError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceException" => {
                    return UpdateFunctionConfigurationError::Service(String::from(error_message));
                }
                "TooManyRequestsException" => {
                    return UpdateFunctionConfigurationError::TooManyRequests(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateFunctionConfigurationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateFunctionConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateFunctionConfigurationError {
    fn from(err: serde_json::error::Error) -> UpdateFunctionConfigurationError {
        UpdateFunctionConfigurationError::ParseError(err.description().to_string())
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
            UpdateFunctionConfigurationError::ParseError(ref cause) => cause,
            UpdateFunctionConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the AWS Lambda API. AWS Lambda clients implement this trait.
pub trait Lambda {
    /// <p>Adds permissions to the resource-based policy of a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Use this action to grant layer usage permission to other accounts. You can grant permission to a single account, all AWS accounts, or all accounts in an organization.</p> <p>To revoke permission, call <a>RemoveLayerVersionPermission</a> with the statement ID that you specified when you added it.</p>
    fn add_layer_version_permission(
        &self,
        input: AddLayerVersionPermissionRequest,
    ) -> RusotoFuture<AddLayerVersionPermissionResponse, AddLayerVersionPermissionError>;

    /// <p>Grants an AWS service or another account permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version or alias to invoke the function.</p> <p>To grant permission to another account, specify the account ID as the <code>Principal</code>. For AWS services, the principal is a domain-style identifier defined by the service, like <code>s3.amazonaws.com</code> or <code>sns.amazonaws.com</code>. For AWS services, you can also specify the ARN or owning account of the associated resource as the <code>SourceArn</code> or <code>SourceAccount</code>. If you grant permission to a service principal without specifying the source, other accounts could potentially configure resources in their account to invoke your Lambda function.</p> <p>This action adds a statement to a resource-based permission policy for the function. For more information about function policies, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Lambda Function Policies</a>. </p>
    fn add_permission(
        &self,
        input: AddPermissionRequest,
    ) -> RusotoFuture<AddPermissionResponse, AddPermissionError>;

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a> for a Lambda function version. Use aliases to provide clients with a function identifier that you can update to invoke a different version.</p> <p>You can also map an alias to split invocation requests between two versions. Use the <code>RoutingConfig</code> parameter to specify a second version and the percentage of invocation requests that it receives.</p>
    fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, CreateAliasError>;

    /// <p><p>Creates a mapping between an event source and an AWS Lambda function. Lambda reads items from the event source and triggers the function.</p> <p>For details about each event source type, see the following topics.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html">Using AWS Lambda with Amazon Kinesis</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html">Using AWS Lambda with Amazon SQS</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html">Using AWS Lambda with Amazon DynamoDB</a> </p> </li> </ul></p>
    fn create_event_source_mapping(
        &self,
        input: CreateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, CreateEventSourceMappingError>;

    /// <p>Creates a Lambda function. To create a function, you need a <a href="https://docs.aws.amazon.com/lambda/latest/dg/deployment-package-v2.html">deployment package</a> and an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role">execution role</a>. The deployment package contains your function code. The execution role grants the function permission to use AWS services, such as Amazon CloudWatch Logs for log streaming and AWS X-Ray for request tracing.</p> <p>A function has an unpublished version, and can have published versions and aliases. The unpublished version changes when you update your function's code and configuration. A published version is a snapshot of your function code and configuration that can't be changed. An alias is a named resource that maps to a version, and can be changed to map to a different version. Use the <code>Publish</code> parameter to create version <code>1</code> of your function from its initial configuration.</p> <p>The other parameters let you configure version-specific and function-level settings. You can modify version-specific settings later with <a>UpdateFunctionConfiguration</a>. Function-level settings apply to both the unpublished and published versions of the function, and include tags (<a>TagResource</a>) and per-function concurrency limits (<a>PutFunctionConcurrency</a>).</p> <p>If another account or an AWS service invokes your function, use <a>AddPermission</a> to grant permission by creating a resource-based IAM policy. You can grant permissions at the function level, on a version, or on an alias.</p> <p>To invoke your function directly, use <a>Invoke</a>. To invoke your function in response to events in other AWS services, create an event source mapping (<a>CreateEventSourceMapping</a>), or configure a function trigger in the other service. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invoking-lambda-functions.html">Invoking Functions</a>.</p>
    fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> RusotoFuture<FunctionConfiguration, CreateFunctionError>;

    /// <p>Deletes a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    fn delete_alias(&self, input: DeleteAliasRequest) -> RusotoFuture<(), DeleteAliasError>;

    /// <p>Deletes an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-invocation-modes.html">event source mapping</a>. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
    fn delete_event_source_mapping(
        &self,
        input: DeleteEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, DeleteEventSourceMappingError>;

    /// <p>Deletes a Lambda function. To delete a specific function version, use the <code>Qualifier</code> parameter. Otherwise, all versions and aliases are deleted.</p> <p>To delete Lambda event source mappings that invoke a function, use <a>DeleteEventSourceMapping</a>. For AWS services and resources that invoke your function directly, delete the trigger in the service where you originally configured it.</p>
    fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> RusotoFuture<(), DeleteFunctionError>;

    /// <p>Removes a concurrent execution limit from a function.</p>
    fn delete_function_concurrency(
        &self,
        input: DeleteFunctionConcurrencyRequest,
    ) -> RusotoFuture<(), DeleteFunctionConcurrencyError>;

    /// <p>Deletes a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Deleted versions can no longer be viewed or added to functions. To avoid breaking functions, a copy of the version remains in Lambda until no functions refer to it.</p>
    fn delete_layer_version(
        &self,
        input: DeleteLayerVersionRequest,
    ) -> RusotoFuture<(), DeleteLayerVersionError>;

    /// <p>Retrieves details about your account's <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limits</a> and usage in an AWS Region.</p>
    fn get_account_settings(
        &self,
    ) -> RusotoFuture<GetAccountSettingsResponse, GetAccountSettingsError>;

    /// <p>Returns details about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    fn get_alias(&self, input: GetAliasRequest) -> RusotoFuture<AliasConfiguration, GetAliasError>;

    /// <p>Returns details about an event source mapping. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
    fn get_event_source_mapping(
        &self,
        input: GetEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, GetEventSourceMappingError>;

    /// <p>Returns information about the function or function version, with a link to download the deployment package that's valid for 10 minutes. If you specify a function version, only details that are specific to that version are returned.</p>
    fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> RusotoFuture<GetFunctionResponse, GetFunctionError>;

    /// <p>Returns the version-specific settings of a Lambda function or version. The output includes only options that can vary between versions of a function. To modify these settings, use <a>UpdateFunctionConfiguration</a>.</p> <p>To get all of a function's details, including function-level settings, use <a>GetFunction</a>.</p>
    fn get_function_configuration(
        &self,
        input: GetFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, GetFunctionConfigurationError>;

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    fn get_layer_version(
        &self,
        input: GetLayerVersionRequest,
    ) -> RusotoFuture<GetLayerVersionResponse, GetLayerVersionError>;

    /// <p>Returns the permission policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    fn get_layer_version_policy(
        &self,
        input: GetLayerVersionPolicyRequest,
    ) -> RusotoFuture<GetLayerVersionPolicyResponse, GetLayerVersionPolicyError>;

    /// <p>Returns the <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">resource-based IAM policy</a> for a function, version, or alias.</p>
    fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> RusotoFuture<GetPolicyResponse, GetPolicyError>;

    /// <p>Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. To invoke a function asynchronously, set <code>InvocationType</code> to <code>Event</code>.</p> <p>For synchronous invocation, details about the function response, including errors, are included in the response body and headers. For either invocation type, you can find more information in the <a href="https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions.html">execution log</a> and <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">trace</a>. To record function errors for asynchronous invocations, configure your function with a <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">dead letter queue</a>.</p> <p>The status code in the API response doesn't reflect function errors. Error codes are reserved for errors that prevent your function from executing, such as permissions errors, <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limit errors</a>, or issues with your function's code and configuration. For example, Lambda returns <code>TooManyRequestsException</code> if executing the function would cause you to exceed a concurrency limit at either the account level (<code>ConcurrentInvocationLimitExceeded</code>) or function level (<code>ReservedFunctionConcurrentInvocationLimitExceeded</code>).</p> <p>For functions with a long timeout, your client might be disconnected during synchronous invocation while it waits for a response. Configure your HTTP client, SDK, firewall, proxy, or operating system to allow for long connections with timeout or keep-alive settings.</p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p>
    fn invoke(&self, input: InvocationRequest) -> RusotoFuture<InvocationResponse, InvokeError>;

    /// <p><important> <p>For asynchronous function invocation, use <a>Invoke</a>.</p> </important> <p>Invokes a function asynchronously.</p></p>
    fn invoke_async(
        &self,
        input: InvokeAsyncRequest,
    ) -> RusotoFuture<InvokeAsyncResponse, InvokeAsyncError>;

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">aliases</a> for a Lambda function.</p>
    fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> RusotoFuture<ListAliasesResponse, ListAliasesError>;

    /// <p>Lists event source mappings. Specify an <code>EventSourceArn</code> to only show event source mappings for a single event source.</p>
    fn list_event_source_mappings(
        &self,
        input: ListEventSourceMappingsRequest,
    ) -> RusotoFuture<ListEventSourceMappingsResponse, ListEventSourceMappingsError>;

    /// <p>Returns a list of Lambda functions, with the version-specific configuration of each.</p> <p>Set <code>FunctionVersion</code> to <code>ALL</code> to include all published versions of each function in addition to the unpublished version. To get more information about a function or version, use <a>GetFunction</a>.</p>
    fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> RusotoFuture<ListFunctionsResponse, ListFunctionsError>;

    /// <p>Lists the versions of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Versions that have been deleted aren't listed. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only versions that indicate that they're compatible with that runtime.</p>
    fn list_layer_versions(
        &self,
        input: ListLayerVersionsRequest,
    ) -> RusotoFuture<ListLayerVersionsResponse, ListLayerVersionsError>;

    /// <p>Lists <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layers</a> and shows information about the latest version of each. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only layers that indicate that they're compatible with that runtime.</p>
    fn list_layers(
        &self,
        input: ListLayersRequest,
    ) -> RusotoFuture<ListLayersResponse, ListLayersError>;

    /// <p>Returns a function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>. You can also view tags with <a>GetFunction</a>.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError>;

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">versions</a>, with the version-specific configuration of each. </p>
    fn list_versions_by_function(
        &self,
        input: ListVersionsByFunctionRequest,
    ) -> RusotoFuture<ListVersionsByFunctionResponse, ListVersionsByFunctionError>;

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a> from a ZIP archive. Each time you call <code>PublishLayerVersion</code> with the same version name, a new version is created.</p> <p>Add layers to your function with <a>CreateFunction</a> or <a>UpdateFunctionConfiguration</a>.</p>
    fn publish_layer_version(
        &self,
        input: PublishLayerVersionRequest,
    ) -> RusotoFuture<PublishLayerVersionResponse, PublishLayerVersionError>;

    /// <p>Creates a <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">version</a> from the current code and configuration of a function. Use versions to create a snapshot of your function code and configuration that doesn't change.</p> <p>AWS Lambda doesn't publish a version if the function's configuration and code haven't changed since the last version. Use <a>UpdateFunctionCode</a> or <a>UpdateFunctionConfiguration</a> to update the function before publishing a version.</p> <p>Clients can invoke versions directly or with an alias. To create an alias, use <a>CreateAlias</a>.</p>
    fn publish_version(
        &self,
        input: PublishVersionRequest,
    ) -> RusotoFuture<FunctionConfiguration, PublishVersionError>;

    /// <p>Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level.</p> <p>Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. Reserving concurrency both ensures that your function has capacity to process the specified number of events simultaneously, and prevents it from scaling beyond that level. Use <a>GetFunction</a> to see the current setting for a function.</p> <p>Use <a>GetAccountSettings</a> to see your regional concurrency limit. You can reserve concurrency for as many functions as you like, as long as you leave at least 100 simultaneous executions unreserved for functions that aren't configured with a per-function limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
    fn put_function_concurrency(
        &self,
        input: PutFunctionConcurrencyRequest,
    ) -> RusotoFuture<Concurrency, PutFunctionConcurrencyError>;

    /// <p>Removes a statement from the permissions policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    fn remove_layer_version_permission(
        &self,
        input: RemoveLayerVersionPermissionRequest,
    ) -> RusotoFuture<(), RemoveLayerVersionPermissionError>;

    /// <p>Revokes function-use permission from an AWS service or another account. You can get the ID of the statement from the output of <a>GetPolicy</a>.</p>
    fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> RusotoFuture<(), RemovePermissionError>;

    /// <p>Adds <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> to a function.</p>
    fn tag_resource(&self, input: TagResourceRequest) -> RusotoFuture<(), TagResourceError>;

    /// <p>Removes <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> from a function.</p>
    fn untag_resource(&self, input: UntagResourceRequest) -> RusotoFuture<(), UntagResourceError>;

    /// <p>Updates the configuration of a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> RusotoFuture<AliasConfiguration, UpdateAliasError>;

    /// <p>Updates an event source mapping. You can change the function that AWS Lambda invokes, or pause invocation and resume later from the same location.</p>
    fn update_event_source_mapping(
        &self,
        input: UpdateEventSourceMappingRequest,
    ) -> RusotoFuture<EventSourceMappingConfiguration, UpdateEventSourceMappingError>;

    /// <p>Updates a Lambda function's code.</p> <p>The function's code is locked when you publish a version. You can't modify the code of a published version, only the unpublished version.</p>
    fn update_function_code(
        &self,
        input: UpdateFunctionCodeRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionCodeError>;

    /// <p>Modify the version-specifc settings of a Lambda function.</p> <p>These settings can vary between versions of a function and are locked when you publish a version. You can't modify the configuration of a published version, only the unpublished version.</p> <p>To configure function concurrency, use <a>PutFunctionConcurrency</a>. To grant invoke permissions to an account or AWS service, use <a>AddPermission</a>.</p>
    fn update_function_configuration(
        &self,
        input: UpdateFunctionConfigurationRequest,
    ) -> RusotoFuture<FunctionConfiguration, UpdateFunctionConfigurationError>;
}
/// A client for the AWS Lambda API.
#[derive(Clone)]
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
    /// <p>Adds permissions to the resource-based policy of a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Use this action to grant layer usage permission to other accounts. You can grant permission to a single account, all AWS accounts, or all accounts in an organization.</p> <p>To revoke permission, call <a>RemoveLayerVersionPermission</a> with the statement ID that you specified when you added it.</p>
    fn add_layer_version_permission(
        &self,
        input: AddLayerVersionPermissionRequest,
    ) -> RusotoFuture<AddLayerVersionPermissionResponse, AddLayerVersionPermissionError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}/policy",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.revision_id {
            params.put("RevisionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<AddLayerVersionPermissionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddLayerVersionPermissionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Grants an AWS service or another account permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version or alias to invoke the function.</p> <p>To grant permission to another account, specify the account ID as the <code>Principal</code>. For AWS services, the principal is a domain-style identifier defined by the service, like <code>s3.amazonaws.com</code> or <code>sns.amazonaws.com</code>. For AWS services, you can also specify the ARN or owning account of the associated resource as the <code>SourceArn</code> or <code>SourceAccount</code>. If you grant permission to a service principal without specifying the source, other accounts could potentially configure resources in their account to invoke your Lambda function.</p> <p>This action adds a statement to a resource-based permission policy for the function. For more information about function policies, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Lambda Function Policies</a>. </p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AddPermissionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(AddPermissionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a> for a Lambda function version. Use aliases to provide clients with a function identifier that you can update to invoke a different version.</p> <p>You can also map an alias to split invocation requests between two versions. Use the <code>RoutingConfig</code> parameter to specify a second version and the percentage of invocation requests that it receives.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p><p>Creates a mapping between an event source and an AWS Lambda function. Lambda reads items from the event source and triggers the function.</p> <p>For details about each event source type, see the following topics.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html">Using AWS Lambda with Amazon Kinesis</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html">Using AWS Lambda with Amazon SQS</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html">Using AWS Lambda with Amazon DynamoDB</a> </p> </li> </ul></p>
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

                    if body == b"null" || body.is_empty() {
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
                    Err(CreateEventSourceMappingError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a Lambda function. To create a function, you need a <a href="https://docs.aws.amazon.com/lambda/latest/dg/deployment-package-v2.html">deployment package</a> and an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role">execution role</a>. The deployment package contains your function code. The execution role grants the function permission to use AWS services, such as Amazon CloudWatch Logs for log streaming and AWS X-Ray for request tracing.</p> <p>A function has an unpublished version, and can have published versions and aliases. The unpublished version changes when you update your function's code and configuration. A published version is a snapshot of your function code and configuration that can't be changed. An alias is a named resource that maps to a version, and can be changed to map to a different version. Use the <code>Publish</code> parameter to create version <code>1</code> of your function from its initial configuration.</p> <p>The other parameters let you configure version-specific and function-level settings. You can modify version-specific settings later with <a>UpdateFunctionConfiguration</a>. Function-level settings apply to both the unpublished and published versions of the function, and include tags (<a>TagResource</a>) and per-function concurrency limits (<a>PutFunctionConcurrency</a>).</p> <p>If another account or an AWS service invokes your function, use <a>AddPermission</a> to grant permission by creating a resource-based IAM policy. You can grant permissions at the function level, on a version, or on an alias.</p> <p>To invoke your function directly, use <a>Invoke</a>. To invoke your function in response to events in other AWS services, create an event source mapping (<a>CreateEventSourceMapping</a>), or configure a function trigger in the other service. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invoking-lambda-functions.html">Invoking Functions</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-invocation-modes.html">event source mapping</a>. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
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

                    if body == b"null" || body.is_empty() {
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
                    Err(DeleteEventSourceMappingError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a Lambda function. To delete a specific function version, use the <code>Qualifier</code> parameter. Otherwise, all versions and aliases are deleted.</p> <p>To delete Lambda event source mappings that invoke a function, use <a>DeleteEventSourceMapping</a>. For AWS services and resources that invoke your function directly, delete the trigger in the service where you originally configured it.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes a concurrent execution limit from a function.</p>
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
                    Err(DeleteFunctionConcurrencyError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Deleted versions can no longer be viewed or added to functions. To avoid breaking functions, a copy of the version remains in Lambda until no functions refer to it.</p>
    fn delete_layer_version(
        &self,
        input: DeleteLayerVersionRequest,
    ) -> RusotoFuture<(), DeleteLayerVersionError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}",
            layer_name = input.layer_name,
            version_number = input.version_number
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteLayerVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves details about your account's <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limits</a> and usage in an AWS Region.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetAccountSettingsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAccountSettingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns details about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns details about an event source mapping. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<EventSourceMappingConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetEventSourceMappingError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns information about the function or function version, with a link to download the deployment package that's valid for 10 minutes. If you specify a function version, only details that are specific to that version are returned.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetFunctionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the version-specific settings of a Lambda function or version. The output includes only options that can vary between versions of a function. To modify these settings, use <a>UpdateFunctionConfiguration</a>.</p> <p>To get all of a function's details, including function-level settings, use <a>GetFunction</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetFunctionConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    fn get_layer_version(
        &self,
        input: GetLayerVersionRequest,
    ) -> RusotoFuture<GetLayerVersionResponse, GetLayerVersionError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetLayerVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetLayerVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns the permission policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    fn get_layer_version_policy(
        &self,
        input: GetLayerVersionPolicyRequest,
    ) -> RusotoFuture<GetLayerVersionPolicyResponse, GetLayerVersionPolicyError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}/policy",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetLayerVersionPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetLayerVersionPolicyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Returns the <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">resource-based IAM policy</a> for a function, version, or alias.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<GetPolicyResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetPolicyError::from_response(response))),
                )
            }
        })
    }

    /// <p>Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. To invoke a function asynchronously, set <code>InvocationType</code> to <code>Event</code>.</p> <p>For synchronous invocation, details about the function response, including errors, are included in the response body and headers. For either invocation type, you can find more information in the <a href="https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions.html">execution log</a> and <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">trace</a>. To record function errors for asynchronous invocations, configure your function with a <a href="https://docs.aws.amazon.com/lambda/latest/dg/dlq.html">dead letter queue</a>.</p> <p>The status code in the API response doesn't reflect function errors. Error codes are reserved for errors that prevent your function from executing, such as permissions errors, <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limit errors</a>, or issues with your function's code and configuration. For example, Lambda returns <code>TooManyRequestsException</code> if executing the function would cause you to exceed a concurrency limit at either the account level (<code>ConcurrentInvocationLimitExceeded</code>) or function level (<code>ReservedFunctionConcurrentInvocationLimitExceeded</code>).</p> <p>For functions with a long timeout, your client might be disconnected during synchronous invocation while it waits for a response. Configure your HTTP client, SDK, firewall, proxy, or operating system to allow for long connections with timeout or keep-alive settings.</p> <p>This operation requires permission for the <code>lambda:InvokeFunction</code> action.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InvokeError::from_response(response))),
                )
            }
        })
    }

    /// <p><important> <p>For asynchronous function invocation, use <a>Invoke</a>.</p> </important> <p>Invokes a function asynchronously.</p></p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let mut result = serde_json::from_slice::<InvokeAsyncResponse>(&body).unwrap();

                    result.status = Some(response.status.as_u16() as i64);
                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(InvokeAsyncError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">aliases</a> for a Lambda function.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListAliasesResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAliasesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists event source mappings. Specify an <code>EventSourceArn</code> to only show event source mappings for a single event source.</p>
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

                    if body == b"null" || body.is_empty() {
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
                    Err(ListEventSourceMappingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Returns a list of Lambda functions, with the version-specific configuration of each.</p> <p>Set <code>FunctionVersion</code> to <code>ALL</code> to include all published versions of each function in addition to the unpublished version. To get more information about a function or version, use <a>GetFunction</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListFunctionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFunctionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the versions of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Versions that have been deleted aren't listed. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only versions that indicate that they're compatible with that runtime.</p>
    fn list_layer_versions(
        &self,
        input: ListLayerVersionsRequest,
    ) -> RusotoFuture<ListLayerVersionsResponse, ListLayerVersionsError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions",
            layer_name = input.layer_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.compatible_runtime {
            params.put("CompatibleRuntime", x);
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListLayerVersionsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLayerVersionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layers</a> and shows information about the latest version of each. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only layers that indicate that they're compatible with that runtime.</p>
    fn list_layers(
        &self,
        input: ListLayersRequest,
    ) -> RusotoFuture<ListLayersResponse, ListLayersError> {
        let request_uri = "/2018-10-31/layers";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.compatible_runtime {
            params.put("CompatibleRuntime", x);
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListLayersResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListLayersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>. You can also view tags with <a>GetFunction</a>.</p>
    fn list_tags(&self, input: ListTagsRequest) -> RusotoFuture<ListTagsResponse, ListTagsError> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListTagsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListTagsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">versions</a>, with the version-specific configuration of each. </p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListVersionsByFunctionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListVersionsByFunctionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a> from a ZIP archive. Each time you call <code>PublishLayerVersion</code> with the same version name, a new version is created.</p> <p>Add layers to your function with <a>CreateFunction</a> or <a>UpdateFunctionConfiguration</a>.</p>
    fn publish_layer_version(
        &self,
        input: PublishLayerVersionRequest,
    ) -> RusotoFuture<PublishLayerVersionResponse, PublishLayerVersionError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions",
            layer_name = input.layer_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<PublishLayerVersionResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PublishLayerVersionError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">version</a> from the current code and configuration of a function. Use versions to create a snapshot of your function code and configuration that doesn't change.</p> <p>AWS Lambda doesn't publish a version if the function's configuration and code haven't changed since the last version. Use <a>UpdateFunctionCode</a> or <a>UpdateFunctionConfiguration</a> to update the function before publishing a version.</p> <p>Clients can invoke versions directly or with an alias. To create an alias, use <a>CreateAlias</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PublishVersionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level.</p> <p>Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. Reserving concurrency both ensures that your function has capacity to process the specified number of events simultaneously, and prevents it from scaling beyond that level. Use <a>GetFunction</a> to see the current setting for a function.</p> <p>Use <a>GetAccountSettings</a> to see your regional concurrency limit. You can reserve concurrency for as many functions as you like, as long as you leave at least 100 simultaneous executions unreserved for functions that aren't configured with a per-function limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<Concurrency>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(PutFunctionConcurrencyError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Removes a statement from the permissions policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    fn remove_layer_version_permission(
        &self,
        input: RemoveLayerVersionPermissionRequest,
    ) -> RusotoFuture<(), RemoveLayerVersionPermissionError> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}/policy/{statement_id}",
            layer_name = input.layer_name,
            statement_id = input.statement_id,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
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
                    Err(RemoveLayerVersionPermissionError::from_response(response))
                }))
            }
        })
    }

    /// <p>Revokes function-use permission from an AWS service or another account. You can get the ID of the statement from the output of <a>GetPolicy</a>.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RemovePermissionError::from_response(response))),
                )
            }
        })
    }

    /// <p>Adds <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> to a function.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Removes <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> from a function.</p>
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
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UntagResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the configuration of a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<AliasConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAliasError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an event source mapping. You can change the function that AWS Lambda invokes, or pause invocation and resume later from the same location.</p>
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

                    if body == b"null" || body.is_empty() {
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
                    Err(UpdateEventSourceMappingError::from_response(response))
                }))
            }
        })
    }

    /// <p>Updates a Lambda function's code.</p> <p>The function's code is locked when you publish a version. You can't modify the code of a published version, only the unpublished version.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateFunctionCodeError::from_response(response))),
                )
            }
        })
    }

    /// <p>Modify the version-specifc settings of a Lambda function.</p> <p>These settings can vary between versions of a function and are locked when you publish a version. You can't modify the configuration of a published version, only the unpublished version.</p> <p>To configure function concurrency, use <a>PutFunctionConcurrency</a>. To grant invoke permissions to an account or AWS service, use <a>AddPermission</a>.</p>
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

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<FunctionConfiguration>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(UpdateFunctionConfigurationError::from_response(response))
                }))
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
