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
#![allow(warnings)]

use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};
use std::error::Error;
use std::fmt;

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
/// <p>An activity that adds other attributes based on existing attributes in the message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddAttributesActivity {
    /// <p><p>A list of 1-50 &quot;AttributeNameMapping&quot; objects that map an existing attribute to a new attribute.</p> <note> <p>The existing attributes remain in the message, so if you want to remove the originals, use &quot;RemoveAttributeActivity&quot;.</p> </note></p>
    #[serde(rename = "attributes")]
    pub attributes: ::std::collections::HashMap<String, String>,
    /// <p>The name of the 'addAttributes' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// <p>Contains informations about errors.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct BatchPutMessageErrorEntry {
    /// <p>The code associated with the error.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The message associated with the error.</p>
    #[serde(rename = "errorMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// <p>The ID of the message that caused the error. (See the value corresponding to the "messageId" key in the message object.)</p>
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BatchPutMessageRequest {
    /// <p>The name of the channel where the messages are sent.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>The list of messages to be sent. Each message has format: '{ "messageId": "string", "payload": "string"}'.</p> <p>Note that the field names of message payloads (data) that you send to AWS IoT Analytics:</p> <ul> <li> <p>Must contain only alphanumeric characters and undescores (_); no other special characters are allowed.</p> </li> <li> <p>Must begin with an alphabetic character or single underscore (_).</p> </li> <li> <p>Cannot contain hyphens (-).</p> </li> <li> <p>In regular expression terms: "^[A-Za-z_]([A-Za-z0-9]*|[A-Za-z0-9][A-Za-z0-9_]*)$". </p> </li> <li> <p>Cannot be greater than 255 characters.</p> </li> <li> <p>Are case-insensitive. (Fields named "foo" and "FOO" in the same payload are considered duplicates.)</p> </li> </ul> <p>For example, {"temp_01": 29} or {"_temp_01": 29} are valid, but {"temp-01": 29}, {"01_temp": 29} or {"__temp_01": 29} are invalid in message payloads. </p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct BatchPutMessageResponse {
    /// <p>A list of any errors encountered when sending the messages to the channel.</p>
    #[serde(rename = "batchPutMessageErrorEntries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_put_message_error_entries: Option<Vec<BatchPutMessageErrorEntry>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CancelPipelineReprocessingRequest {
    /// <p>The name of pipeline for which data reprocessing is canceled.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The ID of the reprocessing task (returned by "StartPipelineReprocessing").</p>
    #[serde(rename = "reprocessingId")]
    pub reprocessing_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CancelPipelineReprocessingResponse {}

/// <p>A collection of data from an MQTT topic. Channels archive the raw, unprocessed messages before publishing the data to a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct Channel {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the channel was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>When the channel was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>How long, in days, message data is kept for the channel.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>The status of the channel.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Where channel data is stored.</p>
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ChannelStorage>,
}

/// <p>The activity that determines the source of the messages to be processed.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelActivity {
    /// <p>The name of the channel from which the messages are processed.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>The name of the 'channel' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// <p>Statistics information about the channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ChannelStatistics {
    /// <p>The estimated size of the channel.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<EstimatedResourceSize>,
}

/// <p>Where channel data is stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChannelStorage {
    /// <p>Use this to store channel data in an S3 bucket that you manage.</p>
    #[serde(rename = "customerManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3: Option<CustomerManagedChannelS3Storage>,
    /// <p>Use this to store channel data in an S3 bucket managed by the AWS IoT Analytics service.</p>
    #[serde(rename = "serviceManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_s3: Option<ServiceManagedChannelS3Storage>,
}

/// <p>Where channel data is stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ChannelStorageSummary {
    /// <p>Used to store channel data in an S3 bucket that you manage.</p>
    #[serde(rename = "customerManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3: Option<CustomerManagedChannelS3StorageSummary>,
    /// <p>Used to store channel data in an S3 bucket managed by the AWS IoT Analytics service.</p>
    #[serde(rename = "serviceManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_s3: Option<ServiceManagedChannelS3StorageSummary>,
}

/// <p>A summary of information about a channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ChannelSummary {
    /// <p>The name of the channel.</p>
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>Where channel data is stored.</p>
    #[serde(rename = "channelStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_storage: Option<ChannelStorageSummary>,
    /// <p>When the channel was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The last time the channel was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The status of the channel.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>Information needed to run the "containerAction" to produce data set contents.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContainerDatasetAction {
    /// <p>The ARN of the role which gives permission to the system to access needed resources in order to run the "containerAction". This includes, at minimum, permission to retrieve the data set contents which are the input to the containerized application.</p>
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The ARN of the Docker container stored in your account. The Docker container contains an application and needed support libraries and is used to generate data set contents.</p>
    #[serde(rename = "image")]
    pub image: String,
    /// <p>Configuration of the resource which executes the "containerAction".</p>
    #[serde(rename = "resourceConfiguration")]
    pub resource_configuration: ResourceConfiguration,
    /// <p>The values of variables used within the context of the execution of the containerized application (basically, parameters passed to the application). Each variable must have a name and a value given by one of "stringValue", "datasetContentVersionValue", or "outputFileUriValue".</p>
    #[serde(rename = "variables")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<Variable>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateChannelRequest {
    /// <p>The name of the channel.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>Where channel data is stored.</p>
    #[serde(rename = "channelStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_storage: Option<ChannelStorage>,
    /// <p>How long, in days, message data is kept for the channel.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>Metadata which can be used to manage the channel.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreateChannelResponse {
    /// <p>The ARN of the channel.</p>
    #[serde(rename = "channelArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_arn: Option<String>,
    /// <p>The name of the channel.</p>
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
    /// <p>How long, in days, message data is kept for the channel.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatasetContentRequest {
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreateDatasetContentResponse {
    /// <p>The version ID of the data set contents which are being created.</p>
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatasetRequest {
    /// <p>A list of actions that create the data set contents.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<DatasetAction>,
    /// <p>When data set contents are created they are delivered to destinations specified here.</p>
    #[serde(rename = "contentDeliveryRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_delivery_rules: Option<Vec<DatasetContentDeliveryRule>>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>[Optional] How long, in days, versions of data set contents are kept for the data set. If not specified or set to null, versions of data set contents are retained for at most 90 days. The number of versions of data set contents retained is determined by the <code>versioningConfiguration</code> parameter. (For more information, see https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions)</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>Metadata which can be used to manage the data set.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of triggers. A trigger causes data set contents to be populated at a specified time interval or when another data set's contents are created. The list of triggers can be empty or contain up to five <b>DataSetTrigger</b> objects.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
    /// <p>[Optional] How many versions of data set contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the "retentionPeriod" parameter. (For more information, see https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions)</p>
    #[serde(rename = "versioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<VersioningConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreateDatasetResponse {
    /// <p>The ARN of the data set.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// <p>How long, in days, data set contents are kept for the data set.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatastoreRequest {
    /// <p>The name of the data store.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    /// <p>Where data store data is stored.</p>
    #[serde(rename = "datastoreStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_storage: Option<DatastoreStorage>,
    /// <p>How long, in days, message data is kept for the data store.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>Metadata which can be used to manage the data store.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreateDatastoreResponse {
    /// <p>The ARN of the data store.</p>
    #[serde(rename = "datastoreArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_arn: Option<String>,
    /// <p>The name of the data store.</p>
    #[serde(rename = "datastoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,
    /// <p>How long, in days, message data is kept for the data store.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreatePipelineRequest {
    /// <p>A list of "PipelineActivity" objects. Activities perform transformations on your messages, such as removing, renaming or adding message attributes; filtering messages based on attribute values; invoking your Lambda functions on messages for advanced processing; or performing mathematical transformations to normalize device data.</p> <p>The list can be 2-25 <b>PipelineActivity</b> objects and must contain both a <code>channel</code> and a <code>datastore</code> activity. Each entry in the list must contain only one activity, for example:</p> <p> <code>pipelineActivities = [ { "channel": { ... } }, { "lambda": { ... } }, ... ]</code> </p>
    #[serde(rename = "pipelineActivities")]
    pub pipeline_activities: Vec<PipelineActivity>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>Metadata which can be used to manage the pipeline.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CreatePipelineResponse {
    /// <p>The ARN of the pipeline.</p>
    #[serde(rename = "pipelineArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_arn: Option<String>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
}

/// <p>Use this to store channel data in an S3 bucket that you manage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerManagedChannelS3Storage {
    /// <p>The name of the Amazon S3 bucket in which channel data is stored.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>The prefix used to create the keys of the channel data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier within the bucket (each object in a bucket has exactly one key).</p>
    #[serde(rename = "keyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to interact with your Amazon S3 resources.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>Used to store channel data in an S3 bucket that you manage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CustomerManagedChannelS3StorageSummary {
    /// <p>The name of the Amazon S3 bucket in which channel data is stored.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The prefix used to create the keys of the channel data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier within the bucket (each object in a bucket has exactly one key).</p>
    #[serde(rename = "keyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to interact with your Amazon S3 resources.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Use this to store data store data in an S3 bucket that you manage.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerManagedDatastoreS3Storage {
    /// <p>The name of the Amazon S3 bucket in which data store data is stored.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>The prefix used to create the keys of the data store data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier within the bucket (each object in a bucket has exactly one key).</p>
    #[serde(rename = "keyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to interact with your Amazon S3 resources.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>Used to store data store data in an S3 bucket that you manage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct CustomerManagedDatastoreS3StorageSummary {
    /// <p>The name of the Amazon S3 bucket in which data store data is stored.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>The prefix used to create the keys of the data store data objects. Each object in an Amazon S3 bucket has a key that is its unique identifier within the bucket (each object in a bucket has exactly one key).</p>
    #[serde(rename = "keyPrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_prefix: Option<String>,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to interact with your Amazon S3 resources.</p>
    #[serde(rename = "roleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>Information about a data set.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct Dataset {
    /// <p>The "DatasetAction" objects that automatically create the data set contents.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DatasetAction>>,
    /// <p>The ARN of the data set.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When data set contents are created they are delivered to destinations specified here.</p>
    #[serde(rename = "contentDeliveryRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_delivery_rules: Option<Vec<DatasetContentDeliveryRule>>,
    /// <p>When the data set was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The last time the data set was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>[Optional] How long, in days, message data is kept for the data set.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>The status of the data set.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The "DatasetTrigger" objects that specify when the data set is automatically updated.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
    /// <p>[Optional] How many versions of data set contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the "retentionPeriod" parameter. (For more information, see https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions)</p>
    #[serde(rename = "versioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<VersioningConfiguration>,
}

/// <p>A "DatasetAction" object that specifies how data set contents are automatically created.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetAction {
    /// <p>The name of the data set action by which data set contents are automatically created.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>Information which allows the system to run a containerized application in order to create the data set contents. The application must be in a Docker container along with any needed support libraries.</p>
    #[serde(rename = "containerAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_action: Option<ContainerDatasetAction>,
    /// <p>An "SqlQueryDatasetAction" object that uses an SQL query to automatically create data set contents.</p>
    #[serde(rename = "queryAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_action: Option<SqlQueryDatasetAction>,
}

/// <p>Information about the action which automatically creates the data set's contents.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatasetActionSummary {
    /// <p>The name of the action which automatically creates the data set's contents.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>The type of action by which the data set's contents are automatically created.</p>
    #[serde(rename = "actionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
}

/// <p>The destination to which data set contents are delivered.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetContentDeliveryDestination {
    /// <p>Configuration information for delivery of data set contents to AWS IoT Events.</p>
    #[serde(rename = "iotEventsDestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_events_destination_configuration: Option<IotEventsDestinationConfiguration>,
    /// <p>Configuration information for delivery of data set contents to Amazon S3.</p>
    #[serde(rename = "s3DestinationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_destination_configuration: Option<S3DestinationConfiguration>,
}

/// <p>When data set contents are created they are delivered to destination specified here.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetContentDeliveryRule {
    /// <p>The destination to which data set contents are delivered.</p>
    #[serde(rename = "destination")]
    pub destination: DatasetContentDeliveryDestination,
    /// <p>The name of the data set content delivery rules entry.</p>
    #[serde(rename = "entryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
}

/// <p>The state of the data set contents and the reason they are in this state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatasetContentStatus {
    /// <p>The reason the data set contents are in this state.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The state of the data set contents. Can be one of "READY", "CREATING", "SUCCEEDED" or "FAILED".</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>Summary information about data set contents.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatasetContentSummary {
    /// <p>The actual time the creation of the data set contents was started.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The time the creation of the data set contents was scheduled to start.</p>
    #[serde(rename = "scheduleTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_time: Option<f64>,
    /// <p>The status of the data set contents.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DatasetContentStatus>,
    /// <p>The version of the data set contents.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>The data set whose latest contents are used as input to the notebook or application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetContentVersionValue {
    /// <p>The name of the data set whose latest contents are used as input to the notebook or application.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
}

/// <p>The reference to a data set entry.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatasetEntry {
    /// <p>The pre-signed URI of the data set item.</p>
    #[serde(rename = "dataURI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_uri: Option<String>,
    /// <p>The name of the data set item.</p>
    #[serde(rename = "entryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
}

/// <p>A summary of information about a data set.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatasetSummary {
    /// <p>A list of "DataActionSummary" objects.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DatasetActionSummary>>,
    /// <p>The time the data set was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// <p>The last time the data set was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The status of the data set.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A list of triggers. A trigger causes data set content to be populated at a specified time interval or when another data set is populated. The list of triggers can be empty or contain up to five DataSetTrigger objects</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
}

/// <p>The "DatasetTrigger" that specifies when the data set is automatically updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetTrigger {
    /// <p>The data set whose content creation triggers the creation of this data set's contents.</p>
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<TriggeringDataset>,
    /// <p>The "Schedule" when the trigger is initiated.</p>
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/// <p>Information about a data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct Datastore {
    /// <p>The ARN of the data store.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the data store was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The last time the data store was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name of the data store.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>How long, in days, message data is kept for the data store.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p><p>The status of a data store:</p> <dl> <dt>CREATING</dt> <dd> <p>The data store is being created.</p> </dd> <dt>ACTIVE</dt> <dd> <p>The data store has been created and can be used.</p> </dd> <dt>DELETING</dt> <dd> <p>The data store is being deleted.</p> </dd> </dl></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>Where data store data is stored.</p>
    #[serde(rename = "storage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<DatastoreStorage>,
}

/// <p>The 'datastore' activity that specifies where to store the processed data.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatastoreActivity {
    /// <p>The name of the data store where processed messages are stored.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    /// <p>The name of the 'datastore' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
}

/// <p>Statistical information about the data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatastoreStatistics {
    /// <p>The estimated size of the data store.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<EstimatedResourceSize>,
}

/// <p>Where data store data is stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatastoreStorage {
    /// <p>Use this to store data store data in an S3 bucket that you manage.</p>
    #[serde(rename = "customerManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3: Option<CustomerManagedDatastoreS3Storage>,
    /// <p>Use this to store data store data in an S3 bucket managed by the AWS IoT Analytics service.</p>
    #[serde(rename = "serviceManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_s3: Option<ServiceManagedDatastoreS3Storage>,
}

/// <p>Where data store data is stored.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatastoreStorageSummary {
    /// <p>Used to store data store data in an S3 bucket that you manage.</p>
    #[serde(rename = "customerManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_s3: Option<CustomerManagedDatastoreS3StorageSummary>,
    /// <p>Used to store data store data in an S3 bucket managed by the AWS IoT Analytics service.</p>
    #[serde(rename = "serviceManagedS3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_managed_s3: Option<ServiceManagedDatastoreS3StorageSummary>,
}

/// <p>A summary of information about a data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DatastoreSummary {
    /// <p>When the data store was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the data store.</p>
    #[serde(rename = "datastoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,
    /// <p>Where data store data is stored.</p>
    #[serde(rename = "datastoreStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_storage: Option<DatastoreStorageSummary>,
    /// <p>The last time the data store was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The status of the data store.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteChannelRequest {
    /// <p>The name of the channel to delete.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatasetContentRequest {
    /// <p>The name of the data set whose content is deleted.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>The version of the data set whose content is deleted. You can also use the strings "$LATEST" or "$LATEST_SUCCEEDED" to delete the latest or latest successfully completed data set. If not specified, "$LATEST_SUCCEEDED" is the default.</p>
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatasetRequest {
    /// <p>The name of the data set to delete.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatastoreRequest {
    /// <p>The name of the data store to delete.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeletePipelineRequest {
    /// <p>The name of the pipeline to delete.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>Used to limit data to that which has arrived since the last execution of the action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeltaTime {
    /// <p>The number of seconds of estimated "in flight" lag time of message data. When you create data set contents using message data from a specified time frame, some message data may still be "in flight" when processing begins, and so will not arrive in time to be processed. Use this field to make allowances for the "in flight" time of your message data, so that data not processed from a previous time frame will be included with the next time frame. Without this, missed message data would be excluded from processing during the next time frame as well, because its timestamp places it within the previous time frame.</p>
    #[serde(rename = "offsetSeconds")]
    pub offset_seconds: i64,
    /// <p>An expression by which the time of the message data may be determined. This may be the name of a timestamp field, or a SQL expression which is used to derive the time the message data was generated.</p>
    #[serde(rename = "timeExpression")]
    pub time_expression: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeChannelRequest {
    /// <p>The name of the channel whose information is retrieved.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>If true, additional statistical information about the channel is included in the response.</p>
    #[serde(rename = "includeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_statistics: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeChannelResponse {
    /// <p>An object that contains information about the channel.</p>
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<Channel>,
    /// <p>Statistics about the channel. Included if the 'includeStatistics' parameter is set to true in the request.</p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<ChannelStatistics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDatasetRequest {
    /// <p>The name of the data set whose information is retrieved.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeDatasetResponse {
    /// <p>An object that contains information about the data set.</p>
    #[serde(rename = "dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDatastoreRequest {
    /// <p>The name of the data store</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    /// <p>If true, additional statistical information about the datastore is included in the response.</p>
    #[serde(rename = "includeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_statistics: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeDatastoreResponse {
    /// <p>Information about the data store.</p>
    #[serde(rename = "datastore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<Datastore>,
    /// <p>Additional statistical information about the data store. Included if the 'includeStatistics' parameter is set to true in the request.</p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatastoreStatistics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLoggingOptionsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribeLoggingOptionsResponse {
    /// <p>The current settings of the AWS IoT Analytics logging options.</p>
    #[serde(rename = "loggingOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_options: Option<LoggingOptions>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribePipelineRequest {
    /// <p>The name of the pipeline whose information is retrieved.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct DescribePipelineResponse {
    /// <p>A "Pipeline" object that contains information about the pipeline.</p>
    #[serde(rename = "pipeline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<Pipeline>,
}

/// <p>An activity that adds data from the AWS IoT device registry to your message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRegistryEnrichActivity {
    /// <p>The name of the attribute that is added to the message.</p>
    #[serde(rename = "attribute")]
    pub attribute: String,
    /// <p>The name of the 'deviceRegistryEnrich' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// <p>The ARN of the role that allows access to the device's registry information.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The name of the IoT device whose registry information is added to the message.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>An activity that adds information from the AWS IoT Device Shadows service to a message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceShadowEnrichActivity {
    /// <p>The name of the attribute that is added to the message.</p>
    #[serde(rename = "attribute")]
    pub attribute: String,
    /// <p>The name of the 'deviceShadowEnrich' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
    /// <p>The ARN of the role that allows access to the device's shadow.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>The name of the IoT device whose shadow information is added to the message.</p>
    #[serde(rename = "thingName")]
    pub thing_name: String,
}

/// <p>The estimated size of the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct EstimatedResourceSize {
    /// <p>The time when the estimate of the size of the resource was made.</p>
    #[serde(rename = "estimatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_on: Option<f64>,
    /// <p>The estimated size of the resource in bytes.</p>
    #[serde(rename = "estimatedSizeInBytes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_size_in_bytes: Option<f64>,
}

/// <p>An activity that filters a message based on its attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterActivity {
    /// <p>An expression that looks like a SQL WHERE clause that must return a Boolean value.</p>
    #[serde(rename = "filter")]
    pub filter: String,
    /// <p>The name of the 'filter' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetDatasetContentRequest {
    /// <p>The name of the data set whose contents are retrieved.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>The version of the data set whose contents are retrieved. You can also use the strings "$LATEST" or "$LATEST_SUCCEEDED" to retrieve the contents of the latest or latest successfully completed data set. If not specified, "$LATEST_SUCCEEDED" is the default.</p>
    #[serde(rename = "versionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct GetDatasetContentResponse {
    /// <p>A list of "DatasetEntry" objects.</p>
    #[serde(rename = "entries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<DatasetEntry>>,
    /// <p>The status of the data set content.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DatasetContentStatus>,
    /// <p>The time when the request was made.</p>
    #[serde(rename = "timestamp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<f64>,
}

/// <p>Configuration information for coordination with the AWS Glue ETL (extract, transform and load) service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlueConfiguration {
    /// <p>The name of the database in your AWS Glue Data Catalog in which the table is located. (An AWS Glue Data Catalog database contains Glue Data tables.)</p>
    #[serde(rename = "databaseName")]
    pub database_name: String,
    /// <p>The name of the table in your AWS Glue Data Catalog which is used to perform the ETL (extract, transform and load) operations. (An AWS Glue Data Catalog table contains partitioned data and descriptions of data sources and targets.)</p>
    #[serde(rename = "tableName")]
    pub table_name: String,
}

/// <p>Configuration information for delivery of data set contents to AWS IoT Events.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IotEventsDestinationConfiguration {
    /// <p>The name of the AWS IoT Events input to which data set contents are delivered.</p>
    #[serde(rename = "inputName")]
    pub input_name: String,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to deliver data set contents to an AWS IoT Events input.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>An activity that runs a Lambda function to modify the message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaActivity {
    /// <p>The number of messages passed to the Lambda function for processing.</p> <p>The AWS Lambda function must be able to process all of these messages within five minutes, which is the maximum timeout duration for Lambda functions.</p>
    #[serde(rename = "batchSize")]
    pub batch_size: i64,
    /// <p>The name of the Lambda function that is run on the message.</p>
    #[serde(rename = "lambdaName")]
    pub lambda_name: String,
    /// <p>The name of the 'lambda' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListChannelsRequest {
    /// <p>The maximum number of results to return in this request.</p> <p>The default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListChannelsResponse {
    /// <p>A list of "ChannelSummary" objects.</p>
    #[serde(rename = "channelSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_summaries: Option<Vec<ChannelSummary>>,
    /// <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDatasetContentsRequest {
    /// <p>The name of the data set whose contents information you want to list.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>The maximum number of results to return in this request.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A filter to limit results to those data set contents whose creation is scheduled before the given time. See the field <code>triggers.schedule</code> in the CreateDataset request. (timestamp)</p>
    #[serde(rename = "scheduledBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_before: Option<f64>,
    /// <p>A filter to limit results to those data set contents whose creation is scheduled on or after the given time. See the field <code>triggers.schedule</code> in the CreateDataset request. (timestamp)</p>
    #[serde(rename = "scheduledOnOrAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_on_or_after: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListDatasetContentsResponse {
    /// <p>Summary information about data set contents that have been created.</p>
    #[serde(rename = "datasetContentSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_content_summaries: Option<Vec<DatasetContentSummary>>,
    /// <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDatasetsRequest {
    /// <p>The maximum number of results to return in this request.</p> <p>The default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListDatasetsResponse {
    /// <p>A list of "DatasetSummary" objects.</p>
    #[serde(rename = "datasetSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_summaries: Option<Vec<DatasetSummary>>,
    /// <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDatastoresRequest {
    /// <p>The maximum number of results to return in this request.</p> <p>The default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListDatastoresResponse {
    /// <p>A list of "DatastoreSummary" objects.</p>
    #[serde(rename = "datastoreSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_summaries: Option<Vec<DatastoreSummary>>,
    /// <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListPipelinesRequest {
    /// <p>The maximum number of results to return in this request.</p> <p>The default value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListPipelinesResponse {
    /// <p>The token to retrieve the next set of results, or <code>null</code> if there are no more results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of "PipelineSummary" objects.</p>
    #[serde(rename = "pipelineSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_summaries: Option<Vec<PipelineSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of the resource whose tags you want to list.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags (metadata) which you have assigned to the resource.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>Information about logging options.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LoggingOptions {
    /// <p>If true, logging is enabled for AWS IoT Analytics.</p>
    #[serde(rename = "enabled")]
    pub enabled: bool,
    /// <p>The logging level. Currently, only "ERROR" is supported.</p>
    #[serde(rename = "level")]
    pub level: String,
    /// <p>The ARN of the role that grants permission to AWS IoT Analytics to perform logging.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

/// <p>An activity that computes an arithmetic expression using the message's attributes.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MathActivity {
    /// <p>The name of the attribute that contains the result of the math operation.</p>
    #[serde(rename = "attribute")]
    pub attribute: String,
    /// <p>An expression that uses one or more existing attributes and must return an integer value.</p>
    #[serde(rename = "math")]
    pub math: String,
    /// <p>The name of the 'math' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// <p>Information about a message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct Message {
    /// <p>The ID you wish to assign to the message. Each "messageId" must be unique within each batch sent.</p>
    #[serde(rename = "messageId")]
    pub message_id: String,
    /// <p>The payload of the message. This may be a JSON string or a Base64-encoded string representing binary data (in which case you must decode it by means of a pipeline activity).</p>
    #[serde(rename = "payload")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub payload: bytes::Bytes,
}

/// <p>The value of the variable as a structure that specifies an output file URI.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutputFileUriValue {
    /// <p>The URI of the location where data set contents are stored, usually the URI of a file in an S3 bucket.</p>
    #[serde(rename = "fileName")]
    pub file_name: String,
}

/// <p>Contains information about a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct Pipeline {
    /// <p>The activities that perform transformations on the messages.</p>
    #[serde(rename = "activities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<PipelineActivity>>,
    /// <p>The ARN of the pipeline.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>When the pipeline was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The last time the pipeline was updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>A summary of information about the pipeline reprocessing.</p>
    #[serde(rename = "reprocessingSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprocessing_summaries: Option<Vec<ReprocessingSummary>>,
}

/// <p>An activity that performs a transformation on a message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PipelineActivity {
    /// <p>Adds other attributes based on existing attributes in the message.</p>
    #[serde(rename = "addAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_attributes: Option<AddAttributesActivity>,
    /// <p>Determines the source of the messages to be processed.</p>
    #[serde(rename = "channel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<ChannelActivity>,
    /// <p>Specifies where to store the processed message data.</p>
    #[serde(rename = "datastore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<DatastoreActivity>,
    /// <p>Adds data from the AWS IoT device registry to your message.</p>
    #[serde(rename = "deviceRegistryEnrich")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_registry_enrich: Option<DeviceRegistryEnrichActivity>,
    /// <p>Adds information from the AWS IoT Device Shadows service to a message.</p>
    #[serde(rename = "deviceShadowEnrich")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_shadow_enrich: Option<DeviceShadowEnrichActivity>,
    /// <p>Filters a message based on its attributes.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FilterActivity>,
    /// <p>Runs a Lambda function to modify the message.</p>
    #[serde(rename = "lambda")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda: Option<LambdaActivity>,
    /// <p>Computes an arithmetic expression using the message's attributes and adds it to the message.</p>
    #[serde(rename = "math")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub math: Option<MathActivity>,
    /// <p>Removes attributes from a message.</p>
    #[serde(rename = "removeAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_attributes: Option<RemoveAttributesActivity>,
    /// <p>Creates a new message using only the specified attributes from the original message. </p>
    #[serde(rename = "selectAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_attributes: Option<SelectAttributesActivity>,
}

/// <p>A summary of information about a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct PipelineSummary {
    /// <p>When the pipeline was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>When the pipeline was last updated.</p>
    #[serde(rename = "lastUpdateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<f64>,
    /// <p>The name of the pipeline.</p>
    #[serde(rename = "pipelineName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_name: Option<String>,
    /// <p>A summary of information about the pipeline reprocessing.</p>
    #[serde(rename = "reprocessingSummaries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprocessing_summaries: Option<Vec<ReprocessingSummary>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutLoggingOptionsRequest {
    /// <p>The new values of the AWS IoT Analytics logging options.</p>
    #[serde(rename = "loggingOptions")]
    pub logging_options: LoggingOptions,
}

/// <p>Information which is used to filter message data, to segregate it according to the time frame in which it arrives.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryFilter {
    /// <p>Used to limit data to that which has arrived since the last execution of the action.</p>
    #[serde(rename = "deltaTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_time: Option<DeltaTime>,
}

/// <p>An activity that removes attributes from a message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RemoveAttributesActivity {
    /// <p>A list of 1-50 attributes to remove from the message.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
    /// <p>The name of the 'removeAttributes' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// <p>Information about pipeline reprocessing.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ReprocessingSummary {
    /// <p>The time the pipeline reprocessing was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The 'reprocessingId' returned by "StartPipelineReprocessing".</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The status of the pipeline reprocessing.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p>The configuration of the resource used to execute the "containerAction".</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceConfiguration {
    /// <p>The type of the compute resource used to execute the "containerAction". Possible values are: ACU_1 (vCPU=4, memory=16GiB) or ACU_2 (vCPU=8, memory=32GiB).</p>
    #[serde(rename = "computeType")]
    pub compute_type: String,
    /// <p>The size (in GB) of the persistent storage available to the resource instance used to execute the "containerAction" (min: 1, max: 50).</p>
    #[serde(rename = "volumeSizeInGB")]
    pub volume_size_in_gb: i64,
}

/// <p>How long, in days, message data is kept.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RetentionPeriod {
    /// <p>The number of days that message data is kept. The "unlimited" parameter must be false.</p>
    #[serde(rename = "numberOfDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_days: Option<i64>,
    /// <p>If true, message data is kept indefinitely.</p>
    #[serde(rename = "unlimited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RunPipelineActivityRequest {
    /// <p>The sample message payloads on which the pipeline activity is run.</p>
    #[serde(rename = "payloads")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlobList::deserialize_blob_list",
        serialize_with = "::rusoto_core::serialization::SerdeBlobList::serialize_blob_list",
        default
    )]
    pub payloads: Vec<bytes::Bytes>,
    /// <p>The pipeline activity that is run. This must not be a 'channel' activity or a 'datastore' activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a 'lambda' activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    #[serde(rename = "pipelineActivity")]
    pub pipeline_activity: PipelineActivity,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct RunPipelineActivityResponse {
    /// <p>In case the pipeline activity fails, the log message that is generated.</p>
    #[serde(rename = "logResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_result: Option<String>,
    /// <p>The enriched or transformed sample message payloads as base64-encoded strings. (The results of running the pipeline activity on each input sample message payload, encoded in base64.)</p>
    #[serde(rename = "payloads")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlobList::deserialize_blob_list",
        serialize_with = "::rusoto_core::serialization::SerdeBlobList::serialize_blob_list",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<bytes::Bytes>>,
}

/// <p>Configuration information for delivery of data set contents to Amazon S3.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3DestinationConfiguration {
    /// <p>The name of the Amazon S3 bucket to which data set contents are delivered.</p>
    #[serde(rename = "bucket")]
    pub bucket: String,
    /// <p>Configuration information for coordination with the AWS Glue ETL (extract, transform and load) service.</p>
    #[serde(rename = "glueConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glue_configuration: Option<GlueConfiguration>,
    /// <p>The key of the data set contents object. Each object in an Amazon S3 bucket has a key that is its unique identifier within the bucket (each object in a bucket has exactly one key).</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The ARN of the role which grants AWS IoT Analytics permission to interact with your Amazon S3 and AWS Glue resources.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SampleChannelDataRequest {
    /// <p>The name of the channel whose message samples are retrieved.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>The end of the time window from which sample messages are retrieved.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The number of sample messages to be retrieved. The limit is 10, the default is also 10.</p>
    #[serde(rename = "maxMessages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_messages: Option<i64>,
    /// <p>The start of the time window from which sample messages are retrieved.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct SampleChannelDataResponse {
    /// <p>The list of message samples. Each sample message is returned as a base64-encoded string.</p>
    #[serde(rename = "payloads")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlobList::deserialize_blob_list",
        serialize_with = "::rusoto_core::serialization::SerdeBlobList::serialize_blob_list",
        default
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<bytes::Bytes>>,
}

/// <p>The schedule for when to trigger an update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// <p>The expression that defines when to trigger an update. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html"> Schedule Expressions for Rules</a> in the Amazon CloudWatch Events User Guide.</p>
    #[serde(rename = "expression")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

/// <p>Creates a new message using only the specified attributes from the original message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectAttributesActivity {
    /// <p>A list of the attributes to select from the message.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<String>,
    /// <p>The name of the 'selectAttributes' activity.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The next activity in the pipeline.</p>
    #[serde(rename = "next")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

/// <p>Use this to store channel data in an S3 bucket managed by the AWS IoT Analytics service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceManagedChannelS3Storage {}

/// <p>Used to store channel data in an S3 bucket managed by the AWS IoT Analytics service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ServiceManagedChannelS3StorageSummary {}

/// <p>Use this to store data store data in an S3 bucket managed by the AWS IoT Analytics service.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServiceManagedDatastoreS3Storage {}

/// <p>Used to store data store data in an S3 bucket managed by the AWS IoT Analytics service.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct ServiceManagedDatastoreS3StorageSummary {}

/// <p>The SQL query to modify the message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqlQueryDatasetAction {
    /// <p>Pre-filters applied to message data.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<QueryFilter>>,
    /// <p>A SQL query string.</p>
    #[serde(rename = "sqlQuery")]
    pub sql_query: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartPipelineReprocessingRequest {
    /// <p>The end time (exclusive) of raw message data that is reprocessed.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The name of the pipeline on which to start reprocessing.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
    /// <p>The start time (inclusive) of raw message data that is reprocessed.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct StartPipelineReprocessingResponse {
    /// <p>The ID of the pipeline reprocessing activity that was started.</p>
    #[serde(rename = "reprocessingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reprocessing_id: Option<String>,
}

/// <p>A set of key/value pairs which are used to manage the resource.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>The tag's key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The tag's value.</p>
    #[serde(rename = "value")]
    pub value: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TagResourceRequest {
    /// <p>The ARN of the resource whose tags you want to modify.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The new or modified tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct TagResourceResponse {}

/// <p>Information about the data set whose content generation triggers the new data set content generation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TriggeringDataset {
    /// <p>The name of the data set whose content generation triggers the new data set content generation.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource whose tags you want to remove.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of those tags which you want to remove.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, serialize_structs), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateChannelRequest {
    /// <p>The name of the channel to be updated.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>Where channel data is stored.</p>
    #[serde(rename = "channelStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_storage: Option<ChannelStorage>,
    /// <p>How long, in days, message data is kept for the channel.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatasetRequest {
    /// <p>A list of "DatasetAction" objects.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<DatasetAction>,
    /// <p>When data set contents are created they are delivered to destinations specified here.</p>
    #[serde(rename = "contentDeliveryRules")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_delivery_rules: Option<Vec<DatasetContentDeliveryRule>>,
    /// <p>The name of the data set to update.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>How long, in days, data set contents are kept for the data set.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
    /// <p>A list of "DatasetTrigger" objects. The list can be empty or can contain up to five <b>DataSetTrigger</b> objects.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
    /// <p>[Optional] How many versions of data set contents are kept. If not specified or set to null, only the latest version plus the latest succeeded version (if they are different) are kept for the time period specified by the "retentionPeriod" parameter. (For more information, see https://docs.aws.amazon.com/iotanalytics/latest/userguide/getting-started.html#aws-iot-analytics-dataset-versions)</p>
    #[serde(rename = "versioningConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioning_configuration: Option<VersioningConfiguration>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatastoreRequest {
    /// <p>The name of the data store to be updated.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    /// <p>Where data store data is stored.</p>
    #[serde(rename = "datastoreStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_storage: Option<DatastoreStorage>,
    /// <p>How long, in days, message data is kept for the data store.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePipelineRequest {
    /// <p>A list of "PipelineActivity" objects. Activities perform transformations on your messages, such as removing, renaming or adding message attributes; filtering messages based on attribute values; invoking your Lambda functions on messages for advanced processing; or performing mathematical transformations to normalize device data.</p> <p>The list can be 2-25 <b>PipelineActivity</b> objects and must contain both a <code>channel</code> and a <code>datastore</code> activity. Each entry in the list must contain only one activity, for example:</p> <p> <code>pipelineActivities = [ { "channel": { ... } }, { "lambda": { ... } }, ... ]</code> </p>
    #[serde(rename = "pipelineActivities")]
    pub pipeline_activities: Vec<PipelineActivity>,
    /// <p>The name of the pipeline to update.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
}

/// <p>An instance of a variable to be passed to the "containerAction" execution. Each variable must have a name and a value given by one of "stringValue", "datasetContentVersionValue", or "outputFileUriValue".</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    /// <p>The value of the variable as a structure that specifies a data set content version.</p>
    #[serde(rename = "datasetContentVersionValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_content_version_value: Option<DatasetContentVersionValue>,
    /// <p>The value of the variable as a double (numeric).</p>
    #[serde(rename = "doubleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub double_value: Option<f64>,
    /// <p>The name of the variable.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The value of the variable as a structure that specifies an output file URI.</p>
    #[serde(rename = "outputFileUriValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_file_uri_value: Option<OutputFileUriValue>,
    /// <p>The value of the variable as a string.</p>
    #[serde(rename = "stringValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string_value: Option<String>,
}

/// <p>Information about the versioning of data set contents.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VersioningConfiguration {
    /// <p>How many versions of data set contents will be kept. The "unlimited" parameter must be false.</p>
    #[serde(rename = "maxVersions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_versions: Option<i64>,
    /// <p>If true, unlimited versions of data set contents will be kept.</p>
    #[serde(rename = "unlimited")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited: Option<bool>,
}

/// Errors returned by BatchPutMessage
#[derive(Debug, PartialEq)]
pub enum BatchPutMessageError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl BatchPutMessageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchPutMessageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(BatchPutMessageError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(BatchPutMessageError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BatchPutMessageError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(BatchPutMessageError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(BatchPutMessageError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BatchPutMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BatchPutMessageError {
    fn description(&self) -> &str {
        match *self {
            BatchPutMessageError::InternalFailure(ref cause) => cause,
            BatchPutMessageError::InvalidRequest(ref cause) => cause,
            BatchPutMessageError::ResourceNotFound(ref cause) => cause,
            BatchPutMessageError::ServiceUnavailable(ref cause) => cause,
            BatchPutMessageError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CancelPipelineReprocessing
#[derive(Debug, PartialEq)]
pub enum CancelPipelineReprocessingError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CancelPipelineReprocessingError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<CancelPipelineReprocessingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CancelPipelineReprocessingError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CancelPipelineReprocessingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CancelPipelineReprocessingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        CancelPipelineReprocessingError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CancelPipelineReprocessingError::Throttling(
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
impl fmt::Display for CancelPipelineReprocessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CancelPipelineReprocessingError {
    fn description(&self) -> &str {
        match *self {
            CancelPipelineReprocessingError::InternalFailure(ref cause) => cause,
            CancelPipelineReprocessingError::InvalidRequest(ref cause) => cause,
            CancelPipelineReprocessingError::ResourceNotFound(ref cause) => cause,
            CancelPipelineReprocessingError::ServiceUnavailable(ref cause) => cause,
            CancelPipelineReprocessingError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateChannel
#[derive(Debug, PartialEq)]
pub enum CreateChannelError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the same name already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateChannelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateChannelError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateChannelError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateChannelError::ResourceAlreadyExists(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateChannelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateChannelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateChannelError {
    fn description(&self) -> &str {
        match *self {
            CreateChannelError::InternalFailure(ref cause) => cause,
            CreateChannelError::InvalidRequest(ref cause) => cause,
            CreateChannelError::LimitExceeded(ref cause) => cause,
            CreateChannelError::ResourceAlreadyExists(ref cause) => cause,
            CreateChannelError::ServiceUnavailable(ref cause) => cause,
            CreateChannelError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDataset
#[derive(Debug, PartialEq)]
pub enum CreateDatasetError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the same name already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDatasetError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDatasetError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDatasetError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDatasetError::ResourceAlreadyExists(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDatasetError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDatasetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatasetError {
    fn description(&self) -> &str {
        match *self {
            CreateDatasetError::InternalFailure(ref cause) => cause,
            CreateDatasetError::InvalidRequest(ref cause) => cause,
            CreateDatasetError::LimitExceeded(ref cause) => cause,
            CreateDatasetError::ResourceAlreadyExists(ref cause) => cause,
            CreateDatasetError::ServiceUnavailable(ref cause) => cause,
            CreateDatasetError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDatasetContent
#[derive(Debug, PartialEq)]
pub enum CreateDatasetContentError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateDatasetContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatasetContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDatasetContentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDatasetContentError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateDatasetContentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDatasetContentError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDatasetContentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDatasetContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatasetContentError {
    fn description(&self) -> &str {
        match *self {
            CreateDatasetContentError::InternalFailure(ref cause) => cause,
            CreateDatasetContentError::InvalidRequest(ref cause) => cause,
            CreateDatasetContentError::ResourceNotFound(ref cause) => cause,
            CreateDatasetContentError::ServiceUnavailable(ref cause) => cause,
            CreateDatasetContentError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateDatastore
#[derive(Debug, PartialEq)]
pub enum CreateDatastoreError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the same name already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreateDatastoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateDatastoreError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreateDatastoreError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreateDatastoreError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateDatastoreError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateDatastoreError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreateDatastoreError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreateDatastoreError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateDatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateDatastoreError {
    fn description(&self) -> &str {
        match *self {
            CreateDatastoreError::InternalFailure(ref cause) => cause,
            CreateDatastoreError::InvalidRequest(ref cause) => cause,
            CreateDatastoreError::LimitExceeded(ref cause) => cause,
            CreateDatastoreError::ResourceAlreadyExists(ref cause) => cause,
            CreateDatastoreError::ServiceUnavailable(ref cause) => cause,
            CreateDatastoreError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by CreatePipeline
#[derive(Debug, PartialEq)]
pub enum CreatePipelineError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the same name already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl CreatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreatePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(CreatePipelineError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(CreatePipelineError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreatePipelineError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreatePipelineError::ResourceAlreadyExists(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(CreatePipelineError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(CreatePipelineError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreatePipelineError {
    fn description(&self) -> &str {
        match *self {
            CreatePipelineError::InternalFailure(ref cause) => cause,
            CreatePipelineError::InvalidRequest(ref cause) => cause,
            CreatePipelineError::LimitExceeded(ref cause) => cause,
            CreatePipelineError::ResourceAlreadyExists(ref cause) => cause,
            CreatePipelineError::ServiceUnavailable(ref cause) => cause,
            CreatePipelineError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteChannel
#[derive(Debug, PartialEq)]
pub enum DeleteChannelError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteChannelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteChannelError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteChannelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteChannelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteChannelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteChannelError {
    fn description(&self) -> &str {
        match *self {
            DeleteChannelError::InternalFailure(ref cause) => cause,
            DeleteChannelError::InvalidRequest(ref cause) => cause,
            DeleteChannelError::ResourceNotFound(ref cause) => cause,
            DeleteChannelError::ServiceUnavailable(ref cause) => cause,
            DeleteChannelError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDataset
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDatasetError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDatasetError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDatasetError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDatasetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatasetError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatasetError::InternalFailure(ref cause) => cause,
            DeleteDatasetError::InvalidRequest(ref cause) => cause,
            DeleteDatasetError::ResourceNotFound(ref cause) => cause,
            DeleteDatasetError::ServiceUnavailable(ref cause) => cause,
            DeleteDatasetError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDatasetContent
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetContentError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteDatasetContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDatasetContentError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDatasetContentError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetContentError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDatasetContentError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDatasetContentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatasetContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatasetContentError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatasetContentError::InternalFailure(ref cause) => cause,
            DeleteDatasetContentError::InvalidRequest(ref cause) => cause,
            DeleteDatasetContentError::ResourceNotFound(ref cause) => cause,
            DeleteDatasetContentError::ServiceUnavailable(ref cause) => cause,
            DeleteDatasetContentError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDatastore
#[derive(Debug, PartialEq)]
pub enum DeleteDatastoreError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeleteDatastoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatastoreError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeleteDatastoreError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeleteDatastoreError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatastoreError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeleteDatastoreError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeleteDatastoreError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteDatastoreError {
    fn description(&self) -> &str {
        match *self {
            DeleteDatastoreError::InternalFailure(ref cause) => cause,
            DeleteDatastoreError::InvalidRequest(ref cause) => cause,
            DeleteDatastoreError::ResourceNotFound(ref cause) => cause,
            DeleteDatastoreError::ServiceUnavailable(ref cause) => cause,
            DeleteDatastoreError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DeletePipeline
#[derive(Debug, PartialEq)]
pub enum DeletePipelineError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DeletePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeletePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DeletePipelineError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DeletePipelineError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeletePipelineError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DeletePipelineError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DeletePipelineError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeletePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeletePipelineError {
    fn description(&self) -> &str {
        match *self {
            DeletePipelineError::InternalFailure(ref cause) => cause,
            DeletePipelineError::InvalidRequest(ref cause) => cause,
            DeletePipelineError::ResourceNotFound(ref cause) => cause,
            DeletePipelineError::ServiceUnavailable(ref cause) => cause,
            DeletePipelineError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeChannel
#[derive(Debug, PartialEq)]
pub enum DescribeChannelError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeChannelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeChannelError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeChannelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeChannelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeChannelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeChannelError {
    fn description(&self) -> &str {
        match *self {
            DescribeChannelError::InternalFailure(ref cause) => cause,
            DescribeChannelError::InvalidRequest(ref cause) => cause,
            DescribeChannelError::ResourceNotFound(ref cause) => cause,
            DescribeChannelError::ServiceUnavailable(ref cause) => cause,
            DescribeChannelError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDataset
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDatasetError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDatasetError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatasetError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDatasetError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDatasetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDatasetError {
    fn description(&self) -> &str {
        match *self {
            DescribeDatasetError::InternalFailure(ref cause) => cause,
            DescribeDatasetError::InvalidRequest(ref cause) => cause,
            DescribeDatasetError::ResourceNotFound(ref cause) => cause,
            DescribeDatasetError::ServiceUnavailable(ref cause) => cause,
            DescribeDatasetError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDatastore
#[derive(Debug, PartialEq)]
pub enum DescribeDatastoreError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeDatastoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatastoreError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeDatastoreError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeDatastoreError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatastoreError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeDatastoreError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeDatastoreError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeDatastoreError {
    fn description(&self) -> &str {
        match *self {
            DescribeDatastoreError::InternalFailure(ref cause) => cause,
            DescribeDatastoreError::InvalidRequest(ref cause) => cause,
            DescribeDatastoreError::ResourceNotFound(ref cause) => cause,
            DescribeDatastoreError::ServiceUnavailable(ref cause) => cause,
            DescribeDatastoreError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeLoggingOptions
#[derive(Debug, PartialEq)]
pub enum DescribeLoggingOptionsError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribeLoggingOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLoggingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribeLoggingOptionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeLoggingOptionsError::InternalFailure(ref cause) => cause,
            DescribeLoggingOptionsError::InvalidRequest(ref cause) => cause,
            DescribeLoggingOptionsError::ResourceNotFound(ref cause) => cause,
            DescribeLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            DescribeLoggingOptionsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribePipeline
#[derive(Debug, PartialEq)]
pub enum DescribePipelineError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl DescribePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(DescribePipelineError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(DescribePipelineError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribePipelineError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(DescribePipelineError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(DescribePipelineError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribePipelineError {
    fn description(&self) -> &str {
        match *self {
            DescribePipelineError::InternalFailure(ref cause) => cause,
            DescribePipelineError::InvalidRequest(ref cause) => cause,
            DescribePipelineError::ResourceNotFound(ref cause) => cause,
            DescribePipelineError::ServiceUnavailable(ref cause) => cause,
            DescribePipelineError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDatasetContent
#[derive(Debug, PartialEq)]
pub enum GetDatasetContentError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl GetDatasetContentError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDatasetContentError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(GetDatasetContentError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(GetDatasetContentError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDatasetContentError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(GetDatasetContentError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(GetDatasetContentError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetDatasetContentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDatasetContentError {
    fn description(&self) -> &str {
        match *self {
            GetDatasetContentError::InternalFailure(ref cause) => cause,
            GetDatasetContentError::InvalidRequest(ref cause) => cause,
            GetDatasetContentError::ResourceNotFound(ref cause) => cause,
            GetDatasetContentError::ServiceUnavailable(ref cause) => cause,
            GetDatasetContentError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListChannels
#[derive(Debug, PartialEq)]
pub enum ListChannelsError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListChannelsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChannelsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListChannelsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListChannelsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListChannelsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListChannelsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListChannelsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListChannelsError {
    fn description(&self) -> &str {
        match *self {
            ListChannelsError::InternalFailure(ref cause) => cause,
            ListChannelsError::InvalidRequest(ref cause) => cause,
            ListChannelsError::ServiceUnavailable(ref cause) => cause,
            ListChannelsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDatasetContents
#[derive(Debug, PartialEq)]
pub enum ListDatasetContentsError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListDatasetContentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetContentsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDatasetContentsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDatasetContentsError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListDatasetContentsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDatasetContentsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDatasetContentsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDatasetContentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDatasetContentsError {
    fn description(&self) -> &str {
        match *self {
            ListDatasetContentsError::InternalFailure(ref cause) => cause,
            ListDatasetContentsError::InvalidRequest(ref cause) => cause,
            ListDatasetContentsError::ResourceNotFound(ref cause) => cause,
            ListDatasetContentsError::ServiceUnavailable(ref cause) => cause,
            ListDatasetContentsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDatasets
#[derive(Debug, PartialEq)]
pub enum ListDatasetsError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListDatasetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDatasetsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDatasetsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDatasetsError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDatasetsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDatasetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDatasetsError {
    fn description(&self) -> &str {
        match *self {
            ListDatasetsError::InternalFailure(ref cause) => cause,
            ListDatasetsError::InvalidRequest(ref cause) => cause,
            ListDatasetsError::ServiceUnavailable(ref cause) => cause,
            ListDatasetsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDatastores
#[derive(Debug, PartialEq)]
pub enum ListDatastoresError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListDatastoresError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatastoresError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListDatastoresError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListDatastoresError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListDatastoresError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListDatastoresError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDatastoresError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListDatastoresError {
    fn description(&self) -> &str {
        match *self {
            ListDatastoresError::InternalFailure(ref cause) => cause,
            ListDatastoresError::InvalidRequest(ref cause) => cause,
            ListDatastoresError::ServiceUnavailable(ref cause) => cause,
            ListDatastoresError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListPipelines
#[derive(Debug, PartialEq)]
pub enum ListPipelinesError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListPipelinesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListPipelinesError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListPipelinesError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListPipelinesError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListPipelinesError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListPipelinesError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListPipelinesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListPipelinesError {
    fn description(&self) -> &str {
        match *self {
            ListPipelinesError::InternalFailure(ref cause) => cause,
            ListPipelinesError::InvalidRequest(ref cause) => cause,
            ListPipelinesError::ServiceUnavailable(ref cause) => cause,
            ListPipelinesError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(ListTagsForResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListTagsForResourceError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(ListTagsForResourceError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(ListTagsForResourceError::Throttling(err.msg))
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
            ListTagsForResourceError::InternalFailure(ref cause) => cause,
            ListTagsForResourceError::InvalidRequest(ref cause) => cause,
            ListTagsForResourceError::LimitExceeded(ref cause) => cause,
            ListTagsForResourceError::ResourceNotFound(ref cause) => cause,
            ListTagsForResourceError::ServiceUnavailable(ref cause) => cause,
            ListTagsForResourceError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by PutLoggingOptions
#[derive(Debug, PartialEq)]
pub enum PutLoggingOptionsError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl PutLoggingOptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutLoggingOptionsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(PutLoggingOptionsError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(PutLoggingOptionsError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(PutLoggingOptionsError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(PutLoggingOptionsError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PutLoggingOptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutLoggingOptionsError {
    fn description(&self) -> &str {
        match *self {
            PutLoggingOptionsError::InternalFailure(ref cause) => cause,
            PutLoggingOptionsError::InvalidRequest(ref cause) => cause,
            PutLoggingOptionsError::ServiceUnavailable(ref cause) => cause,
            PutLoggingOptionsError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by RunPipelineActivity
#[derive(Debug, PartialEq)]
pub enum RunPipelineActivityError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl RunPipelineActivityError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RunPipelineActivityError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(RunPipelineActivityError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(RunPipelineActivityError::InvalidRequest(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(RunPipelineActivityError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(RunPipelineActivityError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RunPipelineActivityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RunPipelineActivityError {
    fn description(&self) -> &str {
        match *self {
            RunPipelineActivityError::InternalFailure(ref cause) => cause,
            RunPipelineActivityError::InvalidRequest(ref cause) => cause,
            RunPipelineActivityError::ServiceUnavailable(ref cause) => cause,
            RunPipelineActivityError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by SampleChannelData
#[derive(Debug, PartialEq)]
pub enum SampleChannelDataError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl SampleChannelDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SampleChannelDataError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(SampleChannelDataError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(SampleChannelDataError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SampleChannelDataError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(SampleChannelDataError::ServiceUnavailable(
                        err.msg,
                    ))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(SampleChannelDataError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SampleChannelDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SampleChannelDataError {
    fn description(&self) -> &str {
        match *self {
            SampleChannelDataError::InternalFailure(ref cause) => cause,
            SampleChannelDataError::InvalidRequest(ref cause) => cause,
            SampleChannelDataError::ResourceNotFound(ref cause) => cause,
            SampleChannelDataError::ServiceUnavailable(ref cause) => cause,
            SampleChannelDataError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by StartPipelineReprocessing
#[derive(Debug, PartialEq)]
pub enum StartPipelineReprocessingError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the same name already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl StartPipelineReprocessingError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartPipelineReprocessingError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(StartPipelineReprocessingError::InternalFailure(
                        err.msg,
                    ))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(StartPipelineReprocessingError::InvalidRequest(
                        err.msg,
                    ))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(
                        StartPipelineReprocessingError::ResourceAlreadyExists(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartPipelineReprocessingError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(
                        StartPipelineReprocessingError::ServiceUnavailable(err.msg),
                    )
                }
                "ThrottlingException" => {
                    return RusotoError::Service(StartPipelineReprocessingError::Throttling(
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
impl fmt::Display for StartPipelineReprocessingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartPipelineReprocessingError {
    fn description(&self) -> &str {
        match *self {
            StartPipelineReprocessingError::InternalFailure(ref cause) => cause,
            StartPipelineReprocessingError::InvalidRequest(ref cause) => cause,
            StartPipelineReprocessingError::ResourceAlreadyExists(ref cause) => cause,
            StartPipelineReprocessingError::ResourceNotFound(ref cause) => cause,
            StartPipelineReprocessingError::ServiceUnavailable(ref cause) => cause,
            StartPipelineReprocessingError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(TagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(TagResourceError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(TagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(TagResourceError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(TagResourceError::Throttling(err.msg))
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
            TagResourceError::InternalFailure(ref cause) => cause,
            TagResourceError::InvalidRequest(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::ServiceUnavailable(ref cause) => cause,
            TagResourceError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UntagResourceError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UntagResourceError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UntagResourceError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UntagResourceError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UntagResourceError::Throttling(err.msg))
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
            UntagResourceError::InternalFailure(ref cause) => cause,
            UntagResourceError::InvalidRequest(ref cause) => cause,
            UntagResourceError::LimitExceeded(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::ServiceUnavailable(ref cause) => cause,
            UntagResourceError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateChannel
#[derive(Debug, PartialEq)]
pub enum UpdateChannelError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateChannelError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateChannelError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateChannelError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateChannelError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateChannelError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateChannelError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateChannelError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateChannelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateChannelError {
    fn description(&self) -> &str {
        match *self {
            UpdateChannelError::InternalFailure(ref cause) => cause,
            UpdateChannelError::InvalidRequest(ref cause) => cause,
            UpdateChannelError::ResourceNotFound(ref cause) => cause,
            UpdateChannelError::ServiceUnavailable(ref cause) => cause,
            UpdateChannelError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDataset
#[derive(Debug, PartialEq)]
pub enum UpdateDatasetError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDatasetError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDatasetError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDatasetError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDatasetError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDatasetError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDatasetError {
    fn description(&self) -> &str {
        match *self {
            UpdateDatasetError::InternalFailure(ref cause) => cause,
            UpdateDatasetError::InvalidRequest(ref cause) => cause,
            UpdateDatasetError::ResourceNotFound(ref cause) => cause,
            UpdateDatasetError::ServiceUnavailable(ref cause) => cause,
            UpdateDatasetError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateDatastore
#[derive(Debug, PartialEq)]
pub enum UpdateDatastoreError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdateDatastoreError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateDatastoreError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdateDatastoreError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdateDatastoreError::InvalidRequest(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateDatastoreError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdateDatastoreError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdateDatastoreError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateDatastoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateDatastoreError {
    fn description(&self) -> &str {
        match *self {
            UpdateDatastoreError::InternalFailure(ref cause) => cause,
            UpdateDatastoreError::InvalidRequest(ref cause) => cause,
            UpdateDatastoreError::ResourceNotFound(ref cause) => cause,
            UpdateDatastoreError::ServiceUnavailable(ref cause) => cause,
            UpdateDatastoreError::Throttling(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdatePipeline
#[derive(Debug, PartialEq)]
pub enum UpdatePipelineError {
    /// <p>There was an internal failure.</p>
    InternalFailure(String),
    /// <p>The request was not valid.</p>
    InvalidRequest(String),
    /// <p>The command caused an internal limit to be exceeded.</p>
    LimitExceeded(String),
    /// <p>A resource with the specified name could not be found.</p>
    ResourceNotFound(String),
    /// <p>The service is temporarily unavailable.</p>
    ServiceUnavailable(String),
    /// <p>The request was denied due to request throttling.</p>
    Throttling(String),
}

impl UpdatePipelineError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdatePipelineError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalFailureException" => {
                    return RusotoError::Service(UpdatePipelineError::InternalFailure(err.msg))
                }
                "InvalidRequestException" => {
                    return RusotoError::Service(UpdatePipelineError::InvalidRequest(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdatePipelineError::LimitExceeded(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdatePipelineError::ResourceNotFound(err.msg))
                }
                "ServiceUnavailableException" => {
                    return RusotoError::Service(UpdatePipelineError::ServiceUnavailable(err.msg))
                }
                "ThrottlingException" => {
                    return RusotoError::Service(UpdatePipelineError::Throttling(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdatePipelineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdatePipelineError {
    fn description(&self) -> &str {
        match *self {
            UpdatePipelineError::InternalFailure(ref cause) => cause,
            UpdatePipelineError::InvalidRequest(ref cause) => cause,
            UpdatePipelineError::LimitExceeded(ref cause) => cause,
            UpdatePipelineError::ResourceNotFound(ref cause) => cause,
            UpdatePipelineError::ServiceUnavailable(ref cause) => cause,
            UpdatePipelineError::Throttling(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS IoT Analytics API. AWS IoT Analytics clients implement this trait.
pub trait IotAnalytics {
    /// <p>Sends messages to a channel.</p>
    fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> RusotoFuture<BatchPutMessageResponse, BatchPutMessageError>;

    /// <p>Cancels the reprocessing of data through the pipeline.</p>
    fn cancel_pipeline_reprocessing(
        &self,
        input: CancelPipelineReprocessingRequest,
    ) -> RusotoFuture<CancelPipelineReprocessingResponse, CancelPipelineReprocessingError>;

    /// <p>Creates a channel. A channel collects data from an MQTT topic and archives the raw, unprocessed messages before publishing the data to a pipeline.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError>;

    /// <p>Creates a data set. A data set stores data retrieved from a data store by applying a "queryAction" (a SQL query) or a "containerAction" (executing a containerized application). This operation creates the skeleton of a data set. The data set can be populated manually by calling "CreateDatasetContent" or automatically according to a "trigger" you specify.</p>
    fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> RusotoFuture<CreateDatasetResponse, CreateDatasetError>;

    /// <p>Creates the content of a data set by applying a "queryAction" (a SQL query) or a "containerAction" (executing a containerized application).</p>
    fn create_dataset_content(
        &self,
        input: CreateDatasetContentRequest,
    ) -> RusotoFuture<CreateDatasetContentResponse, CreateDatasetContentError>;

    /// <p>Creates a data store, which is a repository for messages.</p>
    fn create_datastore(
        &self,
        input: CreateDatastoreRequest,
    ) -> RusotoFuture<CreateDatastoreResponse, CreateDatastoreError>;

    /// <p>Creates a pipeline. A pipeline consumes messages from one or more channels and allows you to process the messages before storing them in a data store. You must specify both a <code>channel</code> and a <code>datastore</code> activity and, optionally, as many as 23 additional activities in the <code>pipelineActivities</code> array.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineRequest,
    ) -> RusotoFuture<CreatePipelineResponse, CreatePipelineError>;

    /// <p>Deletes the specified channel.</p>
    fn delete_channel(&self, input: DeleteChannelRequest) -> RusotoFuture<(), DeleteChannelError>;

    /// <p>Deletes the specified data set.</p> <p>You do not have to delete the content of the data set before you perform this operation.</p>
    fn delete_dataset(&self, input: DeleteDatasetRequest) -> RusotoFuture<(), DeleteDatasetError>;

    /// <p>Deletes the content of the specified data set.</p>
    fn delete_dataset_content(
        &self,
        input: DeleteDatasetContentRequest,
    ) -> RusotoFuture<(), DeleteDatasetContentError>;

    /// <p>Deletes the specified data store.</p>
    fn delete_datastore(
        &self,
        input: DeleteDatastoreRequest,
    ) -> RusotoFuture<(), DeleteDatastoreError>;

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(
        &self,
        input: DeletePipelineRequest,
    ) -> RusotoFuture<(), DeletePipelineError>;

    /// <p>Retrieves information about a channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError>;

    /// <p>Retrieves information about a data set.</p>
    fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> RusotoFuture<DescribeDatasetResponse, DescribeDatasetError>;

    /// <p>Retrieves information about a data store.</p>
    fn describe_datastore(
        &self,
        input: DescribeDatastoreRequest,
    ) -> RusotoFuture<DescribeDatastoreResponse, DescribeDatastoreError>;

    /// <p>Retrieves the current settings of the AWS IoT Analytics logging options.</p>
    fn describe_logging_options(
        &self,
    ) -> RusotoFuture<DescribeLoggingOptionsResponse, DescribeLoggingOptionsError>;

    /// <p>Retrieves information about a pipeline.</p>
    fn describe_pipeline(
        &self,
        input: DescribePipelineRequest,
    ) -> RusotoFuture<DescribePipelineResponse, DescribePipelineError>;

    /// <p>Retrieves the contents of a data set as pre-signed URIs.</p>
    fn get_dataset_content(
        &self,
        input: GetDatasetContentRequest,
    ) -> RusotoFuture<GetDatasetContentResponse, GetDatasetContentError>;

    /// <p>Retrieves a list of channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError>;

    /// <p>Lists information about data set contents that have been created.</p>
    fn list_dataset_contents(
        &self,
        input: ListDatasetContentsRequest,
    ) -> RusotoFuture<ListDatasetContentsResponse, ListDatasetContentsError>;

    /// <p>Retrieves information about data sets.</p>
    fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> RusotoFuture<ListDatasetsResponse, ListDatasetsError>;

    /// <p>Retrieves a list of data stores.</p>
    fn list_datastores(
        &self,
        input: ListDatastoresRequest,
    ) -> RusotoFuture<ListDatastoresResponse, ListDatastoresError>;

    /// <p>Retrieves a list of pipelines.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesRequest,
    ) -> RusotoFuture<ListPipelinesResponse, ListPipelinesError>;

    /// <p>Lists the tags (metadata) which you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Sets or updates the AWS IoT Analytics logging options.</p> <p>Note that if you update the value of any <code>loggingOptions</code> field, it takes up to one minute for the change to take effect. Also, if you change the policy attached to the role you specified in the roleArn field (for example, to correct an invalid policy) it takes up to 5 minutes for that change to take effect. </p>
    fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> RusotoFuture<(), PutLoggingOptionsError>;

    /// <p>Simulates the results of running a pipeline activity on a message payload.</p>
    fn run_pipeline_activity(
        &self,
        input: RunPipelineActivityRequest,
    ) -> RusotoFuture<RunPipelineActivityResponse, RunPipelineActivityError>;

    /// <p>Retrieves a sample of messages from the specified channel ingested during the specified timeframe. Up to 10 messages can be retrieved.</p>
    fn sample_channel_data(
        &self,
        input: SampleChannelDataRequest,
    ) -> RusotoFuture<SampleChannelDataResponse, SampleChannelDataError>;

    /// <p>Starts the reprocessing of raw message data through the pipeline.</p>
    fn start_pipeline_reprocessing(
        &self,
        input: StartPipelineReprocessingRequest,
    ) -> RusotoFuture<StartPipelineReprocessingResponse, StartPipelineReprocessingError>;

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError>;

    /// <p>Removes the given tags (metadata) from the resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError>;

    /// <p>Updates the settings of a channel.</p>
    fn update_channel(&self, input: UpdateChannelRequest) -> RusotoFuture<(), UpdateChannelError>;

    /// <p>Updates the settings of a data set.</p>
    fn update_dataset(&self, input: UpdateDatasetRequest) -> RusotoFuture<(), UpdateDatasetError>;

    /// <p>Updates the settings of a data store.</p>
    fn update_datastore(
        &self,
        input: UpdateDatastoreRequest,
    ) -> RusotoFuture<(), UpdateDatastoreError>;

    /// <p>Updates the settings of a pipeline. You must specify both a <code>channel</code> and a <code>datastore</code> activity and, optionally, as many as 23 additional activities in the <code>pipelineActivities</code> array.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineRequest,
    ) -> RusotoFuture<(), UpdatePipelineError>;
}
/// A client for the AWS IoT Analytics API.
#[derive(Clone)]
pub struct IotAnalyticsClient {
    client: Client,
    region: region::Region,
}

impl IotAnalyticsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotAnalyticsClient {
        Self::new_with_client(Client::shared(), region)
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> IotAnalyticsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        Self::new_with_client(
            Client::new_with(credentials_provider, request_dispatcher),
            region,
        )
    }

    pub fn new_with_client(client: Client, region: region::Region) -> IotAnalyticsClient {
        IotAnalyticsClient { client, region }
    }
}

impl IotAnalytics for IotAnalyticsClient {
    /// <p>Sends messages to a channel.</p>
    fn batch_put_message(
        &self,
        input: BatchPutMessageRequest,
    ) -> RusotoFuture<BatchPutMessageResponse, BatchPutMessageError> {
        let request_uri = "/messages/batch";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<BatchPutMessageResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BatchPutMessageError::from_response(response))),
                )
            }
        })
    }

    /// <p>Cancels the reprocessing of data through the pipeline.</p>
    fn cancel_pipeline_reprocessing(
        &self,
        input: CancelPipelineReprocessingRequest,
    ) -> RusotoFuture<CancelPipelineReprocessingResponse, CancelPipelineReprocessingError> {
        let request_uri = format!(
            "/pipelines/{pipeline_name}/reprocessing/{reprocessing_id}",
            pipeline_name = input.pipeline_name,
            reprocessing_id = input.reprocessing_id
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CancelPipelineReprocessingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CancelPipelineReprocessingError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a channel. A channel collects data from an MQTT topic and archives the raw, unprocessed messages before publishing the data to a pipeline.</p>
    fn create_channel(
        &self,
        input: CreateChannelRequest,
    ) -> RusotoFuture<CreateChannelResponse, CreateChannelError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a data set. A data set stores data retrieved from a data store by applying a "queryAction" (a SQL query) or a "containerAction" (executing a containerized application). This operation creates the skeleton of a data set. The data set can be populated manually by calling "CreateDatasetContent" or automatically according to a "trigger" you specify.</p>
    fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> RusotoFuture<CreateDatasetResponse, CreateDatasetError> {
        let request_uri = "/datasets";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDatasetResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDatasetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates the content of a data set by applying a "queryAction" (a SQL query) or a "containerAction" (executing a containerized application).</p>
    fn create_dataset_content(
        &self,
        input: CreateDatasetContentRequest,
    ) -> RusotoFuture<CreateDatasetContentResponse, CreateDatasetContentError> {
        let request_uri = format!(
            "/datasets/{dataset_name}/content",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDatasetContentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateDatasetContentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates a data store, which is a repository for messages.</p>
    fn create_datastore(
        &self,
        input: CreateDatastoreRequest,
    ) -> RusotoFuture<CreateDatastoreResponse, CreateDatastoreError> {
        let request_uri = "/datastores";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateDatastoreResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateDatastoreError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a pipeline. A pipeline consumes messages from one or more channels and allows you to process the messages before storing them in a data store. You must specify both a <code>channel</code> and a <code>datastore</code> activity and, optionally, as many as 23 additional activities in the <code>pipelineActivities</code> array.</p>
    fn create_pipeline(
        &self,
        input: CreatePipelineRequest,
    ) -> RusotoFuture<CreatePipelineResponse, CreatePipelineError> {
        let request_uri = "/pipelines";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 201 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreatePipelineResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreatePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified channel.</p>
    fn delete_channel(&self, input: DeleteChannelRequest) -> RusotoFuture<(), DeleteChannelError> {
        let request_uri = format!(
            "/channels/{channel_name}",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified data set.</p> <p>You do not have to delete the content of the data set before you perform this operation.</p>
    fn delete_dataset(&self, input: DeleteDatasetRequest) -> RusotoFuture<(), DeleteDatasetError> {
        let request_uri = format!(
            "/datasets/{dataset_name}",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDatasetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the content of the specified data set.</p>
    fn delete_dataset_content(
        &self,
        input: DeleteDatasetContentRequest,
    ) -> RusotoFuture<(), DeleteDatasetContentError> {
        let request_uri = format!(
            "/datasets/{dataset_name}/content",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteDatasetContentError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the specified data store.</p>
    fn delete_datastore(
        &self,
        input: DeleteDatastoreRequest,
    ) -> RusotoFuture<(), DeleteDatastoreError> {
        let request_uri = format!(
            "/datastores/{datastore_name}",
            datastore_name = input.datastore_name
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteDatastoreError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specified pipeline.</p>
    fn delete_pipeline(
        &self,
        input: DeletePipelineRequest,
    ) -> RusotoFuture<(), DeletePipelineError> {
        let request_uri = format!(
            "/pipelines/{pipeline_name}",
            pipeline_name = input.pipeline_name
        );

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeletePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a channel.</p>
    fn describe_channel(
        &self,
        input: DescribeChannelRequest,
    ) -> RusotoFuture<DescribeChannelResponse, DescribeChannelError> {
        let request_uri = format!(
            "/channels/{channel_name}",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_statistics {
            params.put("includeStatistics", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeChannelResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a data set.</p>
    fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> RusotoFuture<DescribeDatasetResponse, DescribeDatasetError> {
        let request_uri = format!(
            "/datasets/{dataset_name}",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDatasetResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDatasetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves information about a data store.</p>
    fn describe_datastore(
        &self,
        input: DescribeDatastoreRequest,
    ) -> RusotoFuture<DescribeDatastoreResponse, DescribeDatastoreError> {
        let request_uri = format!(
            "/datastores/{datastore_name}",
            datastore_name = input.datastore_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.include_statistics {
            params.put("includeStatistics", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeDatastoreResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeDatastoreError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the current settings of the AWS IoT Analytics logging options.</p>
    fn describe_logging_options(
        &self,
    ) -> RusotoFuture<DescribeLoggingOptionsResponse, DescribeLoggingOptionsError> {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeLoggingOptionsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeLoggingOptionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about a pipeline.</p>
    fn describe_pipeline(
        &self,
        input: DescribePipelineRequest,
    ) -> RusotoFuture<DescribePipelineResponse, DescribePipelineError> {
        let request_uri = format!(
            "/pipelines/{pipeline_name}",
            pipeline_name = input.pipeline_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribePipelineResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribePipelineError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the contents of a data set as pre-signed URIs.</p>
    fn get_dataset_content(
        &self,
        input: GetDatasetContentRequest,
    ) -> RusotoFuture<GetDatasetContentResponse, GetDatasetContentError> {
        let request_uri = format!(
            "/datasets/{dataset_name}/content",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.version_id {
            params.put("versionId", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetDatasetContentResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetDatasetContentError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of channels.</p>
    fn list_channels(
        &self,
        input: ListChannelsRequest,
    ) -> RusotoFuture<ListChannelsResponse, ListChannelsError> {
        let request_uri = "/channels";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListChannelsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListChannelsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists information about data set contents that have been created.</p>
    fn list_dataset_contents(
        &self,
        input: ListDatasetContentsRequest,
    ) -> RusotoFuture<ListDatasetContentsResponse, ListDatasetContentsError> {
        let request_uri = format!(
            "/datasets/{dataset_name}/contents",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.scheduled_before {
            params.put("scheduledBefore", x);
        }
        if let Some(ref x) = input.scheduled_on_or_after {
            params.put("scheduledOnOrAfter", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDatasetContentsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListDatasetContentsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves information about data sets.</p>
    fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> RusotoFuture<ListDatasetsResponse, ListDatasetsError> {
        let request_uri = "/datasets";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDatasetsResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDatasetsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of data stores.</p>
    fn list_datastores(
        &self,
        input: ListDatastoresRequest,
    ) -> RusotoFuture<ListDatastoresResponse, ListDatastoresError> {
        let request_uri = "/datastores";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListDatastoresResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListDatastoresError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves a list of pipelines.</p>
    fn list_pipelines(
        &self,
        input: ListPipelinesRequest,
    ) -> RusotoFuture<ListPipelinesResponse, ListPipelinesError> {
        let request_uri = "/pipelines";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListPipelinesResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListPipelinesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the tags (metadata) which you have assigned to the resource.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()?;

                    Ok(result)
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

    /// <p>Sets or updates the AWS IoT Analytics logging options.</p> <p>Note that if you update the value of any <code>loggingOptions</code> field, it takes up to one minute for the change to take effect. Also, if you change the policy attached to the role you specified in the roleArn field (for example, to correct an invalid policy) it takes up to 5 minutes for that change to take effect. </p>
    fn put_logging_options(
        &self,
        input: PutLoggingOptionsRequest,
    ) -> RusotoFuture<(), PutLoggingOptionsError> {
        let request_uri = "/logging";

        let mut request = SignedRequest::new("PUT", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PutLoggingOptionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Simulates the results of running a pipeline activity on a message payload.</p>
    fn run_pipeline_activity(
        &self,
        input: RunPipelineActivityRequest,
    ) -> RusotoFuture<RunPipelineActivityResponse, RunPipelineActivityError> {
        let request_uri = "/pipelineactivities/run";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<RunPipelineActivityResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(RunPipelineActivityError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves a sample of messages from the specified channel ingested during the specified timeframe. Up to 10 messages can be retrieved.</p>
    fn sample_channel_data(
        &self,
        input: SampleChannelDataRequest,
    ) -> RusotoFuture<SampleChannelDataResponse, SampleChannelDataError> {
        let request_uri = format!(
            "/channels/{channel_name}/sample",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("GET", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.end_time {
            params.put("endTime", x);
        }
        if let Some(ref x) = input.max_messages {
            params.put("maxMessages", x);
        }
        if let Some(ref x) = input.start_time {
            params.put("startTime", x);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<SampleChannelDataResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SampleChannelDataError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the reprocessing of raw message data through the pipeline.</p>
    fn start_pipeline_reprocessing(
        &self,
        input: StartPipelineReprocessingRequest,
    ) -> RusotoFuture<StartPipelineReprocessingResponse, StartPipelineReprocessingError> {
        let request_uri = format!(
            "/pipelines/{pipeline_name}/reprocessing",
            pipeline_name = input.pipeline_name
        );

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartPipelineReprocessingResponse, _>()?;

                    Ok(result)
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartPipelineReprocessingError::from_response(response))
                }))
            }
        })
    }

    /// <p>Adds to or modifies the tags of the given resource. Tags are metadata which can be used to manage a resource.</p>
    fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> RusotoFuture<TagResourceResponse, TagResourceError> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<TagResourceResponse, _>()?;

                    Ok(result)
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

    /// <p>Removes the given tags (metadata) from the resource.</p>
    fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> RusotoFuture<UntagResourceResponse, UntagResourceError> {
        let request_uri = "/tags";

        let mut request = SignedRequest::new("DELETE", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("resourceArn", &input.resource_arn);
        for item in input.tag_keys.iter() {
            params.put("tagKeys", item);
        }
        request.set_params(params);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 204 {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = proto::json::ResponsePayload::new(&response)
                        .deserialize::<UntagResourceResponse, _>()?;

                    Ok(result)
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

    /// <p>Updates the settings of a channel.</p>
    fn update_channel(&self, input: UpdateChannelRequest) -> RusotoFuture<(), UpdateChannelError> {
        let request_uri = format!(
            "/channels/{channel_name}",
            channel_name = input.channel_name
        );

        let mut request = SignedRequest::new("PUT", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateChannelError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the settings of a data set.</p>
    fn update_dataset(&self, input: UpdateDatasetRequest) -> RusotoFuture<(), UpdateDatasetError> {
        let request_uri = format!(
            "/datasets/{dataset_name}",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("PUT", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDatasetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the settings of a data store.</p>
    fn update_datastore(
        &self,
        input: UpdateDatastoreRequest,
    ) -> RusotoFuture<(), UpdateDatastoreError> {
        let request_uri = format!(
            "/datastores/{datastore_name}",
            datastore_name = input.datastore_name
        );

        let mut request = SignedRequest::new("PUT", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateDatastoreError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the settings of a pipeline. You must specify both a <code>channel</code> and a <code>datastore</code> activity and, optionally, as many as 23 additional activities in the <code>pipelineActivities</code> array.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineRequest,
    ) -> RusotoFuture<(), UpdatePipelineError> {
        let request_uri = format!(
            "/pipelines/{pipeline_name}",
            pipeline_name = input.pipeline_name
        );

        let mut request = SignedRequest::new("PUT", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    let result = ::std::mem::drop(response);

                    Ok(result)
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdatePipelineError::from_response(response))),
                )
            }
        })
    }
}
