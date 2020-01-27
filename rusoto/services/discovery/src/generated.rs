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
#[allow(warnings)]
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>Information about agents or connectors that were instructed to start collecting data. Information includes the agent/connector ID, a description of the operation, and whether the agent/connector configuration was updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentConfigurationStatus {
    /// <p>The agent/connector ID.</p>
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// <p>A description of the operation performed.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Information about the status of the <code>StartDataCollection</code> and <code>StopDataCollection</code> operations. The system has recorded the data collection operation. The agent/connector receives this command the next time it polls for a new command. </p>
    #[serde(rename = "operationSucceeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_succeeded: Option<bool>,
}

/// <p>Information about agents or connectors associated with the user’s AWS account. Information includes agent or connector IDs, IP addresses, media access control (MAC) addresses, agent or connector health, hostname where the agent or connector resides, and agent version for each agent.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentInfo {
    /// <p>The agent or connector ID.</p>
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// <p>Network details about the host where the agent or connector resides.</p>
    #[serde(rename = "agentNetworkInfoList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_network_info_list: Option<Vec<AgentNetworkInfo>>,
    /// <p>Type of agent.</p>
    #[serde(rename = "agentType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_type: Option<String>,
    /// <p>Status of the collection process for an agent or connector.</p>
    #[serde(rename = "collectionStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_status: Option<String>,
    /// <p>The ID of the connector.</p>
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// <p>The health of the agent or connector.</p>
    #[serde(rename = "health")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    /// <p>The name of the host where the agent or connector resides. The host can be a server or virtual machine.</p>
    #[serde(rename = "hostName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_name: Option<String>,
    /// <p>Time since agent or connector health was reported.</p>
    #[serde(rename = "lastHealthPingTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_ping_time: Option<String>,
    /// <p>Agent's first registration timestamp in UTC.</p>
    #[serde(rename = "registeredTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_time: Option<String>,
    /// <p>The agent or connector version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// <p>Network details about the host where the agent/connector resides.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AgentNetworkInfo {
    /// <p>The IP address for the host where the agent/connector resides.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The MAC address for the host where the agent/connector resides.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct AssociateConfigurationItemsToApplicationRequest {
    /// <p>The configuration ID of an application with which items are to be associated.</p>
    #[serde(rename = "applicationConfigurationId")]
    pub application_configuration_id: String,
    /// <p>The ID of each configuration item to be associated with an application.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct AssociateConfigurationItemsToApplicationResponse {}

/// <p>Error messages returned for each import task that you deleted as a response for this command.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DiscoveryBatchDeleteImportDataError {
    /// <p>The type of error that occurred for a specific import task.</p>
    #[serde(rename = "errorCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    /// <p>The description of the error that occurred for a specific import task.</p>
    #[serde(rename = "errorDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    /// <p>The unique import ID associated with the error that occurred.</p>
    #[serde(rename = "importTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_task_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BatchDeleteImportDataRequest {
    /// <p>The IDs for the import tasks that you want to delete.</p>
    #[serde(rename = "importTaskIds")]
    pub import_task_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct BatchDeleteImportDataResponse {
    /// <p>Error messages returned for each import task that you deleted as a response for this command.</p>
    #[serde(rename = "errors")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<DiscoveryBatchDeleteImportDataError>>,
}

/// <p>Tags for a configuration item. Tags are metadata that help you categorize IT assets.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ConfigurationTag {
    /// <p>The configuration ID for the item to tag. You can specify a list of keys and values.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    /// <p>A type of IT asset to tag.</p>
    #[serde(rename = "configurationType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_type: Option<String>,
    /// <p>A type of tag on which to filter. For example, <i>serverType</i>.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The time the configuration tag was created in Coordinated Universal Time (UTC).</p>
    #[serde(rename = "timeOfCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_of_creation: Option<f64>,
    /// <p>A value on which to filter. For example <i>key = serverType</i> and <i>value = web server</i>.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A list of continuous export descriptions.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ContinuousExportDescription {
    /// <p>The type of data collector used to gather this data (currently only offered for AGENT).</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// <p>The unique ID assigned to this export.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    /// <p>The name of the s3 bucket where the export data parquet files are stored.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p><p>An object which describes how the data is stored.</p> <ul> <li> <p> <code>databaseName</code> - the name of the Glue database used to store the schema.</p> </li> </ul></p>
    #[serde(rename = "schemaStorageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_storage_config: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timestamp representing when the continuous export was started.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p><p>Describes the status of the export. Can be one of the following values:</p> <ul> <li> <p>START<em>IN</em>PROGRESS - setting up resources to start continuous export.</p> </li> <li> <p>START<em>FAILED - an error occurred setting up continuous export. To recover, call start-continuous-export again.</p> </li> <li> <p>ACTIVE - data is being exported to the customer bucket.</p> </li> <li> <p>ERROR - an error occurred during export. To fix the issue, call stop-continuous-export and start-continuous-export.</p> </li> <li> <p>STOP</em>IN<em>PROGRESS - stopping the export.</p> </li> <li> <p>STOP</em>FAILED - an error occurred stopping the export. To recover, call stop-continuous-export again.</p> </li> <li> <p>INACTIVE - the continuous export has been stopped. Data is no longer being exported to the customer bucket.</p> </li> </ul></p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p><p>Contains information about any errors that have occurred. This data type can have the following values:</p> <ul> <li> <p>ACCESS<em>DENIED - You don’t have permission to start Data Exploration in Amazon Athena. Contact your AWS administrator for help. For more information, see <a href="http://docs.aws.amazon.com/application-discovery/latest/userguide/setting-up.html">Setting Up AWS Application Discovery Service</a> in the Application Discovery Service User Guide.</p> </li> <li> <p>DELIVERY</em>STREAM<em>LIMIT</em>FAILURE - You reached the limit for Amazon Kinesis Data Firehose delivery streams. Reduce the number of streams or request a limit increase and try again. For more information, see <a href="http://docs.aws.amazon.com/streams/latest/dev/service-sizes-and-limits.html">Kinesis Data Streams Limits</a> in the Amazon Kinesis Data Streams Developer Guide.</p> </li> <li> <p>FIREHOSE<em>ROLE</em>MISSING - The Data Exploration feature is in an error state because your IAM User is missing the AWSApplicationDiscoveryServiceFirehose role. Turn on Data Exploration in Amazon Athena and try again. For more information, see <a href="http://docs.aws.amazon.com/application-discovery/latest/userguide/setting-up.html#setting-up-user-policy">Step 3: Provide Application Discovery Service Access to Non-Administrator Users by Attaching Policies</a> in the Application Discovery Service User Guide.</p> </li> <li> <p>FIREHOSE<em>STREAM</em>DOES<em>NOT</em>EXIST - The Data Exploration feature is in an error state because your IAM User is missing one or more of the Kinesis data delivery streams.</p> </li> <li> <p>INTERNAL<em>FAILURE - The Data Exploration feature is in an error state because of an internal failure. Try again later. If this problem persists, contact AWS Support.</p> </li> <li> <p>S3</em>BUCKET<em>LIMIT</em>FAILURE - You reached the limit for Amazon S3 buckets. Reduce the number of Amazon S3 buckets or request a limit increase and try again. For more information, see <a href="http://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the Amazon Simple Storage Service Developer Guide.</p> </li> <li> <p>S3<em>NOT</em>SIGNED_UP - Your account is not signed up for the Amazon S3 service. You must sign up before you can use Amazon S3. You can sign up at the following URL: <a href="https://aws.amazon.com/s3">https://aws.amazon.com/s3</a>.</p> </li> </ul></p>
    #[serde(rename = "statusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    /// <p>The timestamp that represents when this continuous export was stopped.</p>
    #[serde(rename = "stopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateApplicationRequest {
    /// <p>Description of the application to be created.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the application to be created.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateApplicationResponse {
    /// <p>Configuration ID of an application to be created.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateTagsRequest {
    /// <p>A list of configuration items that you want to tag.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
    /// <p>Tags that you want to associate with one or more configuration items. Specify the tags that you want to create in a <i>key</i>-<i>value</i> format. For example:</p> <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateTagsResponse {}

/// <p>Inventory data for installed discovery agents.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomerAgentInfo {
    /// <p>Number of active discovery agents.</p>
    #[serde(rename = "activeAgents")]
    pub active_agents: i64,
    /// <p>Number of blacklisted discovery agents.</p>
    #[serde(rename = "blackListedAgents")]
    pub black_listed_agents: i64,
    /// <p>Number of healthy discovery agents</p>
    #[serde(rename = "healthyAgents")]
    pub healthy_agents: i64,
    /// <p>Number of discovery agents with status SHUTDOWN.</p>
    #[serde(rename = "shutdownAgents")]
    pub shutdown_agents: i64,
    /// <p>Total number of discovery agents.</p>
    #[serde(rename = "totalAgents")]
    pub total_agents: i64,
    /// <p>Number of unhealthy discovery agents.</p>
    #[serde(rename = "unhealthyAgents")]
    pub unhealthy_agents: i64,
    /// <p>Number of unknown discovery agents.</p>
    #[serde(rename = "unknownAgents")]
    pub unknown_agents: i64,
}

/// <p>Inventory data for installed discovery connectors.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CustomerConnectorInfo {
    /// <p>Number of active discovery connectors.</p>
    #[serde(rename = "activeConnectors")]
    pub active_connectors: i64,
    /// <p>Number of blacklisted discovery connectors.</p>
    #[serde(rename = "blackListedConnectors")]
    pub black_listed_connectors: i64,
    /// <p>Number of healthy discovery connectors.</p>
    #[serde(rename = "healthyConnectors")]
    pub healthy_connectors: i64,
    /// <p>Number of discovery connectors with status SHUTDOWN,</p>
    #[serde(rename = "shutdownConnectors")]
    pub shutdown_connectors: i64,
    /// <p>Total number of discovery connectors.</p>
    #[serde(rename = "totalConnectors")]
    pub total_connectors: i64,
    /// <p>Number of unhealthy discovery connectors.</p>
    #[serde(rename = "unhealthyConnectors")]
    pub unhealthy_connectors: i64,
    /// <p>Number of unknown discovery connectors.</p>
    #[serde(rename = "unknownConnectors")]
    pub unknown_connectors: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteApplicationsRequest {
    /// <p>Configuration ID of an application to be deleted.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteApplicationsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteTagsRequest {
    /// <p>A list of configuration items with tags that you want to delete.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
    /// <p>Tags that you want to delete from one or more configuration items. Specify the tags that you want to delete in a <i>key</i>-<i>value</i> format. For example:</p> <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteTagsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeAgentsRequest {
    /// <p>The agent or the Connector IDs for which you want information. If you specify no IDs, the system returns information about all agents/Connectors associated with your AWS user account.</p>
    #[serde(rename = "agentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// <p>You can filter the request using various logical operators and a <i>key</i>-<i>value</i> format. For example: </p> <p> <code>{"key": "collectionStatus", "value": "STARTED"}</code> </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The total number of agents/Connectors to return in a single page of output. The maximum value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token to retrieve the next set of results. For example, if you previously specified 100 IDs for <code>DescribeAgentsRequest$agentIds</code> but set <code>DescribeAgentsRequest$maxResults</code> to 10, you received a set of 10 results along with a token. Use that token in this query to get the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeAgentsResponse {
    /// <p>Lists agents or the Connector by ID or lists all agents/Connectors associated with your user account if you did not specify an agent/Connector ID. The output includes agent/Connector IDs, IP addresses, media access control (MAC) addresses, agent/Connector health, host name where the agent/Connector resides, and the version number of each agent/Connector.</p>
    #[serde(rename = "agentsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_info: Option<Vec<AgentInfo>>,
    /// <p>Token to retrieve the next set of results. For example, if you specified 100 IDs for <code>DescribeAgentsRequest$agentIds</code> but set <code>DescribeAgentsRequest$maxResults</code> to 10, you received a set of 10 results along with this token. Use this token in the next query to retrieve the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeConfigurationsRequest {
    /// <p>One or more configuration IDs.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeConfigurationsResponse {
    /// <p>A key in the response map. The value is an array of data.</p>
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<::std::collections::HashMap<String, String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeContinuousExportsRequest {
    /// <p>The unique IDs assigned to the exports.</p>
    #[serde(rename = "exportIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
    /// <p>A number between 1 and 100 specifying the maximum number of continuous export descriptions returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token from the previous call to <code>DescribeExportTasks</code>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeContinuousExportsResponse {
    /// <p>A list of continuous export descriptions.</p>
    #[serde(rename = "descriptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<ContinuousExportDescription>>,
    /// <p>The token from the previous call to <code>DescribeExportTasks</code>.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeExportConfigurationsRequest {
    /// <p>A list of continuous export IDs to search for.</p>
    #[serde(rename = "exportIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
    /// <p>A number between 1 and 100 specifying the maximum number of continuous export descriptions returned.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token from the previous call to describe-export-tasks.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeExportConfigurationsResponse {
    /// <p><p/></p>
    #[serde(rename = "exportsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports_info: Option<Vec<ExportInfo>>,
    /// <p>The token from the previous call to describe-export-tasks.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeExportTasksRequest {
    /// <p>One or more unique identifiers used to query the status of an export request.</p>
    #[serde(rename = "exportIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
    /// <p><p>One or more filters.</p> <ul> <li> <p> <code>AgentId</code> - ID of the agent whose collected data will be exported</p> </li> </ul></p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExportFilter>>,
    /// <p>The maximum number of volume results returned by <code>DescribeExportTasks</code> in paginated output. When this parameter is used, <code>DescribeExportTasks</code> only returns <code>maxResults</code> results in a single page along with a <code>nextToken</code> response element.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The <code>nextToken</code> value returned from a previous paginated <code>DescribeExportTasks</code> request where <code>maxResults</code> was used and the results exceeded the value of that parameter. Pagination continues from the end of the previous results that returned the <code>nextToken</code> value. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeExportTasksResponse {
    /// <p>Contains one or more sets of export request details. When the status of a request is <code>SUCCEEDED</code>, the response includes a URL for an Amazon S3 bucket where you can view the data in a CSV file.</p>
    #[serde(rename = "exportsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports_info: Option<Vec<ExportInfo>>,
    /// <p>The <code>nextToken</code> value to include in a future <code>DescribeExportTasks</code> request. When the results of a <code>DescribeExportTasks</code> request exceed <code>maxResults</code>, this value can be used to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeImportTasksRequest {
    /// <p>An array of name-value pairs that you provide to filter the results for the <code>DescribeImportTask</code> request to a specific subset of results. Currently, wildcard values aren't supported for filters.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ImportTaskFilter>>,
    /// <p>The maximum number of results that you want this request to return, up to 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request a specific page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeImportTasksResponse {
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A returned array of import tasks that match any applied filters, up to the specified number of maximum results.</p>
    #[serde(rename = "tasks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<ImportTask>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeTagsRequest {
    /// <p>You can filter the list using a <i>key</i>-<i>value</i> format. You can separate these items by using logical operators. Allowed filters include <code>tagKey</code>, <code>tagValue</code>, and <code>configurationId</code>. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<TagFilter>>,
    /// <p>The total number of items to return in a single page of output. The maximum value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to start the list. Use this token to get the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeTagsResponse {
    /// <p>The call returns a token. Use this token to get the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Depending on the input, this is a list of configuration items tagged with a specific tag, or a list of tags for a specific configuration item.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<ConfigurationTag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DisassociateConfigurationItemsFromApplicationRequest {
    /// <p>Configuration ID of an application from which each item is disassociated.</p>
    #[serde(rename = "applicationConfigurationId")]
    pub application_configuration_id: String,
    /// <p>Configuration ID of each item to be disassociated from an application.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DisassociateConfigurationItemsFromApplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportConfigurationsResponse {
    /// <p>A unique identifier that you can use to query the export status.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

/// <p>Used to select which agent's data is to be exported. A single agent ID may be selected for export using the <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/API_StartExportTask.html">StartExportTask</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportFilter {
    /// <p>Supported condition: <code>EQUALS</code> </p>
    #[serde(rename = "condition")]
    pub condition: String,
    /// <p>A single <code>ExportFilter</code> name. Supported filters: <code>agentId</code>.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A single <code>agentId</code> for a Discovery Agent. An <code>agentId</code> can be found using the <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/API_DescribeExportTasks.html">DescribeAgents</a> action. Typically an ADS <code>agentId</code> is in the form <code>o-0123456789abcdef0</code>.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

/// <p>Information regarding the export status of discovered data. The value is an array of objects.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportInfo {
    /// <p>A URL for an Amazon S3 bucket where you can review the exported data. The URL is displayed only if the export succeeded.</p>
    #[serde(rename = "configurationsDownloadUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations_download_url: Option<String>,
    /// <p>A unique identifier used to query an export.</p>
    #[serde(rename = "exportId")]
    pub export_id: String,
    /// <p>The time that the data export was initiated.</p>
    #[serde(rename = "exportRequestTime")]
    pub export_request_time: f64,
    /// <p>The status of the data export job.</p>
    #[serde(rename = "exportStatus")]
    pub export_status: String,
    /// <p>If true, the export of agent information exceeded the size limit for a single export and the exported data is incomplete for the requested time range. To address this, select a smaller time range for the export by using <code>startDate</code> and <code>endDate</code>.</p>
    #[serde(rename = "isTruncated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_truncated: Option<bool>,
    /// <p>The <code>endTime</code> used in the <code>StartExportTask</code> request. If no <code>endTime</code> was requested, this result does not appear in <code>ExportInfo</code>.</p>
    #[serde(rename = "requestedEndTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_end_time: Option<f64>,
    /// <p>The value of <code>startTime</code> parameter in the <code>StartExportTask</code> request. If no <code>startTime</code> was requested, this result does not appear in <code>ExportInfo</code>.</p>
    #[serde(rename = "requestedStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_start_time: Option<f64>,
    /// <p>A status message provided for API callers.</p>
    #[serde(rename = "statusMessage")]
    pub status_message: String,
}

/// <p>A filter that can use conditional operators.</p> <p>For more information about filters, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html">Querying Discovered Configuration Items</a> in the <i>AWS Application Discovery Service User Guide</i>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Filter {
    /// <p>A conditional operator. The following operators are valid: EQUALS, NOT_EQUALS, CONTAINS, NOT_CONTAINS. If you specify multiple filters, the system utilizes all filters as though concatenated by <i>AND</i>. If you specify multiple values for a particular filter, the system differentiates the values using <i>OR</i>. Calling either <i>DescribeConfigurations</i> or <i>ListConfigurations</i> returns attributes of matching configuration items.</p>
    #[serde(rename = "condition")]
    pub condition: String,
    /// <p>The name of the filter.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A string value on which to filter. For example, if you choose the <code>destinationServer.osVersion</code> filter name, you could specify <code>Ubuntu</code> for the value.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDiscoverySummaryRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDiscoverySummaryResponse {
    /// <p>Details about discovered agents, including agent status and health.</p>
    #[serde(rename = "agentSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_summary: Option<CustomerAgentInfo>,
    /// <p>The number of applications discovered.</p>
    #[serde(rename = "applications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<i64>,
    /// <p>Details about discovered connectors, including connector status and health.</p>
    #[serde(rename = "connectorSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_summary: Option<CustomerConnectorInfo>,
    /// <p>The number of servers discovered.</p>
    #[serde(rename = "servers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<i64>,
    /// <p>The number of servers mapped to applications.</p>
    #[serde(rename = "serversMappedToApplications")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_mapped_to_applications: Option<i64>,
    /// <p>The number of servers mapped to tags.</p>
    #[serde(rename = "serversMappedtoTags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers_mappedto_tags: Option<i64>,
}

/// <p>An array of information related to the import task request that includes status information, times, IDs, the Amazon S3 Object URL for the import file, and more.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ImportTask {
    /// <p>The total number of application records in the import file that failed to be imported.</p>
    #[serde(rename = "applicationImportFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_import_failure: Option<i64>,
    /// <p>The total number of application records in the import file that were successfully imported.</p>
    #[serde(rename = "applicationImportSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_import_success: Option<i64>,
    /// <p>A unique token used to prevent the same import request from occurring more than once. If you didn't provide a token, a token was automatically generated when the import task request was sent.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p>A link to a compressed archive folder (in the ZIP format) that contains an error log and a file of failed records. You can use these two files to quickly identify records that failed, why they failed, and correct those records. Afterward, you can upload the corrected file to your Amazon S3 bucket and create another import task request.</p> <p>This field also includes authorization information so you can confirm the authenticity of the compressed archive before you download it.</p> <p>If some records failed to be imported we recommend that you correct the records in the failed entries file and then imports that failed entries file. This prevents you from having to correct and update the larger original file and attempt importing it again.</p>
    #[serde(rename = "errorsAndFailedEntriesZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors_and_failed_entries_zip: Option<String>,
    /// <p>The time that the import task request finished, presented in the Unix time stamp format.</p>
    #[serde(rename = "importCompletionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_completion_time: Option<f64>,
    /// <p>The time that the import task request was deleted, presented in the Unix time stamp format.</p>
    #[serde(rename = "importDeletedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_deleted_time: Option<f64>,
    /// <p>The time that the import task request was made, presented in the Unix time stamp format.</p>
    #[serde(rename = "importRequestTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_request_time: Option<f64>,
    /// <p>The unique ID for a specific import task. These IDs aren't globally unique, but they are unique within an AWS account.</p>
    #[serde(rename = "importTaskId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_task_id: Option<String>,
    /// <p>The URL for your import file that you've uploaded to Amazon S3.</p>
    #[serde(rename = "importUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_url: Option<String>,
    /// <p>A descriptive name for an import task. You can use this name to filter future requests related to this import task, such as identifying applications and servers that were included in this import task. We recommend that you use a meaningful name for each import task.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The total number of server records in the import file that failed to be imported.</p>
    #[serde(rename = "serverImportFailure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_import_failure: Option<i64>,
    /// <p>The total number of server records in the import file that were successfully imported.</p>
    #[serde(rename = "serverImportSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_import_success: Option<i64>,
    /// <p>The status of the import task. An import can have the status of <code>IMPORT_COMPLETE</code> and still have some records fail to import from the overall request. More information can be found in the downloadable archive defined in the <code>errorsAndFailedEntriesZip</code> field, or in the Migration Hub management console.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// <p><p>A name-values pair of elements you can use to filter the results when querying your import tasks. Currently, wildcards are not supported for filters.</p> <note> <p>When filtering by import status, all other filter values are ignored.</p> </note></p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ImportTaskFilter {
    /// <p>The name, status, or import task ID for a specific import task.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>An array of strings that you can provide to match against a specific name, status, or import task ID to filter the results for your import task queries.</p>
    #[serde(rename = "values")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListConfigurationsRequest {
    /// <p>A valid configuration identified by Application Discovery Service. </p>
    #[serde(rename = "configurationType")]
    pub configuration_type: String,
    /// <p>You can filter the request using various logical operators and a <i>key</i>-<i>value</i> format. For example: </p> <p> <code>{"key": "serverType", "value": "webServer"}</code> </p> <p>For a complete list of filter options and guidance about using them with this action, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>AWS Application Discovery Service User Guide</i>.</p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    /// <p>The total number of items to return. The maximum value is 100.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>Token to retrieve the next set of results. For example, if a previous call to ListConfigurations returned 100 items, but you set <code>ListConfigurationsRequest$maxResults</code> to 10, you received a set of 10 results along with a token. Use that token in this query to get the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Certain filter criteria return output that can be sorted in ascending or descending order. For a list of output characteristics for each filter, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a> in the <i>AWS Application Discovery Service User Guide</i>.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderByElement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListConfigurationsResponse {
    /// <p>Returns configuration details, including the configuration ID, attribute names, and attribute values.</p>
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<::std::collections::HashMap<String, String>>>,
    /// <p>Token to retrieve the next set of results. For example, if your call to ListConfigurations returned 100 items, but you set <code>ListConfigurationsRequest$maxResults</code> to 10, you received a set of 10 results along with this token. Use this token in the next query to retrieve the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListServerNeighborsRequest {
    /// <p>Configuration ID of the server for which neighbors are being listed.</p>
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    /// <p>Maximum number of results to return in a single page of output.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>List of configuration IDs to test for one-hop-away.</p>
    #[serde(rename = "neighborConfigurationIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub neighbor_configuration_ids: Option<Vec<String>>,
    /// <p>Token to retrieve the next set of results. For example, if you previously specified 100 IDs for <code>ListServerNeighborsRequest$neighborConfigurationIds</code> but set <code>ListServerNeighborsRequest$maxResults</code> to 10, you received a set of 10 results along with a token. Use that token in this query to get the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Flag to indicate if port and protocol information is needed as part of the response.</p>
    #[serde(rename = "portInformationNeeded")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port_information_needed: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListServerNeighborsResponse {
    /// <p>Count of distinct servers that are one hop away from the given server.</p>
    #[serde(rename = "knownDependencyCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub known_dependency_count: Option<i64>,
    /// <p>List of distinct servers that are one hop away from the given server.</p>
    #[serde(rename = "neighbors")]
    pub neighbors: Vec<NeighborConnectionDetail>,
    /// <p>Token to retrieve the next set of results. For example, if you specified 100 IDs for <code>ListServerNeighborsRequest$neighborConfigurationIds</code> but set <code>ListServerNeighborsRequest$maxResults</code> to 10, you received a set of 10 results along with this token. Use this token in the next query to retrieve the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Details about neighboring servers.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct NeighborConnectionDetail {
    /// <p>The number of open network connections with the neighboring server.</p>
    #[serde(rename = "connectionsCount")]
    pub connections_count: i64,
    /// <p>The destination network port for the connection.</p>
    #[serde(rename = "destinationPort")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i64>,
    /// <p>The ID of the server that accepted the network connection.</p>
    #[serde(rename = "destinationServerId")]
    pub destination_server_id: String,
    /// <p>The ID of the server that opened the network connection.</p>
    #[serde(rename = "sourceServerId")]
    pub source_server_id: String,
    /// <p>The network protocol used for the connection.</p>
    #[serde(rename = "transportProtocol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_protocol: Option<String>,
}

/// <p>A field and direction for ordered output.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct OrderByElement {
    /// <p>The field on which to order.</p>
    #[serde(rename = "fieldName")]
    pub field_name: String,
    /// <p>Ordering direction.</p>
    #[serde(rename = "sortOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartContinuousExportRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartContinuousExportResponse {
    /// <p>The type of data collector used to gather this data (currently only offered for AGENT).</p>
    #[serde(rename = "dataSource")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<String>,
    /// <p>The unique ID assigned to this export.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
    /// <p>The name of the s3 bucket where the export data parquet files are stored.</p>
    #[serde(rename = "s3Bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_bucket: Option<String>,
    /// <p><p>A dictionary which describes how the data is stored.</p> <ul> <li> <p> <code>databaseName</code> - the name of the Glue database used to store the schema.</p> </li> </ul></p>
    #[serde(rename = "schemaStorageConfig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_storage_config: Option<::std::collections::HashMap<String, String>>,
    /// <p>The timestamp representing when the continuous export was started.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartDataCollectionByAgentIdsRequest {
    /// <p>The IDs of the agents or connectors from which to start collecting data. If you send a request to an agent/connector ID that you do not have permission to contact, according to your AWS account, the service does not throw an exception. Instead, it returns the error in the <i>Description</i> field. If you send a request to multiple agents/connectors and you do not have permission to contact some of those agents/connectors, the system does not throw an exception. Instead, the system shows <code>Failed</code> in the <i>Description</i> field.</p>
    #[serde(rename = "agentIds")]
    pub agent_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartDataCollectionByAgentIdsResponse {
    /// <p>Information about agents or the connector that were instructed to start collecting data. Information includes the agent/connector ID, a description of the operation performed, and whether the agent/connector configuration was updated.</p>
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartExportTaskRequest {
    /// <p>The end timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, exported data includes the most recent data collected by the agent.</p>
    #[serde(rename = "endTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    /// <p>The file format for the returned export data. Default value is <code>CSV</code>. <b>Note:</b> <i>The</i> <code>GRAPHML</code> <i>option has been deprecated.</i> </p>
    #[serde(rename = "exportDataFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_data_format: Option<Vec<String>>,
    /// <p>If a filter is present, it selects the single <code>agentId</code> of the Application Discovery Agent for which data is exported. The <code>agentId</code> can be found in the results of the <code>DescribeAgents</code> API or CLI. If no filter is present, <code>startTime</code> and <code>endTime</code> are ignored and exported data includes both Agentless Discovery Connector data and summary data from Application Discovery agents. </p>
    #[serde(rename = "filters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<ExportFilter>>,
    /// <p>The start timestamp for exported data from the single Application Discovery Agent selected in the filters. If no value is specified, data is exported starting from the first data collected by the agent.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartExportTaskResponse {
    /// <p>A unique identifier used to query the status of an export request.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartImportTaskRequest {
    /// <p>Optional. A unique token that you can provide to prevent the same import request from occurring more than once. If you don't provide a token, a token is automatically generated.</p> <p>Sending more than one <code>StartImportTask</code> request with the same client request token will return information about the original import task with that client request token.</p>
    #[serde(rename = "clientRequestToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    /// <p><p>The URL for your import file that you&#39;ve uploaded to Amazon S3.</p> <note> <p>If you&#39;re using the AWS CLI, this URL is structured as follows: <code>s3://BucketName/ImportFileName.CSV</code> </p> </note></p>
    #[serde(rename = "importUrl")]
    pub import_url: String,
    /// <p>A descriptive name for this request. You can use this name to filter future requests related to this import task, such as identifying applications and servers that were included in this import task. We recommend that you use a meaningful name for each import task.</p>
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartImportTaskResponse {
    /// <p>An array of information related to the import task request including status information, times, IDs, the Amazon S3 Object URL for the import file, and more. </p>
    #[serde(rename = "task")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<ImportTask>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopContinuousExportRequest {
    /// <p>The unique ID assigned to this export.</p>
    #[serde(rename = "exportId")]
    pub export_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopContinuousExportResponse {
    /// <p>Timestamp that represents when this continuous export started collecting data.</p>
    #[serde(rename = "startTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>Timestamp that represents when this continuous export was stopped.</p>
    #[serde(rename = "stopTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StopDataCollectionByAgentIdsRequest {
    /// <p>The IDs of the agents or connectors from which to stop collecting data.</p>
    #[serde(rename = "agentIds")]
    pub agent_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StopDataCollectionByAgentIdsResponse {
    /// <p>Information about the agents or connector that were instructed to stop collecting data. Information includes the agent/connector ID, a description of the operation performed, and whether the agent/connector configuration was updated.</p>
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

/// <p>Metadata that help you categorize IT assets.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct Tag {
    /// <p>The type of tag on which to filter.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>A value for a tag key on which to filter.</p>
    #[serde(rename = "value")]
    pub value: String,
}

/// <p>The tag filter. Valid names are: <code>tagKey</code>, <code>tagValue</code>, <code>configurationId</code>.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagFilter {
    /// <p>A name of the tag filter.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Values for the tag filter.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateApplicationRequest {
    /// <p>Configuration ID of the application to be updated.</p>
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    /// <p>New description of the application to be updated.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>New name of the application to be updated.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateApplicationResponse {}

/// Errors returned by AssociateConfigurationItemsToApplication
#[derive(Debug, PartialEq)]
pub enum AssociateConfigurationItemsToApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl AssociateConfigurationItemsToApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<AssociateConfigurationItemsToApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        AssociateConfigurationItemsToApplicationError::AuthorizationError(err.msg),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(
                        AssociateConfigurationItemsToApplicationError::HomeRegionNotSet(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        AssociateConfigurationItemsToApplicationError::InvalidParameter(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        AssociateConfigurationItemsToApplicationError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        AssociateConfigurationItemsToApplicationError::ServerInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for AssociateConfigurationItemsToApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssociateConfigurationItemsToApplicationError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateConfigurationItemsToApplicationError::HomeRegionNotSet(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateConfigurationItemsToApplicationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateConfigurationItemsToApplicationError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            AssociateConfigurationItemsToApplicationError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for AssociateConfigurationItemsToApplicationError {}
/// Errors returned by BatchDeleteImportData
#[derive(Debug, PartialEq)]
pub enum BatchDeleteImportDataError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl BatchDeleteImportDataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BatchDeleteImportDataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(BatchDeleteImportDataError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(BatchDeleteImportDataError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(BatchDeleteImportDataError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(BatchDeleteImportDataError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(BatchDeleteImportDataError::ServerInternalError(
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
impl fmt::Display for BatchDeleteImportDataError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BatchDeleteImportDataError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            BatchDeleteImportDataError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            BatchDeleteImportDataError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BatchDeleteImportDataError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            BatchDeleteImportDataError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BatchDeleteImportDataError {}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl CreateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(CreateApplicationError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(CreateApplicationError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateApplicationError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(CreateApplicationError::ServerInternalError(
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
impl fmt::Display for CreateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateApplicationError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateApplicationError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateApplicationError {}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl CreateTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(CreateTagsError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(CreateTagsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateTagsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(CreateTagsError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateTagsError::ResourceNotFound(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(CreateTagsError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CreateTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateTagsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            CreateTagsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            CreateTagsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateTagsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            CreateTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CreateTagsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateTagsError {}
/// Errors returned by DeleteApplications
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DeleteApplicationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteApplicationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DeleteApplicationsError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DeleteApplicationsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteApplicationsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteApplicationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DeleteApplicationsError::ServerInternalError(
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
impl fmt::Display for DeleteApplicationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteApplicationsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DeleteApplicationsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DeleteApplicationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteApplicationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteApplicationsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteApplicationsError {}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DeleteTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DeleteTagsError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DeleteTagsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DeleteTagsError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteTagsError::ResourceNotFound(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DeleteTagsError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteTagsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteTagsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteTagsError {}
/// Errors returned by DescribeAgents
#[derive(Debug, PartialEq)]
pub enum DescribeAgentsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeAgentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAgentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DescribeAgentsError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeAgentsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeAgentsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeAgentsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DescribeAgentsError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeAgentsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeAgentsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeAgentsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeAgentsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeAgentsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeAgentsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeAgentsError {}
/// Errors returned by DescribeConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DescribeConfigurationsError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeConfigurationsError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeConfigurationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DescribeConfigurationsError::ServerInternalError(
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
impl fmt::Display for DescribeConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeConfigurationsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeConfigurationsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeConfigurationsError {}
/// Errors returned by DescribeContinuousExports
#[derive(Debug, PartialEq)]
pub enum DescribeContinuousExportsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeContinuousExportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeContinuousExportsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        DescribeContinuousExportsError::AuthorizationError(err.msg),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeContinuousExportsError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeContinuousExportsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeContinuousExportsError::InvalidParameterValue(err.msg),
                    )
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(
                        DescribeContinuousExportsError::OperationNotPermitted(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeContinuousExportsError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        DescribeContinuousExportsError::ServerInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeContinuousExportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeContinuousExportsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeContinuousExportsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeContinuousExportsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeContinuousExportsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeContinuousExportsError::OperationNotPermitted(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeContinuousExportsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeContinuousExportsError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeContinuousExportsError {}
/// Errors returned by DescribeExportConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeExportConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeExportConfigurationsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeExportConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::AuthorizationError(err.msg),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::HomeRegionNotSet(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::InvalidParameterValue(err.msg),
                    )
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::ResourceNotFound(err.msg),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        DescribeExportConfigurationsError::ServerInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeExportConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExportConfigurationsError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeExportConfigurationsError::HomeRegionNotSet(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeExportConfigurationsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeExportConfigurationsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeExportConfigurationsError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            DescribeExportConfigurationsError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DescribeExportConfigurationsError {}
/// Errors returned by DescribeExportTasks
#[derive(Debug, PartialEq)]
pub enum DescribeExportTasksError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeExportTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExportTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DescribeExportTasksError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeExportTasksError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeExportTasksError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeExportTasksError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DescribeExportTasksError::ServerInternalError(
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
impl fmt::Display for DescribeExportTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeExportTasksError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeExportTasksError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeExportTasksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeExportTasksError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeExportTasksError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeExportTasksError {}
/// Errors returned by DescribeImportTasks
#[derive(Debug, PartialEq)]
pub enum DescribeImportTasksError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeImportTasksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeImportTasksError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DescribeImportTasksError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeImportTasksError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeImportTasksError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeImportTasksError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DescribeImportTasksError::ServerInternalError(
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
impl fmt::Display for DescribeImportTasksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeImportTasksError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeImportTasksError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeImportTasksError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeImportTasksError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeImportTasksError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeImportTasksError {}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DescribeTagsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeTagsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(DescribeTagsError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(DescribeTagsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeTagsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(DescribeTagsError::InvalidParameterValue(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeTagsError::ResourceNotFound(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(DescribeTagsError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeTagsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeTagsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeTagsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeTagsError {}
/// Errors returned by DisassociateConfigurationItemsFromApplication
#[derive(Debug, PartialEq)]
pub enum DisassociateConfigurationItemsFromApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl DisassociateConfigurationItemsFromApplicationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DisassociateConfigurationItemsFromApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        DisassociateConfigurationItemsFromApplicationError::AuthorizationError(
                            err.msg,
                        ),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(
                        DisassociateConfigurationItemsFromApplicationError::HomeRegionNotSet(
                            err.msg,
                        ),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        DisassociateConfigurationItemsFromApplicationError::InvalidParameter(
                            err.msg,
                        ),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        DisassociateConfigurationItemsFromApplicationError::InvalidParameterValue(
                            err.msg,
                        ),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        DisassociateConfigurationItemsFromApplicationError::ServerInternalError(
                            err.msg,
                        ),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DisassociateConfigurationItemsFromApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DisassociateConfigurationItemsFromApplicationError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateConfigurationItemsFromApplicationError::HomeRegionNotSet(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateConfigurationItemsFromApplicationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            DisassociateConfigurationItemsFromApplicationError::InvalidParameterValue(
                ref cause,
            ) => write!(f, "{}", cause),
            DisassociateConfigurationItemsFromApplicationError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for DisassociateConfigurationItemsFromApplicationError {}
/// Errors returned by ExportConfigurations
#[derive(Debug, PartialEq)]
pub enum ExportConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl ExportConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(ExportConfigurationsError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(ExportConfigurationsError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ExportConfigurationsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ExportConfigurationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(ExportConfigurationsError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(ExportConfigurationsError::ServerInternalError(
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
impl fmt::Display for ExportConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportConfigurationsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ExportConfigurationsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            ExportConfigurationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ExportConfigurationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ExportConfigurationsError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            ExportConfigurationsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportConfigurationsError {}
/// Errors returned by GetDiscoverySummary
#[derive(Debug, PartialEq)]
pub enum GetDiscoverySummaryError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl GetDiscoverySummaryError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDiscoverySummaryError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(GetDiscoverySummaryError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(GetDiscoverySummaryError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDiscoverySummaryError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(GetDiscoverySummaryError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(GetDiscoverySummaryError::ServerInternalError(
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
impl fmt::Display for GetDiscoverySummaryError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDiscoverySummaryError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            GetDiscoverySummaryError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            GetDiscoverySummaryError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDiscoverySummaryError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            GetDiscoverySummaryError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDiscoverySummaryError {}
/// Errors returned by ListConfigurations
#[derive(Debug, PartialEq)]
pub enum ListConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl ListConfigurationsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListConfigurationsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(ListConfigurationsError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(ListConfigurationsError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListConfigurationsError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListConfigurationsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListConfigurationsError::ResourceNotFound(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(ListConfigurationsError::ServerInternalError(
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
impl fmt::Display for ListConfigurationsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListConfigurationsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListConfigurationsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListConfigurationsError {}
/// Errors returned by ListServerNeighbors
#[derive(Debug, PartialEq)]
pub enum ListServerNeighborsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl ListServerNeighborsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListServerNeighborsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(ListServerNeighborsError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(ListServerNeighborsError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListServerNeighborsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(ListServerNeighborsError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(ListServerNeighborsError::ServerInternalError(
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
impl fmt::Display for ListServerNeighborsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListServerNeighborsError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            ListServerNeighborsError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            ListServerNeighborsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListServerNeighborsError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            ListServerNeighborsError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListServerNeighborsError {}
/// Errors returned by StartContinuousExport
#[derive(Debug, PartialEq)]
pub enum StartContinuousExportError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p><p/></p>
    ConflictError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This issue occurs when the same <code>clientRequestToken</code> is used with the <code>StartImportTask</code> action, but with different parameters. For example, you use the same request token but have two different import URLs, you can encounter this issue. If the import tasks are meant to be different, use a different <code>clientRequestToken</code>, and try again.</p>
    ResourceInUse(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StartContinuousExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartContinuousExportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(StartContinuousExportError::AuthorizationError(
                        err.msg,
                    ))
                }
                "ConflictErrorException" => {
                    return RusotoError::Service(StartContinuousExportError::ConflictError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(StartContinuousExportError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartContinuousExportError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartContinuousExportError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartContinuousExportError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartContinuousExportError::ResourceInUse(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(StartContinuousExportError::ServerInternalError(
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
impl fmt::Display for StartContinuousExportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartContinuousExportError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::ConflictError(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartContinuousExportError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartContinuousExportError {}
/// Errors returned by StartDataCollectionByAgentIds
#[derive(Debug, PartialEq)]
pub enum StartDataCollectionByAgentIdsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StartDataCollectionByAgentIdsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StartDataCollectionByAgentIdsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        StartDataCollectionByAgentIdsError::AuthorizationError(err.msg),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(
                        StartDataCollectionByAgentIdsError::HomeRegionNotSet(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StartDataCollectionByAgentIdsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        StartDataCollectionByAgentIdsError::InvalidParameterValue(err.msg),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        StartDataCollectionByAgentIdsError::ServerInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartDataCollectionByAgentIdsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartDataCollectionByAgentIdsError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDataCollectionByAgentIdsError::HomeRegionNotSet(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDataCollectionByAgentIdsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDataCollectionByAgentIdsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            StartDataCollectionByAgentIdsError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StartDataCollectionByAgentIdsError {}
/// Errors returned by StartExportTask
#[derive(Debug, PartialEq)]
pub enum StartExportTaskError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StartExportTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartExportTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(StartExportTaskError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(StartExportTaskError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartExportTaskError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartExportTaskError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StartExportTaskError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(StartExportTaskError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartExportTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartExportTaskError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            StartExportTaskError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            StartExportTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartExportTaskError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartExportTaskError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StartExportTaskError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartExportTaskError {}
/// Errors returned by StartImportTask
#[derive(Debug, PartialEq)]
pub enum StartImportTaskError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This issue occurs when the same <code>clientRequestToken</code> is used with the <code>StartImportTask</code> action, but with different parameters. For example, you use the same request token but have two different import URLs, you can encounter this issue. If the import tasks are meant to be different, use a different <code>clientRequestToken</code>, and try again.</p>
    ResourceInUse(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StartImportTaskError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartImportTaskError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(StartImportTaskError::AuthorizationError(err.msg))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(StartImportTaskError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StartImportTaskError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StartImportTaskError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StartImportTaskError::ResourceInUse(err.msg))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(StartImportTaskError::ServerInternalError(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartImportTaskError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartImportTaskError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            StartImportTaskError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            StartImportTaskError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StartImportTaskError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StartImportTaskError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StartImportTaskError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartImportTaskError {}
/// Errors returned by StopContinuousExport
#[derive(Debug, PartialEq)]
pub enum StopContinuousExportError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>This issue occurs when the same <code>clientRequestToken</code> is used with the <code>StartImportTask</code> action, but with different parameters. For example, you use the same request token but have two different import URLs, you can encounter this issue. If the import tasks are meant to be different, use a different <code>clientRequestToken</code>, and try again.</p>
    ResourceInUse(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StopContinuousExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopContinuousExportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(StopContinuousExportError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(StopContinuousExportError::HomeRegionNotSet(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(StopContinuousExportError::InvalidParameter(
                        err.msg,
                    ))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(StopContinuousExportError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "OperationNotPermittedException" => {
                    return RusotoError::Service(StopContinuousExportError::OperationNotPermitted(
                        err.msg,
                    ))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(StopContinuousExportError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StopContinuousExportError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(StopContinuousExportError::ServerInternalError(
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
impl fmt::Display for StopContinuousExportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopContinuousExportError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::OperationNotPermitted(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StopContinuousExportError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StopContinuousExportError {}
/// Errors returned by StopDataCollectionByAgentIds
#[derive(Debug, PartialEq)]
pub enum StopDataCollectionByAgentIdsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl StopDataCollectionByAgentIdsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<StopDataCollectionByAgentIdsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(
                        StopDataCollectionByAgentIdsError::AuthorizationError(err.msg),
                    )
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(
                        StopDataCollectionByAgentIdsError::HomeRegionNotSet(err.msg),
                    )
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        StopDataCollectionByAgentIdsError::InvalidParameter(err.msg),
                    )
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(
                        StopDataCollectionByAgentIdsError::InvalidParameterValue(err.msg),
                    )
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(
                        StopDataCollectionByAgentIdsError::ServerInternalError(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StopDataCollectionByAgentIdsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StopDataCollectionByAgentIdsError::AuthorizationError(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDataCollectionByAgentIdsError::HomeRegionNotSet(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDataCollectionByAgentIdsError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDataCollectionByAgentIdsError::InvalidParameterValue(ref cause) => {
                write!(f, "{}", cause)
            }
            StopDataCollectionByAgentIdsError::ServerInternalError(ref cause) => {
                write!(f, "{}", cause)
            }
        }
    }
}
impl Error for StopDataCollectionByAgentIdsError {}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>The home region is not set. Set the home region to continue.</p>
    HomeRegionNotSet(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
}

impl UpdateApplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateApplicationError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AuthorizationErrorException" => {
                    return RusotoError::Service(UpdateApplicationError::AuthorizationError(
                        err.msg,
                    ))
                }
                "HomeRegionNotSetException" => {
                    return RusotoError::Service(UpdateApplicationError::HomeRegionNotSet(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidParameter(err.msg))
                }
                "InvalidParameterValueException" => {
                    return RusotoError::Service(UpdateApplicationError::InvalidParameterValue(
                        err.msg,
                    ))
                }
                "ServerInternalErrorException" => {
                    return RusotoError::Service(UpdateApplicationError::ServerInternalError(
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
impl fmt::Display for UpdateApplicationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateApplicationError::AuthorizationError(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::HomeRegionNotSet(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::InvalidParameterValue(ref cause) => write!(f, "{}", cause),
            UpdateApplicationError::ServerInternalError(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateApplicationError {}
/// Trait representing the capabilities of the AWS Application Discovery Service API. AWS Application Discovery Service clients implement this trait.
#[async_trait]
pub trait Discovery {
    /// <p>Associates one or more configuration items with an application.</p>
    async fn associate_configuration_items_to_application(
        &self,
        input: AssociateConfigurationItemsToApplicationRequest,
    ) -> Result<
        AssociateConfigurationItemsToApplicationResponse,
        RusotoError<AssociateConfigurationItemsToApplicationError>,
    >;

    /// <p>Deletes one or more import tasks, each identified by their import ID. Each import task has a number of records that can identify servers or applications. </p> <p>AWS Application Discovery Service has built-in matching logic that will identify when discovered servers match existing entries that you've previously discovered, the information for the already-existing discovered server is updated. When you delete an import task that contains records that were used to match, the information in those matched records that comes from the deleted records will also be deleted.</p>
    async fn batch_delete_import_data(
        &self,
        input: BatchDeleteImportDataRequest,
    ) -> Result<BatchDeleteImportDataResponse, RusotoError<BatchDeleteImportDataError>>;

    /// <p>Creates an application with the given name and description.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>>;

    /// <p>Creates one or more tags for configuration items. Tags are metadata that help you categorize IT assets. This API accepts a list of multiple configuration items.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<CreateTagsResponse, RusotoError<CreateTagsError>>;

    /// <p>Deletes a list of applications and their associations with configuration items.</p>
    async fn delete_applications(
        &self,
        input: DeleteApplicationsRequest,
    ) -> Result<DeleteApplicationsResponse, RusotoError<DeleteApplicationsError>>;

    /// <p>Deletes the association between configuration items and one or more tags. This API accepts a list of multiple configuration items.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<DeleteTagsResponse, RusotoError<DeleteTagsError>>;

    /// <p>Lists agents or connectors as specified by ID or other filters. All agents/connectors associated with your user account can be listed if you call <code>DescribeAgents</code> as is without passing any parameters.</p>
    async fn describe_agents(
        &self,
        input: DescribeAgentsRequest,
    ) -> Result<DescribeAgentsResponse, RusotoError<DescribeAgentsError>>;

    /// <p><p>Retrieves attributes for a list of configuration item IDs.</p> <note> <p>All of the supplied IDs must be for the same asset type from one of the following:</p> <ul> <li> <p>server</p> </li> <li> <p>application</p> </li> <li> <p>process</p> </li> <li> <p>connection</p> </li> </ul> <p>Output fields are specific to the asset type specified. For example, the output for a <i>server</i> configuration item includes a list of attributes about the server, such as host name, operating system, number of network cards, etc.</p> <p>For a complete list of outputs for each asset type, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#DescribeConfigurations">Using the DescribeConfigurations Action</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> </note></p>
    async fn describe_configurations(
        &self,
        input: DescribeConfigurationsRequest,
    ) -> Result<DescribeConfigurationsResponse, RusotoError<DescribeConfigurationsError>>;

    /// <p>Lists exports as specified by ID. All continuous exports associated with your user account can be listed if you call <code>DescribeContinuousExports</code> as is without passing any parameters.</p>
    async fn describe_continuous_exports(
        &self,
        input: DescribeContinuousExportsRequest,
    ) -> Result<DescribeContinuousExportsResponse, RusotoError<DescribeContinuousExportsError>>;

    /// <p> <code>DescribeExportConfigurations</code> is deprecated. Use <a href="https://docs.aws.amazon.com/application-discovery/latest/APIReference/API_DescribeExportTasks.html">DescribeImportTasks</a>, instead.</p>
    async fn describe_export_configurations(
        &self,
        input: DescribeExportConfigurationsRequest,
    ) -> Result<DescribeExportConfigurationsResponse, RusotoError<DescribeExportConfigurationsError>>;

    /// <p>Retrieve status of one or more export tasks. You can retrieve the status of up to 100 export tasks.</p>
    async fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> Result<DescribeExportTasksResponse, RusotoError<DescribeExportTasksError>>;

    /// <p>Returns an array of import tasks for your account, including status information, times, IDs, the Amazon S3 Object URL for the import file, and more.</p>
    async fn describe_import_tasks(
        &self,
        input: DescribeImportTasksRequest,
    ) -> Result<DescribeImportTasksResponse, RusotoError<DescribeImportTasksError>>;

    /// <p>Retrieves a list of configuration items that have tags as specified by the key-value pairs, name and value, passed to the optional parameter <code>filters</code>.</p> <p>There are three valid tag filter names:</p> <ul> <li> <p>tagKey</p> </li> <li> <p>tagValue</p> </li> <li> <p>configurationId</p> </li> </ul> <p>Also, all configuration items associated with your user account that have tags can be listed if you call <code>DescribeTags</code> as is without passing any parameters.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>>;

    /// <p>Disassociates one or more configuration items from an application.</p>
    async fn disassociate_configuration_items_from_application(
        &self,
        input: DisassociateConfigurationItemsFromApplicationRequest,
    ) -> Result<
        DisassociateConfigurationItemsFromApplicationResponse,
        RusotoError<DisassociateConfigurationItemsFromApplicationError>,
    >;

    /// <p>Deprecated. Use <code>StartExportTask</code> instead.</p> <p>Exports all discovered configuration data to an Amazon S3 bucket or an application that enables you to view and evaluate the data. Data includes tags and tag associations, processes, connections, servers, and system performance. This API returns an export ID that you can query using the <i>DescribeExportConfigurations</i> API. The system imposes a limit of two configuration exports in six hours.</p>
    async fn export_configurations(
        &self,
    ) -> Result<ExportConfigurationsResponse, RusotoError<ExportConfigurationsError>>;

    /// <p>Retrieves a short summary of discovered assets.</p> <p>This API operation takes no request parameters and is called as is at the command prompt as shown in the example.</p>
    async fn get_discovery_summary(
        &self,
    ) -> Result<GetDiscoverySummaryResponse, RusotoError<GetDiscoverySummaryError>>;

    /// <p>Retrieves a list of configuration items as specified by the value passed to the required parameter <code>configurationType</code>. Optional filtering may be applied to refine search results.</p>
    async fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> Result<ListConfigurationsResponse, RusotoError<ListConfigurationsError>>;

    /// <p>Retrieves a list of servers that are one network hop away from a specified server.</p>
    async fn list_server_neighbors(
        &self,
        input: ListServerNeighborsRequest,
    ) -> Result<ListServerNeighborsResponse, RusotoError<ListServerNeighborsError>>;

    /// <p>Start the continuous flow of agent's discovered data into Amazon Athena.</p>
    async fn start_continuous_export(
        &self,
    ) -> Result<StartContinuousExportResponse, RusotoError<StartContinuousExportError>>;

    /// <p>Instructs the specified agents or connectors to start collecting data.</p>
    async fn start_data_collection_by_agent_ids(
        &self,
        input: StartDataCollectionByAgentIdsRequest,
    ) -> Result<
        StartDataCollectionByAgentIdsResponse,
        RusotoError<StartDataCollectionByAgentIdsError>,
    >;

    /// <p> Begins the export of discovered data to an S3 bucket.</p> <p> If you specify <code>agentIds</code> in a filter, the task exports up to 72 hours of detailed data collected by the identified Application Discovery Agent, including network, process, and performance details. A time range for exported agent data may be set by using <code>startTime</code> and <code>endTime</code>. Export of detailed agent data is limited to five concurrently running exports. </p> <p> If you do not include an <code>agentIds</code> filter, summary data is exported that includes both AWS Agentless Discovery Connector data and summary data from AWS Discovery Agents. Export of summary data is limited to two exports per day. </p>
    async fn start_export_task(
        &self,
        input: StartExportTaskRequest,
    ) -> Result<StartExportTaskResponse, RusotoError<StartExportTaskError>>;

    /// <p><p>Starts an import task, which allows you to import details of your on-premises environment directly into AWS Migration Hub without having to use the Application Discovery Service (ADS) tools such as the Discovery Connector or Discovery Agent. This gives you the option to perform migration assessment and planning directly from your imported data, including the ability to group your devices as applications and track their migration status.</p> <p>To start an import request, do this:</p> <ol> <li> <p>Download the specially formatted comma separated value (CSV) import template, which you can find here: <a href="https://s3-us-west-2.amazonaws.com/templates-7cffcf56-bd96-4b1c-b45b-a5b42f282e46/import_template.csv">https://s3-us-west-2.amazonaws.com/templates-7cffcf56-bd96-4b1c-b45b-a5b42f282e46/import<em>template.csv</a>.</p> </li> <li> <p>Fill out the template with your server and application data.</p> </li> <li> <p>Upload your import file to an Amazon S3 bucket, and make a note of it&#39;s Object URL. Your import file must be in the CSV format.</p> </li> <li> <p>Use the console or the <code>StartImportTask</code> command with the AWS CLI or one of the AWS SDKs to import the records from your file.</p> </li> </ol> <p>For more information, including step-by-step procedures, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-import.html">Migration Hub Import</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> <note> <p>There are limits to the number of import tasks you can create (and delete) in an AWS account. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/application-discovery/latest/userguide/ads</em>service_limits.html&quot;&gt;AWS Application Discovery Service Limits</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> </note></p>
    async fn start_import_task(
        &self,
        input: StartImportTaskRequest,
    ) -> Result<StartImportTaskResponse, RusotoError<StartImportTaskError>>;

    /// <p>Stop the continuous flow of agent's discovered data into Amazon Athena.</p>
    async fn stop_continuous_export(
        &self,
        input: StopContinuousExportRequest,
    ) -> Result<StopContinuousExportResponse, RusotoError<StopContinuousExportError>>;

    /// <p>Instructs the specified agents or connectors to stop collecting data.</p>
    async fn stop_data_collection_by_agent_ids(
        &self,
        input: StopDataCollectionByAgentIdsRequest,
    ) -> Result<StopDataCollectionByAgentIdsResponse, RusotoError<StopDataCollectionByAgentIdsError>>;

    /// <p>Updates metadata about an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>>;
}
/// A client for the AWS Application Discovery Service API.
#[derive(Clone)]
pub struct DiscoveryClient {
    client: Client,
    region: region::Region,
}

impl DiscoveryClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> DiscoveryClient {
        DiscoveryClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> DiscoveryClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        DiscoveryClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> DiscoveryClient {
        DiscoveryClient { client, region }
    }
}

#[async_trait]
impl Discovery for DiscoveryClient {
    /// <p>Associates one or more configuration items with an application.</p>
    async fn associate_configuration_items_to_application(
        &self,
        input: AssociateConfigurationItemsToApplicationRequest,
    ) -> Result<
        AssociateConfigurationItemsToApplicationResponse,
        RusotoError<AssociateConfigurationItemsToApplicationError>,
    > {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.AssociateConfigurationItemsToApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<AssociateConfigurationItemsToApplicationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(AssociateConfigurationItemsToApplicationError::from_response(response))
        }
    }

    /// <p>Deletes one or more import tasks, each identified by their import ID. Each import task has a number of records that can identify servers or applications. </p> <p>AWS Application Discovery Service has built-in matching logic that will identify when discovered servers match existing entries that you've previously discovered, the information for the already-existing discovered server is updated. When you delete an import task that contains records that were used to match, the information in those matched records that comes from the deleted records will also be deleted.</p>
    async fn batch_delete_import_data(
        &self,
        input: BatchDeleteImportDataRequest,
    ) -> Result<BatchDeleteImportDataResponse, RusotoError<BatchDeleteImportDataError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.BatchDeleteImportData",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<BatchDeleteImportDataResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(BatchDeleteImportDataError::from_response(response))
        }
    }

    /// <p>Creates an application with the given name and description.</p>
    async fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> Result<CreateApplicationResponse, RusotoError<CreateApplicationError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.CreateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateApplicationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateApplicationError::from_response(response))
        }
    }

    /// <p>Creates one or more tags for configuration items. Tags are metadata that help you categorize IT assets. This API accepts a list of multiple configuration items.</p>
    async fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> Result<CreateTagsResponse, RusotoError<CreateTagsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPoseidonService_V2015_11_01.CreateTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<CreateTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(CreateTagsError::from_response(response))
        }
    }

    /// <p>Deletes a list of applications and their associations with configuration items.</p>
    async fn delete_applications(
        &self,
        input: DeleteApplicationsRequest,
    ) -> Result<DeleteApplicationsResponse, RusotoError<DeleteApplicationsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DeleteApplications",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteApplicationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteApplicationsError::from_response(response))
        }
    }

    /// <p>Deletes the association between configuration items and one or more tags. This API accepts a list of multiple configuration items.</p>
    async fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> Result<DeleteTagsResponse, RusotoError<DeleteTagsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPoseidonService_V2015_11_01.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DeleteTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteTagsError::from_response(response))
        }
    }

    /// <p>Lists agents or connectors as specified by ID or other filters. All agents/connectors associated with your user account can be listed if you call <code>DescribeAgents</code> as is without passing any parameters.</p>
    async fn describe_agents(
        &self,
        input: DescribeAgentsRequest,
    ) -> Result<DescribeAgentsResponse, RusotoError<DescribeAgentsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeAgents",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeAgentsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeAgentsError::from_response(response))
        }
    }

    /// <p><p>Retrieves attributes for a list of configuration item IDs.</p> <note> <p>All of the supplied IDs must be for the same asset type from one of the following:</p> <ul> <li> <p>server</p> </li> <li> <p>application</p> </li> <li> <p>process</p> </li> <li> <p>connection</p> </li> </ul> <p>Output fields are specific to the asset type specified. For example, the output for a <i>server</i> configuration item includes a list of attributes about the server, such as host name, operating system, number of network cards, etc.</p> <p>For a complete list of outputs for each asset type, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-api-queries.html#DescribeConfigurations">Using the DescribeConfigurations Action</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> </note></p>
    async fn describe_configurations(
        &self,
        input: DescribeConfigurationsRequest,
    ) -> Result<DescribeConfigurationsResponse, RusotoError<DescribeConfigurationsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeConfigurationsError::from_response(response))
        }
    }

    /// <p>Lists exports as specified by ID. All continuous exports associated with your user account can be listed if you call <code>DescribeContinuousExports</code> as is without passing any parameters.</p>
    async fn describe_continuous_exports(
        &self,
        input: DescribeContinuousExportsRequest,
    ) -> Result<DescribeContinuousExportsResponse, RusotoError<DescribeContinuousExportsError>>
    {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeContinuousExports",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeContinuousExportsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeContinuousExportsError::from_response(response))
        }
    }

    /// <p> <code>DescribeExportConfigurations</code> is deprecated. Use <a href="https://docs.aws.amazon.com/application-discovery/latest/APIReference/API_DescribeExportTasks.html">DescribeImportTasks</a>, instead.</p>
    async fn describe_export_configurations(
        &self,
        input: DescribeExportConfigurationsRequest,
    ) -> Result<DescribeExportConfigurationsResponse, RusotoError<DescribeExportConfigurationsError>>
    {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeExportConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeExportConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeExportConfigurationsError::from_response(response))
        }
    }

    /// <p>Retrieve status of one or more export tasks. You can retrieve the status of up to 100 export tasks.</p>
    async fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> Result<DescribeExportTasksResponse, RusotoError<DescribeExportTasksError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeExportTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeExportTasksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeExportTasksError::from_response(response))
        }
    }

    /// <p>Returns an array of import tasks for your account, including status information, times, IDs, the Amazon S3 Object URL for the import file, and more.</p>
    async fn describe_import_tasks(
        &self,
        input: DescribeImportTasksRequest,
    ) -> Result<DescribeImportTasksResponse, RusotoError<DescribeImportTasksError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeImportTasks",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeImportTasksResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeImportTasksError::from_response(response))
        }
    }

    /// <p>Retrieves a list of configuration items that have tags as specified by the key-value pairs, name and value, passed to the optional parameter <code>filters</code>.</p> <p>There are three valid tag filter names:</p> <ul> <li> <p>tagKey</p> </li> <li> <p>tagValue</p> </li> <li> <p>configurationId</p> </li> </ul> <p>Also, all configuration items associated with your user account that have tags can be listed if you call <code>DescribeTags</code> as is without passing any parameters.</p>
    async fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> Result<DescribeTagsResponse, RusotoError<DescribeTagsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeTags",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<DescribeTagsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeTagsError::from_response(response))
        }
    }

    /// <p>Disassociates one or more configuration items from an application.</p>
    async fn disassociate_configuration_items_from_application(
        &self,
        input: DisassociateConfigurationItemsFromApplicationRequest,
    ) -> Result<
        DisassociateConfigurationItemsFromApplicationResponse,
        RusotoError<DisassociateConfigurationItemsFromApplicationError>,
    > {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DisassociateConfigurationItemsFromApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<DisassociateConfigurationItemsFromApplicationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(DisassociateConfigurationItemsFromApplicationError::from_response(response))
        }
    }

    /// <p>Deprecated. Use <code>StartExportTask</code> instead.</p> <p>Exports all discovered configuration data to an Amazon S3 bucket or an application that enables you to view and evaluate the data. Data includes tags and tag associations, processes, connections, servers, and system performance. This API returns an export ID that you can query using the <i>DescribeExportConfigurations</i> API. The system imposes a limit of two configuration exports in six hours.</p>
    async fn export_configurations(
        &self,
    ) -> Result<ExportConfigurationsResponse, RusotoError<ExportConfigurationsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ExportConfigurations",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ExportConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ExportConfigurationsError::from_response(response))
        }
    }

    /// <p>Retrieves a short summary of discovered assets.</p> <p>This API operation takes no request parameters and is called as is at the command prompt as shown in the example.</p>
    async fn get_discovery_summary(
        &self,
    ) -> Result<GetDiscoverySummaryResponse, RusotoError<GetDiscoverySummaryError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.GetDiscoverySummary",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDiscoverySummaryResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(GetDiscoverySummaryError::from_response(response))
        }
    }

    /// <p>Retrieves a list of configuration items as specified by the value passed to the required parameter <code>configurationType</code>. Optional filtering may be applied to refine search results.</p>
    async fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> Result<ListConfigurationsResponse, RusotoError<ListConfigurationsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ListConfigurations",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListConfigurationsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListConfigurationsError::from_response(response))
        }
    }

    /// <p>Retrieves a list of servers that are one network hop away from a specified server.</p>
    async fn list_server_neighbors(
        &self,
        input: ListServerNeighborsRequest,
    ) -> Result<ListServerNeighborsResponse, RusotoError<ListServerNeighborsError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ListServerNeighbors",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<ListServerNeighborsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(ListServerNeighborsError::from_response(response))
        }
    }

    /// <p>Start the continuous flow of agent's discovered data into Amazon Athena.</p>
    async fn start_continuous_export(
        &self,
    ) -> Result<StartContinuousExportResponse, RusotoError<StartContinuousExportError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartContinuousExport",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartContinuousExportResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartContinuousExportError::from_response(response))
        }
    }

    /// <p>Instructs the specified agents or connectors to start collecting data.</p>
    async fn start_data_collection_by_agent_ids(
        &self,
        input: StartDataCollectionByAgentIdsRequest,
    ) -> Result<
        StartDataCollectionByAgentIdsResponse,
        RusotoError<StartDataCollectionByAgentIdsError>,
    > {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartDataCollectionByAgentIds",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StartDataCollectionByAgentIdsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartDataCollectionByAgentIdsError::from_response(response))
        }
    }

    /// <p> Begins the export of discovered data to an S3 bucket.</p> <p> If you specify <code>agentIds</code> in a filter, the task exports up to 72 hours of detailed data collected by the identified Application Discovery Agent, including network, process, and performance details. A time range for exported agent data may be set by using <code>startTime</code> and <code>endTime</code>. Export of detailed agent data is limited to five concurrently running exports. </p> <p> If you do not include an <code>agentIds</code> filter, summary data is exported that includes both AWS Agentless Discovery Connector data and summary data from AWS Discovery Agents. Export of summary data is limited to two exports per day. </p>
    async fn start_export_task(
        &self,
        input: StartExportTaskRequest,
    ) -> Result<StartExportTaskResponse, RusotoError<StartExportTaskError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartExportTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartExportTaskResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartExportTaskError::from_response(response))
        }
    }

    /// <p><p>Starts an import task, which allows you to import details of your on-premises environment directly into AWS Migration Hub without having to use the Application Discovery Service (ADS) tools such as the Discovery Connector or Discovery Agent. This gives you the option to perform migration assessment and planning directly from your imported data, including the ability to group your devices as applications and track their migration status.</p> <p>To start an import request, do this:</p> <ol> <li> <p>Download the specially formatted comma separated value (CSV) import template, which you can find here: <a href="https://s3-us-west-2.amazonaws.com/templates-7cffcf56-bd96-4b1c-b45b-a5b42f282e46/import_template.csv">https://s3-us-west-2.amazonaws.com/templates-7cffcf56-bd96-4b1c-b45b-a5b42f282e46/import<em>template.csv</a>.</p> </li> <li> <p>Fill out the template with your server and application data.</p> </li> <li> <p>Upload your import file to an Amazon S3 bucket, and make a note of it&#39;s Object URL. Your import file must be in the CSV format.</p> </li> <li> <p>Use the console or the <code>StartImportTask</code> command with the AWS CLI or one of the AWS SDKs to import the records from your file.</p> </li> </ol> <p>For more information, including step-by-step procedures, see <a href="https://docs.aws.amazon.com/application-discovery/latest/userguide/discovery-import.html">Migration Hub Import</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> <note> <p>There are limits to the number of import tasks you can create (and delete) in an AWS account. For more information, see &lt;a href=&quot;https://docs.aws.amazon.com/application-discovery/latest/userguide/ads</em>service_limits.html&quot;&gt;AWS Application Discovery Service Limits</a> in the <i>AWS Application Discovery Service User Guide</i>.</p> </note></p>
    async fn start_import_task(
        &self,
        input: StartImportTaskRequest,
    ) -> Result<StartImportTaskResponse, RusotoError<StartImportTaskError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartImportTask",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response).deserialize::<StartImportTaskResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StartImportTaskError::from_response(response))
        }
    }

    /// <p>Stop the continuous flow of agent's discovered data into Amazon Athena.</p>
    async fn stop_continuous_export(
        &self,
        input: StopContinuousExportRequest,
    ) -> Result<StopContinuousExportResponse, RusotoError<StopContinuousExportError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StopContinuousExport",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StopContinuousExportResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopContinuousExportError::from_response(response))
        }
    }

    /// <p>Instructs the specified agents or connectors to stop collecting data.</p>
    async fn stop_data_collection_by_agent_ids(
        &self,
        input: StopDataCollectionByAgentIdsRequest,
    ) -> Result<StopDataCollectionByAgentIdsResponse, RusotoError<StopDataCollectionByAgentIdsError>>
    {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StopDataCollectionByAgentIds",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<StopDataCollectionByAgentIdsResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(StopDataCollectionByAgentIdsError::from_response(response))
        }
    }

    /// <p>Updates metadata about an application.</p>
    async fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> Result<UpdateApplicationResponse, RusotoError<UpdateApplicationError>> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.UpdateApplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateApplicationResponse, _>()
        } else {
            let try_response = response.buffer().await;
            let response = try_response.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateApplicationError::from_response(response))
        }
    }
}
