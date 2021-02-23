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
use rusoto_core::pagination::{aws_stream, Paged, PagedOutput, PagedRequest, RusotoStream};
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};
#[allow(unused_imports)]
use std::borrow::Cow;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Limits that are related to concurrency and storage. All file and storage sizes are in bytes.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AccountLimit {
    /// <p>The maximum size of a function's deployment package and layers when they're extracted.</p>
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::add_layer_version_permission]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// see [Lambda::add_layer_version_permission]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::add_permission]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>For Amazon S3, the ID of the account that owns the resource. Use this together with <code>SourceArn</code> to ensure that the resource is owned by the specified account. It is possible for an Amazon S3 bucket to be deleted by its owner and recreated by another account.</p>
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

/// see [Lambda::add_permission]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AddPermissionResponse {
    /// <p>The permission statement that's added to the function policy.</p>
    #[serde(rename = "Statement")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement: Option<String>,
}

/// <p>Provides configuration information about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
/// see [Lambda::create_alias]
/// see [Lambda::get_alias]
/// see [Lambda::update_alias]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AliasRoutingConfiguration {
    /// <p>The second version, and the percentage of traffic that's routed to it.</p>
    #[serde(rename = "AdditionalVersionWeights")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_version_weights: Option<::std::collections::HashMap<String, f64>>,
}

/// <p>List of signing profiles that can sign a code package. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct AllowedPublishers {
    /// <p>The Amazon Resource Name (ARN) for each of the signing profiles. A signing profile defines a trusted user who can sign a code package. </p>
    #[serde(rename = "SigningProfileVersionArns")]
    pub signing_profile_version_arns: Vec<String>,
}

/// <p>Details about a Code signing configuration. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CodeSigningConfig {
    /// <p>List of allowed publishers.</p>
    #[serde(rename = "AllowedPublishers")]
    pub allowed_publishers: AllowedPublishers,
    /// <p>The Amazon Resource Name (ARN) of the Code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>Unique identifer for the Code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigId")]
    pub code_signing_config_id: String,
    /// <p>The code signing policy controls the validation failure action for signature mismatch or expiry.</p>
    #[serde(rename = "CodeSigningPolicies")]
    pub code_signing_policies: CodeSigningPolicies,
    /// <p>Code signing configuration description.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The date and time that the Code signing configuration was last modified, in ISO-8601 format (YYYY-MM-DDThh:mm:ss.sTZD). </p>
    #[serde(rename = "LastModified")]
    pub last_modified: String,
}

/// <p>Code signing configuration policies specifies the validation failure action for signature mismatch or expiry.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CodeSigningPolicies {
    /// <p>Code signing configuration policy for deployment validation failure. If you set the policy to <code>Enforce</code>, Lambda blocks the deployment request if signature validation checks fail. If you set the policy to <code>Warn</code>, Lambda allows the deployment and creates a CloudWatch log. </p> <p>Default value: <code>Warn</code> </p>
    #[serde(rename = "UntrustedArtifactOnDeployment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub untrusted_artifact_on_deployment: Option<String>,
}

/// see [Lambda::put_function_concurrency]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Concurrency {
    /// <p>The number of concurrent executions that are reserved for this function. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i64>,
}

/// see [Lambda::create_alias]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html#configuring-alias-routing">routing configuration</a> of the alias.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// see [Lambda::create_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateCodeSigningConfigRequest {
    /// <p>Signing profiles for this code signing configuration.</p>
    #[serde(rename = "AllowedPublishers")]
    pub allowed_publishers: AllowedPublishers,
    /// <p>The code signing policies define the actions to take if the validation checks fail. </p>
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    /// <p>Descriptive name for this code signing configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// see [Lambda::create_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateCodeSigningConfigResponse {
    /// <p>The code signing configuration.</p>
    #[serde(rename = "CodeSigningConfig")]
    pub code_signing_config: CodeSigningConfig,
}

/// see [Lambda::create_event_source_mapping]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateEventSourceMappingRequest {
    /// <p><p>The maximum number of items to retrieve in a single batch.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - Default 100. Max 1,000.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - Default 10. For standard queues the max is 10,000. For FIFO queues the max is 10.</p> </li> <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Self-Managed Apache Kafka</b> - Default 100. Max 10,000.</p> </li> </ul></p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>(Streams) If the function returns an error, split the batch in two and retry.</p>
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    /// <p>(Streams) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>If true, the event source mapping is active. Set to false to pause polling and invocation.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p><p>The Amazon Resource Name (ARN) of the event source.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - The ARN of the data stream or a stream consumer.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - The ARN of the stream.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - The ARN of the queue.</p> </li> <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> - The ARN of the cluster.</p> </li> </ul></p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Version or Alias ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>(Streams) A list of current response type enums applied to the event source mapping.</p>
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    /// <p>(Streams and SQS standard queues) The maximum amount of time to gather records before invoking the function, in seconds.</p>
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records older than the specified age. The default value is infinite (-1).</p>
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records will be retried until the record expires.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
    /// <p>(Streams) The number of batches to process from each shard concurrently.</p>
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i64>,
    /// <p> (MQ) The name of the Amazon MQ broker destination queue to consume. </p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    /// <p>The Self-Managed Apache Kafka cluster to send records.</p>
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    /// <p>An array of the authentication protocol, or the VPC components to secure your event source.</p>
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    /// <p>The position in a stream from which to start reading. Required for Amazon Kinesis, Amazon DynamoDB, and Amazon MSK Streams sources. <code>AT_TIMESTAMP</code> is only supported for Amazon Kinesis streams.</p>
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    /// <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
    /// <p>The name of the Kafka topic.</p>
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// <p>(Streams) The duration of a processing window in seconds. The range is between 1 second up to 15 minutes.</p>
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i64>,
}

/// see [Lambda::create_function]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateFunctionRequest {
    /// <p>The code for the function.</p>
    #[serde(rename = "Code")]
    pub code: FunctionCode,
    /// <p>To enable code signing for this function, specify the ARN of a code-signing configuration. A code-signing configuration includes a set of signing profiles, which define the trusted publishers for this function.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_config_arn: Option<String>,
    /// <p>A dead letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#dlq">Dead Letter Queues</a>.</p>
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
    /// <p>Connection settings for an Amazon EFS file system.</p>
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the method within your code that Lambda calls to execute your function. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/programming-model-v2.html">Programming Model</a>.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>Configuration values that override the container image Dockerfile.</p>
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    /// <p>The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">function layers</a> to add to the function's execution environment. Specify each layer by its ARN, including the version.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// <p>The amount of memory available to the function at runtime. Increasing the function's memory also increases its CPU allocation. The default value is 128 MB. The value can be any multiple of 1 MB.</p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>The type of deployment package. Set to <code>Image</code> for container image and set <code>Zip</code> for ZIP archive.</p>
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    /// <p>Set to true to publish the first version of the function during creation.</p>
    #[serde(rename = "Publish")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the function's execution role.</p>
    #[serde(rename = "Role")]
    pub role: String,
    /// <p>The identifier of the function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime</a>.</p>
    #[serde(rename = "Runtime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<String>,
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
    /// <p>For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can only access resources and the internet through that VPC. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html">VPC Settings</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#dlq">dead-letter queue</a> for failed asynchronous invocations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DeadLetterConfig {
    /// <p>The Amazon Resource Name (ARN) of an Amazon SQS queue or Amazon SNS topic.</p>
    #[serde(rename = "TargetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

/// see [Lambda::delete_alias]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteAliasRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// see [Lambda::delete_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteCodeSigningConfigRequest {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
}

/// see [Lambda::delete_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteCodeSigningConfigResponse {}

/// see [Lambda::delete_event_source_mapping]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteEventSourceMappingRequest {
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// see [Lambda::delete_function_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFunctionCodeSigningConfigRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::delete_function_concurrency]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFunctionConcurrencyRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::delete_function_event_invoke_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFunctionEventInvokeConfigRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>A version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::delete_function]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteFunctionRequest {
    /// <p>The name of the Lambda function or version.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:1</code> (with version).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version to delete. You can't delete a version that's referenced by an alias.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::delete_layer_version]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLayerVersionRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

/// see [Lambda::delete_provisioned_concurrency_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteProvisionedConcurrencyConfigRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    pub qualifier: String,
}

/// <p>A configuration object that specifies the destination of an event after Lambda processes it.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct DestinationConfig {
    /// <p>The destination configuration for failed invocations.</p>
    #[serde(rename = "OnFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<OnFailure>,
    /// <p>The destination configuration for successful invocations.</p>
    #[serde(rename = "OnSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_success: Option<OnSuccess>,
}

/// <p>A function's environment variable settings.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Environment {
    /// <p>Environment variable key-value pairs.</p>
    #[serde(rename = "Variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<::std::collections::HashMap<String, String>>,
}

/// <p>Error messages for environment variables that couldn't be applied.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// <p>The results of an operation to update or read environment variables. If the operation is successful, the response contains the environment variables. If it failed, the response contains details about the error.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
/// see [Lambda::create_event_source_mapping]
/// see [Lambda::delete_event_source_mapping]
/// see [Lambda::get_event_source_mapping]
/// see [Lambda::update_event_source_mapping]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct EventSourceMappingConfiguration {
    /// <p>The maximum number of items to retrieve in a single batch.</p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>(Streams) If the function returns an error, split the batch in two and retry. The default value is false.</p>
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    /// <p>(Streams) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>The Amazon Resource Name (ARN) of the event source.</p>
    #[serde(rename = "EventSourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    /// <p>The ARN of the Lambda function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>(Streams) A list of current response type enums applied to the event source mapping.</p>
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    /// <p>The date that the event source mapping was last updated, or its state changed.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The result of the last AWS Lambda invocation of your Lambda function.</p>
    #[serde(rename = "LastProcessingResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_processing_result: Option<String>,
    /// <p>(Streams and SQS standard queues) The maximum amount of time to gather records before invoking the function, in seconds. The default value is zero.</p>
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p>
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
    /// <p>(Streams) The number of batches to process from each shard concurrently. The default value is 1.</p>
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i64>,
    /// <p> (MQ) The name of the Amazon MQ broker destination queue to consume. </p>
    #[serde(rename = "Queues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queues: Option<Vec<String>>,
    /// <p>The Self-Managed Apache Kafka cluster for your event source.</p>
    #[serde(rename = "SelfManagedEventSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_event_source: Option<SelfManagedEventSource>,
    /// <p>An array of the authentication protocol, or the VPC components to secure your event source.</p>
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    /// <p>The position in a stream from which to start reading. Required for Amazon Kinesis, Amazon DynamoDB, and Amazon MSK Streams sources. <code>AT_TIMESTAMP</code> is only supported for Amazon Kinesis streams.</p>
    #[serde(rename = "StartingPosition")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position: Option<String>,
    /// <p>With <code>StartingPosition</code> set to <code>AT_TIMESTAMP</code>, the time from which to start reading.</p>
    #[serde(rename = "StartingPositionTimestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_position_timestamp: Option<f64>,
    /// <p>The state of the event source mapping. It can be one of the following: <code>Creating</code>, <code>Enabling</code>, <code>Enabled</code>, <code>Disabling</code>, <code>Disabled</code>, <code>Updating</code>, or <code>Deleting</code>.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Indicates whether the last change to the event source mapping was made by a user, or by the Lambda service.</p>
    #[serde(rename = "StateTransitionReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_transition_reason: Option<String>,
    /// <p>The name of the Kafka topic.</p>
    #[serde(rename = "Topics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topics: Option<Vec<String>>,
    /// <p>(Streams) The duration of a processing window in seconds. The range is between 1 second up to 15 minutes.</p>
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i64>,
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

/// <p>Details about the connection between a Lambda function and an Amazon EFS file system.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct FileSystemConfig {
    /// <p>The Amazon Resource Name (ARN) of the Amazon EFS access point that provides access to the file system.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
    /// <p>The path where the function can access the file system, starting with <code>/mnt/</code>.</p>
    #[serde(rename = "LocalMountPath")]
    pub local_mount_path: String,
}

/// <p>The code for the Lambda function. You can specify either an object in Amazon S3, upload a .zip file archive deployment package directly, or specify the URI of a container image.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct FunctionCode {
    /// <p>URI of a container image in the Amazon ECR registry.</p>
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
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
    pub zip_file: Option<bytes::Bytes>,
}

/// <p>Details about a function's deployment package.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FunctionCodeLocation {
    /// <p>URI of a container image in the Amazon ECR registry.</p>
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
    /// <p>A presigned URL that you can use to download the deployment package.</p>
    #[serde(rename = "Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// <p>The service that's hosting the file.</p>
    #[serde(rename = "RepositoryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    /// <p>The resolved URI for the image.</p>
    #[serde(rename = "ResolvedImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_image_uri: Option<String>,
}

/// <p>Details about a function's configuration.</p>
/// see [Lambda::create_function]
/// see [Lambda::get_function_configuration]
/// see [Lambda::publish_version]
/// see [Lambda::update_function_code]
/// see [Lambda::update_function_configuration]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>Connection settings for an Amazon EFS file system.</p>
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
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
    /// <p>The function's image configuration values.</p>
    #[serde(rename = "ImageConfigResponse")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config_response: Option<ImageConfigResponse>,
    /// <p>The KMS key that's used to encrypt the function's environment variables. This key is only returned if you've configured a customer managed CMK.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The date and time that the function was last updated, in <a href="https://www.w3.org/TR/NOTE-datetime">ISO-8601 format</a> (YYYY-MM-DDThh:mm:ss.sTZD).</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The status of the last update that was performed on the function. This is first set to <code>Successful</code> after function creation completes.</p>
    #[serde(rename = "LastUpdateStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status: Option<String>,
    /// <p>The reason for the last update that was performed on the function.</p>
    #[serde(rename = "LastUpdateStatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason: Option<String>,
    /// <p>The reason code for the last update that was performed on the function.</p>
    #[serde(rename = "LastUpdateStatusReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_status_reason_code: Option<String>,
    /// <p>The function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html"> layers</a>.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<Layer>>,
    /// <p>For Lambda@Edge functions, the ARN of the master function.</p>
    #[serde(rename = "MasterArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_arn: Option<String>,
    /// <p>The amount of memory available to the function at runtime. </p>
    #[serde(rename = "MemorySize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_size: Option<i64>,
    /// <p>The type of deployment package. Set to <code>Image</code> for container image and set <code>Zip</code> for .zip file archive.</p>
    #[serde(rename = "PackageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
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
    /// <p>The ARN of the signing job.</p>
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    /// <p>The ARN of the signing profile version.</p>
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
    /// <p>The current state of the function. When the state is <code>Inactive</code>, you can reactivate the function by invoking it.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The reason for the function's current state.</p>
    #[serde(rename = "StateReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
    /// <p>The reason code for the function's current state. When the code is <code>Creating</code>, you can't invoke or modify the function.</p>
    #[serde(rename = "StateReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason_code: Option<String>,
    /// <p>The amount of time in seconds that Lambda allows a function to run before stopping it.</p>
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

/// see [Lambda::get_function_event_invoke_config]
/// see [Lambda::put_function_event_invoke_config]
/// see [Lambda::update_function_event_invoke_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct FunctionEventInvokeConfig {
    /// <p><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"> <b>Destinations</b> </p> <ul> <li> <p> <b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p> </li> <li> <p> <b>Queue</b> - The ARN of an SQS queue.</p> </li> <li> <p> <b>Topic</b> - The ARN of an SNS topic.</p> </li> <li> <p> <b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p> </li> </ul></p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>The Amazon Resource Name (ARN) of the function.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The date and time that the configuration was last updated.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>The maximum age of a request that Lambda sends to a function for processing.</p>
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i64>,
    /// <p>The maximum number of times to retry when the function returns an error.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
}

/// see [Lambda::get_account_settings]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAccountSettingsRequest {}

/// see [Lambda::get_account_settings]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::get_alias]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetAliasRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the alias.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

/// see [Lambda::get_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCodeSigningConfigRequest {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration. </p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
}

/// see [Lambda::get_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCodeSigningConfigResponse {
    /// <p>The code signing configuration</p>
    #[serde(rename = "CodeSigningConfig")]
    pub code_signing_config: CodeSigningConfig,
}

/// see [Lambda::get_event_source_mapping]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetEventSourceMappingRequest {
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// see [Lambda::get_function_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFunctionCodeSigningConfigRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::get_function_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFunctionCodeSigningConfigResponse {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::get_function_concurrency]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFunctionConcurrencyRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::get_function_concurrency]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetFunctionConcurrencyResponse {
    /// <p>The number of simultaneous executions that are reserved for the function.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_concurrent_executions: Option<i64>,
}

/// see [Lambda::get_function_configuration]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFunctionConfigurationRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get details about a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::get_function_event_invoke_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFunctionEventInvokeConfigRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>A version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::get_function]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetFunctionRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get details about a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::get_function]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::get_layer_version_by_arn]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLayerVersionByArnRequest {
    /// <p>The ARN of the layer version.</p>
    #[serde(rename = "Arn")]
    pub arn: String,
}

/// see [Lambda::get_layer_version_policy]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLayerVersionPolicyRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

/// see [Lambda::get_layer_version_policy]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::get_layer_version]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetLayerVersionRequest {
    /// <p>The name or Amazon Resource Name (ARN) of the layer.</p>
    #[serde(rename = "LayerName")]
    pub layer_name: String,
    /// <p>The version number.</p>
    #[serde(rename = "VersionNumber")]
    pub version_number: i64,
}

/// see [Lambda::get_layer_version]
/// see [Lambda::get_layer_version_by_arn]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::get_policy]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetPolicyRequest {
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify a version or alias to get the policy for that resource.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::get_policy]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::get_provisioned_concurrency_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetProvisionedConcurrencyConfigRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    pub qualifier: String,
}

/// see [Lambda::get_provisioned_concurrency_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetProvisionedConcurrencyConfigResponse {
    /// <p>The amount of provisioned concurrency allocated.</p>
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i64>,
    /// <p>The amount of provisioned concurrency available.</p>
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i64>,
    /// <p>The date and time that a user last updated the configuration, in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601 format</a>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The amount of provisioned concurrency requested.</p>
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i64>,
    /// <p>The status of the allocation process.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>For failed allocations, the reason that provisioned concurrency could not be allocated.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// <p>Configuration values that override the container image Dockerfile settings. See <a href="https://docs.aws.amazon.com/lambda/latest/dg/images-parms.html">Container settings</a>. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ImageConfig {
    /// <p>Specifies parameters that you want to pass in with ENTRYPOINT. </p>
    #[serde(rename = "Command")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    /// <p>Specifies the entry point to their application, which is typically the location of the runtime executable.</p>
    #[serde(rename = "EntryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Vec<String>>,
    /// <p>Specifies the working directory.</p>
    #[serde(rename = "WorkingDirectory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

/// <p>Error response to GetFunctionConfiguration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageConfigError {
    /// <p>Error code.</p>
    #[serde(rename = "ErrorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>Error message.</p>
    #[serde(rename = "Message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// <p>Response to GetFunctionConfiguration request.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImageConfigResponse {
    /// <p>Error response to GetFunctionConfiguration.</p>
    #[serde(rename = "Error")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ImageConfigError>,
    /// <p>Configuration values that override the container image Dockerfile.</p>
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
}

/// see [Lambda::invoke]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub payload: Option<bytes::Bytes>,
    /// <p>Specify a version or alias to invoke a published version of the function.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::invoke]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct InvocationResponse {
    /// <p>The version of the function that executed. When you invoke a function with an alias, this indicates which version the alias resolved to.</p>
    pub executed_version: Option<String>,
    /// <p>If present, indicates that an error occurred during function execution. Details about the error are included in the response payload.</p>
    pub function_error: Option<String>,
    /// <p>The last 4 KB of the execution log, which is base64 encoded.</p>
    pub log_result: Option<String>,
    /// <p>The response from the function, or an error object.</p>
    pub payload: Option<bytes::Bytes>,
    /// <p>The HTTP status code is in the 200 range for a successful request. For the <code>RequestResponse</code> invocation type, this status code is 200. For the <code>Event</code> invocation type, this status code is 202. For the <code>DryRun</code> invocation type, the status code is 204.</p>
    pub status_code: Option<i64>,
}

/// see [Lambda::invoke_async]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub invoke_args: bytes::Bytes,
}

/// <p>A success response (<code>202 Accepted</code>) indicates that the request is queued for invocation. </p>
/// see [Lambda::invoke_async]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct InvokeAsyncResponse {
    /// <p>The status code.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

/// <p>An <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Layer {
    /// <p>The Amazon Resource Name (ARN) of the function layer.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The size of the layer archive in bytes.</p>
    #[serde(rename = "CodeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_size: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
}

/// <p>A ZIP archive that contains the contents of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. You can specify either an Amazon S3 location, or upload a layer archive directly.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    pub zip_file: Option<bytes::Bytes>,
}

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>The Amazon Resource Name (ARN) of a signing job.</p>
    #[serde(rename = "SigningJobArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_job_arn: Option<String>,
    /// <p>The Amazon Resource Name (ARN) for a signing profile version.</p>
    #[serde(rename = "SigningProfileVersionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_profile_version_arn: Option<String>,
}

/// <p>Details about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::list_aliases]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

impl Paged for ListAliasesRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListAliasesRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_aliases]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListAliasesResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListAliasesResponse {
    type Item = AliasConfiguration;

    fn into_pagination_page(self) -> Vec<AliasConfiguration> {
        self.aliases.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_code_signing_configs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListCodeSigningConfigsRequest {
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Maximum number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListCodeSigningConfigsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListCodeSigningConfigsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_code_signing_configs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListCodeSigningConfigsResponse {
    /// <p>The code signing configurations</p>
    #[serde(rename = "CodeSigningConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_configs: Option<Vec<CodeSigningConfig>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl Paged for ListCodeSigningConfigsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListCodeSigningConfigsResponse {
    type Item = CodeSigningConfig;

    fn into_pagination_page(self) -> Vec<CodeSigningConfig> {
        self.code_signing_configs.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_event_source_mappings]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListEventSourceMappingsRequest {
    /// <p><p>The Amazon Resource Name (ARN) of the event source.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - The ARN of the data stream or a stream consumer.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - The ARN of the stream.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - The ARN of the queue.</p> </li> <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> - The ARN of the cluster.</p> </li> </ul></p>
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

impl Paged for ListEventSourceMappingsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListEventSourceMappingsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_event_source_mappings]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListEventSourceMappingsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListEventSourceMappingsResponse {
    type Item = EventSourceMappingConfiguration;

    fn into_pagination_page(self) -> Vec<EventSourceMappingConfiguration> {
        self.event_source_mappings.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_function_event_invoke_configs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFunctionEventInvokeConfigsRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of configurations to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListFunctionEventInvokeConfigsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListFunctionEventInvokeConfigsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_function_event_invoke_configs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFunctionEventInvokeConfigsResponse {
    /// <p>A list of configurations.</p>
    #[serde(rename = "FunctionEventInvokeConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_event_invoke_configs: Option<Vec<FunctionEventInvokeConfig>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl Paged for ListFunctionEventInvokeConfigsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListFunctionEventInvokeConfigsResponse {
    type Item = FunctionEventInvokeConfig;

    fn into_pagination_page(self) -> Vec<FunctionEventInvokeConfig> {
        self.function_event_invoke_configs.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_functions_by_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFunctionsByCodeSigningConfigRequest {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Maximum number of items to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListFunctionsByCodeSigningConfigRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListFunctionsByCodeSigningConfigRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_functions_by_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListFunctionsByCodeSigningConfigResponse {
    /// <p>The function ARNs. </p>
    #[serde(rename = "FunctionArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arns: Option<Vec<String>>,
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
}

impl Paged for ListFunctionsByCodeSigningConfigResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListFunctionsByCodeSigningConfigResponse {
    type Item = String;

    fn into_pagination_page(self) -> Vec<String> {
        self.function_arns.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_functions]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListFunctionsRequest {
    /// <p>Set to <code>ALL</code> to include entries for all published versions of each function.</p>
    #[serde(rename = "FunctionVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>For Lambda@Edge functions, the AWS Region of the master function. For example, <code>us-east-1</code> filters the list of functions to only include Lambda@Edge functions replicated from a master function in US East (N. Virginia). If specified, you must set <code>FunctionVersion</code> to <code>ALL</code>.</p>
    #[serde(rename = "MasterRegion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_region: Option<String>,
    /// <p>The maximum number of functions to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListFunctionsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListFunctionsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// <p>A list of Lambda functions.</p>
/// see [Lambda::list_functions]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListFunctionsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListFunctionsResponse {
    type Item = FunctionConfiguration;

    fn into_pagination_page(self) -> Vec<FunctionConfiguration> {
        self.functions.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_layer_versions]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

impl Paged for ListLayerVersionsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListLayerVersionsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_layer_versions]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListLayerVersionsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListLayerVersionsResponse {
    type Item = LayerVersionsListItem;

    fn into_pagination_page(self) -> Vec<LayerVersionsListItem> {
        self.layer_versions.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_layers]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

impl Paged for ListLayersRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListLayersRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_layers]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListLayersResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListLayersResponse {
    type Item = LayersListItem;

    fn into_pagination_page(self) -> Vec<LayersListItem> {
        self.layers.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_provisioned_concurrency_configs]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListProvisionedConcurrencyConfigsRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>Specify a number to limit the number of configurations returned.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListProvisionedConcurrencyConfigsRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListProvisionedConcurrencyConfigsRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_provisioned_concurrency_configs]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListProvisionedConcurrencyConfigsResponse {
    /// <p>The pagination token that's included if more results are available.</p>
    #[serde(rename = "NextMarker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_marker: Option<String>,
    /// <p>A list of provisioned concurrency configurations.</p>
    #[serde(rename = "ProvisionedConcurrencyConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_concurrency_configs: Option<Vec<ProvisionedConcurrencyConfigListItem>>,
}

impl Paged for ListProvisionedConcurrencyConfigsResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListProvisionedConcurrencyConfigsResponse {
    type Item = ProvisionedConcurrencyConfigListItem;

    fn into_pagination_page(self) -> Vec<ProvisionedConcurrencyConfigListItem> {
        self.provisioned_concurrency_configs.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// see [Lambda::list_tags]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
}

/// see [Lambda::list_tags]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsResponse {
    /// <p>The function's tags.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// see [Lambda::list_versions_by_function]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVersionsByFunctionRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>Specify the pagination token that's returned by a previous request to retrieve the next page of results.</p>
    #[serde(rename = "Marker")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    /// <p>The maximum number of versions to return.</p>
    #[serde(rename = "MaxItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<i64>,
}

impl Paged for ListVersionsByFunctionRequest {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.marker)
    }
}

impl PagedRequest for ListVersionsByFunctionRequest {
    fn set_pagination_token(&mut self, key: Option<String>) {
        self.marker = key;
    }
}

/// see [Lambda::list_versions_by_function]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

impl Paged for ListVersionsByFunctionResponse {
    type Token = Option<String>;
    fn take_pagination_token(&mut self) -> Option<String> {
        self.next_marker.take()
    }
    fn pagination_token(&self) -> Cow<Option<String>> {
        Cow::Borrowed(&self.next_marker)
    }
}

impl PagedOutput for ListVersionsByFunctionResponse {
    type Item = FunctionConfiguration;

    fn into_pagination_page(self) -> Vec<FunctionConfiguration> {
        self.versions.unwrap_or_default()
    }

    fn has_another_page(&self) -> bool {
        self.pagination_token().is_some()
    }
}

/// <p>A destination for events that failed processing.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnFailure {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// <p>A destination for events that were processed successfully.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct OnSuccess {
    /// <p>The Amazon Resource Name (ARN) of the destination resource.</p>
    #[serde(rename = "Destination")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// <p>Details about the provisioned concurrency configuration for a function alias or version.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ProvisionedConcurrencyConfigListItem {
    /// <p>The amount of provisioned concurrency allocated.</p>
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i64>,
    /// <p>The amount of provisioned concurrency available.</p>
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i64>,
    /// <p>The Amazon Resource Name (ARN) of the alias or version.</p>
    #[serde(rename = "FunctionArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    /// <p>The date and time that a user last updated the configuration, in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601 format</a>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The amount of provisioned concurrency requested.</p>
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i64>,
    /// <p>The status of the allocation process.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>For failed allocations, the reason that provisioned concurrency could not be allocated.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// see [Lambda::publish_layer_version]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// see [Lambda::publish_layer_version]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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

/// see [Lambda::publish_version]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// see [Lambda::put_function_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutFunctionCodeSigningConfigRequest {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::put_function_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutFunctionCodeSigningConfigResponse {
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
}

/// see [Lambda::put_function_concurrency]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutFunctionConcurrencyRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The number of simultaneous executions to reserve for the function.</p>
    #[serde(rename = "ReservedConcurrentExecutions")]
    pub reserved_concurrent_executions: i64,
}

/// see [Lambda::put_function_event_invoke_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutFunctionEventInvokeConfigRequest {
    /// <p><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"> <b>Destinations</b> </p> <ul> <li> <p> <b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p> </li> <li> <p> <b>Queue</b> - The ARN of an SQS queue.</p> </li> <li> <p> <b>Topic</b> - The ARN of an SNS topic.</p> </li> <li> <p> <b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p> </li> </ul></p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The maximum age of a request that Lambda sends to a function for processing.</p>
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i64>,
    /// <p>The maximum number of times to retry when the function returns an error.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
    /// <p>A version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// see [Lambda::put_provisioned_concurrency_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutProvisionedConcurrencyConfigRequest {
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The amount of provisioned concurrency to allocate for the version or alias.</p>
    #[serde(rename = "ProvisionedConcurrentExecutions")]
    pub provisioned_concurrent_executions: i64,
    /// <p>The version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    pub qualifier: String,
}

/// see [Lambda::put_provisioned_concurrency_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutProvisionedConcurrencyConfigResponse {
    /// <p>The amount of provisioned concurrency allocated.</p>
    #[serde(rename = "AllocatedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_provisioned_concurrent_executions: Option<i64>,
    /// <p>The amount of provisioned concurrency available.</p>
    #[serde(rename = "AvailableProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_provisioned_concurrent_executions: Option<i64>,
    /// <p>The date and time that a user last updated the configuration, in <a href="https://www.iso.org/iso-8601-date-and-time-format.html">ISO 8601 format</a>.</p>
    #[serde(rename = "LastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    /// <p>The amount of provisioned concurrency requested.</p>
    #[serde(rename = "RequestedProvisionedConcurrentExecutions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_provisioned_concurrent_executions: Option<i64>,
    /// <p>The status of the allocation process.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>For failed allocations, the reason that provisioned concurrency could not be allocated.</p>
    #[serde(rename = "StatusReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

/// see [Lambda::remove_layer_version_permission]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// see [Lambda::remove_permission]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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

/// <p>The Self-Managed Apache Kafka cluster for your event source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SelfManagedEventSource {
    /// <p>The list of bootstrap servers for your Kafka brokers in the following format: <code>"KAFKA_BOOTSTRAP_SERVERS": ["abc.xyz.com:xxxx","abc2.xyz.com:xxxx"]</code>.</p>
    #[serde(rename = "Endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<::std::collections::HashMap<String, Vec<String>>>,
}

/// <p>You can specify the authentication protocol, or the VPC components to secure access to your event source.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SourceAccessConfiguration {
    /// <p><p>The type of authentication protocol or the VPC components for your event source. For example: <code>&quot;Type&quot;:&quot;SASL<em>SCRAM</em>512<em>AUTH&quot;</code>.</p> <ul> <li> <p> <code>BASIC</em>AUTH</code> - (MQ) The Secrets Manager secret that stores your broker credentials.</p> </li> <li> <p> <code>VPC<em>SUBNET</code> - The subnets associated with your VPC. Lambda connects to these subnets to fetch data from your Kafka cluster.</p> </li> <li> <p> <code>VPC</em>SECURITY<em>GROUP</code> - The VPC security group used to manage access to your Kafka brokers.</p> </li> <li> <p> <code>SASL</em>SCRAM<em>256</em>AUTH</code> - The ARN of your secret key used for SASL SCRAM-256 authentication of your Kafka brokers.</p> </li> <li> <p> <code>SASL<em>SCRAM</em>512_AUTH</code> - The ARN of your secret key used for SASL SCRAM-512 authentication of your Kafka brokers.</p> </li> </ul></p>
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The value for your chosen configuration in <code>Type</code>. For example: <code>"URI": "arn:aws:secretsmanager:us-east-1:01234567890:secret:MyBrokerSecretName"</code>.</p>
    #[serde(rename = "URI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}

/// see [Lambda::tag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A list of tags to apply to the function.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

/// <p>The function's AWS X-Ray tracing configuration. To sample and record incoming requests, set <code>Mode</code> to <code>Active</code>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TracingConfig {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// <p>The function's AWS X-Ray tracing configuration.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TracingConfigResponse {
    /// <p>The tracing mode.</p>
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// see [Lambda::untag_resource]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The function's Amazon Resource Name (ARN).</p>
    #[serde(rename = "Resource")]
    pub resource: String,
    /// <p>A list of tag keys to remove from the function.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

/// see [Lambda::update_alias]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
    /// <p>The <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-aliases.html#configuring-alias-routing">routing configuration</a> of the alias.</p>
    #[serde(rename = "RoutingConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<AliasRoutingConfiguration>,
}

/// see [Lambda::update_code_signing_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateCodeSigningConfigRequest {
    /// <p>Signing profiles for this code signing configuration.</p>
    #[serde(rename = "AllowedPublishers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_publishers: Option<AllowedPublishers>,
    /// <p>The The Amazon Resource Name (ARN) of the code signing configuration.</p>
    #[serde(rename = "CodeSigningConfigArn")]
    pub code_signing_config_arn: String,
    /// <p>The code signing policy.</p>
    #[serde(rename = "CodeSigningPolicies")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_signing_policies: Option<CodeSigningPolicies>,
    /// <p>Descriptive name for this code signing configuration.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// see [Lambda::update_code_signing_config]
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateCodeSigningConfigResponse {
    /// <p>The code signing configuration</p>
    #[serde(rename = "CodeSigningConfig")]
    pub code_signing_config: CodeSigningConfig,
}

/// see [Lambda::update_event_source_mapping]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateEventSourceMappingRequest {
    /// <p><p>The maximum number of items to retrieve in a single batch.</p> <ul> <li> <p> <b>Amazon Kinesis</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Amazon DynamoDB Streams</b> - Default 100. Max 1,000.</p> </li> <li> <p> <b>Amazon Simple Queue Service</b> - Default 10. For standard queues the max is 10,000. For FIFO queues the max is 10.</p> </li> <li> <p> <b>Amazon Managed Streaming for Apache Kafka</b> - Default 100. Max 10,000.</p> </li> <li> <p> <b>Self-Managed Apache Kafka</b> - Default 100. Max 10,000.</p> </li> </ul></p>
    #[serde(rename = "BatchSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i64>,
    /// <p>(Streams) If the function returns an error, split the batch in two and retry.</p>
    #[serde(rename = "BisectBatchOnFunctionError")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bisect_batch_on_function_error: Option<bool>,
    /// <p>(Streams) An Amazon SQS queue or Amazon SNS topic destination for discarded records.</p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>If true, the event source mapping is active. Set to false to pause polling and invocation.</p>
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>MyFunction</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction</code>.</p> </li> <li> <p> <b>Version or Alias ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:MyFunction:PROD</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:MyFunction</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it's limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    /// <p>(Streams) A list of current response type enums applied to the event source mapping.</p>
    #[serde(rename = "FunctionResponseTypes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_response_types: Option<Vec<String>>,
    /// <p>(Streams and SQS standard queues) The maximum amount of time to gather records before invoking the function, in seconds.</p>
    #[serde(rename = "MaximumBatchingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_batching_window_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records older than the specified age. The default value is infinite (-1).</p>
    #[serde(rename = "MaximumRecordAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_record_age_in_seconds: Option<i64>,
    /// <p>(Streams) Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records will be retried until the record expires.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
    /// <p>(Streams) The number of batches to process from each shard concurrently.</p>
    #[serde(rename = "ParallelizationFactor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallelization_factor: Option<i64>,
    /// <p>An array of the authentication protocol, or the VPC components to secure your event source.</p>
    #[serde(rename = "SourceAccessConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_access_configurations: Option<Vec<SourceAccessConfiguration>>,
    /// <p>(Streams) The duration of a processing window in seconds. The range is between 1 second up to 15 minutes.</p>
    #[serde(rename = "TumblingWindowInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumbling_window_in_seconds: Option<i64>,
    /// <p>The identifier of the event source mapping.</p>
    #[serde(rename = "UUID")]
    pub uuid: String,
}

/// see [Lambda::update_function_code]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFunctionCodeRequest {
    /// <p>Set to true to validate the request parameters and access permissions without modifying the function code.</p>
    #[serde(rename = "DryRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>URI of a container image in the Amazon ECR registry.</p>
    #[serde(rename = "ImageUri")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_uri: Option<String>,
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
    pub zip_file: Option<bytes::Bytes>,
}

/// see [Lambda::update_function_configuration]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFunctionConfigurationRequest {
    /// <p>A dead letter queue configuration that specifies the queue or topic where Lambda sends asynchronous events when they fail processing. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#dlq">Dead Letter Queues</a>.</p>
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
    /// <p>Connection settings for an Amazon EFS file system.</p>
    #[serde(rename = "FileSystemConfigs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_configs: Option<Vec<FileSystemConfig>>,
    /// <p>The name of the Lambda function.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code>.</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The name of the method within your code that Lambda calls to execute your function. The format includes the file name. It can also include namespaces and other qualifiers, depending on the runtime. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/programming-model-v2.html">Programming Model</a>.</p>
    #[serde(rename = "Handler")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<String>,
    /// <p>Configuration values that override the container image Dockerfile.</p>
    #[serde(rename = "ImageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_config: Option<ImageConfig>,
    /// <p>The ARN of the AWS Key Management Service (AWS KMS) key that's used to encrypt your function's environment variables. If it's not provided, AWS Lambda uses a default service key.</p>
    #[serde(rename = "KMSKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>A list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">function layers</a> to add to the function's execution environment. Specify each layer by its ARN, including the version.</p>
    #[serde(rename = "Layers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub layers: Option<Vec<String>>,
    /// <p>The amount of memory available to the function at runtime. Increasing the function's memory also increases its CPU allocation. The default value is 128 MB. The value can be any multiple of 1 MB.</p>
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
    /// <p>For network connectivity to AWS resources in a VPC, specify a list of security groups and subnets in the VPC. When you connect a function to a VPC, it can only access resources and the internet through that VPC. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html">VPC Settings</a>.</p>
    #[serde(rename = "VpcConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_config: Option<VpcConfig>,
}

/// see [Lambda::update_function_event_invoke_config]
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateFunctionEventInvokeConfigRequest {
    /// <p><p>A destination for events after they have been sent to a function for processing.</p> <p class="title"> <b>Destinations</b> </p> <ul> <li> <p> <b>Function</b> - The Amazon Resource Name (ARN) of a Lambda function.</p> </li> <li> <p> <b>Queue</b> - The ARN of an SQS queue.</p> </li> <li> <p> <b>Topic</b> - The ARN of an SNS topic.</p> </li> <li> <p> <b>Event Bus</b> - The ARN of an Amazon EventBridge event bus.</p> </li> </ul></p>
    #[serde(rename = "DestinationConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_config: Option<DestinationConfig>,
    /// <p>The name of the Lambda function, version, or alias.</p> <p class="title"> <b>Name formats</b> </p> <ul> <li> <p> <b>Function name</b> - <code>my-function</code> (name-only), <code>my-function:v1</code> (with alias).</p> </li> <li> <p> <b>Function ARN</b> - <code>arn:aws:lambda:us-west-2:123456789012:function:my-function</code>.</p> </li> <li> <p> <b>Partial ARN</b> - <code>123456789012:function:my-function</code>.</p> </li> </ul> <p>You can append a version number or alias to any of the formats. The length constraint applies only to the full ARN. If you specify only the function name, it is limited to 64 characters in length.</p>
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    /// <p>The maximum age of a request that Lambda sends to a function for processing.</p>
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i64>,
    /// <p>The maximum number of times to retry when the function returns an error.</p>
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i64>,
    /// <p>A version number or alias name.</p>
    #[serde(rename = "Qualifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualifier: Option<String>,
}

/// <p>The VPC security groups and subnets that are attached to a Lambda function. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-vpc.html">VPC Settings</a>.</p>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The permissions policy for the resource is too large. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    PolicyLengthExceeded(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl AddLayerVersionPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddLayerVersionPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        AddLayerVersionPermissionError::InvalidParameterValue(err.msg),
                    )
                }
                "PolicyLengthExceededException" => {
                    return RusotoError::Service(
                        AddLayerVersionPermissionError::PolicyLengthExceeded(err.msg),
                    )
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(
                        AddLayerVersionPermissionError::PreconditionFailed(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AddLayerVersionPermissionError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddLayerVersionPermissionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddLayerVersionPermissionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddLayerVersionPermissionError::TooManyRequests(
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
impl fmt::Display for AddLayerVersionPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddLayerVersionPermissionError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            AddLayerVersionPermissionError::PolicyLengthExceeded(ref cause) => {
                write!(f, "{}", cause)
            }
            AddLayerVersionPermissionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            AddLayerVersionPermissionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AddLayerVersionPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddLayerVersionPermissionError::Service(ref cause) => write!(f, "{}", cause),
            AddLayerVersionPermissionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddLayerVersionPermissionError {}
/// Errors returned by AddPermission
#[derive(Debug, PartialEq)]
pub enum AddPermissionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The permissions policy for the resource is too large. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    PolicyLengthExceeded(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl AddPermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(AddPermissionError::InvalidParameterValue(err.msg))
                }
                "PolicyLengthExceededException" => {
                    return RusotoError::Service(AddPermissionError::PolicyLengthExceeded(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(AddPermissionError::PreconditionFailed(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(AddPermissionError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(AddPermissionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(AddPermissionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(AddPermissionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AddPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AddPermissionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            AddPermissionError::PolicyLengthExceeded(ref cause) => write!(f, "{}", cause),
            AddPermissionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            AddPermissionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            AddPermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            AddPermissionError::Service(ref cause) => write!(f, "{}", cause),
            AddPermissionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for AddPermissionError {}
/// Errors returned by CreateAlias
#[derive(Debug, PartialEq)]
pub enum CreateAliasError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl CreateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateAliasError::InvalidParameterValue(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateAliasError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateAliasError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateAliasError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateAliasError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateAliasError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            CreateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateAliasError::Service(ref cause) => write!(f, "{}", cause),
            CreateAliasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateAliasError {}
/// Errors returned by CreateCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum CreateCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl CreateCodeSigningConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateCodeSigningConfigError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateCodeSigningConfigError {}
/// Errors returned by CreateEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum CreateEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl CreateEventSourceMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateEventSourceMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        CreateEventSourceMappingError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateEventSourceMappingError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateEventSourceMappingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateEventSourceMappingError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateEventSourceMappingError::TooManyRequests(
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
impl fmt::Display for CreateEventSourceMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateEventSourceMappingError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            CreateEventSourceMappingError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            CreateEventSourceMappingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateEventSourceMappingError::Service(ref cause) => write!(f, "{}", cause),
            CreateEventSourceMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateEventSourceMappingError {}
/// Errors returned by CreateFunction
#[derive(Debug, PartialEq)]
pub enum CreateFunctionError {
    /// <p>The specified code signing configuration does not exist.</p>
    CodeSigningConfigNotFound(String),
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    CodeStorageExceeded(String),
    /// <p>The code signature failed one or more of the validation checks for signature mismatch or expiry, and the code signing policy is set to ENFORCE. Lambda blocks the deployment. </p>
    CodeVerificationFailed(String),
    /// <p>The code signature failed the integrity check. Lambda always blocks deployment if the integrity check fails, even if code signing policy is set to WARN.</p>
    InvalidCodeSignature(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl CreateFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeSigningConfigNotFoundException" => {
                    return RusotoError::Service(CreateFunctionError::CodeSigningConfigNotFound(
                        err.msg,
                    ))
                }
                "CodeStorageExceededException" => {
                    return RusotoError::Service(CreateFunctionError::CodeStorageExceeded(err.msg))
                }
                "CodeVerificationFailedException" => {
                    return RusotoError::Service(CreateFunctionError::CodeVerificationFailed(
                        err.msg,
                    ))
                }
                "InvalidCodeSignatureException" => {
                    return RusotoError::Service(CreateFunctionError::InvalidCodeSignature(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateFunctionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(CreateFunctionError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateFunctionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(CreateFunctionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(CreateFunctionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateFunctionError::CodeSigningConfigNotFound(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::CodeStorageExceeded(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::CodeVerificationFailed(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::InvalidCodeSignature(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::Service(ref cause) => write!(f, "{}", cause),
            CreateFunctionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateFunctionError {}
/// Errors returned by DeleteAlias
#[derive(Debug, PartialEq)]
pub enum DeleteAliasError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteAliasError::InvalidParameterValue(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(DeleteAliasError::ResourceConflict(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteAliasError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteAliasError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::Service(ref cause) => write!(f, "{}", cause),
            DeleteAliasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteAliasError {}
/// Errors returned by DeleteCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum DeleteCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl DeleteCodeSigningConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(DeleteCodeSigningConfigError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteCodeSigningConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteCodeSigningConfigError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteCodeSigningConfigError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            DeleteCodeSigningConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteCodeSigningConfigError {}
/// Errors returned by DeleteEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum DeleteEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSource Mapping in CREATING, or tried to delete a EventSource mapping currently in the UPDATING state.</p>
    ResourceInUse(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteEventSourceMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteEventSourceMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteEventSourceMappingError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteEventSourceMappingError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteEventSourceMappingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteEventSourceMappingError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteEventSourceMappingError::TooManyRequests(
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
impl fmt::Display for DeleteEventSourceMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteEventSourceMappingError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteEventSourceMappingError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteEventSourceMappingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteEventSourceMappingError::Service(ref cause) => write!(f, "{}", cause),
            DeleteEventSourceMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteEventSourceMappingError {}
/// Errors returned by DeleteFunction
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteFunctionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(DeleteFunctionError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFunctionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteFunctionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteFunctionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFunctionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteFunctionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            DeleteFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFunctionError::Service(ref cause) => write!(f, "{}", cause),
            DeleteFunctionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFunctionError {}
/// Errors returned by DeleteFunctionCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionCodeSigningConfigError {
    /// <p>The specified code signing configuration does not exist.</p>
    CodeSigningConfigNotFound(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteFunctionCodeSigningConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteFunctionCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeSigningConfigNotFoundException" => {
                    return RusotoError::Service(
                        DeleteFunctionCodeSigningConfigError::CodeSigningConfigNotFound(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteFunctionCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        DeleteFunctionCodeSigningConfigError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteFunctionCodeSigningConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteFunctionCodeSigningConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteFunctionCodeSigningConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFunctionCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFunctionCodeSigningConfigError::CodeSigningConfigNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionCodeSigningConfigError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionCodeSigningConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
            DeleteFunctionCodeSigningConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteFunctionCodeSigningConfigError {}
/// Errors returned by DeleteFunctionConcurrency
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionConcurrencyError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteFunctionConcurrencyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteFunctionConcurrencyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteFunctionConcurrencyError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(DeleteFunctionConcurrencyError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteFunctionConcurrencyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteFunctionConcurrencyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteFunctionConcurrencyError::TooManyRequests(
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
impl fmt::Display for DeleteFunctionConcurrencyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFunctionConcurrencyError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionConcurrencyError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            DeleteFunctionConcurrencyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteFunctionConcurrencyError::Service(ref cause) => write!(f, "{}", cause),
            DeleteFunctionConcurrencyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteFunctionConcurrencyError {}
/// Errors returned by DeleteFunctionEventInvokeConfig
#[derive(Debug, PartialEq)]
pub enum DeleteFunctionEventInvokeConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteFunctionEventInvokeConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteFunctionEventInvokeConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteFunctionEventInvokeConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteFunctionEventInvokeConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteFunctionEventInvokeConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteFunctionEventInvokeConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteFunctionEventInvokeConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteFunctionEventInvokeConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionEventInvokeConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteFunctionEventInvokeConfigError::Service(ref cause) => write!(f, "{}", cause),
            DeleteFunctionEventInvokeConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteFunctionEventInvokeConfigError {}
/// Errors returned by DeleteLayerVersion
#[derive(Debug, PartialEq)]
pub enum DeleteLayerVersionError {
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteLayerVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLayerVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ServiceException" => {
                    return RusotoError::Service(DeleteLayerVersionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteLayerVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteLayerVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLayerVersionError::Service(ref cause) => write!(f, "{}", cause),
            DeleteLayerVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLayerVersionError {}
/// Errors returned by DeleteProvisionedConcurrencyConfig
#[derive(Debug, PartialEq)]
pub enum DeleteProvisionedConcurrencyConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl DeleteProvisionedConcurrencyConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DeleteProvisionedConcurrencyConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DeleteProvisionedConcurrencyConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        DeleteProvisionedConcurrencyConfigError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DeleteProvisionedConcurrencyConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(DeleteProvisionedConcurrencyConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        DeleteProvisionedConcurrencyConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteProvisionedConcurrencyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteProvisionedConcurrencyConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteProvisionedConcurrencyConfigError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteProvisionedConcurrencyConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DeleteProvisionedConcurrencyConfigError::Service(ref cause) => write!(f, "{}", cause),
            DeleteProvisionedConcurrencyConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DeleteProvisionedConcurrencyConfigError {}
/// Errors returned by GetAccountSettings
#[derive(Debug, PartialEq)]
pub enum GetAccountSettingsError {
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetAccountSettingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAccountSettingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ServiceException" => {
                    return RusotoError::Service(GetAccountSettingsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAccountSettingsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAccountSettingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAccountSettingsError::Service(ref cause) => write!(f, "{}", cause),
            GetAccountSettingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAccountSettingsError {}
/// Errors returned by GetAlias
#[derive(Debug, PartialEq)]
pub enum GetAliasError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetAliasError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetAliasError::ResourceNotFound(err.msg))
                }
                "ServiceException" => return RusotoError::Service(GetAliasError::Service(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetAliasError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetAliasError::Service(ref cause) => write!(f, "{}", cause),
            GetAliasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetAliasError {}
/// Errors returned by GetCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum GetCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl GetCodeSigningConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetCodeSigningConfigError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCodeSigningConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetCodeSigningConfigError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCodeSigningConfigError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetCodeSigningConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCodeSigningConfigError {}
/// Errors returned by GetEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum GetEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetEventSourceMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetEventSourceMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetEventSourceMappingError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetEventSourceMappingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetEventSourceMappingError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetEventSourceMappingError::TooManyRequests(
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
impl fmt::Display for GetEventSourceMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetEventSourceMappingError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetEventSourceMappingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetEventSourceMappingError::Service(ref cause) => write!(f, "{}", cause),
            GetEventSourceMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetEventSourceMappingError {}
/// Errors returned by GetFunction
#[derive(Debug, PartialEq)]
pub enum GetFunctionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetFunctionError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFunctionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetFunctionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetFunctionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFunctionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFunctionError::Service(ref cause) => write!(f, "{}", cause),
            GetFunctionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFunctionError {}
/// Errors returned by GetFunctionCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum GetFunctionCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetFunctionCodeSigningConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFunctionCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetFunctionCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetFunctionCodeSigningConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetFunctionCodeSigningConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetFunctionCodeSigningConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFunctionCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFunctionCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFunctionCodeSigningConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFunctionCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
            GetFunctionCodeSigningConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFunctionCodeSigningConfigError {}
/// Errors returned by GetFunctionConcurrency
#[derive(Debug, PartialEq)]
pub enum GetFunctionConcurrencyError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetFunctionConcurrencyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFunctionConcurrencyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetFunctionConcurrencyError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFunctionConcurrencyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetFunctionConcurrencyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetFunctionConcurrencyError::TooManyRequests(
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
impl fmt::Display for GetFunctionConcurrencyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFunctionConcurrencyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetFunctionConcurrencyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFunctionConcurrencyError::Service(ref cause) => write!(f, "{}", cause),
            GetFunctionConcurrencyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFunctionConcurrencyError {}
/// Errors returned by GetFunctionConfiguration
#[derive(Debug, PartialEq)]
pub enum GetFunctionConfigurationError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetFunctionConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetFunctionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetFunctionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetFunctionConfigurationError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetFunctionConfigurationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetFunctionConfigurationError::TooManyRequests(
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
impl fmt::Display for GetFunctionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFunctionConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFunctionConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetFunctionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            GetFunctionConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFunctionConfigurationError {}
/// Errors returned by GetFunctionEventInvokeConfig
#[derive(Debug, PartialEq)]
pub enum GetFunctionEventInvokeConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetFunctionEventInvokeConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetFunctionEventInvokeConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetFunctionEventInvokeConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetFunctionEventInvokeConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetFunctionEventInvokeConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetFunctionEventInvokeConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetFunctionEventInvokeConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetFunctionEventInvokeConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFunctionEventInvokeConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetFunctionEventInvokeConfigError::Service(ref cause) => write!(f, "{}", cause),
            GetFunctionEventInvokeConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetFunctionEventInvokeConfigError {}
/// Errors returned by GetLayerVersion
#[derive(Debug, PartialEq)]
pub enum GetLayerVersionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetLayerVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLayerVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetLayerVersionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLayerVersionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLayerVersionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetLayerVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetLayerVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLayerVersionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetLayerVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLayerVersionError::Service(ref cause) => write!(f, "{}", cause),
            GetLayerVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLayerVersionError {}
/// Errors returned by GetLayerVersionByArn
#[derive(Debug, PartialEq)]
pub enum GetLayerVersionByArnError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetLayerVersionByArnError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLayerVersionByArnError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetLayerVersionByArnError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLayerVersionByArnError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLayerVersionByArnError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetLayerVersionByArnError::TooManyRequests(
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
impl fmt::Display for GetLayerVersionByArnError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLayerVersionByArnError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetLayerVersionByArnError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLayerVersionByArnError::Service(ref cause) => write!(f, "{}", cause),
            GetLayerVersionByArnError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLayerVersionByArnError {}
/// Errors returned by GetLayerVersionPolicy
#[derive(Debug, PartialEq)]
pub enum GetLayerVersionPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetLayerVersionPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetLayerVersionPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetLayerVersionPolicyError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetLayerVersionPolicyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetLayerVersionPolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetLayerVersionPolicyError::TooManyRequests(
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
impl fmt::Display for GetLayerVersionPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetLayerVersionPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetLayerVersionPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetLayerVersionPolicyError::Service(ref cause) => write!(f, "{}", cause),
            GetLayerVersionPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetLayerVersionPolicyError {}
/// Errors returned by GetPolicy
#[derive(Debug, PartialEq)]
pub enum GetPolicyError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetPolicyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetPolicyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetPolicyError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetPolicyError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(GetPolicyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetPolicyError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetPolicyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetPolicyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetPolicyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetPolicyError::Service(ref cause) => write!(f, "{}", cause),
            GetPolicyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetPolicyError {}
/// Errors returned by GetProvisionedConcurrencyConfig
#[derive(Debug, PartialEq)]
pub enum GetProvisionedConcurrencyConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration does not exist.</p>
    ProvisionedConcurrencyConfigNotFound(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl GetProvisionedConcurrencyConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetProvisionedConcurrencyConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        GetProvisionedConcurrencyConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ProvisionedConcurrencyConfigNotFoundException" => {
                    return RusotoError::Service(
                        GetProvisionedConcurrencyConfigError::ProvisionedConcurrencyConfigNotFound(
                            err.msg,
                        ),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetProvisionedConcurrencyConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(GetProvisionedConcurrencyConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetProvisionedConcurrencyConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetProvisionedConcurrencyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetProvisionedConcurrencyConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            GetProvisionedConcurrencyConfigError::ProvisionedConcurrencyConfigNotFound(
                ref cause,
            ) => write!(f, "{}", cause),
            GetProvisionedConcurrencyConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetProvisionedConcurrencyConfigError::Service(ref cause) => write!(f, "{}", cause),
            GetProvisionedConcurrencyConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for GetProvisionedConcurrencyConfigError {}
/// Errors returned by Invoke
#[derive(Debug, PartialEq)]
pub enum InvokeError {
    /// <p>Need additional permissions to configure VPC settings.</p>
    EC2AccessDenied(String),
    /// <p>AWS Lambda was throttled by Amazon EC2 during Lambda function initialization using the execution role provided for the Lambda function.</p>
    EC2Throttled(String),
    /// <p>AWS Lambda received an unexpected EC2 client exception while setting up for the Lambda function.</p>
    EC2Unexpected(String),
    /// <p>An error occured when reading from or writing to a connected file system.</p>
    EFSIO(String),
    /// <p>The function couldn't make a network connection to the configured file system.</p>
    EFSMountConnectivity(String),
    /// <p>The function couldn't mount the configured file system due to a permission or configuration issue.</p>
    EFSMountFailure(String),
    /// <p>The function was able to make a network connection to the configured file system, but the mount operation timed out.</p>
    EFSMountTimeout(String),
    /// <p>AWS Lambda was not able to create an elastic network interface in the VPC, specified as part of Lambda function configuration, because the limit for network interfaces has been reached.</p>
    ENILimitReached(String),
    /// <p>One of the parameters in the request is invalid.</p>
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
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The function is inactive and its VPC connection is no longer available. Wait for the VPC connection to reestablish and try again.</p>
    ResourceNotReady(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>AWS Lambda was not able to set up VPC access for the Lambda function because one or more configured subnets has no available IP addresses.</p>
    SubnetIPAddressLimitReached(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
    /// <p>The content type of the <code>Invoke</code> request body is not JSON.</p>
    UnsupportedMediaType(String),
}

impl InvokeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InvokeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "EC2AccessDeniedException" => {
                    return RusotoError::Service(InvokeError::EC2AccessDenied(err.msg))
                }
                "EC2ThrottledException" => {
                    return RusotoError::Service(InvokeError::EC2Throttled(err.msg))
                }
                "EC2UnexpectedException" => {
                    return RusotoError::Service(InvokeError::EC2Unexpected(err.msg))
                }
                "EFSIOException" => return RusotoError::Service(InvokeError::EFSIO(err.msg)),
                "EFSMountConnectivityException" => {
                    return RusotoError::Service(InvokeError::EFSMountConnectivity(err.msg))
                }
                "EFSMountFailureException" => {
                    return RusotoError::Service(InvokeError::EFSMountFailure(err.msg))
                }
                "EFSMountTimeoutException" => {
                    return RusotoError::Service(InvokeError::EFSMountTimeout(err.msg))
                }
                "ENILimitReachedException" => {
                    return RusotoError::Service(InvokeError::ENILimitReached(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(InvokeError::InvalidParameterValue(err.msg))
                }
                "InvalidRequestContentException" => {
                    return RusotoError::Service(InvokeError::InvalidRequestContent(err.msg))
                }
                "InvalidRuntimeException" => {
                    return RusotoError::Service(InvokeError::InvalidRuntime(err.msg))
                }
                "InvalidSecurityGroupIDException" => {
                    return RusotoError::Service(InvokeError::InvalidSecurityGroupID(err.msg))
                }
                "InvalidSubnetIDException" => {
                    return RusotoError::Service(InvokeError::InvalidSubnetID(err.msg))
                }
                "InvalidZipFileException" => {
                    return RusotoError::Service(InvokeError::InvalidZipFile(err.msg))
                }
                "KMSAccessDeniedException" => {
                    return RusotoError::Service(InvokeError::KMSAccessDenied(err.msg))
                }
                "KMSDisabledException" => {
                    return RusotoError::Service(InvokeError::KMSDisabled(err.msg))
                }
                "KMSInvalidStateException" => {
                    return RusotoError::Service(InvokeError::KMSInvalidState(err.msg))
                }
                "KMSNotFoundException" => {
                    return RusotoError::Service(InvokeError::KMSNotFound(err.msg))
                }
                "RequestTooLargeException" => {
                    return RusotoError::Service(InvokeError::RequestTooLarge(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(InvokeError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InvokeError::ResourceNotFound(err.msg))
                }
                "ResourceNotReadyException" => {
                    return RusotoError::Service(InvokeError::ResourceNotReady(err.msg))
                }
                "ServiceException" => return RusotoError::Service(InvokeError::Service(err.msg)),
                "SubnetIPAddressLimitReachedException" => {
                    return RusotoError::Service(InvokeError::SubnetIPAddressLimitReached(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(InvokeError::TooManyRequests(err.msg))
                }
                "UnsupportedMediaTypeException" => {
                    return RusotoError::Service(InvokeError::UnsupportedMediaType(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InvokeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvokeError::EC2AccessDenied(ref cause) => write!(f, "{}", cause),
            InvokeError::EC2Throttled(ref cause) => write!(f, "{}", cause),
            InvokeError::EC2Unexpected(ref cause) => write!(f, "{}", cause),
            InvokeError::EFSIO(ref cause) => write!(f, "{}", cause),
            InvokeError::EFSMountConnectivity(ref cause) => write!(f, "{}", cause),
            InvokeError::EFSMountFailure(ref cause) => write!(f, "{}", cause),
            InvokeError::EFSMountTimeout(ref cause) => write!(f, "{}", cause),
            InvokeError::ENILimitReached(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidRequestContent(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidRuntime(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidSecurityGroupID(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidSubnetID(ref cause) => write!(f, "{}", cause),
            InvokeError::InvalidZipFile(ref cause) => write!(f, "{}", cause),
            InvokeError::KMSAccessDenied(ref cause) => write!(f, "{}", cause),
            InvokeError::KMSDisabled(ref cause) => write!(f, "{}", cause),
            InvokeError::KMSInvalidState(ref cause) => write!(f, "{}", cause),
            InvokeError::KMSNotFound(ref cause) => write!(f, "{}", cause),
            InvokeError::RequestTooLarge(ref cause) => write!(f, "{}", cause),
            InvokeError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            InvokeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InvokeError::ResourceNotReady(ref cause) => write!(f, "{}", cause),
            InvokeError::Service(ref cause) => write!(f, "{}", cause),
            InvokeError::SubnetIPAddressLimitReached(ref cause) => write!(f, "{}", cause),
            InvokeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
            InvokeError::UnsupportedMediaType(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InvokeError {}
/// Errors returned by InvokeAsync
#[derive(Debug, PartialEq)]
pub enum InvokeAsyncError {
    /// <p>The request body could not be parsed as JSON.</p>
    InvalidRequestContent(String),
    /// <p>The runtime or runtime version specified is not supported.</p>
    InvalidRuntime(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl InvokeAsyncError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<InvokeAsyncError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidRequestContentException" => {
                    return RusotoError::Service(InvokeAsyncError::InvalidRequestContent(err.msg))
                }
                "InvalidRuntimeException" => {
                    return RusotoError::Service(InvokeAsyncError::InvalidRuntime(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(InvokeAsyncError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(InvokeAsyncError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(InvokeAsyncError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for InvokeAsyncError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvokeAsyncError::InvalidRequestContent(ref cause) => write!(f, "{}", cause),
            InvokeAsyncError::InvalidRuntime(ref cause) => write!(f, "{}", cause),
            InvokeAsyncError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            InvokeAsyncError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            InvokeAsyncError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for InvokeAsyncError {}
/// Errors returned by ListAliases
#[derive(Debug, PartialEq)]
pub enum ListAliasesError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListAliasesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAliasesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListAliasesError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListAliasesError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListAliasesError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListAliasesError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListAliasesError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListAliasesError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListAliasesError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListAliasesError::Service(ref cause) => write!(f, "{}", cause),
            ListAliasesError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListAliasesError {}
/// Errors returned by ListCodeSigningConfigs
#[derive(Debug, PartialEq)]
pub enum ListCodeSigningConfigsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl ListCodeSigningConfigsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListCodeSigningConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListCodeSigningConfigsError::InvalidParameterValue(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListCodeSigningConfigsError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListCodeSigningConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListCodeSigningConfigsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListCodeSigningConfigsError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListCodeSigningConfigsError {}
/// Errors returned by ListEventSourceMappings
#[derive(Debug, PartialEq)]
pub enum ListEventSourceMappingsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListEventSourceMappingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventSourceMappingsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListEventSourceMappingsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListEventSourceMappingsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListEventSourceMappingsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListEventSourceMappingsError::TooManyRequests(
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
impl fmt::Display for ListEventSourceMappingsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListEventSourceMappingsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListEventSourceMappingsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListEventSourceMappingsError::Service(ref cause) => write!(f, "{}", cause),
            ListEventSourceMappingsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListEventSourceMappingsError {}
/// Errors returned by ListFunctionEventInvokeConfigs
#[derive(Debug, PartialEq)]
pub enum ListFunctionEventInvokeConfigsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListFunctionEventInvokeConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFunctionEventInvokeConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListFunctionEventInvokeConfigsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListFunctionEventInvokeConfigsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListFunctionEventInvokeConfigsError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListFunctionEventInvokeConfigsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFunctionEventInvokeConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFunctionEventInvokeConfigsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFunctionEventInvokeConfigsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFunctionEventInvokeConfigsError::Service(ref cause) => write!(f, "{}", cause),
            ListFunctionEventInvokeConfigsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListFunctionEventInvokeConfigsError {}
/// Errors returned by ListFunctions
#[derive(Debug, PartialEq)]
pub enum ListFunctionsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListFunctionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFunctionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListFunctionsError::InvalidParameterValue(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListFunctionsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListFunctionsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListFunctionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFunctionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListFunctionsError::Service(ref cause) => write!(f, "{}", cause),
            ListFunctionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFunctionsError {}
/// Errors returned by ListFunctionsByCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum ListFunctionsByCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl ListFunctionsByCodeSigningConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListFunctionsByCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListFunctionsByCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListFunctionsByCodeSigningConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListFunctionsByCodeSigningConfigError::Service(
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
impl fmt::Display for ListFunctionsByCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListFunctionsByCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFunctionsByCodeSigningConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListFunctionsByCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListFunctionsByCodeSigningConfigError {}
/// Errors returned by ListLayerVersions
#[derive(Debug, PartialEq)]
pub enum ListLayerVersionsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListLayerVersionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLayerVersionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListLayerVersionsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListLayerVersionsError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListLayerVersionsError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListLayerVersionsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLayerVersionsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLayerVersionsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListLayerVersionsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListLayerVersionsError::Service(ref cause) => write!(f, "{}", cause),
            ListLayerVersionsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLayerVersionsError {}
/// Errors returned by ListLayers
#[derive(Debug, PartialEq)]
pub enum ListLayersError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListLayersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLayersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListLayersError::InvalidParameterValue(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListLayersError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListLayersError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListLayersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListLayersError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListLayersError::Service(ref cause) => write!(f, "{}", cause),
            ListLayersError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListLayersError {}
/// Errors returned by ListProvisionedConcurrencyConfigs
#[derive(Debug, PartialEq)]
pub enum ListProvisionedConcurrencyConfigsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListProvisionedConcurrencyConfigsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListProvisionedConcurrencyConfigsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListProvisionedConcurrencyConfigsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        ListProvisionedConcurrencyConfigsError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(ListProvisionedConcurrencyConfigsError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        ListProvisionedConcurrencyConfigsError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListProvisionedConcurrencyConfigsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListProvisionedConcurrencyConfigsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisionedConcurrencyConfigsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            ListProvisionedConcurrencyConfigsError::Service(ref cause) => write!(f, "{}", cause),
            ListProvisionedConcurrencyConfigsError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for ListProvisionedConcurrencyConfigsError {}
/// Errors returned by ListTags
#[derive(Debug, PartialEq)]
pub enum ListTagsError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListTagsError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsError::ResourceNotFound(err.msg))
                }
                "ServiceException" => return RusotoError::Service(ListTagsError::Service(err.msg)),
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListTagsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListTagsError::Service(ref cause) => write!(f, "{}", cause),
            ListTagsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsError {}
/// Errors returned by ListVersionsByFunction
#[derive(Debug, PartialEq)]
pub enum ListVersionsByFunctionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl ListVersionsByFunctionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVersionsByFunctionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        ListVersionsByFunctionError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListVersionsByFunctionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(ListVersionsByFunctionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListVersionsByFunctionError::TooManyRequests(
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
impl fmt::Display for ListVersionsByFunctionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVersionsByFunctionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListVersionsByFunctionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListVersionsByFunctionError::Service(ref cause) => write!(f, "{}", cause),
            ListVersionsByFunctionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVersionsByFunctionError {}
/// Errors returned by PublishLayerVersion
#[derive(Debug, PartialEq)]
pub enum PublishLayerVersionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PublishLayerVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishLayerVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeStorageExceededException" => {
                    return RusotoError::Service(PublishLayerVersionError::CodeStorageExceeded(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PublishLayerVersionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PublishLayerVersionError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(PublishLayerVersionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PublishLayerVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PublishLayerVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PublishLayerVersionError::CodeStorageExceeded(ref cause) => write!(f, "{}", cause),
            PublishLayerVersionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PublishLayerVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PublishLayerVersionError::Service(ref cause) => write!(f, "{}", cause),
            PublishLayerVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PublishLayerVersionError {}
/// Errors returned by PublishVersion
#[derive(Debug, PartialEq)]
pub enum PublishVersionError {
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    CodeStorageExceeded(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PublishVersionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PublishVersionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeStorageExceededException" => {
                    return RusotoError::Service(PublishVersionError::CodeStorageExceeded(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(PublishVersionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(PublishVersionError::PreconditionFailed(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(PublishVersionError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PublishVersionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(PublishVersionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PublishVersionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PublishVersionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PublishVersionError::CodeStorageExceeded(ref cause) => write!(f, "{}", cause),
            PublishVersionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PublishVersionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            PublishVersionError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            PublishVersionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PublishVersionError::Service(ref cause) => write!(f, "{}", cause),
            PublishVersionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PublishVersionError {}
/// Errors returned by PutFunctionCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum PutFunctionCodeSigningConfigError {
    /// <p>The specified code signing configuration does not exist.</p>
    CodeSigningConfigNotFound(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PutFunctionCodeSigningConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutFunctionCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeSigningConfigNotFoundException" => {
                    return RusotoError::Service(
                        PutFunctionCodeSigningConfigError::CodeSigningConfigNotFound(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutFunctionCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        PutFunctionCodeSigningConfigError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        PutFunctionCodeSigningConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(PutFunctionCodeSigningConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutFunctionCodeSigningConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutFunctionCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutFunctionCodeSigningConfigError::CodeSigningConfigNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionCodeSigningConfigError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionCodeSigningConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
            PutFunctionCodeSigningConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutFunctionCodeSigningConfigError {}
/// Errors returned by PutFunctionConcurrency
#[derive(Debug, PartialEq)]
pub enum PutFunctionConcurrencyError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PutFunctionConcurrencyError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutFunctionConcurrencyError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutFunctionConcurrencyError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(PutFunctionConcurrencyError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutFunctionConcurrencyError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(PutFunctionConcurrencyError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(PutFunctionConcurrencyError::TooManyRequests(
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
impl fmt::Display for PutFunctionConcurrencyError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutFunctionConcurrencyError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            PutFunctionConcurrencyError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            PutFunctionConcurrencyError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutFunctionConcurrencyError::Service(ref cause) => write!(f, "{}", cause),
            PutFunctionConcurrencyError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutFunctionConcurrencyError {}
/// Errors returned by PutFunctionEventInvokeConfig
#[derive(Debug, PartialEq)]
pub enum PutFunctionEventInvokeConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PutFunctionEventInvokeConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutFunctionEventInvokeConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutFunctionEventInvokeConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        PutFunctionEventInvokeConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(PutFunctionEventInvokeConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutFunctionEventInvokeConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutFunctionEventInvokeConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutFunctionEventInvokeConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionEventInvokeConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutFunctionEventInvokeConfigError::Service(ref cause) => write!(f, "{}", cause),
            PutFunctionEventInvokeConfigError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutFunctionEventInvokeConfigError {}
/// Errors returned by PutProvisionedConcurrencyConfig
#[derive(Debug, PartialEq)]
pub enum PutProvisionedConcurrencyConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl PutProvisionedConcurrencyConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<PutProvisionedConcurrencyConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        PutProvisionedConcurrencyConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        PutProvisionedConcurrencyConfigError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        PutProvisionedConcurrencyConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(PutProvisionedConcurrencyConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        PutProvisionedConcurrencyConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for PutProvisionedConcurrencyConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutProvisionedConcurrencyConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            PutProvisionedConcurrencyConfigError::ResourceConflict(ref cause) => {
                write!(f, "{}", cause)
            }
            PutProvisionedConcurrencyConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            PutProvisionedConcurrencyConfigError::Service(ref cause) => write!(f, "{}", cause),
            PutProvisionedConcurrencyConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for PutProvisionedConcurrencyConfigError {}
/// Errors returned by RemoveLayerVersionPermission
#[derive(Debug, PartialEq)]
pub enum RemoveLayerVersionPermissionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl RemoveLayerVersionPermissionError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveLayerVersionPermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        RemoveLayerVersionPermissionError::InvalidParameterValue(err.msg),
                    )
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(
                        RemoveLayerVersionPermissionError::PreconditionFailed(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        RemoveLayerVersionPermissionError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(RemoveLayerVersionPermissionError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        RemoveLayerVersionPermissionError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemoveLayerVersionPermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemoveLayerVersionPermissionError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveLayerVersionPermissionError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveLayerVersionPermissionError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            RemoveLayerVersionPermissionError::Service(ref cause) => write!(f, "{}", cause),
            RemoveLayerVersionPermissionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemoveLayerVersionPermissionError {}
/// Errors returned by RemovePermission
#[derive(Debug, PartialEq)]
pub enum RemovePermissionError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl RemovePermissionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RemovePermissionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(RemovePermissionError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(RemovePermissionError::PreconditionFailed(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RemovePermissionError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(RemovePermissionError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RemovePermissionError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for RemovePermissionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RemovePermissionError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::Service(ref cause) => write!(f, "{}", cause),
            RemovePermissionError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RemovePermissionError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameterValue(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(TagResourceError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(TagResourceError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(TagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            TagResourceError::Service(ref cause) => write!(f, "{}", cause),
            TagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameterValue(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UntagResourceError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UntagResourceError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UntagResourceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UntagResourceError::Service(ref cause) => write!(f, "{}", cause),
            UntagResourceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateAlias
#[derive(Debug, PartialEq)]
pub enum UpdateAliasError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UpdateAliasError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAliasError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateAliasError::InvalidParameterValue(err.msg))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(UpdateAliasError::PreconditionFailed(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UpdateAliasError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateAliasError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateAliasError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateAliasError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateAliasError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateAliasError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::Service(ref cause) => write!(f, "{}", cause),
            UpdateAliasError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateAliasError {}
/// Errors returned by UpdateCodeSigningConfig
#[derive(Debug, PartialEq)]
pub enum UpdateCodeSigningConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
}

impl UpdateCodeSigningConfigError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateCodeSigningConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateCodeSigningConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateCodeSigningConfigError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateCodeSigningConfigError::Service(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateCodeSigningConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateCodeSigningConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateCodeSigningConfigError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateCodeSigningConfigError::Service(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateCodeSigningConfigError {}
/// Errors returned by UpdateEventSourceMapping
#[derive(Debug, PartialEq)]
pub enum UpdateEventSourceMappingError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The operation conflicts with the resource's availability. For example, you attempted to update an EventSource Mapping in CREATING, or tried to delete a EventSource mapping currently in the UPDATING state.</p>
    ResourceInUse(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UpdateEventSourceMappingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateEventSourceMappingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateEventSourceMappingError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UpdateEventSourceMappingError::ResourceConflict(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(UpdateEventSourceMappingError::ResourceInUse(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateEventSourceMappingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateEventSourceMappingError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateEventSourceMappingError::TooManyRequests(
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
impl fmt::Display for UpdateEventSourceMappingError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateEventSourceMappingError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateEventSourceMappingError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UpdateEventSourceMappingError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            UpdateEventSourceMappingError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateEventSourceMappingError::Service(ref cause) => write!(f, "{}", cause),
            UpdateEventSourceMappingError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateEventSourceMappingError {}
/// Errors returned by UpdateFunctionCode
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionCodeError {
    /// <p>The specified code signing configuration does not exist.</p>
    CodeSigningConfigNotFound(String),
    /// <p>You have exceeded your maximum total code size per account. <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">Learn more</a> </p>
    CodeStorageExceeded(String),
    /// <p>The code signature failed one or more of the validation checks for signature mismatch or expiry, and the code signing policy is set to ENFORCE. Lambda blocks the deployment. </p>
    CodeVerificationFailed(String),
    /// <p>The code signature failed the integrity check. Lambda always blocks deployment if the integrity check fails, even if code signing policy is set to WARN.</p>
    InvalidCodeSignature(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UpdateFunctionCodeError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateFunctionCodeError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeSigningConfigNotFoundException" => {
                    return RusotoError::Service(
                        UpdateFunctionCodeError::CodeSigningConfigNotFound(err.msg),
                    )
                }
                "CodeStorageExceededException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::CodeStorageExceeded(
                        err.msg,
                    ))
                }
                "CodeVerificationFailedException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::CodeVerificationFailed(
                        err.msg,
                    ))
                }
                "InvalidCodeSignatureException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::InvalidCodeSignature(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::PreconditionFailed(
                        err.msg,
                    ))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::ResourceNotFound(err.msg))
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFunctionCodeError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFunctionCodeError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFunctionCodeError::CodeSigningConfigNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::CodeStorageExceeded(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::CodeVerificationFailed(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::InvalidCodeSignature(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::PreconditionFailed(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::Service(ref cause) => write!(f, "{}", cause),
            UpdateFunctionCodeError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFunctionCodeError {}
/// Errors returned by UpdateFunctionConfiguration
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionConfigurationError {
    /// <p>The specified code signing configuration does not exist.</p>
    CodeSigningConfigNotFound(String),
    /// <p>The code signature failed one or more of the validation checks for signature mismatch or expiry, and the code signing policy is set to ENFORCE. Lambda blocks the deployment. </p>
    CodeVerificationFailed(String),
    /// <p>The code signature failed the integrity check. Lambda always blocks deployment if the integrity check fails, even if code signing policy is set to WARN.</p>
    InvalidCodeSignature(String),
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The RevisionId provided does not match the latest RevisionId for the Lambda function or alias. Call the <code>GetFunction</code> or the <code>GetAlias</code> API to retrieve the latest RevisionId for your resource.</p>
    PreconditionFailed(String),
    /// <p>The resource already exists, or another operation is in progress.</p>
    ResourceConflict(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UpdateFunctionConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateFunctionConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "CodeSigningConfigNotFoundException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::CodeSigningConfigNotFound(err.msg),
                    )
                }
                "CodeVerificationFailedException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::CodeVerificationFailed(err.msg),
                    )
                }
                "InvalidCodeSignatureException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::InvalidCodeSignature(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::InvalidParameterValue(err.msg),
                    )
                }
                "PreconditionFailedException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::PreconditionFailed(err.msg),
                    )
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::ResourceConflict(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateFunctionConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateFunctionConfigurationError::Service(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateFunctionConfigurationError::TooManyRequests(
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
impl fmt::Display for UpdateFunctionConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFunctionConfigurationError::CodeSigningConfigNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionConfigurationError::CodeVerificationFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionConfigurationError::InvalidCodeSignature(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionConfigurationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionConfigurationError::PreconditionFailed(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionConfigurationError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UpdateFunctionConfigurationError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateFunctionConfigurationError::Service(ref cause) => write!(f, "{}", cause),
            UpdateFunctionConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateFunctionConfigurationError {}
/// Errors returned by UpdateFunctionEventInvokeConfig
#[derive(Debug, PartialEq)]
pub enum UpdateFunctionEventInvokeConfigError {
    /// <p>One of the parameters in the request is invalid.</p>
    InvalidParameterValue(String),
    /// <p>The resource specified in the request does not exist.</p>
    ResourceNotFound(String),
    /// <p>The AWS Lambda service encountered an internal error.</p>
    Service(String),
    /// <p>The request throughput limit was exceeded.</p>
    TooManyRequests(String),
}

impl UpdateFunctionEventInvokeConfigError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<UpdateFunctionEventInvokeConfigError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        UpdateFunctionEventInvokeConfigError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        UpdateFunctionEventInvokeConfigError::ResourceNotFound(err.msg),
                    )
                }
                "ServiceException" => {
                    return RusotoError::Service(UpdateFunctionEventInvokeConfigError::Service(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        UpdateFunctionEventInvokeConfigError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for UpdateFunctionEventInvokeConfigError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateFunctionEventInvokeConfigError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionEventInvokeConfigError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            UpdateFunctionEventInvokeConfigError::Service(ref cause) => write!(f, "{}", cause),
            UpdateFunctionEventInvokeConfigError::TooManyRequests(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for UpdateFunctionEventInvokeConfigError {}
/// Trait representing the capabilities of the AWS Lambda API. AWS Lambda clients implement this trait.
#[async_trait]
pub trait Lambda: Clone + Sync + Send + 'static {
    /// <p>Adds permissions to the resource-based policy of a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Use this action to grant layer usage permission to other accounts. You can grant permission to a single account, all AWS accounts, or all accounts in an organization.</p> <p>To revoke permission, call <a>RemoveLayerVersionPermission</a> with the statement ID that you specified when you added it.</p>
    async fn add_layer_version_permission(
        &self,
        input: AddLayerVersionPermissionRequest,
    ) -> Result<AddLayerVersionPermissionResponse, RusotoError<AddLayerVersionPermissionError>>;

    /// <p>Grants an AWS service or another account permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version or alias to invoke the function.</p> <p>To grant permission to another account, specify the account ID as the <code>Principal</code>. For AWS services, the principal is a domain-style identifier defined by the service, like <code>s3.amazonaws.com</code> or <code>sns.amazonaws.com</code>. For AWS services, you can also specify the ARN of the associated resource as the <code>SourceArn</code>. If you grant permission to a service principal without specifying the source, other accounts could potentially configure resources in their account to invoke your Lambda function.</p> <p>This action adds a statement to a resource-based permissions policy for the function. For more information about function policies, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Lambda Function Policies</a>. </p>
    async fn add_permission(
        &self,
        input: AddPermissionRequest,
    ) -> Result<AddPermissionResponse, RusotoError<AddPermissionError>>;

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a> for a Lambda function version. Use aliases to provide clients with a function identifier that you can update to invoke a different version.</p> <p>You can also map an alias to split invocation requests between two versions. Use the <code>RoutingConfig</code> parameter to specify a second version and the percentage of invocation requests that it receives.</p>
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<CreateAliasError>>;

    /// <p>Creates a code signing configuration. A <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-trustedcode.html">code signing configuration</a> defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail). </p>
    async fn create_code_signing_config(
        &self,
        input: CreateCodeSigningConfigRequest,
    ) -> Result<CreateCodeSigningConfigResponse, RusotoError<CreateCodeSigningConfigError>>;

    /// <p><p>Creates a mapping between an event source and an AWS Lambda function. Lambda reads items from the event source and triggers the function.</p> <p>For details about each event source type, see the following topics.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html">Using AWS Lambda with Amazon DynamoDB</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html">Using AWS Lambda with Amazon Kinesis</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html">Using AWS Lambda with Amazon SQS</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-mq.html">Using AWS Lambda with Amazon MQ</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html">Using AWS Lambda with Amazon MSK</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/kafka-smaa.html">Using AWS Lambda with Self-Managed Apache Kafka</a> </p> </li> </ul> <p>The following error handling options are only available for stream sources (DynamoDB and Kinesis):</p> <ul> <li> <p> <code>BisectBatchOnFunctionError</code> - If the function returns an error, split the batch in two and retry.</p> </li> <li> <p> <code>DestinationConfig</code> - Send discarded records to an Amazon SQS queue or Amazon SNS topic.</p> </li> <li> <p> <code>MaximumRecordAgeInSeconds</code> - Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires</p> </li> <li> <p> <code>MaximumRetryAttempts</code> - Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p> </li> <li> <p> <code>ParallelizationFactor</code> - Process multiple batches from each shard concurrently.</p> </li> </ul></p>
    async fn create_event_source_mapping(
        &self,
        input: CreateEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<CreateEventSourceMappingError>>;

    /// <p>Creates a Lambda function. To create a function, you need a <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-package.html">deployment package</a> and an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role">execution role</a>. The deployment package is a .zip file archive or container image that contains your function code. The execution role grants the function permission to use AWS services, such as Amazon CloudWatch Logs for log streaming and AWS X-Ray for request tracing.</p> <p>When you create a function, Lambda provisions an instance of the function and its supporting resources. If your function connects to a VPC, this process can take a minute or so. During this time, you can't invoke or modify the function. The <code>State</code>, <code>StateReason</code>, and <code>StateReasonCode</code> fields in the response from <a>GetFunctionConfiguration</a> indicate when the function is ready to invoke. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/functions-states.html">Function States</a>.</p> <p>A function has an unpublished version, and can have published versions and aliases. The unpublished version changes when you update your function's code and configuration. A published version is a snapshot of your function code and configuration that can't be changed. An alias is a named resource that maps to a version, and can be changed to map to a different version. Use the <code>Publish</code> parameter to create version <code>1</code> of your function from its initial configuration.</p> <p>The other parameters let you configure version-specific and function-level settings. You can modify version-specific settings later with <a>UpdateFunctionConfiguration</a>. Function-level settings apply to both the unpublished and published versions of the function, and include tags (<a>TagResource</a>) and per-function concurrency limits (<a>PutFunctionConcurrency</a>).</p> <p>You can use code signing if your deployment package is a .zip file archive. To enable code signing for this function, specify the ARN of a code-signing configuration. When a user attempts to deploy a code package with <a>UpdateFunctionCode</a>, Lambda checks that the code package has a valid signature from a trusted publisher. The code-signing configuration includes set set of signing profiles, which define the trusted publishers for this function.</p> <p>If another account or an AWS service invokes your function, use <a>AddPermission</a> to grant permission by creating a resource-based IAM policy. You can grant permissions at the function level, on a version, or on an alias.</p> <p>To invoke your function directly, use <a>Invoke</a>. To invoke your function in response to events in other AWS services, create an event source mapping (<a>CreateEventSourceMapping</a>), or configure a function trigger in the other service. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-invocation.html">Invoking Functions</a>.</p>
    async fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> Result<FunctionConfiguration, RusotoError<CreateFunctionError>>;

    /// <p>Deletes a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<(), RusotoError<DeleteAliasError>>;

    /// <p>Deletes the code signing configuration. You can delete the code signing configuration only if no function is using it. </p>
    async fn delete_code_signing_config(
        &self,
        input: DeleteCodeSigningConfigRequest,
    ) -> Result<DeleteCodeSigningConfigResponse, RusotoError<DeleteCodeSigningConfigError>>;

    /// <p>Deletes an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-invocation-modes.html">event source mapping</a>. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p> <p>When you delete an event source mapping, it enters a <code>Deleting</code> state and might not be completely deleted for several seconds.</p>
    async fn delete_event_source_mapping(
        &self,
        input: DeleteEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<DeleteEventSourceMappingError>>;

    /// <p>Deletes a Lambda function. To delete a specific function version, use the <code>Qualifier</code> parameter. Otherwise, all versions and aliases are deleted.</p> <p>To delete Lambda event source mappings that invoke a function, use <a>DeleteEventSourceMapping</a>. For AWS services and resources that invoke your function directly, delete the trigger in the service where you originally configured it.</p>
    async fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> Result<(), RusotoError<DeleteFunctionError>>;

    /// <p>Removes the code signing configuration from the function.</p>
    async fn delete_function_code_signing_config(
        &self,
        input: DeleteFunctionCodeSigningConfigRequest,
    ) -> Result<(), RusotoError<DeleteFunctionCodeSigningConfigError>>;

    /// <p>Removes a concurrent execution limit from a function.</p>
    async fn delete_function_concurrency(
        &self,
        input: DeleteFunctionConcurrencyRequest,
    ) -> Result<(), RusotoError<DeleteFunctionConcurrencyError>>;

    /// <p>Deletes the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    async fn delete_function_event_invoke_config(
        &self,
        input: DeleteFunctionEventInvokeConfigRequest,
    ) -> Result<(), RusotoError<DeleteFunctionEventInvokeConfigError>>;

    /// <p>Deletes a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Deleted versions can no longer be viewed or added to functions. To avoid breaking functions, a copy of the version remains in Lambda until no functions refer to it.</p>
    async fn delete_layer_version(
        &self,
        input: DeleteLayerVersionRequest,
    ) -> Result<(), RusotoError<DeleteLayerVersionError>>;

    /// <p>Deletes the provisioned concurrency configuration for a function.</p>
    async fn delete_provisioned_concurrency_config(
        &self,
        input: DeleteProvisionedConcurrencyConfigRequest,
    ) -> Result<(), RusotoError<DeleteProvisionedConcurrencyConfigError>>;

    /// <p>Retrieves details about your account's <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limits</a> and usage in an AWS Region.</p>
    async fn get_account_settings(
        &self,
    ) -> Result<GetAccountSettingsResponse, RusotoError<GetAccountSettingsError>>;

    /// <p>Returns details about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    async fn get_alias(
        &self,
        input: GetAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<GetAliasError>>;

    /// <p>Returns information about the specified code signing configuration.</p>
    async fn get_code_signing_config(
        &self,
        input: GetCodeSigningConfigRequest,
    ) -> Result<GetCodeSigningConfigResponse, RusotoError<GetCodeSigningConfigError>>;

    /// <p>Returns details about an event source mapping. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
    async fn get_event_source_mapping(
        &self,
        input: GetEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<GetEventSourceMappingError>>;

    /// <p>Returns information about the function or function version, with a link to download the deployment package that's valid for 10 minutes. If you specify a function version, only details that are specific to that version are returned.</p>
    async fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> Result<GetFunctionResponse, RusotoError<GetFunctionError>>;

    /// <p>Returns the code signing configuration for the specified function.</p>
    async fn get_function_code_signing_config(
        &self,
        input: GetFunctionCodeSigningConfigRequest,
    ) -> Result<GetFunctionCodeSigningConfigResponse, RusotoError<GetFunctionCodeSigningConfigError>>;

    /// <p>Returns details about the reserved concurrency configuration for a function. To set a concurrency limit for a function, use <a>PutFunctionConcurrency</a>.</p>
    async fn get_function_concurrency(
        &self,
        input: GetFunctionConcurrencyRequest,
    ) -> Result<GetFunctionConcurrencyResponse, RusotoError<GetFunctionConcurrencyError>>;

    /// <p>Returns the version-specific settings of a Lambda function or version. The output includes only options that can vary between versions of a function. To modify these settings, use <a>UpdateFunctionConfiguration</a>.</p> <p>To get all of a function's details, including function-level settings, use <a>GetFunction</a>.</p>
    async fn get_function_configuration(
        &self,
        input: GetFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration, RusotoError<GetFunctionConfigurationError>>;

    /// <p>Retrieves the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    async fn get_function_event_invoke_config(
        &self,
        input: GetFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<GetFunctionEventInvokeConfigError>>;

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    async fn get_layer_version(
        &self,
        input: GetLayerVersionRequest,
    ) -> Result<GetLayerVersionResponse, RusotoError<GetLayerVersionError>>;

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    async fn get_layer_version_by_arn(
        &self,
        input: GetLayerVersionByArnRequest,
    ) -> Result<GetLayerVersionResponse, RusotoError<GetLayerVersionByArnError>>;

    /// <p>Returns the permission policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    async fn get_layer_version_policy(
        &self,
        input: GetLayerVersionPolicyRequest,
    ) -> Result<GetLayerVersionPolicyResponse, RusotoError<GetLayerVersionPolicyError>>;

    /// <p>Returns the <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">resource-based IAM policy</a> for a function, version, or alias.</p>
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>>;

    /// <p>Retrieves the provisioned concurrency configuration for a function's alias or version.</p>
    async fn get_provisioned_concurrency_config(
        &self,
        input: GetProvisionedConcurrencyConfigRequest,
    ) -> Result<
        GetProvisionedConcurrencyConfigResponse,
        RusotoError<GetProvisionedConcurrencyConfigError>,
    >;

    /// <p>Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. To invoke a function asynchronously, set <code>InvocationType</code> to <code>Event</code>.</p> <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-sync.html">synchronous invocation</a>, details about the function response, including errors, are included in the response body and headers. For either invocation type, you can find more information in the <a href="https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions.html">execution log</a> and <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-x-ray.html">trace</a>.</p> <p>When an error occurs, your function may be invoked multiple times. Retry behavior varies by error type, client, event source, and invocation type. For example, if you invoke a function asynchronously and it returns an error, Lambda executes the function up to two more times. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/retries-on-errors.html">Retry Behavior</a>.</p> <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html">asynchronous invocation</a>, Lambda adds events to a queue before sending them to your function. If your function does not have enough capacity to keep up with the queue, events may be lost. Occasionally, your function may receive the same event multiple times, even if no error occurs. To retain events that were not processed, configure your function with a <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#dlq">dead-letter queue</a>.</p> <p>The status code in the API response doesn't reflect function errors. Error codes are reserved for errors that prevent your function from executing, such as permissions errors, <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limit errors</a>, or issues with your function's code and configuration. For example, Lambda returns <code>TooManyRequestsException</code> if executing the function would cause you to exceed a concurrency limit at either the account level (<code>ConcurrentInvocationLimitExceeded</code>) or function level (<code>ReservedFunctionConcurrentInvocationLimitExceeded</code>).</p> <p>For functions with a long timeout, your client might be disconnected during synchronous invocation while it waits for a response. Configure your HTTP client, SDK, firewall, proxy, or operating system to allow for long connections with timeout or keep-alive settings.</p> <p>This operation requires permission for the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html">lambda:InvokeFunction</a> action.</p>
    async fn invoke(
        &self,
        input: InvocationRequest,
    ) -> Result<InvocationResponse, RusotoError<InvokeError>>;

    /// <p><important> <p>For asynchronous function invocation, use <a>Invoke</a>.</p> </important> <p>Invokes a function asynchronously.</p></p>
    async fn invoke_async(
        &self,
        input: InvokeAsyncRequest,
    ) -> Result<InvokeAsyncResponse, RusotoError<InvokeAsyncError>>;

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">aliases</a> for a Lambda function.</p>
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>>;

    /// Auto-paginating version of `list_aliases`
    fn list_aliases_pages<'a>(
        &'a self,
        mut input: ListAliasesRequest,
    ) -> RusotoStream<'a, AliasConfiguration, ListAliasesError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_aliases(input.clone())
        }))
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuring-codesigning.html">code signing configurations</a>. A request returns up to 10,000 configurations per call. You can use the <code>MaxItems</code> parameter to return fewer configurations per call. </p>
    async fn list_code_signing_configs(
        &self,
        input: ListCodeSigningConfigsRequest,
    ) -> Result<ListCodeSigningConfigsResponse, RusotoError<ListCodeSigningConfigsError>>;

    /// Auto-paginating version of `list_code_signing_configs`
    fn list_code_signing_configs_pages<'a>(
        &'a self,
        mut input: ListCodeSigningConfigsRequest,
    ) -> RusotoStream<'a, CodeSigningConfig, ListCodeSigningConfigsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_code_signing_configs(input.clone())
        }))
    }

    /// <p>Lists event source mappings. Specify an <code>EventSourceArn</code> to only show event source mappings for a single event source.</p>
    async fn list_event_source_mappings(
        &self,
        input: ListEventSourceMappingsRequest,
    ) -> Result<ListEventSourceMappingsResponse, RusotoError<ListEventSourceMappingsError>>;

    /// Auto-paginating version of `list_event_source_mappings`
    fn list_event_source_mappings_pages<'a>(
        &'a self,
        mut input: ListEventSourceMappingsRequest,
    ) -> RusotoStream<'a, EventSourceMappingConfiguration, ListEventSourceMappingsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_event_source_mappings(input.clone())
        }))
    }

    /// <p>Retrieves a list of configurations for asynchronous invocation for a function.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    async fn list_function_event_invoke_configs(
        &self,
        input: ListFunctionEventInvokeConfigsRequest,
    ) -> Result<
        ListFunctionEventInvokeConfigsResponse,
        RusotoError<ListFunctionEventInvokeConfigsError>,
    >;

    /// Auto-paginating version of `list_function_event_invoke_configs`
    fn list_function_event_invoke_configs_pages<'a>(
        &'a self,
        mut input: ListFunctionEventInvokeConfigsRequest,
    ) -> RusotoStream<'a, FunctionEventInvokeConfig, ListFunctionEventInvokeConfigsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_function_event_invoke_configs(input.clone())
        }))
    }

    /// <p>Returns a list of Lambda functions, with the version-specific configuration of each. Lambda returns up to 50 functions per call.</p> <p>Set <code>FunctionVersion</code> to <code>ALL</code> to include all published versions of each function in addition to the unpublished version. To get more information about a function or version, use <a>GetFunction</a>.</p>
    async fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> Result<ListFunctionsResponse, RusotoError<ListFunctionsError>>;

    /// Auto-paginating version of `list_functions`
    fn list_functions_pages<'a>(
        &'a self,
        mut input: ListFunctionsRequest,
    ) -> RusotoStream<'a, FunctionConfiguration, ListFunctionsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_functions(input.clone())
        }))
    }

    /// <p>List the functions that use the specified code signing configuration. You can use this method prior to deleting a code signing configuration, to verify that no functions are using it.</p>
    async fn list_functions_by_code_signing_config(
        &self,
        input: ListFunctionsByCodeSigningConfigRequest,
    ) -> Result<
        ListFunctionsByCodeSigningConfigResponse,
        RusotoError<ListFunctionsByCodeSigningConfigError>,
    >;

    /// Auto-paginating version of `list_functions_by_code_signing_config`
    fn list_functions_by_code_signing_config_pages<'a>(
        &'a self,
        mut input: ListFunctionsByCodeSigningConfigRequest,
    ) -> RusotoStream<'a, String, ListFunctionsByCodeSigningConfigError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_functions_by_code_signing_config(input.clone())
        }))
    }

    /// <p>Lists the versions of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Versions that have been deleted aren't listed. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only versions that indicate that they're compatible with that runtime.</p>
    async fn list_layer_versions(
        &self,
        input: ListLayerVersionsRequest,
    ) -> Result<ListLayerVersionsResponse, RusotoError<ListLayerVersionsError>>;

    /// Auto-paginating version of `list_layer_versions`
    fn list_layer_versions_pages<'a>(
        &'a self,
        mut input: ListLayerVersionsRequest,
    ) -> RusotoStream<'a, LayerVersionsListItem, ListLayerVersionsError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_layer_versions(input.clone())
        }))
    }

    /// <p>Lists <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layers</a> and shows information about the latest version of each. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only layers that indicate that they're compatible with that runtime.</p>
    async fn list_layers(
        &self,
        input: ListLayersRequest,
    ) -> Result<ListLayersResponse, RusotoError<ListLayersError>>;

    /// Auto-paginating version of `list_layers`
    fn list_layers_pages<'a>(
        &'a self,
        mut input: ListLayersRequest,
    ) -> RusotoStream<'a, LayersListItem, ListLayersError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_layers(input.clone())
        }))
    }

    /// <p>Retrieves a list of provisioned concurrency configurations for a function.</p>
    async fn list_provisioned_concurrency_configs(
        &self,
        input: ListProvisionedConcurrencyConfigsRequest,
    ) -> Result<
        ListProvisionedConcurrencyConfigsResponse,
        RusotoError<ListProvisionedConcurrencyConfigsError>,
    >;

    /// Auto-paginating version of `list_provisioned_concurrency_configs`
    fn list_provisioned_concurrency_configs_pages<'a>(
        &'a self,
        mut input: ListProvisionedConcurrencyConfigsRequest,
    ) -> RusotoStream<
        'a,
        ProvisionedConcurrencyConfigListItem,
        ListProvisionedConcurrencyConfigsError,
    > {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_provisioned_concurrency_configs(input.clone())
        }))
    }

    /// <p>Returns a function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>. You can also view tags with <a>GetFunction</a>.</p>
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>>;

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">versions</a>, with the version-specific configuration of each. Lambda returns up to 50 versions per call.</p>
    async fn list_versions_by_function(
        &self,
        input: ListVersionsByFunctionRequest,
    ) -> Result<ListVersionsByFunctionResponse, RusotoError<ListVersionsByFunctionError>>;

    /// Auto-paginating version of `list_versions_by_function`
    fn list_versions_by_function_pages<'a>(
        &'a self,
        mut input: ListVersionsByFunctionRequest,
    ) -> RusotoStream<'a, FunctionConfiguration, ListVersionsByFunctionError> {
        Box::new(aws_stream(input.take_pagination_token(), move |token| {
            input.set_pagination_token(token);
            self.list_versions_by_function(input.clone())
        }))
    }

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a> from a ZIP archive. Each time you call <code>PublishLayerVersion</code> with the same layer name, a new version is created.</p> <p>Add layers to your function with <a>CreateFunction</a> or <a>UpdateFunctionConfiguration</a>.</p>
    async fn publish_layer_version(
        &self,
        input: PublishLayerVersionRequest,
    ) -> Result<PublishLayerVersionResponse, RusotoError<PublishLayerVersionError>>;

    /// <p>Creates a <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">version</a> from the current code and configuration of a function. Use versions to create a snapshot of your function code and configuration that doesn't change.</p> <p>AWS Lambda doesn't publish a version if the function's configuration and code haven't changed since the last version. Use <a>UpdateFunctionCode</a> or <a>UpdateFunctionConfiguration</a> to update the function before publishing a version.</p> <p>Clients can invoke versions directly or with an alias. To create an alias, use <a>CreateAlias</a>.</p>
    async fn publish_version(
        &self,
        input: PublishVersionRequest,
    ) -> Result<FunctionConfiguration, RusotoError<PublishVersionError>>;

    /// <p>Update the code signing configuration for the function. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. </p>
    async fn put_function_code_signing_config(
        &self,
        input: PutFunctionCodeSigningConfigRequest,
    ) -> Result<PutFunctionCodeSigningConfigResponse, RusotoError<PutFunctionCodeSigningConfigError>>;

    /// <p>Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level.</p> <p>Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. Reserving concurrency both ensures that your function has capacity to process the specified number of events simultaneously, and prevents it from scaling beyond that level. Use <a>GetFunction</a> to see the current setting for a function.</p> <p>Use <a>GetAccountSettings</a> to see your Regional concurrency limit. You can reserve concurrency for as many functions as you like, as long as you leave at least 100 simultaneous executions unreserved for functions that aren't configured with a per-function limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
    async fn put_function_concurrency(
        &self,
        input: PutFunctionConcurrencyRequest,
    ) -> Result<Concurrency, RusotoError<PutFunctionConcurrencyError>>;

    /// <p>Configures options for <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html">asynchronous invocation</a> on a function, version, or alias. If a configuration already exists for a function, version, or alias, this operation overwrites it. If you exclude any settings, they are removed. To set one option without affecting existing settings for other options, use <a>UpdateFunctionEventInvokeConfig</a>.</p> <p>By default, Lambda retries an asynchronous invocation twice if the function returns an error. It retains events in a queue for up to six hours. When an event fails all processing attempts or stays in the asynchronous invocation queue for too long, Lambda discards it. To retain discarded events, configure a dead-letter queue with <a>UpdateFunctionConfiguration</a>.</p> <p>To send an invocation record to a queue, topic, function, or event bus, specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-async-destinations">destination</a>. You can configure separate destinations for successful invocations (on-success) and events that fail all processing attempts (on-failure). You can configure destinations in addition to or instead of a dead-letter queue.</p>
    async fn put_function_event_invoke_config(
        &self,
        input: PutFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<PutFunctionEventInvokeConfigError>>;

    /// <p>Adds a provisioned concurrency configuration to a function's alias or version.</p>
    async fn put_provisioned_concurrency_config(
        &self,
        input: PutProvisionedConcurrencyConfigRequest,
    ) -> Result<
        PutProvisionedConcurrencyConfigResponse,
        RusotoError<PutProvisionedConcurrencyConfigError>,
    >;

    /// <p>Removes a statement from the permissions policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    async fn remove_layer_version_permission(
        &self,
        input: RemoveLayerVersionPermissionRequest,
    ) -> Result<(), RusotoError<RemoveLayerVersionPermissionError>>;

    /// <p>Revokes function-use permission from an AWS service or another account. You can get the ID of the statement from the output of <a>GetPolicy</a>.</p>
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>>;

    /// <p>Adds <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> to a function.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>>;

    /// <p>Removes <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> from a function.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>>;

    /// <p>Updates the configuration of a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    async fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<UpdateAliasError>>;

    /// <p>Update the code signing configuration. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. </p>
    async fn update_code_signing_config(
        &self,
        input: UpdateCodeSigningConfigRequest,
    ) -> Result<UpdateCodeSigningConfigResponse, RusotoError<UpdateCodeSigningConfigError>>;

    /// <p><p>Updates an event source mapping. You can change the function that AWS Lambda invokes, or pause invocation and resume later from the same location.</p> <p>The following error handling options are only available for stream sources (DynamoDB and Kinesis):</p> <ul> <li> <p> <code>BisectBatchOnFunctionError</code> - If the function returns an error, split the batch in two and retry.</p> </li> <li> <p> <code>DestinationConfig</code> - Send discarded records to an Amazon SQS queue or Amazon SNS topic.</p> </li> <li> <p> <code>MaximumRecordAgeInSeconds</code> - Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires</p> </li> <li> <p> <code>MaximumRetryAttempts</code> - Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p> </li> <li> <p> <code>ParallelizationFactor</code> - Process multiple batches from each shard concurrently.</p> </li> </ul></p>
    async fn update_event_source_mapping(
        &self,
        input: UpdateEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<UpdateEventSourceMappingError>>;

    /// <p><p>Updates a Lambda function&#39;s code. If code signing is enabled for the function, the code package must be signed by a trusted publisher. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-trustedcode.html">Configuring code signing</a>.</p> <p>The function&#39;s code is locked when you publish a version. You can&#39;t modify the code of a published version, only the unpublished version.</p> <note> <p>For a function defined as a container image, Lambda resolves the image tag to an image digest. In Amazon ECR, if you update the image tag to a new image, Lambda does not automatically update the function.</p> </note></p>
    async fn update_function_code(
        &self,
        input: UpdateFunctionCodeRequest,
    ) -> Result<FunctionConfiguration, RusotoError<UpdateFunctionCodeError>>;

    /// <p>Modify the version-specific settings of a Lambda function.</p> <p>When you update a function, Lambda provisions an instance of the function and its supporting resources. If your function connects to a VPC, this process can take a minute. During this time, you can't modify the function, but you can still invoke it. The <code>LastUpdateStatus</code>, <code>LastUpdateStatusReason</code>, and <code>LastUpdateStatusReasonCode</code> fields in the response from <a>GetFunctionConfiguration</a> indicate when the update is complete and the function is processing events with the new configuration. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/functions-states.html">Function States</a>.</p> <p>These settings can vary between versions of a function and are locked when you publish a version. You can't modify the configuration of a published version, only the unpublished version.</p> <p>To configure function concurrency, use <a>PutFunctionConcurrency</a>. To grant invoke permissions to an account or AWS service, use <a>AddPermission</a>.</p>
    async fn update_function_configuration(
        &self,
        input: UpdateFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration, RusotoError<UpdateFunctionConfigurationError>>;

    /// <p>Updates the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    async fn update_function_event_invoke_config(
        &self,
        input: UpdateFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<UpdateFunctionEventInvokeConfigError>>;
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> LambdaClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        LambdaClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> LambdaClient {
        LambdaClient { client, region }
    }
}

#[async_trait]
impl Lambda for LambdaClient {
    /// <p>Adds permissions to the resource-based policy of a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Use this action to grant layer usage permission to other accounts. You can grant permission to a single account, all AWS accounts, or all accounts in an organization.</p> <p>To revoke permission, call <a>RemoveLayerVersionPermission</a> with the statement ID that you specified when you added it.</p>
    #[allow(unused_mut)]
    async fn add_layer_version_permission(
        &self,
        input: AddLayerVersionPermissionRequest,
    ) -> Result<AddLayerVersionPermissionResponse, RusotoError<AddLayerVersionPermissionError>>
    {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddLayerVersionPermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddLayerVersionPermissionError::from_response(response))
        }
    }

    /// <p>Grants an AWS service or another account permission to use a function. You can apply the policy at the function level, or specify a qualifier to restrict access to a single version or alias. If you use a qualifier, the invoker must use the full Amazon Resource Name (ARN) of that version or alias to invoke the function.</p> <p>To grant permission to another account, specify the account ID as the <code>Principal</code>. For AWS services, the principal is a domain-style identifier defined by the service, like <code>s3.amazonaws.com</code> or <code>sns.amazonaws.com</code>. For AWS services, you can also specify the ARN of the associated resource as the <code>SourceArn</code>. If you grant permission to a service principal without specifying the source, other accounts could potentially configure resources in their account to invoke your Lambda function.</p> <p>This action adds a statement to a resource-based permissions policy for the function. For more information about function policies, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">Lambda Function Policies</a>. </p>
    #[allow(unused_mut)]
    async fn add_permission(
        &self,
        input: AddPermissionRequest,
    ) -> Result<AddPermissionResponse, RusotoError<AddPermissionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AddPermissionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(AddPermissionError::from_response(response))
        }
    }

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a> for a Lambda function version. Use aliases to provide clients with a function identifier that you can update to invoke a different version.</p> <p>You can also map an alias to split invocation requests between two versions. Use the <code>RoutingConfig</code> parameter to specify a second version and the percentage of invocation requests that it receives.</p>
    #[allow(unused_mut)]
    async fn create_alias(
        &self,
        input: CreateAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<CreateAliasError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AliasConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateAliasError::from_response(response))
        }
    }

    /// <p>Creates a code signing configuration. A <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-trustedcode.html">code signing configuration</a> defines a list of allowed signing profiles and defines the code-signing validation policy (action to be taken if deployment validation checks fail). </p>
    #[allow(unused_mut)]
    async fn create_code_signing_config(
        &self,
        input: CreateCodeSigningConfigRequest,
    ) -> Result<CreateCodeSigningConfigResponse, RusotoError<CreateCodeSigningConfigError>> {
        let request_uri = "/2020-04-22/code-signing-configs/";

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateCodeSigningConfigError::from_response(response))
        }
    }

    /// <p><p>Creates a mapping between an event source and an AWS Lambda function. Lambda reads items from the event source and triggers the function.</p> <p>For details about each event source type, see the following topics.</p> <ul> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-ddb.html">Using AWS Lambda with Amazon DynamoDB</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-kinesis.html">Using AWS Lambda with Amazon Kinesis</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-sqs.html">Using AWS Lambda with Amazon SQS</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-mq.html">Using AWS Lambda with Amazon MQ</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/with-msk.html">Using AWS Lambda with Amazon MSK</a> </p> </li> <li> <p> <a href="https://docs.aws.amazon.com/lambda/latest/dg/kafka-smaa.html">Using AWS Lambda with Self-Managed Apache Kafka</a> </p> </li> </ul> <p>The following error handling options are only available for stream sources (DynamoDB and Kinesis):</p> <ul> <li> <p> <code>BisectBatchOnFunctionError</code> - If the function returns an error, split the batch in two and retry.</p> </li> <li> <p> <code>DestinationConfig</code> - Send discarded records to an Amazon SQS queue or Amazon SNS topic.</p> </li> <li> <p> <code>MaximumRecordAgeInSeconds</code> - Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires</p> </li> <li> <p> <code>MaximumRetryAttempts</code> - Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p> </li> <li> <p> <code>ParallelizationFactor</code> - Process multiple batches from each shard concurrently.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn create_event_source_mapping(
        &self,
        input: CreateEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<CreateEventSourceMappingError>> {
        let request_uri = "/2015-03-31/event-source-mappings/";

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EventSourceMappingConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateEventSourceMappingError::from_response(response))
        }
    }

    /// <p>Creates a Lambda function. To create a function, you need a <a href="https://docs.aws.amazon.com/lambda/latest/dg/gettingstarted-package.html">deployment package</a> and an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-permission-model.html#lambda-intro-execution-role">execution role</a>. The deployment package is a .zip file archive or container image that contains your function code. The execution role grants the function permission to use AWS services, such as Amazon CloudWatch Logs for log streaming and AWS X-Ray for request tracing.</p> <p>When you create a function, Lambda provisions an instance of the function and its supporting resources. If your function connects to a VPC, this process can take a minute or so. During this time, you can't invoke or modify the function. The <code>State</code>, <code>StateReason</code>, and <code>StateReasonCode</code> fields in the response from <a>GetFunctionConfiguration</a> indicate when the function is ready to invoke. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/functions-states.html">Function States</a>.</p> <p>A function has an unpublished version, and can have published versions and aliases. The unpublished version changes when you update your function's code and configuration. A published version is a snapshot of your function code and configuration that can't be changed. An alias is a named resource that maps to a version, and can be changed to map to a different version. Use the <code>Publish</code> parameter to create version <code>1</code> of your function from its initial configuration.</p> <p>The other parameters let you configure version-specific and function-level settings. You can modify version-specific settings later with <a>UpdateFunctionConfiguration</a>. Function-level settings apply to both the unpublished and published versions of the function, and include tags (<a>TagResource</a>) and per-function concurrency limits (<a>PutFunctionConcurrency</a>).</p> <p>You can use code signing if your deployment package is a .zip file archive. To enable code signing for this function, specify the ARN of a code-signing configuration. When a user attempts to deploy a code package with <a>UpdateFunctionCode</a>, Lambda checks that the code package has a valid signature from a trusted publisher. The code-signing configuration includes set set of signing profiles, which define the trusted publishers for this function.</p> <p>If another account or an AWS service invokes your function, use <a>AddPermission</a> to grant permission by creating a resource-based IAM policy. You can grant permissions at the function level, on a version, or on an alias.</p> <p>To invoke your function directly, use <a>Invoke</a>. To invoke your function in response to events in other AWS services, create an event source mapping (<a>CreateEventSourceMapping</a>), or configure a function trigger in the other service. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-invocation.html">Invoking Functions</a>.</p>
    #[allow(unused_mut)]
    async fn create_function(
        &self,
        input: CreateFunctionRequest,
    ) -> Result<FunctionConfiguration, RusotoError<CreateFunctionError>> {
        let request_uri = "/2015-03-31/functions";

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateFunctionError::from_response(response))
        }
    }

    /// <p>Deletes a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    #[allow(unused_mut)]
    async fn delete_alias(
        &self,
        input: DeleteAliasRequest,
    ) -> Result<(), RusotoError<DeleteAliasError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteAliasError::from_response(response))
        }
    }

    /// <p>Deletes the code signing configuration. You can delete the code signing configuration only if no function is using it. </p>
    #[allow(unused_mut)]
    async fn delete_code_signing_config(
        &self,
        input: DeleteCodeSigningConfigRequest,
    ) -> Result<DeleteCodeSigningConfigResponse, RusotoError<DeleteCodeSigningConfigError>> {
        let request_uri = format!(
            "/2020-04-22/code-signing-configs/{code_signing_config_arn}",
            code_signing_config_arn = input.code_signing_config_arn
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteCodeSigningConfigError::from_response(response))
        }
    }

    /// <p>Deletes an <a href="https://docs.aws.amazon.com/lambda/latest/dg/intro-invocation-modes.html">event source mapping</a>. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p> <p>When you delete an event source mapping, it enters a <code>Deleting</code> state and might not be completely deleted for several seconds.</p>
    #[allow(unused_mut)]
    async fn delete_event_source_mapping(
        &self,
        input: DeleteEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<DeleteEventSourceMappingError>> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EventSourceMappingConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteEventSourceMappingError::from_response(response))
        }
    }

    /// <p>Deletes a Lambda function. To delete a specific function version, use the <code>Qualifier</code> parameter. Otherwise, all versions and aliases are deleted.</p> <p>To delete Lambda event source mappings that invoke a function, use <a>DeleteEventSourceMapping</a>. For AWS services and resources that invoke your function directly, delete the trigger in the service where you originally configured it.</p>
    #[allow(unused_mut)]
    async fn delete_function(
        &self,
        input: DeleteFunctionRequest,
    ) -> Result<(), RusotoError<DeleteFunctionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFunctionError::from_response(response))
        }
    }

    /// <p>Removes the code signing configuration from the function.</p>
    #[allow(unused_mut)]
    async fn delete_function_code_signing_config(
        &self,
        input: DeleteFunctionCodeSigningConfigRequest,
    ) -> Result<(), RusotoError<DeleteFunctionCodeSigningConfigError>> {
        let request_uri = format!(
            "/2020-06-30/functions/{function_name}/code-signing-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFunctionCodeSigningConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Removes a concurrent execution limit from a function.</p>
    #[allow(unused_mut)]
    async fn delete_function_concurrency(
        &self,
        input: DeleteFunctionConcurrencyRequest,
    ) -> Result<(), RusotoError<DeleteFunctionConcurrencyError>> {
        let request_uri = format!(
            "/2017-10-31/functions/{function_name}/concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFunctionConcurrencyError::from_response(response))
        }
    }

    /// <p>Deletes the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    #[allow(unused_mut)]
    async fn delete_function_event_invoke_config(
        &self,
        input: DeleteFunctionEventInvokeConfigRequest,
    ) -> Result<(), RusotoError<DeleteFunctionEventInvokeConfigError>> {
        let request_uri = format!(
            "/2019-09-25/functions/{function_name}/event-invoke-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteFunctionEventInvokeConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Deletes a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Deleted versions can no longer be viewed or added to functions. To avoid breaking functions, a copy of the version remains in Lambda until no functions refer to it.</p>
    #[allow(unused_mut)]
    async fn delete_layer_version(
        &self,
        input: DeleteLayerVersionRequest,
    ) -> Result<(), RusotoError<DeleteLayerVersionError>> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLayerVersionError::from_response(response))
        }
    }

    /// <p>Deletes the provisioned concurrency configuration for a function.</p>
    #[allow(unused_mut)]
    async fn delete_provisioned_concurrency_config(
        &self,
        input: DeleteProvisionedConcurrencyConfigRequest,
    ) -> Result<(), RusotoError<DeleteProvisionedConcurrencyConfigError>> {
        let request_uri = format!(
            "/2019-09-30/functions/{function_name}/provisioned-concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("Qualifier", &input.qualifier);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteProvisionedConcurrencyConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Retrieves details about your account's <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limits</a> and usage in an AWS Region.</p>
    #[allow(unused_mut)]
    async fn get_account_settings(
        &self,
    ) -> Result<GetAccountSettingsResponse, RusotoError<GetAccountSettingsError>> {
        let request_uri = "/2016-08-19/account-settings/";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetAccountSettingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAccountSettingsError::from_response(response))
        }
    }

    /// <p>Returns details about a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    #[allow(unused_mut)]
    async fn get_alias(
        &self,
        input: GetAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<GetAliasError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AliasConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetAliasError::from_response(response))
        }
    }

    /// <p>Returns information about the specified code signing configuration.</p>
    #[allow(unused_mut)]
    async fn get_code_signing_config(
        &self,
        input: GetCodeSigningConfigRequest,
    ) -> Result<GetCodeSigningConfigResponse, RusotoError<GetCodeSigningConfigError>> {
        let request_uri = format!(
            "/2020-04-22/code-signing-configs/{code_signing_config_arn}",
            code_signing_config_arn = input.code_signing_config_arn
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCodeSigningConfigError::from_response(response))
        }
    }

    /// <p>Returns details about an event source mapping. You can get the identifier of a mapping from the output of <a>ListEventSourceMappings</a>.</p>
    #[allow(unused_mut)]
    async fn get_event_source_mapping(
        &self,
        input: GetEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<GetEventSourceMappingError>> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EventSourceMappingConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetEventSourceMappingError::from_response(response))
        }
    }

    /// <p>Returns information about the function or function version, with a link to download the deployment package that's valid for 10 minutes. If you specify a function version, only details that are specific to that version are returned.</p>
    #[allow(unused_mut)]
    async fn get_function(
        &self,
        input: GetFunctionRequest,
    ) -> Result<GetFunctionResponse, RusotoError<GetFunctionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFunctionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFunctionError::from_response(response))
        }
    }

    /// <p>Returns the code signing configuration for the specified function.</p>
    #[allow(unused_mut)]
    async fn get_function_code_signing_config(
        &self,
        input: GetFunctionCodeSigningConfigRequest,
    ) -> Result<GetFunctionCodeSigningConfigResponse, RusotoError<GetFunctionCodeSigningConfigError>>
    {
        let request_uri = format!(
            "/2020-06-30/functions/{function_name}/code-signing-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFunctionCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFunctionCodeSigningConfigError::from_response(response))
        }
    }

    /// <p>Returns details about the reserved concurrency configuration for a function. To set a concurrency limit for a function, use <a>PutFunctionConcurrency</a>.</p>
    #[allow(unused_mut)]
    async fn get_function_concurrency(
        &self,
        input: GetFunctionConcurrencyRequest,
    ) -> Result<GetFunctionConcurrencyResponse, RusotoError<GetFunctionConcurrencyError>> {
        let request_uri = format!(
            "/2019-09-30/functions/{function_name}/concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetFunctionConcurrencyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFunctionConcurrencyError::from_response(response))
        }
    }

    /// <p>Returns the version-specific settings of a Lambda function or version. The output includes only options that can vary between versions of a function. To modify these settings, use <a>UpdateFunctionConfiguration</a>.</p> <p>To get all of a function's details, including function-level settings, use <a>GetFunction</a>.</p>
    #[allow(unused_mut)]
    async fn get_function_configuration(
        &self,
        input: GetFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration, RusotoError<GetFunctionConfigurationError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFunctionConfigurationError::from_response(response))
        }
    }

    /// <p>Retrieves the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    #[allow(unused_mut)]
    async fn get_function_event_invoke_config(
        &self,
        input: GetFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<GetFunctionEventInvokeConfigError>> {
        let request_uri = format!(
            "/2019-09-25/functions/{function_name}/event-invoke-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionEventInvokeConfig, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetFunctionEventInvokeConfigError::from_response(response))
        }
    }

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    #[allow(unused_mut)]
    async fn get_layer_version(
        &self,
        input: GetLayerVersionRequest,
    ) -> Result<GetLayerVersionResponse, RusotoError<GetLayerVersionError>> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLayerVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLayerVersionError::from_response(response))
        }
    }

    /// <p>Returns information about a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>, with a link to download the layer archive that's valid for 10 minutes.</p>
    #[allow(unused_mut)]
    async fn get_layer_version_by_arn(
        &self,
        input: GetLayerVersionByArnRequest,
    ) -> Result<GetLayerVersionResponse, RusotoError<GetLayerVersionByArnError>> {
        let request_uri = "/2018-10-31/layers";

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("Arn", &input.arn);
        params.put("find", "LayerVersion");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLayerVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLayerVersionByArnError::from_response(response))
        }
    }

    /// <p>Returns the permission policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    #[allow(unused_mut)]
    async fn get_layer_version_policy(
        &self,
        input: GetLayerVersionPolicyRequest,
    ) -> Result<GetLayerVersionPolicyResponse, RusotoError<GetLayerVersionPolicyError>> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions/{version_number}/policy",
            layer_name = input.layer_name,
            version_number = input.version_number
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetLayerVersionPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetLayerVersionPolicyError::from_response(response))
        }
    }

    /// <p>Returns the <a href="https://docs.aws.amazon.com/lambda/latest/dg/access-control-resource-based.html">resource-based IAM policy</a> for a function, version, or alias.</p>
    #[allow(unused_mut)]
    async fn get_policy(
        &self,
        input: GetPolicyRequest,
    ) -> Result<GetPolicyResponse, RusotoError<GetPolicyError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetPolicyResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetPolicyError::from_response(response))
        }
    }

    /// <p>Retrieves the provisioned concurrency configuration for a function's alias or version.</p>
    #[allow(unused_mut)]
    async fn get_provisioned_concurrency_config(
        &self,
        input: GetProvisionedConcurrencyConfigRequest,
    ) -> Result<
        GetProvisionedConcurrencyConfigResponse,
        RusotoError<GetProvisionedConcurrencyConfigError>,
    > {
        let request_uri = format!(
            "/2019-09-30/functions/{function_name}/provisioned-concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("Qualifier", &input.qualifier);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetProvisionedConcurrencyConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetProvisionedConcurrencyConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Invokes a Lambda function. You can invoke a function synchronously (and wait for the response), or asynchronously. To invoke a function asynchronously, set <code>InvocationType</code> to <code>Event</code>.</p> <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-sync.html">synchronous invocation</a>, details about the function response, including errors, are included in the response body and headers. For either invocation type, you can find more information in the <a href="https://docs.aws.amazon.com/lambda/latest/dg/monitoring-functions.html">execution log</a> and <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-x-ray.html">trace</a>.</p> <p>When an error occurs, your function may be invoked multiple times. Retry behavior varies by error type, client, event source, and invocation type. For example, if you invoke a function asynchronously and it returns an error, Lambda executes the function up to two more times. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/retries-on-errors.html">Retry Behavior</a>.</p> <p>For <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html">asynchronous invocation</a>, Lambda adds events to a queue before sending them to your function. If your function does not have enough capacity to keep up with the queue, events may be lost. Occasionally, your function may receive the same event multiple times, even if no error occurs. To retain events that were not processed, configure your function with a <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#dlq">dead-letter queue</a>.</p> <p>The status code in the API response doesn't reflect function errors. Error codes are reserved for errors that prevent your function from executing, such as permissions errors, <a href="https://docs.aws.amazon.com/lambda/latest/dg/limits.html">limit errors</a>, or issues with your function's code and configuration. For example, Lambda returns <code>TooManyRequestsException</code> if executing the function would cause you to exceed a concurrency limit at either the account level (<code>ConcurrentInvocationLimitExceeded</code>) or function level (<code>ReservedFunctionConcurrentInvocationLimitExceeded</code>).</p> <p>For functions with a long timeout, your client might be disconnected during synchronous invocation while it waits for a response. Configure your HTTP client, SDK, firewall, proxy, or operating system to allow for long connections with timeout or keep-alive settings.</p> <p>This operation requires permission for the <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html">lambda:InvokeFunction</a> action.</p>
    #[allow(unused_mut)]
    async fn invoke(
        &self,
        input: InvocationRequest,
    ) -> Result<InvocationResponse, RusotoError<InvokeError>> {
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
        request.add_optional_header("X-Amz-Client-Context", input.client_context.as_ref());
        request.add_optional_header("X-Amz-Invocation-Type", input.invocation_type.as_ref());
        request.add_optional_header("X-Amz-Log-Type", input.log_type.as_ref());
        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = InvocationResponse::default();
            result.payload = Some(response.body);

            result.executed_version = response.headers.remove("X-Amz-Executed-Version");
            result.function_error = response.headers.remove("X-Amz-Function-Error");
            result.log_result = response.headers.remove("X-Amz-Log-Result");
            result.status_code = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InvokeError::from_response(response))
        }
    }

    /// <p><important> <p>For asynchronous function invocation, use <a>Invoke</a>.</p> </important> <p>Invokes a function asynchronously.</p></p>
    #[allow(unused_mut)]
    async fn invoke_async(
        &self,
        input: InvokeAsyncRequest,
    ) -> Result<InvokeAsyncResponse, RusotoError<InvokeAsyncError>> {
        let request_uri = format!(
            "/2014-11-13/functions/{function_name}/invoke-async/",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.invoke_args.to_owned());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<InvokeAsyncResponse, _>()?;

            result.status = Some(response.status.as_u16() as i64);
            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(InvokeAsyncError::from_response(response))
        }
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">aliases</a> for a Lambda function.</p>
    #[allow(unused_mut)]
    async fn list_aliases(
        &self,
        input: ListAliasesRequest,
    ) -> Result<ListAliasesResponse, RusotoError<ListAliasesError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListAliasesResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListAliasesError::from_response(response))
        }
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuring-codesigning.html">code signing configurations</a>. A request returns up to 10,000 configurations per call. You can use the <code>MaxItems</code> parameter to return fewer configurations per call. </p>
    #[allow(unused_mut)]
    async fn list_code_signing_configs(
        &self,
        input: ListCodeSigningConfigsRequest,
    ) -> Result<ListCodeSigningConfigsResponse, RusotoError<ListCodeSigningConfigsError>> {
        let request_uri = "/2020-04-22/code-signing-configs/";

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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListCodeSigningConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListCodeSigningConfigsError::from_response(response))
        }
    }

    /// <p>Lists event source mappings. Specify an <code>EventSourceArn</code> to only show event source mappings for a single event source.</p>
    #[allow(unused_mut)]
    async fn list_event_source_mappings(
        &self,
        input: ListEventSourceMappingsRequest,
    ) -> Result<ListEventSourceMappingsResponse, RusotoError<ListEventSourceMappingsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListEventSourceMappingsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListEventSourceMappingsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of configurations for asynchronous invocation for a function.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    #[allow(unused_mut)]
    async fn list_function_event_invoke_configs(
        &self,
        input: ListFunctionEventInvokeConfigsRequest,
    ) -> Result<
        ListFunctionEventInvokeConfigsResponse,
        RusotoError<ListFunctionEventInvokeConfigsError>,
    > {
        let request_uri = format!(
            "/2019-09-25/functions/{function_name}/event-invoke-config/list",
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFunctionEventInvokeConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFunctionEventInvokeConfigsError::from_response(response))
        }
    }

    /// <p>Returns a list of Lambda functions, with the version-specific configuration of each. Lambda returns up to 50 functions per call.</p> <p>Set <code>FunctionVersion</code> to <code>ALL</code> to include all published versions of each function in addition to the unpublished version. To get more information about a function or version, use <a>GetFunction</a>.</p>
    #[allow(unused_mut)]
    async fn list_functions(
        &self,
        input: ListFunctionsRequest,
    ) -> Result<ListFunctionsResponse, RusotoError<ListFunctionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFunctionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFunctionsError::from_response(response))
        }
    }

    /// <p>List the functions that use the specified code signing configuration. You can use this method prior to deleting a code signing configuration, to verify that no functions are using it.</p>
    #[allow(unused_mut)]
    async fn list_functions_by_code_signing_config(
        &self,
        input: ListFunctionsByCodeSigningConfigRequest,
    ) -> Result<
        ListFunctionsByCodeSigningConfigResponse,
        RusotoError<ListFunctionsByCodeSigningConfigError>,
    > {
        let request_uri = format!(
            "/2020-04-22/code-signing-configs/{code_signing_config_arn}/functions",
            code_signing_config_arn = input.code_signing_config_arn
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListFunctionsByCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListFunctionsByCodeSigningConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Lists the versions of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. Versions that have been deleted aren't listed. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only versions that indicate that they're compatible with that runtime.</p>
    #[allow(unused_mut)]
    async fn list_layer_versions(
        &self,
        input: ListLayerVersionsRequest,
    ) -> Result<ListLayerVersionsResponse, RusotoError<ListLayerVersionsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLayerVersionsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListLayerVersionsError::from_response(response))
        }
    }

    /// <p>Lists <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layers</a> and shows information about the latest version of each. Specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/lambda-runtimes.html">runtime identifier</a> to list only layers that indicate that they're compatible with that runtime.</p>
    #[allow(unused_mut)]
    async fn list_layers(
        &self,
        input: ListLayersRequest,
    ) -> Result<ListLayersResponse, RusotoError<ListLayersError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListLayersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListLayersError::from_response(response))
        }
    }

    /// <p>Retrieves a list of provisioned concurrency configurations for a function.</p>
    #[allow(unused_mut)]
    async fn list_provisioned_concurrency_configs(
        &self,
        input: ListProvisionedConcurrencyConfigsRequest,
    ) -> Result<
        ListProvisionedConcurrencyConfigsResponse,
        RusotoError<ListProvisionedConcurrencyConfigsError>,
    > {
        let request_uri = format!(
            "/2019-09-30/functions/{function_name}/provisioned-concurrency",
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
        params.put("List", "ALL");
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListProvisionedConcurrencyConfigsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListProvisionedConcurrencyConfigsError::from_response(
                response,
            ))
        }
    }

    /// <p>Returns a function's <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a>. You can also view tags with <a>GetFunction</a>.</p>
    #[allow(unused_mut)]
    async fn list_tags(
        &self,
        input: ListTagsRequest,
    ) -> Result<ListTagsResponse, RusotoError<ListTagsError>> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("GET", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsError::from_response(response))
        }
    }

    /// <p>Returns a list of <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">versions</a>, with the version-specific configuration of each. Lambda returns up to 50 versions per call.</p>
    #[allow(unused_mut)]
    async fn list_versions_by_function(
        &self,
        input: ListVersionsByFunctionRequest,
    ) -> Result<ListVersionsByFunctionResponse, RusotoError<ListVersionsByFunctionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVersionsByFunctionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVersionsByFunctionError::from_response(response))
        }
    }

    /// <p>Creates an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a> from a ZIP archive. Each time you call <code>PublishLayerVersion</code> with the same layer name, a new version is created.</p> <p>Add layers to your function with <a>CreateFunction</a> or <a>UpdateFunctionConfiguration</a>.</p>
    #[allow(unused_mut)]
    async fn publish_layer_version(
        &self,
        input: PublishLayerVersionRequest,
    ) -> Result<PublishLayerVersionResponse, RusotoError<PublishLayerVersionError>> {
        let request_uri = format!(
            "/2018-10-31/layers/{layer_name}/versions",
            layer_name = input.layer_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PublishLayerVersionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PublishLayerVersionError::from_response(response))
        }
    }

    /// <p>Creates a <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">version</a> from the current code and configuration of a function. Use versions to create a snapshot of your function code and configuration that doesn't change.</p> <p>AWS Lambda doesn't publish a version if the function's configuration and code haven't changed since the last version. Use <a>UpdateFunctionCode</a> or <a>UpdateFunctionConfiguration</a> to update the function before publishing a version.</p> <p>Clients can invoke versions directly or with an alias. To create an alias, use <a>CreateAlias</a>.</p>
    #[allow(unused_mut)]
    async fn publish_version(
        &self,
        input: PublishVersionRequest,
    ) -> Result<FunctionConfiguration, RusotoError<PublishVersionError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/versions",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PublishVersionError::from_response(response))
        }
    }

    /// <p>Update the code signing configuration for the function. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. </p>
    #[allow(unused_mut)]
    async fn put_function_code_signing_config(
        &self,
        input: PutFunctionCodeSigningConfigRequest,
    ) -> Result<PutFunctionCodeSigningConfigResponse, RusotoError<PutFunctionCodeSigningConfigError>>
    {
        let request_uri = format!(
            "/2020-06-30/functions/{function_name}/code-signing-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutFunctionCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutFunctionCodeSigningConfigError::from_response(response))
        }
    }

    /// <p>Sets the maximum number of simultaneous executions for a function, and reserves capacity for that concurrency level.</p> <p>Concurrency settings apply to the function as a whole, including all published versions and the unpublished version. Reserving concurrency both ensures that your function has capacity to process the specified number of events simultaneously, and prevents it from scaling beyond that level. Use <a>GetFunction</a> to see the current setting for a function.</p> <p>Use <a>GetAccountSettings</a> to see your Regional concurrency limit. You can reserve concurrency for as many functions as you like, as long as you leave at least 100 simultaneous executions unreserved for functions that aren't configured with a per-function limit. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/concurrent-executions.html">Managing Concurrency</a>.</p>
    #[allow(unused_mut)]
    async fn put_function_concurrency(
        &self,
        input: PutFunctionConcurrencyRequest,
    ) -> Result<Concurrency, RusotoError<PutFunctionConcurrencyError>> {
        let request_uri = format!(
            "/2017-10-31/functions/{function_name}/concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result =
                proto::json::ResponsePayload::new(&response).deserialize::<Concurrency, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutFunctionConcurrencyError::from_response(response))
        }
    }

    /// <p>Configures options for <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html">asynchronous invocation</a> on a function, version, or alias. If a configuration already exists for a function, version, or alias, this operation overwrites it. If you exclude any settings, they are removed. To set one option without affecting existing settings for other options, use <a>UpdateFunctionEventInvokeConfig</a>.</p> <p>By default, Lambda retries an asynchronous invocation twice if the function returns an error. It retains events in a queue for up to six hours. When an event fails all processing attempts or stays in the asynchronous invocation queue for too long, Lambda discards it. To retain discarded events, configure a dead-letter queue with <a>UpdateFunctionConfiguration</a>.</p> <p>To send an invocation record to a queue, topic, function, or event bus, specify a <a href="https://docs.aws.amazon.com/lambda/latest/dg/invocation-async.html#invocation-async-destinations">destination</a>. You can configure separate destinations for successful invocations (on-success) and events that fail all processing attempts (on-failure). You can configure destinations in addition to or instead of a dead-letter queue.</p>
    #[allow(unused_mut)]
    async fn put_function_event_invoke_config(
        &self,
        input: PutFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<PutFunctionEventInvokeConfigError>> {
        let request_uri = format!(
            "/2019-09-25/functions/{function_name}/event-invoke-config",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        if let Some(ref x) = input.qualifier {
            params.put("Qualifier", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionEventInvokeConfig, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutFunctionEventInvokeConfigError::from_response(response))
        }
    }

    /// <p>Adds a provisioned concurrency configuration to a function's alias or version.</p>
    #[allow(unused_mut)]
    async fn put_provisioned_concurrency_config(
        &self,
        input: PutProvisionedConcurrencyConfigRequest,
    ) -> Result<
        PutProvisionedConcurrencyConfigResponse,
        RusotoError<PutProvisionedConcurrencyConfigError>,
    > {
        let request_uri = format!(
            "/2019-09-30/functions/{function_name}/provisioned-concurrency",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("Qualifier", &input.qualifier);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutProvisionedConcurrencyConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutProvisionedConcurrencyConfigError::from_response(
                response,
            ))
        }
    }

    /// <p>Removes a statement from the permissions policy for a version of an <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-layers.html">AWS Lambda layer</a>. For more information, see <a>AddLayerVersionPermission</a>.</p>
    #[allow(unused_mut)]
    async fn remove_layer_version_permission(
        &self,
        input: RemoveLayerVersionPermissionRequest,
    ) -> Result<(), RusotoError<RemoveLayerVersionPermissionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemoveLayerVersionPermissionError::from_response(response))
        }
    }

    /// <p>Revokes function-use permission from an AWS service or another account. You can get the ID of the statement from the output of <a>GetPolicy</a>.</p>
    #[allow(unused_mut)]
    async fn remove_permission(
        &self,
        input: RemovePermissionRequest,
    ) -> Result<(), RusotoError<RemovePermissionError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RemovePermissionError::from_response(response))
        }
    }

    /// <p>Adds <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> to a function.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<(), RusotoError<TagResourceError>> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("POST", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes <a href="https://docs.aws.amazon.com/lambda/latest/dg/tagging.html">tags</a> from a function.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<(), RusotoError<UntagResourceError>> {
        let request_uri = format!("/2017-03-31/tags/{arn}", arn = input.resource);

        let mut request = SignedRequest::new("DELETE", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 204 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates the configuration of a Lambda function <a href="https://docs.aws.amazon.com/lambda/latest/dg/versioning-aliases.html">alias</a>.</p>
    #[allow(unused_mut)]
    async fn update_alias(
        &self,
        input: UpdateAliasRequest,
    ) -> Result<AliasConfiguration, RusotoError<UpdateAliasError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/aliases/{name}",
            function_name = input.function_name,
            name = input.name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<AliasConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateAliasError::from_response(response))
        }
    }

    /// <p>Update the code signing configuration. Changes to the code signing configuration take effect the next time a user tries to deploy a code package to the function. </p>
    #[allow(unused_mut)]
    async fn update_code_signing_config(
        &self,
        input: UpdateCodeSigningConfigRequest,
    ) -> Result<UpdateCodeSigningConfigResponse, RusotoError<UpdateCodeSigningConfigError>> {
        let request_uri = format!(
            "/2020-04-22/code-signing-configs/{code_signing_config_arn}",
            code_signing_config_arn = input.code_signing_config_arn
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateCodeSigningConfigResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateCodeSigningConfigError::from_response(response))
        }
    }

    /// <p><p>Updates an event source mapping. You can change the function that AWS Lambda invokes, or pause invocation and resume later from the same location.</p> <p>The following error handling options are only available for stream sources (DynamoDB and Kinesis):</p> <ul> <li> <p> <code>BisectBatchOnFunctionError</code> - If the function returns an error, split the batch in two and retry.</p> </li> <li> <p> <code>DestinationConfig</code> - Send discarded records to an Amazon SQS queue or Amazon SNS topic.</p> </li> <li> <p> <code>MaximumRecordAgeInSeconds</code> - Discard records older than the specified age. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires</p> </li> <li> <p> <code>MaximumRetryAttempts</code> - Discard records after the specified number of retries. The default value is infinite (-1). When set to infinite (-1), failed records are retried until the record expires.</p> </li> <li> <p> <code>ParallelizationFactor</code> - Process multiple batches from each shard concurrently.</p> </li> </ul></p>
    #[allow(unused_mut)]
    async fn update_event_source_mapping(
        &self,
        input: UpdateEventSourceMappingRequest,
    ) -> Result<EventSourceMappingConfiguration, RusotoError<UpdateEventSourceMappingError>> {
        let request_uri = format!(
            "/2015-03-31/event-source-mappings/{uuid}",
            uuid = input.uuid
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<EventSourceMappingConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateEventSourceMappingError::from_response(response))
        }
    }

    /// <p><p>Updates a Lambda function&#39;s code. If code signing is enabled for the function, the code package must be signed by a trusted publisher. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/configuration-trustedcode.html">Configuring code signing</a>.</p> <p>The function&#39;s code is locked when you publish a version. You can&#39;t modify the code of a published version, only the unpublished version.</p> <note> <p>For a function defined as a container image, Lambda resolves the image tag to an image digest. In Amazon ECR, if you update the image tag to a new image, Lambda does not automatically update the function.</p> </note></p>
    #[allow(unused_mut)]
    async fn update_function_code(
        &self,
        input: UpdateFunctionCodeRequest,
    ) -> Result<FunctionConfiguration, RusotoError<UpdateFunctionCodeError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/code",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFunctionCodeError::from_response(response))
        }
    }

    /// <p>Modify the version-specific settings of a Lambda function.</p> <p>When you update a function, Lambda provisions an instance of the function and its supporting resources. If your function connects to a VPC, this process can take a minute. During this time, you can't modify the function, but you can still invoke it. The <code>LastUpdateStatus</code>, <code>LastUpdateStatusReason</code>, and <code>LastUpdateStatusReasonCode</code> fields in the response from <a>GetFunctionConfiguration</a> indicate when the update is complete and the function is processing events with the new configuration. For more information, see <a href="https://docs.aws.amazon.com/lambda/latest/dg/functions-states.html">Function States</a>.</p> <p>These settings can vary between versions of a function and are locked when you publish a version. You can't modify the configuration of a published version, only the unpublished version.</p> <p>To configure function concurrency, use <a>PutFunctionConcurrency</a>. To grant invoke permissions to an account or AWS service, use <a>AddPermission</a>.</p>
    #[allow(unused_mut)]
    async fn update_function_configuration(
        &self,
        input: UpdateFunctionConfigurationRequest,
    ) -> Result<FunctionConfiguration, RusotoError<UpdateFunctionConfigurationError>> {
        let request_uri = format!(
            "/2015-03-31/functions/{function_name}/configuration",
            function_name = input.function_name
        );

        let mut request = SignedRequest::new("PUT", "lambda", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionConfiguration, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFunctionConfigurationError::from_response(response))
        }
    }

    /// <p>Updates the configuration for asynchronous invocation for a function, version, or alias.</p> <p>To configure options for asynchronous invocation, use <a>PutFunctionEventInvokeConfig</a>.</p>
    #[allow(unused_mut)]
    async fn update_function_event_invoke_config(
        &self,
        input: UpdateFunctionEventInvokeConfigRequest,
    ) -> Result<FunctionEventInvokeConfig, RusotoError<UpdateFunctionEventInvokeConfigError>> {
        let request_uri = format!(
            "/2019-09-25/functions/{function_name}/event-invoke-config",
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<FunctionEventInvokeConfig, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateFunctionEventInvokeConfigError::from_response(
                response,
            ))
        }
    }
}
