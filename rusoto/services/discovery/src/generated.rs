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
/// <p>Information about agents or connectors that were instructed to start collecting data. Information includes the agent/connector ID, a description of the operation, and whether the agent/connector configuration was updated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct AssociateConfigurationItemsToApplicationRequest {
    /// <p>The configuration ID of an application with which items are to be associated.</p>
    #[serde(rename = "applicationConfigurationId")]
    pub application_configuration_id: String,
    /// <p>The ID of each configuration item to be associated with an application.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct AssociateConfigurationItemsToApplicationResponse {}

/// <p>Tags for a configuration item. Tags are metadata that help you categorize IT assets.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct CreateApplicationResponse {
    /// <p>Configuration ID of an application to be created.</p>
    #[serde(rename = "configurationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateTagsRequest {
    /// <p>A list of configuration items that you want to tag.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
    /// <p>Tags that you want to associate with one or more configuration items. Specify the tags that you want to create in a <i>key</i>-<i>value</i> format. For example:</p> <p> <code>{"key": "serverType", "value": "webServer"}</code> </p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct CreateTagsResponse {}

/// <p>Inventory data for installed discovery agents.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct DeleteApplicationsRequest {
    /// <p>Configuration ID of an application to be deleted.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DeleteApplicationsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DeleteTagsResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DescribeConfigurationsRequest {
    /// <p>One or more configuration IDs.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeConfigurationsResponse {
    /// <p>A key in the response map. The value is an array of data.</p>
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<::std::collections::HashMap<String, String>>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeExportConfigurationsRequest {
    /// <p>A unique identifier that you can use to query the export status.</p>
    #[serde(rename = "exportIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_ids: Option<Vec<String>>,
    /// <p>The maximum number of results that you want to display as a part of the query.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A token to get the next set of results. For example, if you specify 100 IDs for <code>DescribeExportConfigurationsRequest$exportIds</code> but set <code>DescribeExportConfigurationsRequest$maxResults</code> to 10, you get results in a set of 10. Use the token in the query to get the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DescribeExportConfigurationsResponse {
    /// <p>Returns export details. When the status is complete, the response includes a URL for an Amazon S3 bucket where you can view the data in a CSV file.</p>
    #[serde(rename = "exportsInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports_info: Option<Vec<ExportInfo>>,
    /// <p>A token to get the next set of results. For example, if you specify 100 IDs for <code>DescribeExportConfigurationsRequest$exportIds</code> but set <code>DescribeExportConfigurationsRequest$maxResults</code> to 10, you get results in a set of 10. Use the token in the query to get the next set of 10.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct DisassociateConfigurationItemsFromApplicationRequest {
    /// <p>Configuration ID of an application from which each item is disassociated.</p>
    #[serde(rename = "applicationConfigurationId")]
    pub application_configuration_id: String,
    /// <p>Configuration ID of each item to be disassociated from an application.</p>
    #[serde(rename = "configurationIds")]
    pub configuration_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct DisassociateConfigurationItemsFromApplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct ExportConfigurationsResponse {
    /// <p>A unique identifier that you can use to query the export status.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

/// <p>Used to select which agent's data is to be exported. A single agent ID may be selected for export using the <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/API_StartExportTask.html">StartExportTask</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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

/// <p>A filter that can use conditional operators.</p> <p>For more information about filters, see <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/discovery-api-queries.html">Querying Discovered Configuration Items</a>. </p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct GetDiscoverySummaryRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListConfigurationsRequest {
    /// <p>A valid configuration identified by Application Discovery Service. </p>
    #[serde(rename = "configurationType")]
    pub configuration_type: String,
    /// <p>You can filter the request using various logical operators and a <i>key</i>-<i>value</i> format. For example: </p> <p> <code>{"key": "serverType", "value": "webServer"}</code> </p> <p>For a complete list of filter options and guidance about using them with this action, see <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/discovery-api-queries.html#ListConfigurations">Querying Discovered Configuration Items</a>. </p>
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
    /// <p>Certain filter criteria return output that can be sorted in ascending or descending order. For a list of output characteristics for each filter, see <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/discovery-api-queries.html#ListConfigurations">Using the ListConfigurations Action</a>.</p>
    #[serde(rename = "orderBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<OrderByElement>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
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
pub struct StartDataCollectionByAgentIdsRequest {
    /// <p>The IDs of the agents or connectors from which to start collecting data. If you send a request to an agent/connector ID that you do not have permission to contact, according to your AWS account, the service does not throw an exception. Instead, it returns the error in the <i>Description</i> field. If you send a request to multiple agents/connectors and you do not have permission to contact some of those agents/connectors, the system does not throw an exception. Instead, the system shows <code>Failed</code> in the <i>Description</i> field.</p>
    #[serde(rename = "agentIds")]
    pub agent_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StartDataCollectionByAgentIdsResponse {
    /// <p>Information about agents or the connector that were instructed to start collecting data. Information includes the agent/connector ID, a description of the operation performed, and whether the agent/connector configuration was updated.</p>
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct StartExportTaskResponse {
    /// <p>A unique identifier used to query the status of an export request.</p>
    #[serde(rename = "exportId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopDataCollectionByAgentIdsRequest {
    /// <p>The IDs of the agents or connectors from which to stop collecting data.</p>
    #[serde(rename = "agentIds")]
    pub agent_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct StopDataCollectionByAgentIdsResponse {
    /// <p>Information about the agents or connector that were instructed to stop collecting data. Information includes the agent/connector ID, a description of the operation performed, and whether the agent/connector configuration was updated.</p>
    #[serde(rename = "agentsConfigurationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents_configuration_status: Option<Vec<AgentConfigurationStatus>>,
}

/// <p>Metadata that help you categorize IT assets.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct TagFilter {
    /// <p>A name of the tag filter.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>Values for the tag filter.</p>
    #[serde(rename = "values")]
    pub values: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
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
pub struct UpdateApplicationResponse {}

/// Errors returned by AssociateConfigurationItemsToApplication
#[derive(Debug, PartialEq)]
pub enum AssociateConfigurationItemsToApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl AssociateConfigurationItemsToApplicationError {
    pub fn from_body(body: &str) -> AssociateConfigurationItemsToApplicationError {
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
                    "AuthorizationErrorException" => {
                        AssociateConfigurationItemsToApplicationError::AuthorizationError(
                            String::from(error_message),
                        )
                    }
                    "InvalidParameterException" => {
                        AssociateConfigurationItemsToApplicationError::InvalidParameter(
                            String::from(error_message),
                        )
                    }
                    "InvalidParameterValueException" => {
                        AssociateConfigurationItemsToApplicationError::InvalidParameterValue(
                            String::from(error_message),
                        )
                    }
                    "ServerInternalErrorException" => {
                        AssociateConfigurationItemsToApplicationError::ServerInternalError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        AssociateConfigurationItemsToApplicationError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => AssociateConfigurationItemsToApplicationError::Unknown(String::from(body)),
                }
            }
            Err(_) => AssociateConfigurationItemsToApplicationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for AssociateConfigurationItemsToApplicationError {
    fn from(err: serde_json::error::Error) -> AssociateConfigurationItemsToApplicationError {
        AssociateConfigurationItemsToApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for AssociateConfigurationItemsToApplicationError {
    fn from(err: CredentialsError) -> AssociateConfigurationItemsToApplicationError {
        AssociateConfigurationItemsToApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for AssociateConfigurationItemsToApplicationError {
    fn from(err: HttpDispatchError) -> AssociateConfigurationItemsToApplicationError {
        AssociateConfigurationItemsToApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for AssociateConfigurationItemsToApplicationError {
    fn from(err: io::Error) -> AssociateConfigurationItemsToApplicationError {
        AssociateConfigurationItemsToApplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for AssociateConfigurationItemsToApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AssociateConfigurationItemsToApplicationError {
    fn description(&self) -> &str {
        match *self {
            AssociateConfigurationItemsToApplicationError::AuthorizationError(ref cause) => cause,
            AssociateConfigurationItemsToApplicationError::InvalidParameter(ref cause) => cause,
            AssociateConfigurationItemsToApplicationError::InvalidParameterValue(ref cause) => {
                cause
            }
            AssociateConfigurationItemsToApplicationError::ServerInternalError(ref cause) => cause,
            AssociateConfigurationItemsToApplicationError::Validation(ref cause) => cause,
            AssociateConfigurationItemsToApplicationError::Credentials(ref err) => {
                err.description()
            }
            AssociateConfigurationItemsToApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            AssociateConfigurationItemsToApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateApplication
#[derive(Debug, PartialEq)]
pub enum CreateApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
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
                    "AuthorizationErrorException" => {
                        CreateApplicationError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateApplicationError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        CreateApplicationError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        CreateApplicationError::ServerInternalError(String::from(error_message))
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
            CreateApplicationError::AuthorizationError(ref cause) => cause,
            CreateApplicationError::InvalidParameter(ref cause) => cause,
            CreateApplicationError::InvalidParameterValue(ref cause) => cause,
            CreateApplicationError::ServerInternalError(ref cause) => cause,
            CreateApplicationError::Validation(ref cause) => cause,
            CreateApplicationError::Credentials(ref err) => err.description(),
            CreateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateTags
#[derive(Debug, PartialEq)]
pub enum CreateTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl CreateTagsError {
    pub fn from_body(body: &str) -> CreateTagsError {
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
                    "AuthorizationErrorException" => {
                        CreateTagsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        CreateTagsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        CreateTagsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        CreateTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        CreateTagsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => CreateTagsError::Validation(error_message.to_string()),
                    _ => CreateTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => CreateTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for CreateTagsError {
    fn from(err: serde_json::error::Error) -> CreateTagsError {
        CreateTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateTagsError {
    fn from(err: CredentialsError) -> CreateTagsError {
        CreateTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateTagsError {
    fn from(err: HttpDispatchError) -> CreateTagsError {
        CreateTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateTagsError {
    fn from(err: io::Error) -> CreateTagsError {
        CreateTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateTagsError {
    fn description(&self) -> &str {
        match *self {
            CreateTagsError::AuthorizationError(ref cause) => cause,
            CreateTagsError::InvalidParameter(ref cause) => cause,
            CreateTagsError::InvalidParameterValue(ref cause) => cause,
            CreateTagsError::ResourceNotFound(ref cause) => cause,
            CreateTagsError::ServerInternalError(ref cause) => cause,
            CreateTagsError::Validation(ref cause) => cause,
            CreateTagsError::Credentials(ref err) => err.description(),
            CreateTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteApplications
#[derive(Debug, PartialEq)]
pub enum DeleteApplicationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteApplicationsError {
    pub fn from_body(body: &str) -> DeleteApplicationsError {
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
                    "AuthorizationErrorException" => {
                        DeleteApplicationsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteApplicationsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DeleteApplicationsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        DeleteApplicationsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteApplicationsError::Validation(error_message.to_string())
                    }
                    _ => DeleteApplicationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteApplicationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteApplicationsError {
    fn from(err: serde_json::error::Error) -> DeleteApplicationsError {
        DeleteApplicationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteApplicationsError {
    fn from(err: CredentialsError) -> DeleteApplicationsError {
        DeleteApplicationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteApplicationsError {
    fn from(err: HttpDispatchError) -> DeleteApplicationsError {
        DeleteApplicationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteApplicationsError {
    fn from(err: io::Error) -> DeleteApplicationsError {
        DeleteApplicationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteApplicationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteApplicationsError {
    fn description(&self) -> &str {
        match *self {
            DeleteApplicationsError::AuthorizationError(ref cause) => cause,
            DeleteApplicationsError::InvalidParameter(ref cause) => cause,
            DeleteApplicationsError::InvalidParameterValue(ref cause) => cause,
            DeleteApplicationsError::ServerInternalError(ref cause) => cause,
            DeleteApplicationsError::Validation(ref cause) => cause,
            DeleteApplicationsError::Credentials(ref err) => err.description(),
            DeleteApplicationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteApplicationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteTags
#[derive(Debug, PartialEq)]
pub enum DeleteTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DeleteTagsError {
    pub fn from_body(body: &str) -> DeleteTagsError {
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
                    "AuthorizationErrorException" => {
                        DeleteTagsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteTagsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DeleteTagsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        DeleteTagsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => DeleteTagsError::Validation(error_message.to_string()),
                    _ => DeleteTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteTagsError {
    fn from(err: serde_json::error::Error) -> DeleteTagsError {
        DeleteTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteTagsError {
    fn from(err: CredentialsError) -> DeleteTagsError {
        DeleteTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteTagsError {
    fn from(err: HttpDispatchError) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteTagsError {
    fn from(err: io::Error) -> DeleteTagsError {
        DeleteTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteTagsError {
    fn description(&self) -> &str {
        match *self {
            DeleteTagsError::AuthorizationError(ref cause) => cause,
            DeleteTagsError::InvalidParameter(ref cause) => cause,
            DeleteTagsError::InvalidParameterValue(ref cause) => cause,
            DeleteTagsError::ResourceNotFound(ref cause) => cause,
            DeleteTagsError::ServerInternalError(ref cause) => cause,
            DeleteTagsError::Validation(ref cause) => cause,
            DeleteTagsError::Credentials(ref err) => err.description(),
            DeleteTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAgents
#[derive(Debug, PartialEq)]
pub enum DescribeAgentsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeAgentsError {
    pub fn from_body(body: &str) -> DescribeAgentsError {
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
                    "AuthorizationErrorException" => {
                        DescribeAgentsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeAgentsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DescribeAgentsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        DescribeAgentsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeAgentsError::Validation(error_message.to_string())
                    }
                    _ => DescribeAgentsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeAgentsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeAgentsError {
    fn from(err: serde_json::error::Error) -> DescribeAgentsError {
        DescribeAgentsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeAgentsError {
    fn from(err: CredentialsError) -> DescribeAgentsError {
        DescribeAgentsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeAgentsError {
    fn from(err: HttpDispatchError) -> DescribeAgentsError {
        DescribeAgentsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeAgentsError {
    fn from(err: io::Error) -> DescribeAgentsError {
        DescribeAgentsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAgentsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAgentsError::AuthorizationError(ref cause) => cause,
            DescribeAgentsError::InvalidParameter(ref cause) => cause,
            DescribeAgentsError::InvalidParameterValue(ref cause) => cause,
            DescribeAgentsError::ServerInternalError(ref cause) => cause,
            DescribeAgentsError::Validation(ref cause) => cause,
            DescribeAgentsError::Credentials(ref err) => err.description(),
            DescribeAgentsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeAgentsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeConfigurationsError {
    pub fn from_body(body: &str) -> DescribeConfigurationsError {
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
                    "AuthorizationErrorException" => {
                        DescribeConfigurationsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeConfigurationsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DescribeConfigurationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServerInternalErrorException" => {
                        DescribeConfigurationsError::ServerInternalError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeConfigurationsError {
    fn from(err: serde_json::error::Error) -> DescribeConfigurationsError {
        DescribeConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeConfigurationsError {
    fn from(err: CredentialsError) -> DescribeConfigurationsError {
        DescribeConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeConfigurationsError {
        DescribeConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeConfigurationsError {
    fn from(err: io::Error) -> DescribeConfigurationsError {
        DescribeConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeConfigurationsError::AuthorizationError(ref cause) => cause,
            DescribeConfigurationsError::InvalidParameter(ref cause) => cause,
            DescribeConfigurationsError::InvalidParameterValue(ref cause) => cause,
            DescribeConfigurationsError::ServerInternalError(ref cause) => cause,
            DescribeConfigurationsError::Validation(ref cause) => cause,
            DescribeConfigurationsError::Credentials(ref err) => err.description(),
            DescribeConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExportConfigurations
#[derive(Debug, PartialEq)]
pub enum DescribeExportConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeExportConfigurationsError {
    pub fn from_body(body: &str) -> DescribeExportConfigurationsError {
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
                    "AuthorizationErrorException" => {
                        DescribeExportConfigurationsError::AuthorizationError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        DescribeExportConfigurationsError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        DescribeExportConfigurationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ResourceNotFoundException" => {
                        DescribeExportConfigurationsError::ResourceNotFound(String::from(
                            error_message,
                        ))
                    }
                    "ServerInternalErrorException" => {
                        DescribeExportConfigurationsError::ServerInternalError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        DescribeExportConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => DescribeExportConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeExportConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeExportConfigurationsError {
    fn from(err: serde_json::error::Error) -> DescribeExportConfigurationsError {
        DescribeExportConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeExportConfigurationsError {
    fn from(err: CredentialsError) -> DescribeExportConfigurationsError {
        DescribeExportConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeExportConfigurationsError {
    fn from(err: HttpDispatchError) -> DescribeExportConfigurationsError {
        DescribeExportConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeExportConfigurationsError {
    fn from(err: io::Error) -> DescribeExportConfigurationsError {
        DescribeExportConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeExportConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExportConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            DescribeExportConfigurationsError::AuthorizationError(ref cause) => cause,
            DescribeExportConfigurationsError::InvalidParameter(ref cause) => cause,
            DescribeExportConfigurationsError::InvalidParameterValue(ref cause) => cause,
            DescribeExportConfigurationsError::ResourceNotFound(ref cause) => cause,
            DescribeExportConfigurationsError::ServerInternalError(ref cause) => cause,
            DescribeExportConfigurationsError::Validation(ref cause) => cause,
            DescribeExportConfigurationsError::Credentials(ref err) => err.description(),
            DescribeExportConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeExportConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExportTasks
#[derive(Debug, PartialEq)]
pub enum DescribeExportTasksError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeExportTasksError {
    pub fn from_body(body: &str) -> DescribeExportTasksError {
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
                    "AuthorizationErrorException" => {
                        DescribeExportTasksError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeExportTasksError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DescribeExportTasksError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        DescribeExportTasksError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeExportTasksError::Validation(error_message.to_string())
                    }
                    _ => DescribeExportTasksError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeExportTasksError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeExportTasksError {
    fn from(err: serde_json::error::Error) -> DescribeExportTasksError {
        DescribeExportTasksError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeExportTasksError {
    fn from(err: CredentialsError) -> DescribeExportTasksError {
        DescribeExportTasksError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeExportTasksError {
    fn from(err: HttpDispatchError) -> DescribeExportTasksError {
        DescribeExportTasksError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeExportTasksError {
    fn from(err: io::Error) -> DescribeExportTasksError {
        DescribeExportTasksError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeExportTasksError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExportTasksError {
    fn description(&self) -> &str {
        match *self {
            DescribeExportTasksError::AuthorizationError(ref cause) => cause,
            DescribeExportTasksError::InvalidParameter(ref cause) => cause,
            DescribeExportTasksError::InvalidParameterValue(ref cause) => cause,
            DescribeExportTasksError::ServerInternalError(ref cause) => cause,
            DescribeExportTasksError::Validation(ref cause) => cause,
            DescribeExportTasksError::Credentials(ref err) => err.description(),
            DescribeExportTasksError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeExportTasksError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeTags
#[derive(Debug, PartialEq)]
pub enum DescribeTagsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DescribeTagsError {
    pub fn from_body(body: &str) -> DescribeTagsError {
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
                    "AuthorizationErrorException" => {
                        DescribeTagsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeTagsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        DescribeTagsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeTagsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        DescribeTagsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeTagsError::Validation(error_message.to_string())
                    }
                    _ => DescribeTagsError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeTagsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeTagsError {
    fn from(err: serde_json::error::Error) -> DescribeTagsError {
        DescribeTagsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeTagsError {
    fn from(err: CredentialsError) -> DescribeTagsError {
        DescribeTagsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeTagsError {
    fn from(err: HttpDispatchError) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeTagsError {
    fn from(err: io::Error) -> DescribeTagsError {
        DescribeTagsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeTagsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeTagsError {
    fn description(&self) -> &str {
        match *self {
            DescribeTagsError::AuthorizationError(ref cause) => cause,
            DescribeTagsError::InvalidParameter(ref cause) => cause,
            DescribeTagsError::InvalidParameterValue(ref cause) => cause,
            DescribeTagsError::ResourceNotFound(ref cause) => cause,
            DescribeTagsError::ServerInternalError(ref cause) => cause,
            DescribeTagsError::Validation(ref cause) => cause,
            DescribeTagsError::Credentials(ref err) => err.description(),
            DescribeTagsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeTagsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DisassociateConfigurationItemsFromApplication
#[derive(Debug, PartialEq)]
pub enum DisassociateConfigurationItemsFromApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl DisassociateConfigurationItemsFromApplicationError {
    pub fn from_body(body: &str) -> DisassociateConfigurationItemsFromApplicationError {
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
                    "AuthorizationErrorException" => {
                        DisassociateConfigurationItemsFromApplicationError::AuthorizationError(
                            String::from(error_message),
                        )
                    }
                    "InvalidParameterException" => {
                        DisassociateConfigurationItemsFromApplicationError::InvalidParameter(
                            String::from(error_message),
                        )
                    }
                    "InvalidParameterValueException" => {
                        DisassociateConfigurationItemsFromApplicationError::InvalidParameterValue(
                            String::from(error_message),
                        )
                    }
                    "ServerInternalErrorException" => {
                        DisassociateConfigurationItemsFromApplicationError::ServerInternalError(
                            String::from(error_message),
                        )
                    }
                    "ValidationException" => {
                        DisassociateConfigurationItemsFromApplicationError::Validation(
                            error_message.to_string(),
                        )
                    }
                    _ => DisassociateConfigurationItemsFromApplicationError::Unknown(String::from(
                        body,
                    )),
                }
            }
            Err(_) => {
                DisassociateConfigurationItemsFromApplicationError::Unknown(String::from(body))
            }
        }
    }
}

impl From<serde_json::error::Error> for DisassociateConfigurationItemsFromApplicationError {
    fn from(err: serde_json::error::Error) -> DisassociateConfigurationItemsFromApplicationError {
        DisassociateConfigurationItemsFromApplicationError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateConfigurationItemsFromApplicationError {
    fn from(err: CredentialsError) -> DisassociateConfigurationItemsFromApplicationError {
        DisassociateConfigurationItemsFromApplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateConfigurationItemsFromApplicationError {
    fn from(err: HttpDispatchError) -> DisassociateConfigurationItemsFromApplicationError {
        DisassociateConfigurationItemsFromApplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateConfigurationItemsFromApplicationError {
    fn from(err: io::Error) -> DisassociateConfigurationItemsFromApplicationError {
        DisassociateConfigurationItemsFromApplicationError::HttpDispatch(HttpDispatchError::from(
            err,
        ))
    }
}
impl fmt::Display for DisassociateConfigurationItemsFromApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConfigurationItemsFromApplicationError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConfigurationItemsFromApplicationError::AuthorizationError(ref cause) => {
                cause
            }
            DisassociateConfigurationItemsFromApplicationError::InvalidParameter(ref cause) => {
                cause
            }
            DisassociateConfigurationItemsFromApplicationError::InvalidParameterValue(
                ref cause,
            ) => cause,
            DisassociateConfigurationItemsFromApplicationError::ServerInternalError(ref cause) => {
                cause
            }
            DisassociateConfigurationItemsFromApplicationError::Validation(ref cause) => cause,
            DisassociateConfigurationItemsFromApplicationError::Credentials(ref err) => {
                err.description()
            }
            DisassociateConfigurationItemsFromApplicationError::HttpDispatch(
                ref dispatch_error,
            ) => dispatch_error.description(),
            DisassociateConfigurationItemsFromApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ExportConfigurations
#[derive(Debug, PartialEq)]
pub enum ExportConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ExportConfigurationsError {
    pub fn from_body(body: &str) -> ExportConfigurationsError {
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
                    "AuthorizationErrorException" => {
                        ExportConfigurationsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ExportConfigurationsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        ExportConfigurationsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "OperationNotPermittedException" => {
                        ExportConfigurationsError::OperationNotPermitted(String::from(
                            error_message,
                        ))
                    }
                    "ServerInternalErrorException" => {
                        ExportConfigurationsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ExportConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => ExportConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ExportConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ExportConfigurationsError {
    fn from(err: serde_json::error::Error) -> ExportConfigurationsError {
        ExportConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ExportConfigurationsError {
    fn from(err: CredentialsError) -> ExportConfigurationsError {
        ExportConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ExportConfigurationsError {
    fn from(err: HttpDispatchError) -> ExportConfigurationsError {
        ExportConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ExportConfigurationsError {
    fn from(err: io::Error) -> ExportConfigurationsError {
        ExportConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ExportConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ExportConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ExportConfigurationsError::AuthorizationError(ref cause) => cause,
            ExportConfigurationsError::InvalidParameter(ref cause) => cause,
            ExportConfigurationsError::InvalidParameterValue(ref cause) => cause,
            ExportConfigurationsError::OperationNotPermitted(ref cause) => cause,
            ExportConfigurationsError::ServerInternalError(ref cause) => cause,
            ExportConfigurationsError::Validation(ref cause) => cause,
            ExportConfigurationsError::Credentials(ref err) => err.description(),
            ExportConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ExportConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetDiscoverySummary
#[derive(Debug, PartialEq)]
pub enum GetDiscoverySummaryError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl GetDiscoverySummaryError {
    pub fn from_body(body: &str) -> GetDiscoverySummaryError {
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
                    "AuthorizationErrorException" => {
                        GetDiscoverySummaryError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetDiscoverySummaryError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        GetDiscoverySummaryError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        GetDiscoverySummaryError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetDiscoverySummaryError::Validation(error_message.to_string())
                    }
                    _ => GetDiscoverySummaryError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetDiscoverySummaryError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetDiscoverySummaryError {
    fn from(err: serde_json::error::Error) -> GetDiscoverySummaryError {
        GetDiscoverySummaryError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for GetDiscoverySummaryError {
    fn from(err: CredentialsError) -> GetDiscoverySummaryError {
        GetDiscoverySummaryError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetDiscoverySummaryError {
    fn from(err: HttpDispatchError) -> GetDiscoverySummaryError {
        GetDiscoverySummaryError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetDiscoverySummaryError {
    fn from(err: io::Error) -> GetDiscoverySummaryError {
        GetDiscoverySummaryError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetDiscoverySummaryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetDiscoverySummaryError {
    fn description(&self) -> &str {
        match *self {
            GetDiscoverySummaryError::AuthorizationError(ref cause) => cause,
            GetDiscoverySummaryError::InvalidParameter(ref cause) => cause,
            GetDiscoverySummaryError::InvalidParameterValue(ref cause) => cause,
            GetDiscoverySummaryError::ServerInternalError(ref cause) => cause,
            GetDiscoverySummaryError::Validation(ref cause) => cause,
            GetDiscoverySummaryError::Credentials(ref err) => err.description(),
            GetDiscoverySummaryError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetDiscoverySummaryError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListConfigurations
#[derive(Debug, PartialEq)]
pub enum ListConfigurationsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The specified configuration ID was not located. Verify the configuration ID and try again.</p>
    ResourceNotFound(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListConfigurationsError {
    pub fn from_body(body: &str) -> ListConfigurationsError {
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
                    "AuthorizationErrorException" => {
                        ListConfigurationsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListConfigurationsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        ListConfigurationsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        ListConfigurationsError::ResourceNotFound(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        ListConfigurationsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListConfigurationsError::Validation(error_message.to_string())
                    }
                    _ => ListConfigurationsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListConfigurationsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListConfigurationsError {
    fn from(err: serde_json::error::Error) -> ListConfigurationsError {
        ListConfigurationsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListConfigurationsError {
    fn from(err: CredentialsError) -> ListConfigurationsError {
        ListConfigurationsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListConfigurationsError {
    fn from(err: HttpDispatchError) -> ListConfigurationsError {
        ListConfigurationsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListConfigurationsError {
    fn from(err: io::Error) -> ListConfigurationsError {
        ListConfigurationsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListConfigurationsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListConfigurationsError {
    fn description(&self) -> &str {
        match *self {
            ListConfigurationsError::AuthorizationError(ref cause) => cause,
            ListConfigurationsError::InvalidParameter(ref cause) => cause,
            ListConfigurationsError::InvalidParameterValue(ref cause) => cause,
            ListConfigurationsError::ResourceNotFound(ref cause) => cause,
            ListConfigurationsError::ServerInternalError(ref cause) => cause,
            ListConfigurationsError::Validation(ref cause) => cause,
            ListConfigurationsError::Credentials(ref err) => err.description(),
            ListConfigurationsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListConfigurationsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListServerNeighbors
#[derive(Debug, PartialEq)]
pub enum ListServerNeighborsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl ListServerNeighborsError {
    pub fn from_body(body: &str) -> ListServerNeighborsError {
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
                    "AuthorizationErrorException" => {
                        ListServerNeighborsError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListServerNeighborsError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        ListServerNeighborsError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        ListServerNeighborsError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListServerNeighborsError::Validation(error_message.to_string())
                    }
                    _ => ListServerNeighborsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListServerNeighborsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListServerNeighborsError {
    fn from(err: serde_json::error::Error) -> ListServerNeighborsError {
        ListServerNeighborsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for ListServerNeighborsError {
    fn from(err: CredentialsError) -> ListServerNeighborsError {
        ListServerNeighborsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListServerNeighborsError {
    fn from(err: HttpDispatchError) -> ListServerNeighborsError {
        ListServerNeighborsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListServerNeighborsError {
    fn from(err: io::Error) -> ListServerNeighborsError {
        ListServerNeighborsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListServerNeighborsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListServerNeighborsError {
    fn description(&self) -> &str {
        match *self {
            ListServerNeighborsError::AuthorizationError(ref cause) => cause,
            ListServerNeighborsError::InvalidParameter(ref cause) => cause,
            ListServerNeighborsError::InvalidParameterValue(ref cause) => cause,
            ListServerNeighborsError::ServerInternalError(ref cause) => cause,
            ListServerNeighborsError::Validation(ref cause) => cause,
            ListServerNeighborsError::Credentials(ref err) => err.description(),
            ListServerNeighborsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListServerNeighborsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartDataCollectionByAgentIds
#[derive(Debug, PartialEq)]
pub enum StartDataCollectionByAgentIdsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartDataCollectionByAgentIdsError {
    pub fn from_body(body: &str) -> StartDataCollectionByAgentIdsError {
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
                    "AuthorizationErrorException" => {
                        StartDataCollectionByAgentIdsError::AuthorizationError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        StartDataCollectionByAgentIdsError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        StartDataCollectionByAgentIdsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServerInternalErrorException" => {
                        StartDataCollectionByAgentIdsError::ServerInternalError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        StartDataCollectionByAgentIdsError::Validation(error_message.to_string())
                    }
                    _ => StartDataCollectionByAgentIdsError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartDataCollectionByAgentIdsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartDataCollectionByAgentIdsError {
    fn from(err: serde_json::error::Error) -> StartDataCollectionByAgentIdsError {
        StartDataCollectionByAgentIdsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartDataCollectionByAgentIdsError {
    fn from(err: CredentialsError) -> StartDataCollectionByAgentIdsError {
        StartDataCollectionByAgentIdsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartDataCollectionByAgentIdsError {
    fn from(err: HttpDispatchError) -> StartDataCollectionByAgentIdsError {
        StartDataCollectionByAgentIdsError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartDataCollectionByAgentIdsError {
    fn from(err: io::Error) -> StartDataCollectionByAgentIdsError {
        StartDataCollectionByAgentIdsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartDataCollectionByAgentIdsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartDataCollectionByAgentIdsError {
    fn description(&self) -> &str {
        match *self {
            StartDataCollectionByAgentIdsError::AuthorizationError(ref cause) => cause,
            StartDataCollectionByAgentIdsError::InvalidParameter(ref cause) => cause,
            StartDataCollectionByAgentIdsError::InvalidParameterValue(ref cause) => cause,
            StartDataCollectionByAgentIdsError::ServerInternalError(ref cause) => cause,
            StartDataCollectionByAgentIdsError::Validation(ref cause) => cause,
            StartDataCollectionByAgentIdsError::Credentials(ref err) => err.description(),
            StartDataCollectionByAgentIdsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartDataCollectionByAgentIdsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StartExportTask
#[derive(Debug, PartialEq)]
pub enum StartExportTaskError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>This operation is not permitted.</p>
    OperationNotPermitted(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StartExportTaskError {
    pub fn from_body(body: &str) -> StartExportTaskError {
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
                    "AuthorizationErrorException" => {
                        StartExportTaskError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        StartExportTaskError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        StartExportTaskError::InvalidParameterValue(String::from(error_message))
                    }
                    "OperationNotPermittedException" => {
                        StartExportTaskError::OperationNotPermitted(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        StartExportTaskError::ServerInternalError(String::from(error_message))
                    }
                    "ValidationException" => {
                        StartExportTaskError::Validation(error_message.to_string())
                    }
                    _ => StartExportTaskError::Unknown(String::from(body)),
                }
            }
            Err(_) => StartExportTaskError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StartExportTaskError {
    fn from(err: serde_json::error::Error) -> StartExportTaskError {
        StartExportTaskError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StartExportTaskError {
    fn from(err: CredentialsError) -> StartExportTaskError {
        StartExportTaskError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartExportTaskError {
    fn from(err: HttpDispatchError) -> StartExportTaskError {
        StartExportTaskError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartExportTaskError {
    fn from(err: io::Error) -> StartExportTaskError {
        StartExportTaskError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartExportTaskError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartExportTaskError {
    fn description(&self) -> &str {
        match *self {
            StartExportTaskError::AuthorizationError(ref cause) => cause,
            StartExportTaskError::InvalidParameter(ref cause) => cause,
            StartExportTaskError::InvalidParameterValue(ref cause) => cause,
            StartExportTaskError::OperationNotPermitted(ref cause) => cause,
            StartExportTaskError::ServerInternalError(ref cause) => cause,
            StartExportTaskError::Validation(ref cause) => cause,
            StartExportTaskError::Credentials(ref err) => err.description(),
            StartExportTaskError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            StartExportTaskError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by StopDataCollectionByAgentIds
#[derive(Debug, PartialEq)]
pub enum StopDataCollectionByAgentIdsError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}

impl StopDataCollectionByAgentIdsError {
    pub fn from_body(body: &str) -> StopDataCollectionByAgentIdsError {
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
                    "AuthorizationErrorException" => {
                        StopDataCollectionByAgentIdsError::AuthorizationError(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterException" => {
                        StopDataCollectionByAgentIdsError::InvalidParameter(String::from(
                            error_message,
                        ))
                    }
                    "InvalidParameterValueException" => {
                        StopDataCollectionByAgentIdsError::InvalidParameterValue(String::from(
                            error_message,
                        ))
                    }
                    "ServerInternalErrorException" => {
                        StopDataCollectionByAgentIdsError::ServerInternalError(String::from(
                            error_message,
                        ))
                    }
                    "ValidationException" => {
                        StopDataCollectionByAgentIdsError::Validation(error_message.to_string())
                    }
                    _ => StopDataCollectionByAgentIdsError::Unknown(String::from(body)),
                }
            }
            Err(_) => StopDataCollectionByAgentIdsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for StopDataCollectionByAgentIdsError {
    fn from(err: serde_json::error::Error) -> StopDataCollectionByAgentIdsError {
        StopDataCollectionByAgentIdsError::Unknown(err.description().to_string())
    }
}
impl From<CredentialsError> for StopDataCollectionByAgentIdsError {
    fn from(err: CredentialsError) -> StopDataCollectionByAgentIdsError {
        StopDataCollectionByAgentIdsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopDataCollectionByAgentIdsError {
    fn from(err: HttpDispatchError) -> StopDataCollectionByAgentIdsError {
        StopDataCollectionByAgentIdsError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopDataCollectionByAgentIdsError {
    fn from(err: io::Error) -> StopDataCollectionByAgentIdsError {
        StopDataCollectionByAgentIdsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopDataCollectionByAgentIdsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopDataCollectionByAgentIdsError {
    fn description(&self) -> &str {
        match *self {
            StopDataCollectionByAgentIdsError::AuthorizationError(ref cause) => cause,
            StopDataCollectionByAgentIdsError::InvalidParameter(ref cause) => cause,
            StopDataCollectionByAgentIdsError::InvalidParameterValue(ref cause) => cause,
            StopDataCollectionByAgentIdsError::ServerInternalError(ref cause) => cause,
            StopDataCollectionByAgentIdsError::Validation(ref cause) => cause,
            StopDataCollectionByAgentIdsError::Credentials(ref err) => err.description(),
            StopDataCollectionByAgentIdsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopDataCollectionByAgentIdsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateApplication
#[derive(Debug, PartialEq)]
pub enum UpdateApplicationError {
    /// <p>The AWS user account does not have permission to perform the action. Check the IAM policy associated with this account.</p>
    AuthorizationError(String),
    /// <p>One or more parameters are not valid. Verify the parameters and try again.</p>
    InvalidParameter(String),
    /// <p>The value of one or more parameters are either invalid or out of range. Verify the parameter values and try again.</p>
    InvalidParameterValue(String),
    /// <p>The server experienced an internal error. Try again.</p>
    ServerInternalError(String),
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
                    "AuthorizationErrorException" => {
                        UpdateApplicationError::AuthorizationError(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateApplicationError::InvalidParameter(String::from(error_message))
                    }
                    "InvalidParameterValueException" => {
                        UpdateApplicationError::InvalidParameterValue(String::from(error_message))
                    }
                    "ServerInternalErrorException" => {
                        UpdateApplicationError::ServerInternalError(String::from(error_message))
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
            UpdateApplicationError::AuthorizationError(ref cause) => cause,
            UpdateApplicationError::InvalidParameter(ref cause) => cause,
            UpdateApplicationError::InvalidParameterValue(ref cause) => cause,
            UpdateApplicationError::ServerInternalError(ref cause) => cause,
            UpdateApplicationError::Validation(ref cause) => cause,
            UpdateApplicationError::Credentials(ref err) => err.description(),
            UpdateApplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateApplicationError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the AWS Application Discovery Service API. AWS Application Discovery Service clients implement this trait.
pub trait Discovery {
    /// <p>Associates one or more configuration items with an application.</p>
    fn associate_configuration_items_to_application(
        &self,
        input: AssociateConfigurationItemsToApplicationRequest,
    ) -> RusotoFuture<
        AssociateConfigurationItemsToApplicationResponse,
        AssociateConfigurationItemsToApplicationError,
    >;

    /// <p>Creates an application with the given name and description.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError>;

    /// <p>Creates one or more tags for configuration items. Tags are metadata that help you categorize IT assets. This API accepts a list of multiple configuration items.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResponse, CreateTagsError>;

    /// <p>Deletes a list of applications and their associations with configuration items.</p>
    fn delete_applications(
        &self,
        input: DeleteApplicationsRequest,
    ) -> RusotoFuture<DeleteApplicationsResponse, DeleteApplicationsError>;

    /// <p>Deletes the association between configuration items and one or more tags. This API accepts a list of multiple configuration items.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResponse, DeleteTagsError>;

    /// <p>Lists agents or the Connector by ID or lists all agents/Connectors associated with your user account if you did not specify an ID.</p>
    fn describe_agents(
        &self,
        input: DescribeAgentsRequest,
    ) -> RusotoFuture<DescribeAgentsResponse, DescribeAgentsError>;

    /// <p>Retrieves attributes for a list of configuration item IDs. All of the supplied IDs must be for the same asset type (server, application, process, or connection). Output fields are specific to the asset type selected. For example, the output for a <i>server</i> configuration item includes a list of attributes about the server, such as host name, operating system, and number of network cards.</p> <p>For a complete list of outputs for each asset type, see <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/discovery-api-queries.html#DescribeConfigurations">Using the DescribeConfigurations Action</a>.</p>
    fn describe_configurations(
        &self,
        input: DescribeConfigurationsRequest,
    ) -> RusotoFuture<DescribeConfigurationsResponse, DescribeConfigurationsError>;

    /// <p>Deprecated. Use <code>DescribeExportTasks</code> instead.</p> <p>Retrieves the status of a given export process. You can retrieve status from a maximum of 100 processes.</p>
    fn describe_export_configurations(
        &self,
        input: DescribeExportConfigurationsRequest,
    ) -> RusotoFuture<DescribeExportConfigurationsResponse, DescribeExportConfigurationsError>;

    /// <p>Retrieve status of one or more export tasks. You can retrieve the status of up to 100 export tasks.</p>
    fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> RusotoFuture<DescribeExportTasksResponse, DescribeExportTasksError>;

    /// <p>Retrieves a list of configuration items that are tagged with a specific tag. Or retrieves a list of all tags assigned to a specific configuration item.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError>;

    /// <p>Disassociates one or more configuration items from an application.</p>
    fn disassociate_configuration_items_from_application(
        &self,
        input: DisassociateConfigurationItemsFromApplicationRequest,
    ) -> RusotoFuture<
        DisassociateConfigurationItemsFromApplicationResponse,
        DisassociateConfigurationItemsFromApplicationError,
    >;

    /// <p>Deprecated. Use <code>StartExportTask</code> instead.</p> <p>Exports all discovered configuration data to an Amazon S3 bucket or an application that enables you to view and evaluate the data. Data includes tags and tag associations, processes, connections, servers, and system performance. This API returns an export ID that you can query using the <i>DescribeExportConfigurations</i> API. The system imposes a limit of two configuration exports in six hours.</p>
    fn export_configurations(
        &self,
    ) -> RusotoFuture<ExportConfigurationsResponse, ExportConfigurationsError>;

    /// <p>Retrieves a short summary of discovered assets.</p>
    fn get_discovery_summary(
        &self,
    ) -> RusotoFuture<GetDiscoverySummaryResponse, GetDiscoverySummaryError>;

    /// <p>Retrieves a list of configuration items according to criteria that you specify in a filter. The filter criteria identifies the relationship requirements.</p>
    fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> RusotoFuture<ListConfigurationsResponse, ListConfigurationsError>;

    /// <p>Retrieves a list of servers that are one network hop away from a specified server.</p>
    fn list_server_neighbors(
        &self,
        input: ListServerNeighborsRequest,
    ) -> RusotoFuture<ListServerNeighborsResponse, ListServerNeighborsError>;

    /// <p>Instructs the specified agents or connectors to start collecting data.</p>
    fn start_data_collection_by_agent_ids(
        &self,
        input: StartDataCollectionByAgentIdsRequest,
    ) -> RusotoFuture<StartDataCollectionByAgentIdsResponse, StartDataCollectionByAgentIdsError>;

    /// <p> Begins the export of discovered data to an S3 bucket.</p> <p> If you specify <code>agentIds</code> in a filter, the task exports up to 72 hours of detailed data collected by the identified Application Discovery Agent, including network, process, and performance details. A time range for exported agent data may be set by using <code>startTime</code> and <code>endTime</code>. Export of detailed agent data is limited to five concurrently running exports. </p> <p> If you do not include an <code>agentIds</code> filter, summary data is exported that includes both AWS Agentless Discovery Connector data and summary data from AWS Discovery Agents. Export of summary data is limited to two exports per day. </p>
    fn start_export_task(
        &self,
        input: StartExportTaskRequest,
    ) -> RusotoFuture<StartExportTaskResponse, StartExportTaskError>;

    /// <p>Instructs the specified agents or connectors to stop collecting data.</p>
    fn stop_data_collection_by_agent_ids(
        &self,
        input: StopDataCollectionByAgentIdsRequest,
    ) -> RusotoFuture<StopDataCollectionByAgentIdsResponse, StopDataCollectionByAgentIdsError>;

    /// <p>Updates metadata about an application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError>;
}
/// A client for the AWS Application Discovery Service API.
pub struct DiscoveryClient<P = CredentialsProvider, D = RequestDispatcher>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    inner: ClientInner<P, D>,
    region: region::Region,
}

impl DiscoveryClient {
    /// Creates a simple client backed by an implicit event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    ///
    /// See the `rusoto_core::reactor` module for more details.
    pub fn simple(region: region::Region) -> DiscoveryClient {
        DiscoveryClient::new(
            RequestDispatcher::default(),
            CredentialsProvider::default(),
            region,
        )
    }
}

impl<P, D> DiscoveryClient<P, D>
where
    P: ProvideAwsCredentials,
    D: DispatchSignedRequest,
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        DiscoveryClient {
            inner: ClientInner::new(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl<P, D> Discovery for DiscoveryClient<P, D>
where
    P: ProvideAwsCredentials + 'static,
    D: DispatchSignedRequest + 'static,
{
    /// <p>Associates one or more configuration items with an application.</p>
    fn associate_configuration_items_to_application(
        &self,
        input: AssociateConfigurationItemsToApplicationRequest,
    ) -> RusotoFuture<
        AssociateConfigurationItemsToApplicationResponse,
        AssociateConfigurationItemsToApplicationError,
    > {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.AssociateConfigurationItemsToApplication",
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

                    serde_json::from_str::<AssociateConfigurationItemsToApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(AssociateConfigurationItemsToApplicationError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Creates an application with the given name and description.</p>
    fn create_application(
        &self,
        input: CreateApplicationRequest,
    ) -> RusotoFuture<CreateApplicationResponse, CreateApplicationError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.CreateApplication",
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

    /// <p>Creates one or more tags for configuration items. Tags are metadata that help you categorize IT assets. This API accepts a list of multiple configuration items.</p>
    fn create_tags(
        &self,
        input: CreateTagsRequest,
    ) -> RusotoFuture<CreateTagsResponse, CreateTagsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPoseidonService_V2015_11_01.CreateTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(CreateTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes a list of applications and their associations with configuration items.</p>
    fn delete_applications(
        &self,
        input: DeleteApplicationsRequest,
    ) -> RusotoFuture<DeleteApplicationsResponse, DeleteApplicationsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DeleteApplications",
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

                    serde_json::from_str::<DeleteApplicationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteApplicationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deletes the association between configuration items and one or more tags. This API accepts a list of multiple configuration items.</p>
    fn delete_tags(
        &self,
        input: DeleteTagsRequest,
    ) -> RusotoFuture<DeleteTagsResponse, DeleteTagsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "AWSPoseidonService_V2015_11_01.DeleteTags");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DeleteTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Lists agents or the Connector by ID or lists all agents/Connectors associated with your user account if you did not specify an ID.</p>
    fn describe_agents(
        &self,
        input: DescribeAgentsRequest,
    ) -> RusotoFuture<DescribeAgentsResponse, DescribeAgentsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeAgents",
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

                    serde_json::from_str::<DescribeAgentsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAgentsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves attributes for a list of configuration item IDs. All of the supplied IDs must be for the same asset type (server, application, process, or connection). Output fields are specific to the asset type selected. For example, the output for a <i>server</i> configuration item includes a list of attributes about the server, such as host name, operating system, and number of network cards.</p> <p>For a complete list of outputs for each asset type, see <a href="http://docs.aws.amazon.com/application-discovery/latest/APIReference/discovery-api-queries.html#DescribeConfigurations">Using the DescribeConfigurations Action</a>.</p>
    fn describe_configurations(
        &self,
        input: DescribeConfigurationsRequest,
    ) -> RusotoFuture<DescribeConfigurationsResponse, DescribeConfigurationsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeConfigurations",
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

                    serde_json::from_str::<DescribeConfigurationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecated. Use <code>DescribeExportTasks</code> instead.</p> <p>Retrieves the status of a given export process. You can retrieve status from a maximum of 100 processes.</p>
    fn describe_export_configurations(
        &self,
        input: DescribeExportConfigurationsRequest,
    ) -> RusotoFuture<DescribeExportConfigurationsResponse, DescribeExportConfigurationsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeExportConfigurations",
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

                    serde_json::from_str::<DescribeExportConfigurationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeExportConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieve status of one or more export tasks. You can retrieve the status of up to 100 export tasks.</p>
    fn describe_export_tasks(
        &self,
        input: DescribeExportTasksRequest,
    ) -> RusotoFuture<DescribeExportTasksResponse, DescribeExportTasksError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeExportTasks",
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

                    serde_json::from_str::<DescribeExportTasksResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeExportTasksError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of configuration items that are tagged with a specific tag. Or retrieves a list of all tags assigned to a specific configuration item.</p>
    fn describe_tags(
        &self,
        input: DescribeTagsRequest,
    ) -> RusotoFuture<DescribeTagsResponse, DescribeTagsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DescribeTags",
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

                    serde_json::from_str::<DescribeTagsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(DescribeTagsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Disassociates one or more configuration items from an application.</p>
    fn disassociate_configuration_items_from_application(
        &self,
        input: DisassociateConfigurationItemsFromApplicationRequest,
    ) -> RusotoFuture<
        DisassociateConfigurationItemsFromApplicationResponse,
        DisassociateConfigurationItemsFromApplicationError,
    > {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.DisassociateConfigurationItemsFromApplication",
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

                    serde_json::from_str::<DisassociateConfigurationItemsFromApplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(
                        DisassociateConfigurationItemsFromApplicationError::from_body(
                            String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                        ),
                    )
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Deprecated. Use <code>StartExportTask</code> instead.</p> <p>Exports all discovered configuration data to an Amazon S3 bucket or an application that enables you to view and evaluate the data. Data includes tags and tag associations, processes, connections, servers, and system performance. This API returns an export ID that you can query using the <i>DescribeExportConfigurations</i> API. The system imposes a limit of two configuration exports in six hours.</p>
    fn export_configurations(
        &self,
    ) -> RusotoFuture<ExportConfigurationsResponse, ExportConfigurationsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ExportConfigurations",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ExportConfigurationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ExportConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a short summary of discovered assets.</p>
    fn get_discovery_summary(
        &self,
    ) -> RusotoFuture<GetDiscoverySummaryResponse, GetDiscoverySummaryError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.GetDiscoverySummary",
        );
        request.set_payload(Some(b"{}".to_vec()));

        let future = self.inner.sign_and_dispatch(request, |response| {
            if response.status == StatusCode::OK {
                future::Either::A(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetDiscoverySummaryResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(GetDiscoverySummaryError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of configuration items according to criteria that you specify in a filter. The filter criteria identifies the relationship requirements.</p>
    fn list_configurations(
        &self,
        input: ListConfigurationsRequest,
    ) -> RusotoFuture<ListConfigurationsResponse, ListConfigurationsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ListConfigurations",
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

                    serde_json::from_str::<ListConfigurationsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListConfigurationsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Retrieves a list of servers that are one network hop away from a specified server.</p>
    fn list_server_neighbors(
        &self,
        input: ListServerNeighborsRequest,
    ) -> RusotoFuture<ListServerNeighborsResponse, ListServerNeighborsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.ListServerNeighbors",
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

                    serde_json::from_str::<ListServerNeighborsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(ListServerNeighborsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Instructs the specified agents or connectors to start collecting data.</p>
    fn start_data_collection_by_agent_ids(
        &self,
        input: StartDataCollectionByAgentIdsRequest,
    ) -> RusotoFuture<StartDataCollectionByAgentIdsResponse, StartDataCollectionByAgentIdsError>
    {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartDataCollectionByAgentIds",
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

                    serde_json::from_str::<StartDataCollectionByAgentIdsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartDataCollectionByAgentIdsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p> Begins the export of discovered data to an S3 bucket.</p> <p> If you specify <code>agentIds</code> in a filter, the task exports up to 72 hours of detailed data collected by the identified Application Discovery Agent, including network, process, and performance details. A time range for exported agent data may be set by using <code>startTime</code> and <code>endTime</code>. Export of detailed agent data is limited to five concurrently running exports. </p> <p> If you do not include an <code>agentIds</code> filter, summary data is exported that includes both AWS Agentless Discovery Connector data and summary data from AWS Discovery Agents. Export of summary data is limited to two exports per day. </p>
    fn start_export_task(
        &self,
        input: StartExportTaskRequest,
    ) -> RusotoFuture<StartExportTaskResponse, StartExportTaskError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StartExportTask",
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

                    serde_json::from_str::<StartExportTaskResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StartExportTaskError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Instructs the specified agents or connectors to stop collecting data.</p>
    fn stop_data_collection_by_agent_ids(
        &self,
        input: StopDataCollectionByAgentIdsRequest,
    ) -> RusotoFuture<StopDataCollectionByAgentIdsResponse, StopDataCollectionByAgentIdsError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.StopDataCollectionByAgentIds",
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

                    serde_json::from_str::<StopDataCollectionByAgentIdsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    ).unwrap()
                }))
            } else {
                future::Either::B(response.buffer().from_err().and_then(|response| {
                    Err(StopDataCollectionByAgentIdsError::from_body(
                        String::from_utf8_lossy(response.body.as_ref()).as_ref(),
                    ))
                }))
            }
        });

        RusotoFuture::new(future)
    }

    /// <p>Updates metadata about an application.</p>
    fn update_application(
        &self,
        input: UpdateApplicationRequest,
    ) -> RusotoFuture<UpdateApplicationResponse, UpdateApplicationError> {
        let mut request = SignedRequest::new("POST", "discovery", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSPoseidonService_V2015_11_01.UpdateApplication",
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
