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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The list of messages to be sent. Each message has format: '{ "messageId": "string", "payload": "string"}'.</p>
    #[serde(rename = "messages")]
    pub messages: Vec<Message>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct CancelPipelineReprocessingResponse {}

/// <p>A collection of data from an MQTT topic. Channels archive the raw, unprocessed messages before publishing the data to a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelStatistics {
    /// <p>The estimated size of the channel.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<EstimatedResourceSize>,
}

/// <p>A summary of information about a channel.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ChannelSummary {
    /// <p>The name of the channel.</p>
    #[serde(rename = "channelName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_name: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateChannelRequest {
    /// <p>The name of the channel.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
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
#[cfg_attr(test, derive(Serialize))]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatasetRequest {
    /// <p>A list of actions that create the data set. Only one action is supported at this time.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<DatasetAction>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>Metadata which can be used to manage the data set.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>A list of triggers. A trigger causes data set content to be populated at a specified time or time interval. The list of triggers can be empty or contain up to five <b>DataSetTrigger</b> objects.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateDatasetResponse {
    /// <p>The ARN of the data set.</p>
    #[serde(rename = "datasetArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_arn: Option<String>,
    /// <p>The name of the data set.</p>
    #[serde(rename = "datasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateDatastoreRequest {
    /// <p>The name of the data store.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>A list of pipeline activities.</p> <p>The list can be 1-25 <b>PipelineActivity</b> objects. Activities perform transformations on your messages, such as removing, renaming, or adding message attributes; filtering messages based on attribute values; invoking your Lambda functions on messages for advanced processing; or performing mathematical transformations to normalize device data.</p>
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
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Information about a data set.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Dataset {
    /// <p>The "DatasetAction" objects that create the data set.</p>
    #[serde(rename = "actions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<DatasetAction>>,
    /// <p>The ARN of the data set.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
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
    /// <p>The status of the data set.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The "DatasetTrigger" objects that specify when the data set is automatically updated.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
}

/// <p>A "DatasetAction" object specifying the query that creates the data set content.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetAction {
    /// <p>The name of the data set action.</p>
    #[serde(rename = "actionName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// <p>An "SqlQueryDatasetAction" object that contains the SQL query to modify the message.</p>
    #[serde(rename = "queryAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_action: Option<SqlQueryDatasetAction>,
}

/// <p>The state of the data set and the reason it is in this state.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DatasetContentStatus {
    /// <p>The reason the data set is in this state.</p>
    #[serde(rename = "reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// <p>The state of the data set. Can be one of "CREATING", "SUCCEEDED" or "FAILED".</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>The reference to a data set entry.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
pub struct DatasetSummary {
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
}

/// <p>The "DatasetTrigger" that specifies when the data set is automatically updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DatasetTrigger {
    /// <p>The "Schedule" when the trigger is initiated.</p>
    #[serde(rename = "schedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<Schedule>,
}

/// <p>Information about a data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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

/// <p>Statistics information about the data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DatastoreStatistics {
    /// <p>The estimated size of the data store.</p>
    #[serde(rename = "size")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<EstimatedResourceSize>,
}

/// <p>A summary of information about a data store.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DatastoreSummary {
    /// <p>When the data store was created.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>The name of the data store.</p>
    #[serde(rename = "datastoreName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore_name: Option<String>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeChannelRequest {
    /// <p>The name of the channel whose information is retrieved.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>If true, include statistics about the channel in the response.</p>
    #[serde(rename = "includeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_statistics: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>If true, include statistics about the data store in the response.</p>
    #[serde(rename = "includeStatistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_statistics: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDatastoreResponse {
    /// <p>Information about the data store.</p>
    #[serde(rename = "datastore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datastore: Option<Datastore>,
    /// <p>Statistics about the data store. Included if the 'includeStatistics' parameter is set to true in the request.</p>
    #[serde(rename = "statistics")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<DatastoreStatistics>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeLoggingOptionsRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>An expression that looks like an SQL WHERE clause that must return a Boolean value.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The name of the attribute that will contain the result of the math operation.</p>
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
    pub payload: Vec<u8>,
}

/// <p>Contains information about a pipeline.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
#[cfg_attr(test, derive(Serialize))]
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
    pub payloads: Vec<Vec<u8>>,
    /// <p>The pipeline activity that is run. This must not be a 'channel' activity or a 'datastore' activity because these activities are used in a pipeline only to load the original message and to store the (possibly) transformed message. If a 'lambda' activity is specified, only short-running Lambda functions (those with a timeout of less than 30 seconds or less) can be used.</p>
    #[serde(rename = "pipelineActivity")]
    pub pipeline_activity: PipelineActivity,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RunPipelineActivityResponse {
    /// <p>In case the pipeline activity fails, the log message that is generated.</p>
    #[serde(rename = "logResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_result: Option<String>,
    /// <p>The enriched or transformed sample message payloads as base64-encoded strings. (The results of running the pipeline activity on each input sample message payload, encoded in base64.)</p>
    #[serde(rename = "payloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<Vec<u8>>>,
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
#[cfg_attr(test, derive(Serialize))]
pub struct SampleChannelDataResponse {
    /// <p>The list of message samples. Each sample message is returned as a base64-encoded string.</p>
    #[serde(rename = "payloads")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payloads: Option<Vec<Vec<u8>>>,
}

/// <p>The schedule for when to trigger an update.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    /// <p>The expression that defines when to trigger an update. For more information, see <a href="https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/ScheduledEvents.html"> Schedule Expressions for Rules</a> in the Amazon CloudWatch documentation.</p>
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

/// <p>The SQL query to modify the message.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SqlQueryDatasetAction {
    /// <p>An SQL query string.</p>
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
#[cfg_attr(test, derive(Serialize))]
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
    /// <p>The ARN of the resource whose tags will be modified.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The new or modified tags for the resource.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UntagResourceRequest {
    /// <p>The ARN of the resource whose tags will be removed.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The keys of those tags which will be removed.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateChannelRequest {
    /// <p>The name of the channel to be updated.</p>
    #[serde(rename = "channelName")]
    pub channel_name: String,
    /// <p>How long, in days, message data is kept for the channel.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatasetRequest {
    /// <p>A list of "DatasetAction" objects. Only one action is supported at this time.</p>
    #[serde(rename = "actions")]
    pub actions: Vec<DatasetAction>,
    /// <p>The name of the data set to update.</p>
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    /// <p>A list of "DatasetTrigger" objects. The list can be empty or can contain up to five <b>DataSetTrigger</b> objects.</p>
    #[serde(rename = "triggers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggers: Option<Vec<DatasetTrigger>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateDatastoreRequest {
    /// <p>The name of the data store to be updated.</p>
    #[serde(rename = "datastoreName")]
    pub datastore_name: String,
    /// <p>How long, in days, message data is kept for the data store.</p>
    #[serde(rename = "retentionPeriod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<RetentionPeriod>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdatePipelineRequest {
    /// <p>A list of "PipelineActivity" objects.</p> <p>The list can be 1-25 <b>PipelineActivity</b> objects. Activities perform transformations on your messages, such as removing, renaming or adding message attributes; filtering messages based on attribute values; invoking your Lambda functions on messages for advanced processing; or performing mathematical transformations to normalize device data.</p>
    #[serde(rename = "pipelineActivities")]
    pub pipeline_activities: Vec<PipelineActivity>,
    /// <p>The name of the pipeline to update.</p>
    #[serde(rename = "pipelineName")]
    pub pipeline_name: String,
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

impl BatchPutMessageError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> BatchPutMessageError {
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
                "InternalFailureException" => {
                    return BatchPutMessageError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return BatchPutMessageError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return BatchPutMessageError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return BatchPutMessageError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return BatchPutMessageError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return BatchPutMessageError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return BatchPutMessageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BatchPutMessageError {
    fn from(err: serde_json::error::Error) -> BatchPutMessageError {
        BatchPutMessageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BatchPutMessageError {
    fn from(err: CredentialsError) -> BatchPutMessageError {
        BatchPutMessageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BatchPutMessageError {
    fn from(err: HttpDispatchError) -> BatchPutMessageError {
        BatchPutMessageError::HttpDispatch(err)
    }
}
impl From<io::Error> for BatchPutMessageError {
    fn from(err: io::Error) -> BatchPutMessageError {
        BatchPutMessageError::HttpDispatch(HttpDispatchError::from(err))
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
            BatchPutMessageError::Validation(ref cause) => cause,
            BatchPutMessageError::Credentials(ref err) => err.description(),
            BatchPutMessageError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BatchPutMessageError::ParseError(ref cause) => cause,
            BatchPutMessageError::Unknown(_) => "unknown error",
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

impl CancelPipelineReprocessingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CancelPipelineReprocessingError {
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
                "InternalFailureException" => {
                    return CancelPipelineReprocessingError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return CancelPipelineReprocessingError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return CancelPipelineReprocessingError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return CancelPipelineReprocessingError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ThrottlingException" => {
                    return CancelPipelineReprocessingError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CancelPipelineReprocessingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CancelPipelineReprocessingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CancelPipelineReprocessingError {
    fn from(err: serde_json::error::Error) -> CancelPipelineReprocessingError {
        CancelPipelineReprocessingError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CancelPipelineReprocessingError {
    fn from(err: CredentialsError) -> CancelPipelineReprocessingError {
        CancelPipelineReprocessingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CancelPipelineReprocessingError {
    fn from(err: HttpDispatchError) -> CancelPipelineReprocessingError {
        CancelPipelineReprocessingError::HttpDispatch(err)
    }
}
impl From<io::Error> for CancelPipelineReprocessingError {
    fn from(err: io::Error) -> CancelPipelineReprocessingError {
        CancelPipelineReprocessingError::HttpDispatch(HttpDispatchError::from(err))
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
            CancelPipelineReprocessingError::Validation(ref cause) => cause,
            CancelPipelineReprocessingError::Credentials(ref err) => err.description(),
            CancelPipelineReprocessingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CancelPipelineReprocessingError::ParseError(ref cause) => cause,
            CancelPipelineReprocessingError::Unknown(_) => "unknown error",
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

impl CreateChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateChannelError {
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
                "InternalFailureException" => {
                    return CreateChannelError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreateChannelError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateChannelError::LimitExceeded(String::from(error_message));
                }
                "ResourceAlreadyExistsException" => {
                    return CreateChannelError::ResourceAlreadyExists(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateChannelError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return CreateChannelError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateChannelError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateChannelError {
    fn from(err: serde_json::error::Error) -> CreateChannelError {
        CreateChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateChannelError {
    fn from(err: CredentialsError) -> CreateChannelError {
        CreateChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateChannelError {
    fn from(err: HttpDispatchError) -> CreateChannelError {
        CreateChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateChannelError {
    fn from(err: io::Error) -> CreateChannelError {
        CreateChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateChannelError::Validation(ref cause) => cause,
            CreateChannelError::Credentials(ref err) => err.description(),
            CreateChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateChannelError::ParseError(ref cause) => cause,
            CreateChannelError::Unknown(_) => "unknown error",
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

impl CreateDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateDatasetError {
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
                "InternalFailureException" => {
                    return CreateDatasetError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreateDatasetError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateDatasetError::LimitExceeded(String::from(error_message));
                }
                "ResourceAlreadyExistsException" => {
                    return CreateDatasetError::ResourceAlreadyExists(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateDatasetError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return CreateDatasetError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateDatasetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDatasetError {
    fn from(err: serde_json::error::Error) -> CreateDatasetError {
        CreateDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDatasetError {
    fn from(err: CredentialsError) -> CreateDatasetError {
        CreateDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDatasetError {
    fn from(err: HttpDispatchError) -> CreateDatasetError {
        CreateDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDatasetError {
    fn from(err: io::Error) -> CreateDatasetError {
        CreateDatasetError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateDatasetError::Validation(ref cause) => cause,
            CreateDatasetError::Credentials(ref err) => err.description(),
            CreateDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDatasetError::ParseError(ref cause) => cause,
            CreateDatasetError::Unknown(_) => "unknown error",
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

impl CreateDatasetContentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateDatasetContentError {
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
                "InternalFailureException" => {
                    return CreateDatasetContentError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreateDatasetContentError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return CreateDatasetContentError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateDatasetContentError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ThrottlingException" => {
                    return CreateDatasetContentError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateDatasetContentError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDatasetContentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDatasetContentError {
    fn from(err: serde_json::error::Error) -> CreateDatasetContentError {
        CreateDatasetContentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDatasetContentError {
    fn from(err: CredentialsError) -> CreateDatasetContentError {
        CreateDatasetContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDatasetContentError {
    fn from(err: HttpDispatchError) -> CreateDatasetContentError {
        CreateDatasetContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDatasetContentError {
    fn from(err: io::Error) -> CreateDatasetContentError {
        CreateDatasetContentError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateDatasetContentError::Validation(ref cause) => cause,
            CreateDatasetContentError::Credentials(ref err) => err.description(),
            CreateDatasetContentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateDatasetContentError::ParseError(ref cause) => cause,
            CreateDatasetContentError::Unknown(_) => "unknown error",
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

impl CreateDatastoreError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreateDatastoreError {
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
                "InternalFailureException" => {
                    return CreateDatastoreError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreateDatastoreError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreateDatastoreError::LimitExceeded(String::from(error_message));
                }
                "ResourceAlreadyExistsException" => {
                    return CreateDatastoreError::ResourceAlreadyExists(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreateDatastoreError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return CreateDatastoreError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateDatastoreError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateDatastoreError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateDatastoreError {
    fn from(err: serde_json::error::Error) -> CreateDatastoreError {
        CreateDatastoreError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateDatastoreError {
    fn from(err: CredentialsError) -> CreateDatastoreError {
        CreateDatastoreError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateDatastoreError {
    fn from(err: HttpDispatchError) -> CreateDatastoreError {
        CreateDatastoreError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateDatastoreError {
    fn from(err: io::Error) -> CreateDatastoreError {
        CreateDatastoreError::HttpDispatch(HttpDispatchError::from(err))
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
            CreateDatastoreError::Validation(ref cause) => cause,
            CreateDatastoreError::Credentials(ref err) => err.description(),
            CreateDatastoreError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateDatastoreError::ParseError(ref cause) => cause,
            CreateDatastoreError::Unknown(_) => "unknown error",
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

impl CreatePipelineError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> CreatePipelineError {
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
                "InternalFailureException" => {
                    return CreatePipelineError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return CreatePipelineError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return CreatePipelineError::LimitExceeded(String::from(error_message));
                }
                "ResourceAlreadyExistsException" => {
                    return CreatePipelineError::ResourceAlreadyExists(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return CreatePipelineError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return CreatePipelineError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return CreatePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreatePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreatePipelineError {
    fn from(err: serde_json::error::Error) -> CreatePipelineError {
        CreatePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreatePipelineError {
    fn from(err: CredentialsError) -> CreatePipelineError {
        CreatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreatePipelineError {
    fn from(err: HttpDispatchError) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreatePipelineError {
    fn from(err: io::Error) -> CreatePipelineError {
        CreatePipelineError::HttpDispatch(HttpDispatchError::from(err))
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
            CreatePipelineError::Validation(ref cause) => cause,
            CreatePipelineError::Credentials(ref err) => err.description(),
            CreatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreatePipelineError::ParseError(ref cause) => cause,
            CreatePipelineError::Unknown(_) => "unknown error",
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

impl DeleteChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteChannelError {
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
                "InternalFailureException" => {
                    return DeleteChannelError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeleteChannelError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteChannelError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteChannelError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DeleteChannelError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteChannelError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteChannelError {
    fn from(err: serde_json::error::Error) -> DeleteChannelError {
        DeleteChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteChannelError {
    fn from(err: CredentialsError) -> DeleteChannelError {
        DeleteChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteChannelError {
    fn from(err: HttpDispatchError) -> DeleteChannelError {
        DeleteChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteChannelError {
    fn from(err: io::Error) -> DeleteChannelError {
        DeleteChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteChannelError::Validation(ref cause) => cause,
            DeleteChannelError::Credentials(ref err) => err.description(),
            DeleteChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteChannelError::ParseError(ref cause) => cause,
            DeleteChannelError::Unknown(_) => "unknown error",
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

impl DeleteDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDatasetError {
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
                "InternalFailureException" => {
                    return DeleteDatasetError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeleteDatasetError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteDatasetError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteDatasetError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DeleteDatasetError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDatasetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDatasetError {
    fn from(err: serde_json::error::Error) -> DeleteDatasetError {
        DeleteDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDatasetError {
    fn from(err: CredentialsError) -> DeleteDatasetError {
        DeleteDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDatasetError {
    fn from(err: HttpDispatchError) -> DeleteDatasetError {
        DeleteDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDatasetError {
    fn from(err: io::Error) -> DeleteDatasetError {
        DeleteDatasetError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteDatasetError::Validation(ref cause) => cause,
            DeleteDatasetError::Credentials(ref err) => err.description(),
            DeleteDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDatasetError::ParseError(ref cause) => cause,
            DeleteDatasetError::Unknown(_) => "unknown error",
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

impl DeleteDatasetContentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDatasetContentError {
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
                "InternalFailureException" => {
                    return DeleteDatasetContentError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeleteDatasetContentError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteDatasetContentError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteDatasetContentError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ThrottlingException" => {
                    return DeleteDatasetContentError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDatasetContentError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDatasetContentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDatasetContentError {
    fn from(err: serde_json::error::Error) -> DeleteDatasetContentError {
        DeleteDatasetContentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDatasetContentError {
    fn from(err: CredentialsError) -> DeleteDatasetContentError {
        DeleteDatasetContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDatasetContentError {
    fn from(err: HttpDispatchError) -> DeleteDatasetContentError {
        DeleteDatasetContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDatasetContentError {
    fn from(err: io::Error) -> DeleteDatasetContentError {
        DeleteDatasetContentError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteDatasetContentError::Validation(ref cause) => cause,
            DeleteDatasetContentError::Credentials(ref err) => err.description(),
            DeleteDatasetContentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteDatasetContentError::ParseError(ref cause) => cause,
            DeleteDatasetContentError::Unknown(_) => "unknown error",
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

impl DeleteDatastoreError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeleteDatastoreError {
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
                "InternalFailureException" => {
                    return DeleteDatastoreError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeleteDatastoreError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeleteDatastoreError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeleteDatastoreError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DeleteDatastoreError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteDatastoreError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteDatastoreError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteDatastoreError {
    fn from(err: serde_json::error::Error) -> DeleteDatastoreError {
        DeleteDatastoreError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteDatastoreError {
    fn from(err: CredentialsError) -> DeleteDatastoreError {
        DeleteDatastoreError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteDatastoreError {
    fn from(err: HttpDispatchError) -> DeleteDatastoreError {
        DeleteDatastoreError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteDatastoreError {
    fn from(err: io::Error) -> DeleteDatastoreError {
        DeleteDatastoreError::HttpDispatch(HttpDispatchError::from(err))
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
            DeleteDatastoreError::Validation(ref cause) => cause,
            DeleteDatastoreError::Credentials(ref err) => err.description(),
            DeleteDatastoreError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDatastoreError::ParseError(ref cause) => cause,
            DeleteDatastoreError::Unknown(_) => "unknown error",
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

impl DeletePipelineError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DeletePipelineError {
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
                "InternalFailureException" => {
                    return DeletePipelineError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DeletePipelineError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DeletePipelineError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DeletePipelineError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DeletePipelineError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DeletePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeletePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeletePipelineError {
    fn from(err: serde_json::error::Error) -> DeletePipelineError {
        DeletePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeletePipelineError {
    fn from(err: CredentialsError) -> DeletePipelineError {
        DeletePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeletePipelineError {
    fn from(err: HttpDispatchError) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeletePipelineError {
    fn from(err: io::Error) -> DeletePipelineError {
        DeletePipelineError::HttpDispatch(HttpDispatchError::from(err))
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
            DeletePipelineError::Validation(ref cause) => cause,
            DeletePipelineError::Credentials(ref err) => err.description(),
            DeletePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeletePipelineError::ParseError(ref cause) => cause,
            DeletePipelineError::Unknown(_) => "unknown error",
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

impl DescribeChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeChannelError {
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
                "InternalFailureException" => {
                    return DescribeChannelError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribeChannelError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeChannelError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribeChannelError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DescribeChannelError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeChannelError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeChannelError {
    fn from(err: serde_json::error::Error) -> DescribeChannelError {
        DescribeChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeChannelError {
    fn from(err: CredentialsError) -> DescribeChannelError {
        DescribeChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeChannelError {
    fn from(err: HttpDispatchError) -> DescribeChannelError {
        DescribeChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeChannelError {
    fn from(err: io::Error) -> DescribeChannelError {
        DescribeChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeChannelError::Validation(ref cause) => cause,
            DescribeChannelError::Credentials(ref err) => err.description(),
            DescribeChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeChannelError::ParseError(ref cause) => cause,
            DescribeChannelError::Unknown(_) => "unknown error",
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

impl DescribeDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDatasetError {
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
                "InternalFailureException" => {
                    return DescribeDatasetError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribeDatasetError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeDatasetError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribeDatasetError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DescribeDatasetError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeDatasetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDatasetError {
    fn from(err: serde_json::error::Error) -> DescribeDatasetError {
        DescribeDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDatasetError {
    fn from(err: CredentialsError) -> DescribeDatasetError {
        DescribeDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDatasetError {
    fn from(err: HttpDispatchError) -> DescribeDatasetError {
        DescribeDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDatasetError {
    fn from(err: io::Error) -> DescribeDatasetError {
        DescribeDatasetError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeDatasetError::Validation(ref cause) => cause,
            DescribeDatasetError::Credentials(ref err) => err.description(),
            DescribeDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDatasetError::ParseError(ref cause) => cause,
            DescribeDatasetError::Unknown(_) => "unknown error",
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

impl DescribeDatastoreError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeDatastoreError {
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
                "InternalFailureException" => {
                    return DescribeDatastoreError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribeDatastoreError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeDatastoreError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribeDatastoreError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DescribeDatastoreError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeDatastoreError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeDatastoreError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeDatastoreError {
    fn from(err: serde_json::error::Error) -> DescribeDatastoreError {
        DescribeDatastoreError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeDatastoreError {
    fn from(err: CredentialsError) -> DescribeDatastoreError {
        DescribeDatastoreError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeDatastoreError {
    fn from(err: HttpDispatchError) -> DescribeDatastoreError {
        DescribeDatastoreError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeDatastoreError {
    fn from(err: io::Error) -> DescribeDatastoreError {
        DescribeDatastoreError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeDatastoreError::Validation(ref cause) => cause,
            DescribeDatastoreError::Credentials(ref err) => err.description(),
            DescribeDatastoreError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeDatastoreError::ParseError(ref cause) => cause,
            DescribeDatastoreError::Unknown(_) => "unknown error",
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

impl DescribeLoggingOptionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeLoggingOptionsError {
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
                "InternalFailureException" => {
                    return DescribeLoggingOptionsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribeLoggingOptionsError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribeLoggingOptionsError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return DescribeLoggingOptionsError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ThrottlingException" => {
                    return DescribeLoggingOptionsError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribeLoggingOptionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribeLoggingOptionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> DescribeLoggingOptionsError {
        DescribeLoggingOptionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeLoggingOptionsError {
    fn from(err: CredentialsError) -> DescribeLoggingOptionsError {
        DescribeLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeLoggingOptionsError {
    fn from(err: HttpDispatchError) -> DescribeLoggingOptionsError {
        DescribeLoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeLoggingOptionsError {
    fn from(err: io::Error) -> DescribeLoggingOptionsError {
        DescribeLoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribeLoggingOptionsError::Validation(ref cause) => cause,
            DescribeLoggingOptionsError::Credentials(ref err) => err.description(),
            DescribeLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeLoggingOptionsError::ParseError(ref cause) => cause,
            DescribeLoggingOptionsError::Unknown(_) => "unknown error",
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

impl DescribePipelineError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribePipelineError {
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
                "InternalFailureException" => {
                    return DescribePipelineError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return DescribePipelineError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return DescribePipelineError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return DescribePipelineError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return DescribePipelineError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return DescribePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DescribePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribePipelineError {
    fn from(err: serde_json::error::Error) -> DescribePipelineError {
        DescribePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribePipelineError {
    fn from(err: CredentialsError) -> DescribePipelineError {
        DescribePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribePipelineError {
    fn from(err: HttpDispatchError) -> DescribePipelineError {
        DescribePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribePipelineError {
    fn from(err: io::Error) -> DescribePipelineError {
        DescribePipelineError::HttpDispatch(HttpDispatchError::from(err))
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
            DescribePipelineError::Validation(ref cause) => cause,
            DescribePipelineError::Credentials(ref err) => err.description(),
            DescribePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribePipelineError::ParseError(ref cause) => cause,
            DescribePipelineError::Unknown(_) => "unknown error",
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

impl GetDatasetContentError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetDatasetContentError {
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
                "InternalFailureException" => {
                    return GetDatasetContentError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return GetDatasetContentError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return GetDatasetContentError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return GetDatasetContentError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return GetDatasetContentError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return GetDatasetContentError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetDatasetContentError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetDatasetContentError {
    fn from(err: serde_json::error::Error) -> GetDatasetContentError {
        GetDatasetContentError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDatasetContentError {
    fn from(err: CredentialsError) -> GetDatasetContentError {
        GetDatasetContentError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDatasetContentError {
    fn from(err: HttpDispatchError) -> GetDatasetContentError {
        GetDatasetContentError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDatasetContentError {
    fn from(err: io::Error) -> GetDatasetContentError {
        GetDatasetContentError::HttpDispatch(HttpDispatchError::from(err))
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
            GetDatasetContentError::Validation(ref cause) => cause,
            GetDatasetContentError::Credentials(ref err) => err.description(),
            GetDatasetContentError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDatasetContentError::ParseError(ref cause) => cause,
            GetDatasetContentError::Unknown(_) => "unknown error",
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

impl ListChannelsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListChannelsError {
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
                "InternalFailureException" => {
                    return ListChannelsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListChannelsError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListChannelsError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return ListChannelsError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return ListChannelsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListChannelsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListChannelsError {
    fn from(err: serde_json::error::Error) -> ListChannelsError {
        ListChannelsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListChannelsError {
    fn from(err: CredentialsError) -> ListChannelsError {
        ListChannelsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListChannelsError {
    fn from(err: HttpDispatchError) -> ListChannelsError {
        ListChannelsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListChannelsError {
    fn from(err: io::Error) -> ListChannelsError {
        ListChannelsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListChannelsError::Validation(ref cause) => cause,
            ListChannelsError::Credentials(ref err) => err.description(),
            ListChannelsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListChannelsError::ParseError(ref cause) => cause,
            ListChannelsError::Unknown(_) => "unknown error",
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

impl ListDatasetsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDatasetsError {
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
                "InternalFailureException" => {
                    return ListDatasetsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListDatasetsError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListDatasetsError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return ListDatasetsError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDatasetsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDatasetsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDatasetsError {
    fn from(err: serde_json::error::Error) -> ListDatasetsError {
        ListDatasetsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDatasetsError {
    fn from(err: CredentialsError) -> ListDatasetsError {
        ListDatasetsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDatasetsError {
    fn from(err: HttpDispatchError) -> ListDatasetsError {
        ListDatasetsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDatasetsError {
    fn from(err: io::Error) -> ListDatasetsError {
        ListDatasetsError::HttpDispatch(HttpDispatchError::from(err))
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
            ListDatasetsError::Validation(ref cause) => cause,
            ListDatasetsError::Credentials(ref err) => err.description(),
            ListDatasetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDatasetsError::ParseError(ref cause) => cause,
            ListDatasetsError::Unknown(_) => "unknown error",
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

impl ListDatastoresError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListDatastoresError {
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
                "InternalFailureException" => {
                    return ListDatastoresError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListDatastoresError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListDatastoresError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return ListDatastoresError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return ListDatastoresError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListDatastoresError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListDatastoresError {
    fn from(err: serde_json::error::Error) -> ListDatastoresError {
        ListDatastoresError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListDatastoresError {
    fn from(err: CredentialsError) -> ListDatastoresError {
        ListDatastoresError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListDatastoresError {
    fn from(err: HttpDispatchError) -> ListDatastoresError {
        ListDatastoresError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListDatastoresError {
    fn from(err: io::Error) -> ListDatastoresError {
        ListDatastoresError::HttpDispatch(HttpDispatchError::from(err))
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
            ListDatastoresError::Validation(ref cause) => cause,
            ListDatastoresError::Credentials(ref err) => err.description(),
            ListDatastoresError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDatastoresError::ParseError(ref cause) => cause,
            ListDatastoresError::Unknown(_) => "unknown error",
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

impl ListPipelinesError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListPipelinesError {
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
                "InternalFailureException" => {
                    return ListPipelinesError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListPipelinesError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListPipelinesError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return ListPipelinesError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return ListPipelinesError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListPipelinesError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListPipelinesError {
    fn from(err: serde_json::error::Error) -> ListPipelinesError {
        ListPipelinesError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListPipelinesError {
    fn from(err: CredentialsError) -> ListPipelinesError {
        ListPipelinesError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListPipelinesError {
    fn from(err: HttpDispatchError) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListPipelinesError {
    fn from(err: io::Error) -> ListPipelinesError {
        ListPipelinesError::HttpDispatch(HttpDispatchError::from(err))
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
            ListPipelinesError::Validation(ref cause) => cause,
            ListPipelinesError::Credentials(ref err) => err.description(),
            ListPipelinesError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListPipelinesError::ParseError(ref cause) => cause,
            ListPipelinesError::Unknown(_) => "unknown error",
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

impl ListTagsForResourceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListTagsForResourceError {
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
                "InternalFailureException" => {
                    return ListTagsForResourceError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return ListTagsForResourceError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return ListTagsForResourceError::LimitExceeded(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return ListTagsForResourceError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return ListTagsForResourceError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return ListTagsForResourceError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return ListTagsForResourceError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListTagsForResourceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListTagsForResourceError {
    fn from(err: serde_json::error::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListTagsForResourceError {
    fn from(err: CredentialsError) -> ListTagsForResourceError {
        ListTagsForResourceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListTagsForResourceError {
    fn from(err: HttpDispatchError) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListTagsForResourceError {
    fn from(err: io::Error) -> ListTagsForResourceError {
        ListTagsForResourceError::HttpDispatch(HttpDispatchError::from(err))
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
            ListTagsForResourceError::Validation(ref cause) => cause,
            ListTagsForResourceError::Credentials(ref err) => err.description(),
            ListTagsForResourceError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListTagsForResourceError::ParseError(ref cause) => cause,
            ListTagsForResourceError::Unknown(_) => "unknown error",
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

impl PutLoggingOptionsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> PutLoggingOptionsError {
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
                "InternalFailureException" => {
                    return PutLoggingOptionsError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return PutLoggingOptionsError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return PutLoggingOptionsError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return PutLoggingOptionsError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return PutLoggingOptionsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutLoggingOptionsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutLoggingOptionsError {
    fn from(err: serde_json::error::Error) -> PutLoggingOptionsError {
        PutLoggingOptionsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutLoggingOptionsError {
    fn from(err: CredentialsError) -> PutLoggingOptionsError {
        PutLoggingOptionsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutLoggingOptionsError {
    fn from(err: HttpDispatchError) -> PutLoggingOptionsError {
        PutLoggingOptionsError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutLoggingOptionsError {
    fn from(err: io::Error) -> PutLoggingOptionsError {
        PutLoggingOptionsError::HttpDispatch(HttpDispatchError::from(err))
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
            PutLoggingOptionsError::Validation(ref cause) => cause,
            PutLoggingOptionsError::Credentials(ref err) => err.description(),
            PutLoggingOptionsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutLoggingOptionsError::ParseError(ref cause) => cause,
            PutLoggingOptionsError::Unknown(_) => "unknown error",
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

impl RunPipelineActivityError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RunPipelineActivityError {
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
                "InternalFailureException" => {
                    return RunPipelineActivityError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return RunPipelineActivityError::InvalidRequest(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return RunPipelineActivityError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return RunPipelineActivityError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return RunPipelineActivityError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return RunPipelineActivityError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RunPipelineActivityError {
    fn from(err: serde_json::error::Error) -> RunPipelineActivityError {
        RunPipelineActivityError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RunPipelineActivityError {
    fn from(err: CredentialsError) -> RunPipelineActivityError {
        RunPipelineActivityError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RunPipelineActivityError {
    fn from(err: HttpDispatchError) -> RunPipelineActivityError {
        RunPipelineActivityError::HttpDispatch(err)
    }
}
impl From<io::Error> for RunPipelineActivityError {
    fn from(err: io::Error) -> RunPipelineActivityError {
        RunPipelineActivityError::HttpDispatch(HttpDispatchError::from(err))
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
            RunPipelineActivityError::Validation(ref cause) => cause,
            RunPipelineActivityError::Credentials(ref err) => err.description(),
            RunPipelineActivityError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            RunPipelineActivityError::ParseError(ref cause) => cause,
            RunPipelineActivityError::Unknown(_) => "unknown error",
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

impl SampleChannelDataError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SampleChannelDataError {
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
                "InternalFailureException" => {
                    return SampleChannelDataError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return SampleChannelDataError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return SampleChannelDataError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return SampleChannelDataError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return SampleChannelDataError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return SampleChannelDataError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return SampleChannelDataError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SampleChannelDataError {
    fn from(err: serde_json::error::Error) -> SampleChannelDataError {
        SampleChannelDataError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SampleChannelDataError {
    fn from(err: CredentialsError) -> SampleChannelDataError {
        SampleChannelDataError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SampleChannelDataError {
    fn from(err: HttpDispatchError) -> SampleChannelDataError {
        SampleChannelDataError::HttpDispatch(err)
    }
}
impl From<io::Error> for SampleChannelDataError {
    fn from(err: io::Error) -> SampleChannelDataError {
        SampleChannelDataError::HttpDispatch(HttpDispatchError::from(err))
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
            SampleChannelDataError::Validation(ref cause) => cause,
            SampleChannelDataError::Credentials(ref err) => err.description(),
            SampleChannelDataError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SampleChannelDataError::ParseError(ref cause) => cause,
            SampleChannelDataError::Unknown(_) => "unknown error",
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

impl StartPipelineReprocessingError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> StartPipelineReprocessingError {
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
                "InternalFailureException" => {
                    return StartPipelineReprocessingError::InternalFailure(String::from(
                        error_message,
                    ));
                }
                "InvalidRequestException" => {
                    return StartPipelineReprocessingError::InvalidRequest(String::from(
                        error_message,
                    ));
                }
                "ResourceAlreadyExistsException" => {
                    return StartPipelineReprocessingError::ResourceAlreadyExists(String::from(
                        error_message,
                    ));
                }
                "ResourceNotFoundException" => {
                    return StartPipelineReprocessingError::ResourceNotFound(String::from(
                        error_message,
                    ));
                }
                "ServiceUnavailableException" => {
                    return StartPipelineReprocessingError::ServiceUnavailable(String::from(
                        error_message,
                    ));
                }
                "ThrottlingException" => {
                    return StartPipelineReprocessingError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return StartPipelineReprocessingError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartPipelineReprocessingError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartPipelineReprocessingError {
    fn from(err: serde_json::error::Error) -> StartPipelineReprocessingError {
        StartPipelineReprocessingError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartPipelineReprocessingError {
    fn from(err: CredentialsError) -> StartPipelineReprocessingError {
        StartPipelineReprocessingError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartPipelineReprocessingError {
    fn from(err: HttpDispatchError) -> StartPipelineReprocessingError {
        StartPipelineReprocessingError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartPipelineReprocessingError {
    fn from(err: io::Error) -> StartPipelineReprocessingError {
        StartPipelineReprocessingError::HttpDispatch(HttpDispatchError::from(err))
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
            StartPipelineReprocessingError::Validation(ref cause) => cause,
            StartPipelineReprocessingError::Credentials(ref err) => err.description(),
            StartPipelineReprocessingError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartPipelineReprocessingError::ParseError(ref cause) => cause,
            StartPipelineReprocessingError::Unknown(_) => "unknown error",
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
                "InternalFailureException" => {
                    return TagResourceError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return TagResourceError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return TagResourceError::LimitExceeded(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return TagResourceError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return TagResourceError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return TagResourceError::Throttling(String::from(error_message));
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
            TagResourceError::InternalFailure(ref cause) => cause,
            TagResourceError::InvalidRequest(ref cause) => cause,
            TagResourceError::LimitExceeded(ref cause) => cause,
            TagResourceError::ResourceNotFound(ref cause) => cause,
            TagResourceError::ServiceUnavailable(ref cause) => cause,
            TagResourceError::Throttling(ref cause) => cause,
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
                "InternalFailureException" => {
                    return UntagResourceError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UntagResourceError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return UntagResourceError::LimitExceeded(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UntagResourceError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UntagResourceError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return UntagResourceError::Throttling(String::from(error_message));
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
            UntagResourceError::InternalFailure(ref cause) => cause,
            UntagResourceError::InvalidRequest(ref cause) => cause,
            UntagResourceError::LimitExceeded(ref cause) => cause,
            UntagResourceError::ResourceNotFound(ref cause) => cause,
            UntagResourceError::ServiceUnavailable(ref cause) => cause,
            UntagResourceError::Throttling(ref cause) => cause,
            UntagResourceError::Validation(ref cause) => cause,
            UntagResourceError::Credentials(ref err) => err.description(),
            UntagResourceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UntagResourceError::ParseError(ref cause) => cause,
            UntagResourceError::Unknown(_) => "unknown error",
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

impl UpdateChannelError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateChannelError {
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
                "InternalFailureException" => {
                    return UpdateChannelError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UpdateChannelError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateChannelError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdateChannelError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return UpdateChannelError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateChannelError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateChannelError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateChannelError {
    fn from(err: serde_json::error::Error) -> UpdateChannelError {
        UpdateChannelError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateChannelError {
    fn from(err: CredentialsError) -> UpdateChannelError {
        UpdateChannelError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateChannelError {
    fn from(err: HttpDispatchError) -> UpdateChannelError {
        UpdateChannelError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateChannelError {
    fn from(err: io::Error) -> UpdateChannelError {
        UpdateChannelError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateChannelError::Validation(ref cause) => cause,
            UpdateChannelError::Credentials(ref err) => err.description(),
            UpdateChannelError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateChannelError::ParseError(ref cause) => cause,
            UpdateChannelError::Unknown(_) => "unknown error",
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

impl UpdateDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDatasetError {
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
                "InternalFailureException" => {
                    return UpdateDatasetError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UpdateDatasetError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateDatasetError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdateDatasetError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return UpdateDatasetError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDatasetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDatasetError {
    fn from(err: serde_json::error::Error) -> UpdateDatasetError {
        UpdateDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDatasetError {
    fn from(err: CredentialsError) -> UpdateDatasetError {
        UpdateDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDatasetError {
    fn from(err: HttpDispatchError) -> UpdateDatasetError {
        UpdateDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDatasetError {
    fn from(err: io::Error) -> UpdateDatasetError {
        UpdateDatasetError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateDatasetError::Validation(ref cause) => cause,
            UpdateDatasetError::Credentials(ref err) => err.description(),
            UpdateDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDatasetError::ParseError(ref cause) => cause,
            UpdateDatasetError::Unknown(_) => "unknown error",
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

impl UpdateDatastoreError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateDatastoreError {
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
                "InternalFailureException" => {
                    return UpdateDatastoreError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UpdateDatastoreError::InvalidRequest(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdateDatastoreError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdateDatastoreError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return UpdateDatastoreError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateDatastoreError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateDatastoreError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateDatastoreError {
    fn from(err: serde_json::error::Error) -> UpdateDatastoreError {
        UpdateDatastoreError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateDatastoreError {
    fn from(err: CredentialsError) -> UpdateDatastoreError {
        UpdateDatastoreError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateDatastoreError {
    fn from(err: HttpDispatchError) -> UpdateDatastoreError {
        UpdateDatastoreError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateDatastoreError {
    fn from(err: io::Error) -> UpdateDatastoreError {
        UpdateDatastoreError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdateDatastoreError::Validation(ref cause) => cause,
            UpdateDatastoreError::Credentials(ref err) => err.description(),
            UpdateDatastoreError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateDatastoreError::ParseError(ref cause) => cause,
            UpdateDatastoreError::Unknown(_) => "unknown error",
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

impl UpdatePipelineError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdatePipelineError {
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
                "InternalFailureException" => {
                    return UpdatePipelineError::InternalFailure(String::from(error_message));
                }
                "InvalidRequestException" => {
                    return UpdatePipelineError::InvalidRequest(String::from(error_message));
                }
                "LimitExceededException" => {
                    return UpdatePipelineError::LimitExceeded(String::from(error_message));
                }
                "ResourceNotFoundException" => {
                    return UpdatePipelineError::ResourceNotFound(String::from(error_message));
                }
                "ServiceUnavailableException" => {
                    return UpdatePipelineError::ServiceUnavailable(String::from(error_message));
                }
                "ThrottlingException" => {
                    return UpdatePipelineError::Throttling(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdatePipelineError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdatePipelineError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdatePipelineError {
    fn from(err: serde_json::error::Error) -> UpdatePipelineError {
        UpdatePipelineError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdatePipelineError {
    fn from(err: CredentialsError) -> UpdatePipelineError {
        UpdatePipelineError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdatePipelineError {
    fn from(err: HttpDispatchError) -> UpdatePipelineError {
        UpdatePipelineError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdatePipelineError {
    fn from(err: io::Error) -> UpdatePipelineError {
        UpdatePipelineError::HttpDispatch(HttpDispatchError::from(err))
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
            UpdatePipelineError::Validation(ref cause) => cause,
            UpdatePipelineError::Credentials(ref err) => err.description(),
            UpdatePipelineError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdatePipelineError::ParseError(ref cause) => cause,
            UpdatePipelineError::Unknown(_) => "unknown error",
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

    /// <p><p>Creates a data set. A data set stores data retrieved from a data store by applying an SQL action.</p> <note> <p>This operation creates the skeleton of a data set. To populate the data set, call &quot;CreateDatasetContent&quot;.</p> </note></p>
    fn create_dataset(
        &self,
        input: CreateDatasetRequest,
    ) -> RusotoFuture<CreateDatasetResponse, CreateDatasetError>;

    /// <p>Creates the content of a data set by applying an SQL action.</p>
    fn create_dataset_content(
        &self,
        input: CreateDatasetContentRequest,
    ) -> RusotoFuture<(), CreateDatasetContentError>;

    /// <p>Creates a data store, which is a repository for messages.</p>
    fn create_datastore(
        &self,
        input: CreateDatastoreRequest,
    ) -> RusotoFuture<CreateDatastoreResponse, CreateDatastoreError>;

    /// <p>Creates a pipeline. A pipeline consumes messages from one or more channels and allows you to process the messages before storing them in a data store.</p>
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

    /// <p>Updates the settings of a pipeline.</p>
    fn update_pipeline(
        &self,
        input: UpdatePipelineRequest,
    ) -> RusotoFuture<(), UpdatePipelineError>;
}
/// A client for the AWS IoT Analytics API.
pub struct IotAnalyticsClient {
    client: Client,
    region: region::Region,
}

impl IotAnalyticsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> IotAnalyticsClient {
        IotAnalyticsClient {
            client: Client::shared(),
            region: region,
        }
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
        IotAnalyticsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<BatchPutMessageResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<CancelPipelineReprocessingResponse>(&body)
                            .unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateChannelResponse>(&body).unwrap();

                    result
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

    /// <p><p>Creates a data set. A data set stores data retrieved from a data store by applying an SQL action.</p> <note> <p>This operation creates the skeleton of a data set. To populate the data set, call &quot;CreateDatasetContent&quot;.</p> </note></p>
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDatasetResponse>(&body).unwrap();

                    result
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

    /// <p>Creates the content of a data set by applying an SQL action.</p>
    fn create_dataset_content(
        &self,
        input: CreateDatasetContentRequest,
    ) -> RusotoFuture<(), CreateDatasetContentError> {
        let request_uri = format!(
            "/datasets/{dataset_name}/content",
            dataset_name = input.dataset_name
        );

        let mut request = SignedRequest::new("POST", "iotanalytics", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreateDatastoreResponse>(&body).unwrap();

                    result
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

    /// <p>Creates a pipeline. A pipeline consumes messages from one or more channels and allows you to process the messages before storing them in a data store.</p>
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<CreatePipelineResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeChannelResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribeDatasetResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeDatastoreResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<DescribeLoggingOptionsResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<DescribePipelineResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<GetDatasetContentResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListChannelsResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDatasetsResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListDatastoresResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<ListPipelinesResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListTagsForResourceResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<RunPipelineActivityResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<SampleChannelDataResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<StartPipelineReprocessingResponse>(&body).unwrap();

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<TagResourceResponse>(&body).unwrap();

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
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" || body.is_empty() {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UntagResourceResponse>(&body).unwrap();

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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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

    /// <p>Updates the settings of a pipeline.</p>
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
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
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

#[cfg(test)]
mod protocol_tests {}
