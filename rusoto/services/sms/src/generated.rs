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

use rusoto_core::signature::SignedRequest;
use serde_json;
use serde_json::from_slice;
use serde_json::Value as SerdeJsonValue;
/// <p>Information about the application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AppSummary {
    /// <p>Unique ID of the application.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Time of creation of this application.</p>
    #[serde(rename = "creationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    /// <p>Description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Timestamp of the application's creation.</p>
    #[serde(rename = "lastModified")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    /// <p>Timestamp of the application's most recent successful replication.</p>
    #[serde(rename = "latestReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_replication_time: Option<f64>,
    /// <p>Details about the latest launch of the application.</p>
    #[serde(rename = "launchDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_details: Option<LaunchDetails>,
    /// <p>Launch status of the application.</p>
    #[serde(rename = "launchStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status: Option<String>,
    /// <p>A message related to the launch status of the application.</p>
    #[serde(rename = "launchStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_status_message: Option<String>,
    /// <p>Name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Replication status of the application.</p>
    #[serde(rename = "replicationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<String>,
    /// <p>A message related to the replication status of the application.</p>
    #[serde(rename = "replicationStatusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status_message: Option<String>,
    /// <p>Name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Status of the application.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>A message related to the status of the application</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Number of server groups present in the application.</p>
    #[serde(rename = "totalServerGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_server_groups: Option<i64>,
    /// <p>Number of servers present in the application.</p>
    #[serde(rename = "totalServers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_servers: Option<i64>,
}

/// <p>Represents a connector.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Connector {
    /// <p>The time the connector was associated.</p>
    #[serde(rename = "associatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_on: Option<f64>,
    /// <p>The capabilities of the connector.</p>
    #[serde(rename = "capabilityList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_list: Option<Vec<String>>,
    /// <p>The identifier of the connector.</p>
    #[serde(rename = "connectorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
    /// <p>The IP address of the connector.</p>
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// <p>The MAC address of the connector.</p>
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// <p>The status of the connector.</p>
    #[serde(rename = "status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The connector version.</p>
    #[serde(rename = "version")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// <p>The identifier of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAppRequest {
    /// <p>A unique, case-sensitive identifier you provide to ensure idempotency of application creation.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>Description of the new application</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Name of the new application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Name of service role in customer's account to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of server groups to include in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags to be associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAppResponse {
    /// <p>Summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of server groups included in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of taags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When <i>true</i>, the replication job produces encrypted AMIs. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest will be deleted once the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The name of the IAM role to be used by the AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    pub seed_replication_time: f64,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    pub server_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateReplicationJobResponse {
    /// <p>The unique identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppLaunchConfigurationRequest {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppLaunchConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppReplicationConfigurationRequest {
    /// <p>ID of the application associated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppReplicationConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAppRequest {
    /// <p>ID of the application to delete.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>While deleting the application, stop all replication jobs corresponding to the servers in the application.</p>
    #[serde(rename = "forceStopAppReplication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_stop_app_replication: Option<bool>,
    /// <p>While deleting the application, terminate the stack corresponding to the application.</p>
    #[serde(rename = "forceTerminateApp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_terminate_app: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteAppResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteReplicationJobRequest {
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteReplicationJobResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteServerCatalogResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DisassociateConnectorRequest {
    /// <p>The identifier of the connector.</p>
    #[serde(rename = "connectorId")]
    pub connector_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DisassociateConnectorResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateChangeSetRequest {
    /// <p>ID of the application associated with the change set.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Format for the change set.</p>
    #[serde(rename = "changesetFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changeset_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateChangeSetResponse {
    /// <p>Location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GenerateTemplateRequest {
    /// <p>ID of the application associated with the Amazon CloudFormation template.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Format for generating the Amazon CloudFormation template.</p>
    #[serde(rename = "templateFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_format: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GenerateTemplateResponse {
    /// <p>Location of the Amazon S3 object.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppLaunchConfigurationRequest {
    /// <p>ID of the application launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppLaunchConfigurationResponse {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Name of the service role in the customer's account that Amazon CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of launch configurations for server groups in this application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppReplicationConfigurationRequest {
    /// <p>ID of the application associated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppReplicationConfigurationResponse {
    /// <p>Replication configurations associated with server groups in this application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAppRequest {
    /// <p>ID of the application whose information is being retrieved.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAppResponse {
    /// <p>Information about the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of server groups belonging to the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetConnectorsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
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
pub struct GetConnectorsResponse {
    /// <p>Information about the registered connectors.</p>
    #[serde(rename = "connectorList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_list: Option<Vec<Connector>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationJobsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetReplicationJobsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication jobs.</p>
    #[serde(rename = "replicationJobList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_list: Option<Vec<ReplicationJob>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetReplicationRunsRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetReplicationRunsResponse {
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Information about the replication job.</p>
    #[serde(rename = "replicationJob")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job: Option<ReplicationJob>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetServersRequest {
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of results.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>List of <code>VmServerAddress</code> objects</p>
    #[serde(rename = "vmServerAddressList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address_list: Option<Vec<VmServerAddress>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetServersResponse {
    /// <p>The time when the server was last modified.</p>
    #[serde(rename = "lastModifiedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_on: Option<f64>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The status of the server catalog.</p>
    #[serde(rename = "serverCatalogStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_catalog_status: Option<String>,
    /// <p>Information about the servers.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ImportServerCatalogRequest {}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ImportServerCatalogResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct LaunchAppRequest {
    /// <p>ID of the application to launch.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LaunchAppResponse {}

/// <p>Details about the latest launch of an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct LaunchDetails {
    /// <p>Latest time this application was launched successfully.</p>
    #[serde(rename = "latestLaunchTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_launch_time: Option<f64>,
    /// <p>Identifier of the latest stack launched for this application.</p>
    #[serde(rename = "stackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_id: Option<String>,
    /// <p>Name of the latest stack launched for this application.</p>
    #[serde(rename = "stackName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stack_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAppsRequest {
    /// <p><p/></p>
    #[serde(rename = "appIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ids: Option<Vec<String>>,
    /// <p>The maximum number of results to return in a single call. The default value is 50. To retrieve the remaining results, make another call with the returned <code>NextToken</code> value. </p>
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
pub struct ListAppsResponse {
    /// <p>A list of application summaries.</p>
    #[serde(rename = "apps")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apps: Option<Vec<AppSummary>>,
    /// <p>The token required to retrieve the next set of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAppLaunchConfigurationRequest {
    /// <p>ID of the application associated with the launch configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Name of service role in the customer's account that Amazon CloudFormation uses to launch the application.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>Launch configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_launch_configurations: Option<Vec<ServerGroupLaunchConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAppLaunchConfigurationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PutAppReplicationConfigurationRequest {
    /// <p>ID of the application tassociated with the replication configuration.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>Replication configurations for server groups in the application.</p>
    #[serde(rename = "serverGroupReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_replication_configurations: Option<Vec<ServerGroupReplicationConfiguration>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PutAppReplicationConfigurationResponse {}

/// <p>Represents a replication job.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationJob {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the replication job should produce encrypted AMIs or not. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The ID of the latest Amazon Machine Image (AMI).</p>
    #[serde(rename = "latestAmiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_ami_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>Number of recent AMIs to keep in the customer's account for a replication job. By default the value is set to zero, meaning that all AMIs are kept.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Information about the replication runs.</p>
    #[serde(rename = "replicationRunList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_list: Option<Vec<ReplicationRun>>,
    /// <p>The name of the IAM role to be used by the Server Migration Service.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>The seed replication time.</p>
    #[serde(rename = "seedReplicationTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_replication_time: Option<f64>,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>The state of the replication job.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>Represents a replication run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationRun {
    /// <p>The identifier of the Amazon Machine Image (AMI) from the replication run.</p>
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The completion time of the last replication run.</p>
    #[serde(rename = "completedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_time: Option<f64>,
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Whether the replication run should produce encrypted AMI or not. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer's default KMS key for EBS is used. </p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The identifier of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "scheduledStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<f64>,
    /// <p>Details of the current stage of the replication run.</p>
    #[serde(rename = "stageDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_details: Option<ReplicationRunStageDetails>,
    /// <p>The state of the replication run.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The description of the current status of the replication job.</p>
    #[serde(rename = "statusMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    /// <p>The type of replication run.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// <p>Details of the current stage of a replication run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ReplicationRunStageDetails {
    /// <p>String describing the current stage of a replication run.</p>
    #[serde(rename = "stage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    /// <p>String describing the progress of the current stage of a replication run.</p>
    #[serde(rename = "stageProgress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stage_progress: Option<String>,
}

/// <p>Location of the Amazon S3 object in the customer's account.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3Location {
    /// <p>Amazon S3 bucket name.</p>
    #[serde(rename = "bucket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// <p>Amazon S3 bucket key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// <p>Represents a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Server {
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_id: Option<String>,
    /// <p>Indicates whether the replication job is deleted or failed.</p>
    #[serde(rename = "replicationJobTerminated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_job_terminated: Option<bool>,
    /// <p>The identifier of the server.</p>
    #[serde(rename = "serverId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// <p>The type of server.</p>
    #[serde(rename = "serverType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,
    /// <p>Information about the VM server.</p>
    #[serde(rename = "vmServer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server: Option<VmServer>,
}

/// <p>A logical grouping of servers.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroup {
    /// <p>Name of a server group.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Identifier of a server group.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>List of servers belonging to a server group.</p>
    #[serde(rename = "serverList")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_list: Option<Vec<Server>>,
}

/// <p>Launch configuration for a server group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupLaunchConfiguration {
    /// <p>Launch order of servers in the server group.</p>
    #[serde(rename = "launchOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_order: Option<i64>,
    /// <p>Identifier of the server group the launch configuration is associated with.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>Launch configuration for servers in the server group.</p>
    #[serde(rename = "serverLaunchConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_launch_configurations: Option<Vec<ServerLaunchConfiguration>>,
}

/// <p>Replication configuration for a server group.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerGroupReplicationConfiguration {
    /// <p>Identifier of the server group this replication configuration is associated with.</p>
    #[serde(rename = "serverGroupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_group_id: Option<String>,
    /// <p>Replication configuration for servers in the server group.</p>
    #[serde(rename = "serverReplicationConfigurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_configurations: Option<Vec<ServerReplicationConfiguration>>,
}

/// <p>Launch configuration for a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerLaunchConfiguration {
    /// <p>If true, a publicly accessible IP address is created when launching the server.</p>
    #[serde(rename = "associatePublicIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_public_ip_address: Option<bool>,
    /// <p>Name of the EC2 SSH Key to be used for connecting to the launched server.</p>
    #[serde(rename = "ec2KeyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec_2_key_name: Option<String>,
    /// <p>Instance type to be used for launching the server.</p>
    #[serde(rename = "instanceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    /// <p>Logical ID of the server in the Amazon CloudFormation template.</p>
    #[serde(rename = "logicalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_id: Option<String>,
    /// <p>Identifier of the security group that applies to the launched server.</p>
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>Identifier of the server the launch configuration is associated with.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>Identifier of the subnet the server should be launched into.</p>
    #[serde(rename = "subnet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
    /// <p>Location of the user-data script to be executed when launching the server.</p>
    #[serde(rename = "userData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<UserData>,
    /// <p>Identifier of the VPC the server should be launched into.</p>
    #[serde(rename = "vpc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<String>,
}

/// <p>Replication configuration of a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerReplicationConfiguration {
    /// <p>Identifier of the server this replication configuration is associated with.</p>
    #[serde(rename = "server")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<Server>,
    /// <p>Parameters for replicating the server.</p>
    #[serde(rename = "serverReplicationParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_replication_parameters: Option<ServerReplicationParameters>,
}

/// <p>Replication parameters for replicating a server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerReplicationParameters {
    /// <p>When true, the replication job produces encrypted AMIs. See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>Frequency of creating replication jobs for the server.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p><p/> <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer&#39;s default KMS key for EBS is used. </p></p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>License type for creating a replication job for the server.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>Number of recent AMIs to keep when creating a replication job for this server.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p><p/></p>
    #[serde(rename = "runOnce")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_once: Option<bool>,
    /// <p>Seed time for creating a replication job for the server.</p>
    #[serde(rename = "seedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed_time: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAppReplicationRequest {
    /// <p>ID of the application to replicate.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartAppReplicationResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartOnDemandReplicationRunRequest {
    /// <p>The description of the replication run.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartOnDemandReplicationRunResponse {
    /// <p>The identifier of the replication run.</p>
    #[serde(rename = "replicationRunId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_run_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAppReplicationRequest {
    /// <p>ID of the application to stop replicating.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StopAppReplicationResponse {}

/// <p>A label that can be assigned to an application.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>Tag key.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>Tag value.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TerminateAppRequest {
    /// <p>ID of the application to terminate.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TerminateAppResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAppRequest {
    /// <p>ID of the application to update.</p>
    #[serde(rename = "appId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// <p>New description of the application.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>New name of the application.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>Name of the service role in the customer's account used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// <p>List of server groups in the application to update.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags to associate with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateAppResponse {
    /// <p>Summary description of the application.</p>
    #[serde(rename = "appSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_summary: Option<AppSummary>,
    /// <p>List of updated server groups in the application.</p>
    #[serde(rename = "serverGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_groups: Option<Vec<ServerGroup>>,
    /// <p>List of tags associated with the application.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateReplicationJobRequest {
    /// <p>The description of the replication job.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>When true, the replication job produces encrypted AMIs . See also <code>KmsKeyId</code> below.</p>
    #[serde(rename = "encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The time between consecutive replication runs, in hours.</p>
    #[serde(rename = "frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i64>,
    /// <p><p/> <p>KMS key ID for replication jobs that produce encrypted AMIs. Can be any of the following: </p> <ul> <li> <p>KMS key ID</p> </li> <li> <p>KMS key alias</p> </li> <li> <p>ARN referring to KMS key ID</p> </li> <li> <p>ARN referring to KMS key alias</p> </li> </ul> <p> If encrypted is <i>true</i> but a KMS key id is not specified, the customer&#39;s default KMS key for EBS is used. </p></p>
    #[serde(rename = "kmsKeyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    /// <p>The license type to be used for the AMI created by a successful replication run.</p>
    #[serde(rename = "licenseType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,
    /// <p>The start time of the next replication run.</p>
    #[serde(rename = "nextReplicationRunStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_replication_run_start_time: Option<f64>,
    /// <p>The maximum number of SMS-created AMIs to retain. The oldest will be deleted once the maximum number is reached and a new AMI is created.</p>
    #[serde(rename = "numberOfRecentAmisToKeep")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_recent_amis_to_keep: Option<i64>,
    /// <p>The identifier of the replication job.</p>
    #[serde(rename = "replicationJobId")]
    pub replication_job_id: String,
    /// <p>The name of the IAM role to be used by AWS SMS.</p>
    #[serde(rename = "roleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateReplicationJobResponse {}

/// <p>A script that runs on first launch of an Amazon EC2 instance. Used for configuring the server during launch.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserData {
    /// <p>Amazon S3 location of the user-data script.</p>
    #[serde(rename = "s3Location")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_location: Option<S3Location>,
}

/// <p>Represents a VM server.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VmServer {
    /// <p>The name of the VM manager.</p>
    #[serde(rename = "vmManagerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_name: Option<String>,
    /// <p>The type of VM management product.</p>
    #[serde(rename = "vmManagerType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_type: Option<String>,
    /// <p>The name of the VM.</p>
    #[serde(rename = "vmName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<String>,
    /// <p>The VM folder path in the vCenter Server virtual machine inventory tree.</p>
    #[serde(rename = "vmPath")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_path: Option<String>,
    /// <p>Information about the VM server location.</p>
    #[serde(rename = "vmServerAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_server_address: Option<VmServerAddress>,
}

/// <p>Represents a VM server location.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VmServerAddress {
    /// <p>The identifier of the VM.</p>
    #[serde(rename = "vmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_id: Option<String>,
    /// <p>The identifier of the VM manager.</p>
    #[serde(rename = "vmManagerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vm_manager_id: Option<String>,
}

/// Errors returned by CreateApp
#[derive(Debug, PartialEq)]
pub enum CreateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl CreateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return CreateAppError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return CreateAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return CreateAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return CreateAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return CreateAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateAppError {
    fn from(err: serde_json::error::Error) -> CreateAppError {
        CreateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateAppError {
    fn from(err: CredentialsError) -> CreateAppError {
        CreateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateAppError {
    fn from(err: HttpDispatchError) -> CreateAppError {
        CreateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateAppError {
    fn from(err: io::Error) -> CreateAppError {
        CreateAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAppError {
    fn description(&self) -> &str {
        match *self {
            CreateAppError::InternalError(ref cause) => cause,
            CreateAppError::InvalidParameter(ref cause) => cause,
            CreateAppError::MissingRequiredParameter(ref cause) => cause,
            CreateAppError::OperationNotPermitted(ref cause) => cause,
            CreateAppError::UnauthorizedOperation(ref cause) => cause,
            CreateAppError::Validation(ref cause) => cause,
            CreateAppError::Credentials(ref err) => err.description(),
            CreateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            CreateAppError::ParseError(ref cause) => cause,
            CreateAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by CreateReplicationJob
#[derive(Debug, PartialEq)]
pub enum CreateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job already exists.</p>
    ReplicationJobAlreadyExists(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl CreateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> CreateReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return CreateReplicationJobError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return CreateReplicationJobError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return CreateReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "NoConnectorsAvailableException" => {
                    return CreateReplicationJobError::NoConnectorsAvailable(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return CreateReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ReplicationJobAlreadyExistsException" => {
                    return CreateReplicationJobError::ReplicationJobAlreadyExists(String::from(
                        error_message,
                    ));
                }
                "ServerCannotBeReplicatedException" => {
                    return CreateReplicationJobError::ServerCannotBeReplicated(String::from(
                        error_message,
                    ));
                }
                "TemporarilyUnavailableException" => {
                    return CreateReplicationJobError::TemporarilyUnavailable(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return CreateReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return CreateReplicationJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return CreateReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for CreateReplicationJobError {
    fn from(err: serde_json::error::Error) -> CreateReplicationJobError {
        CreateReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for CreateReplicationJobError {
    fn from(err: CredentialsError) -> CreateReplicationJobError {
        CreateReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for CreateReplicationJobError {
    fn from(err: HttpDispatchError) -> CreateReplicationJobError {
        CreateReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for CreateReplicationJobError {
    fn from(err: io::Error) -> CreateReplicationJobError {
        CreateReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for CreateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            CreateReplicationJobError::InternalError(ref cause) => cause,
            CreateReplicationJobError::InvalidParameter(ref cause) => cause,
            CreateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            CreateReplicationJobError::NoConnectorsAvailable(ref cause) => cause,
            CreateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            CreateReplicationJobError::ReplicationJobAlreadyExists(ref cause) => cause,
            CreateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            CreateReplicationJobError::TemporarilyUnavailable(ref cause) => cause,
            CreateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            CreateReplicationJobError::Validation(ref cause) => cause,
            CreateReplicationJobError::Credentials(ref err) => err.description(),
            CreateReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            CreateReplicationJobError::ParseError(ref cause) => cause,
            CreateReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteApp
#[derive(Debug, PartialEq)]
pub enum DeleteAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DeleteAppError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return DeleteAppError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return DeleteAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return DeleteAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return DeleteAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return DeleteAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return DeleteAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAppError {
    fn from(err: serde_json::error::Error) -> DeleteAppError {
        DeleteAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppError {
    fn from(err: CredentialsError) -> DeleteAppError {
        DeleteAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppError {
    fn from(err: HttpDispatchError) -> DeleteAppError {
        DeleteAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppError {
    fn from(err: io::Error) -> DeleteAppError {
        DeleteAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppError::InternalError(ref cause) => cause,
            DeleteAppError::InvalidParameter(ref cause) => cause,
            DeleteAppError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppError::OperationNotPermitted(ref cause) => cause,
            DeleteAppError::UnauthorizedOperation(ref cause) => cause,
            DeleteAppError::Validation(ref cause) => cause,
            DeleteAppError::Credentials(ref err) => err.description(),
            DeleteAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteAppError::ParseError(ref cause) => cause,
            DeleteAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DeleteAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAppLaunchConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return DeleteAppLaunchConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return DeleteAppLaunchConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return DeleteAppLaunchConfigurationError::MissingRequiredParameter(
                        String::from(error_message),
                    );
                }
                "OperationNotPermittedException" => {
                    return DeleteAppLaunchConfigurationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return DeleteAppLaunchConfigurationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteAppLaunchConfigurationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteAppLaunchConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAppLaunchConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteAppLaunchConfigurationError {
        DeleteAppLaunchConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppLaunchConfigurationError {
    fn from(err: CredentialsError) -> DeleteAppLaunchConfigurationError {
        DeleteAppLaunchConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppLaunchConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteAppLaunchConfigurationError {
        DeleteAppLaunchConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppLaunchConfigurationError {
    fn from(err: io::Error) -> DeleteAppLaunchConfigurationError {
        DeleteAppLaunchConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppLaunchConfigurationError::InternalError(ref cause) => cause,
            DeleteAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            DeleteAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            DeleteAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
            DeleteAppLaunchConfigurationError::Validation(ref cause) => cause,
            DeleteAppLaunchConfigurationError::Credentials(ref err) => err.description(),
            DeleteAppLaunchConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAppLaunchConfigurationError::ParseError(ref cause) => cause,
            DeleteAppLaunchConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum DeleteAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DeleteAppReplicationConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteAppReplicationConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return DeleteAppReplicationConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return DeleteAppReplicationConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return DeleteAppReplicationConfigurationError::MissingRequiredParameter(
                        String::from(error_message),
                    );
                }
                "OperationNotPermittedException" => {
                    return DeleteAppReplicationConfigurationError::OperationNotPermitted(
                        String::from(error_message),
                    );
                }
                "UnauthorizedOperationException" => {
                    return DeleteAppReplicationConfigurationError::UnauthorizedOperation(
                        String::from(error_message),
                    );
                }
                "ValidationException" => {
                    return DeleteAppReplicationConfigurationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return DeleteAppReplicationConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteAppReplicationConfigurationError {
    fn from(err: serde_json::error::Error) -> DeleteAppReplicationConfigurationError {
        DeleteAppReplicationConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteAppReplicationConfigurationError {
    fn from(err: CredentialsError) -> DeleteAppReplicationConfigurationError {
        DeleteAppReplicationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteAppReplicationConfigurationError {
    fn from(err: HttpDispatchError) -> DeleteAppReplicationConfigurationError {
        DeleteAppReplicationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteAppReplicationConfigurationError {
    fn from(err: io::Error) -> DeleteAppReplicationConfigurationError {
        DeleteAppReplicationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            DeleteAppReplicationConfigurationError::InternalError(ref cause) => cause,
            DeleteAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            DeleteAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            DeleteAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            DeleteAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
            DeleteAppReplicationConfigurationError::Validation(ref cause) => cause,
            DeleteAppReplicationConfigurationError::Credentials(ref err) => err.description(),
            DeleteAppReplicationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteAppReplicationConfigurationError::ParseError(ref cause) => cause,
            DeleteAppReplicationConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteReplicationJob
#[derive(Debug, PartialEq)]
pub enum DeleteReplicationJobError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DeleteReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DeleteReplicationJobError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return DeleteReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return DeleteReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ReplicationJobNotFoundException" => {
                    return DeleteReplicationJobError::ReplicationJobNotFound(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return DeleteReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteReplicationJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteReplicationJobError {
    fn from(err: serde_json::error::Error) -> DeleteReplicationJobError {
        DeleteReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteReplicationJobError {
    fn from(err: CredentialsError) -> DeleteReplicationJobError {
        DeleteReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteReplicationJobError {
    fn from(err: HttpDispatchError) -> DeleteReplicationJobError {
        DeleteReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteReplicationJobError {
    fn from(err: io::Error) -> DeleteReplicationJobError {
        DeleteReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            DeleteReplicationJobError::InvalidParameter(ref cause) => cause,
            DeleteReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            DeleteReplicationJobError::OperationNotPermitted(ref cause) => cause,
            DeleteReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            DeleteReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            DeleteReplicationJobError::Validation(ref cause) => cause,
            DeleteReplicationJobError::Credentials(ref err) => err.description(),
            DeleteReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteReplicationJobError::ParseError(ref cause) => cause,
            DeleteReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteServerCatalog
#[derive(Debug, PartialEq)]
pub enum DeleteServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DeleteServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> DeleteServerCatalogError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DeleteServerCatalogError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return DeleteServerCatalogError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return DeleteServerCatalogError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return DeleteServerCatalogError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DeleteServerCatalogError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DeleteServerCatalogError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DeleteServerCatalogError {
    fn from(err: serde_json::error::Error) -> DeleteServerCatalogError {
        DeleteServerCatalogError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DeleteServerCatalogError {
    fn from(err: CredentialsError) -> DeleteServerCatalogError {
        DeleteServerCatalogError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DeleteServerCatalogError {
    fn from(err: HttpDispatchError) -> DeleteServerCatalogError {
        DeleteServerCatalogError::HttpDispatch(err)
    }
}
impl From<io::Error> for DeleteServerCatalogError {
    fn from(err: io::Error) -> DeleteServerCatalogError {
        DeleteServerCatalogError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DeleteServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            DeleteServerCatalogError::InvalidParameter(ref cause) => cause,
            DeleteServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            DeleteServerCatalogError::OperationNotPermitted(ref cause) => cause,
            DeleteServerCatalogError::UnauthorizedOperation(ref cause) => cause,
            DeleteServerCatalogError::Validation(ref cause) => cause,
            DeleteServerCatalogError::Credentials(ref err) => err.description(),
            DeleteServerCatalogError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DeleteServerCatalogError::ParseError(ref cause) => cause,
            DeleteServerCatalogError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DisassociateConnector
#[derive(Debug, PartialEq)]
pub enum DisassociateConnectorError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl DisassociateConnectorError {
    pub fn from_response(res: BufferedHttpResponse) -> DisassociateConnectorError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return DisassociateConnectorError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return DisassociateConnectorError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return DisassociateConnectorError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return DisassociateConnectorError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return DisassociateConnectorError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return DisassociateConnectorError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DisassociateConnectorError {
    fn from(err: serde_json::error::Error) -> DisassociateConnectorError {
        DisassociateConnectorError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DisassociateConnectorError {
    fn from(err: CredentialsError) -> DisassociateConnectorError {
        DisassociateConnectorError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DisassociateConnectorError {
    fn from(err: HttpDispatchError) -> DisassociateConnectorError {
        DisassociateConnectorError::HttpDispatch(err)
    }
}
impl From<io::Error> for DisassociateConnectorError {
    fn from(err: io::Error) -> DisassociateConnectorError {
        DisassociateConnectorError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DisassociateConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DisassociateConnectorError {
    fn description(&self) -> &str {
        match *self {
            DisassociateConnectorError::InvalidParameter(ref cause) => cause,
            DisassociateConnectorError::MissingRequiredParameter(ref cause) => cause,
            DisassociateConnectorError::OperationNotPermitted(ref cause) => cause,
            DisassociateConnectorError::UnauthorizedOperation(ref cause) => cause,
            DisassociateConnectorError::Validation(ref cause) => cause,
            DisassociateConnectorError::Credentials(ref err) => err.description(),
            DisassociateConnectorError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DisassociateConnectorError::ParseError(ref cause) => cause,
            DisassociateConnectorError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GenerateChangeSet
#[derive(Debug, PartialEq)]
pub enum GenerateChangeSetError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GenerateChangeSetError {
    pub fn from_response(res: BufferedHttpResponse) -> GenerateChangeSetError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return GenerateChangeSetError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return GenerateChangeSetError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return GenerateChangeSetError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return GenerateChangeSetError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return GenerateChangeSetError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GenerateChangeSetError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GenerateChangeSetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GenerateChangeSetError {
    fn from(err: serde_json::error::Error) -> GenerateChangeSetError {
        GenerateChangeSetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateChangeSetError {
    fn from(err: CredentialsError) -> GenerateChangeSetError {
        GenerateChangeSetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateChangeSetError {
    fn from(err: HttpDispatchError) -> GenerateChangeSetError {
        GenerateChangeSetError::HttpDispatch(err)
    }
}
impl From<io::Error> for GenerateChangeSetError {
    fn from(err: io::Error) -> GenerateChangeSetError {
        GenerateChangeSetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GenerateChangeSetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateChangeSetError {
    fn description(&self) -> &str {
        match *self {
            GenerateChangeSetError::InternalError(ref cause) => cause,
            GenerateChangeSetError::InvalidParameter(ref cause) => cause,
            GenerateChangeSetError::MissingRequiredParameter(ref cause) => cause,
            GenerateChangeSetError::OperationNotPermitted(ref cause) => cause,
            GenerateChangeSetError::UnauthorizedOperation(ref cause) => cause,
            GenerateChangeSetError::Validation(ref cause) => cause,
            GenerateChangeSetError::Credentials(ref err) => err.description(),
            GenerateChangeSetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GenerateChangeSetError::ParseError(ref cause) => cause,
            GenerateChangeSetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GenerateTemplate
#[derive(Debug, PartialEq)]
pub enum GenerateTemplateError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GenerateTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> GenerateTemplateError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return GenerateTemplateError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return GenerateTemplateError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return GenerateTemplateError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return GenerateTemplateError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return GenerateTemplateError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return GenerateTemplateError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GenerateTemplateError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GenerateTemplateError {
    fn from(err: serde_json::error::Error) -> GenerateTemplateError {
        GenerateTemplateError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GenerateTemplateError {
    fn from(err: CredentialsError) -> GenerateTemplateError {
        GenerateTemplateError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GenerateTemplateError {
    fn from(err: HttpDispatchError) -> GenerateTemplateError {
        GenerateTemplateError::HttpDispatch(err)
    }
}
impl From<io::Error> for GenerateTemplateError {
    fn from(err: io::Error) -> GenerateTemplateError {
        GenerateTemplateError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GenerateTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GenerateTemplateError {
    fn description(&self) -> &str {
        match *self {
            GenerateTemplateError::InternalError(ref cause) => cause,
            GenerateTemplateError::InvalidParameter(ref cause) => cause,
            GenerateTemplateError::MissingRequiredParameter(ref cause) => cause,
            GenerateTemplateError::OperationNotPermitted(ref cause) => cause,
            GenerateTemplateError::UnauthorizedOperation(ref cause) => cause,
            GenerateTemplateError::Validation(ref cause) => cause,
            GenerateTemplateError::Credentials(ref err) => err.description(),
            GenerateTemplateError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GenerateTemplateError::ParseError(ref cause) => cause,
            GenerateTemplateError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetApp
#[derive(Debug, PartialEq)]
pub enum GetAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetAppError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => return GetAppError::InternalError(String::from(error_message)),
                "InvalidParameterException" => {
                    return GetAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return GetAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return GetAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return GetAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => return GetAppError::Validation(error_message.to_string()),
                _ => {}
            }
        }
        return GetAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppError {
    fn from(err: serde_json::error::Error) -> GetAppError {
        GetAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppError {
    fn from(err: CredentialsError) -> GetAppError {
        GetAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppError {
    fn from(err: HttpDispatchError) -> GetAppError {
        GetAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppError {
    fn from(err: io::Error) -> GetAppError {
        GetAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppError {
    fn description(&self) -> &str {
        match *self {
            GetAppError::InternalError(ref cause) => cause,
            GetAppError::InvalidParameter(ref cause) => cause,
            GetAppError::MissingRequiredParameter(ref cause) => cause,
            GetAppError::OperationNotPermitted(ref cause) => cause,
            GetAppError::UnauthorizedOperation(ref cause) => cause,
            GetAppError::Validation(ref cause) => cause,
            GetAppError::Credentials(ref err) => err.description(),
            GetAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetAppError::ParseError(ref cause) => cause,
            GetAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAppLaunchConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return GetAppLaunchConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return GetAppLaunchConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return GetAppLaunchConfigurationError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return GetAppLaunchConfigurationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return GetAppLaunchConfigurationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetAppLaunchConfigurationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetAppLaunchConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppLaunchConfigurationError {
    fn from(err: serde_json::error::Error) -> GetAppLaunchConfigurationError {
        GetAppLaunchConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppLaunchConfigurationError {
    fn from(err: CredentialsError) -> GetAppLaunchConfigurationError {
        GetAppLaunchConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppLaunchConfigurationError {
    fn from(err: HttpDispatchError) -> GetAppLaunchConfigurationError {
        GetAppLaunchConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppLaunchConfigurationError {
    fn from(err: io::Error) -> GetAppLaunchConfigurationError {
        GetAppLaunchConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetAppLaunchConfigurationError::InternalError(ref cause) => cause,
            GetAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            GetAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            GetAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            GetAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
            GetAppLaunchConfigurationError::Validation(ref cause) => cause,
            GetAppLaunchConfigurationError::Credentials(ref err) => err.description(),
            GetAppLaunchConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAppLaunchConfigurationError::ParseError(ref cause) => cause,
            GetAppLaunchConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum GetAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetAppReplicationConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> GetAppReplicationConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return GetAppReplicationConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return GetAppReplicationConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return GetAppReplicationConfigurationError::MissingRequiredParameter(
                        String::from(error_message),
                    );
                }
                "OperationNotPermittedException" => {
                    return GetAppReplicationConfigurationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return GetAppReplicationConfigurationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetAppReplicationConfigurationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return GetAppReplicationConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetAppReplicationConfigurationError {
    fn from(err: serde_json::error::Error) -> GetAppReplicationConfigurationError {
        GetAppReplicationConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetAppReplicationConfigurationError {
    fn from(err: CredentialsError) -> GetAppReplicationConfigurationError {
        GetAppReplicationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetAppReplicationConfigurationError {
    fn from(err: HttpDispatchError) -> GetAppReplicationConfigurationError {
        GetAppReplicationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetAppReplicationConfigurationError {
    fn from(err: io::Error) -> GetAppReplicationConfigurationError {
        GetAppReplicationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetAppReplicationConfigurationError::InternalError(ref cause) => cause,
            GetAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            GetAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            GetAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            GetAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
            GetAppReplicationConfigurationError::Validation(ref cause) => cause,
            GetAppReplicationConfigurationError::Credentials(ref err) => err.description(),
            GetAppReplicationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetAppReplicationConfigurationError::ParseError(ref cause) => cause,
            GetAppReplicationConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetConnectors
#[derive(Debug, PartialEq)]
pub enum GetConnectorsError {
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetConnectorsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetConnectorsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UnauthorizedOperationException" => {
                    return GetConnectorsError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return GetConnectorsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetConnectorsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetConnectorsError {
    fn from(err: serde_json::error::Error) -> GetConnectorsError {
        GetConnectorsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetConnectorsError {
    fn from(err: CredentialsError) -> GetConnectorsError {
        GetConnectorsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetConnectorsError {
    fn from(err: HttpDispatchError) -> GetConnectorsError {
        GetConnectorsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetConnectorsError {
    fn from(err: io::Error) -> GetConnectorsError {
        GetConnectorsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetConnectorsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetConnectorsError {
    fn description(&self) -> &str {
        match *self {
            GetConnectorsError::UnauthorizedOperation(ref cause) => cause,
            GetConnectorsError::Validation(ref cause) => cause,
            GetConnectorsError::Credentials(ref err) => err.description(),
            GetConnectorsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetConnectorsError::ParseError(ref cause) => cause,
            GetConnectorsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetReplicationJobs
#[derive(Debug, PartialEq)]
pub enum GetReplicationJobsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetReplicationJobsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetReplicationJobsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return GetReplicationJobsError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return GetReplicationJobsError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return GetReplicationJobsError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetReplicationJobsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetReplicationJobsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetReplicationJobsError {
    fn from(err: serde_json::error::Error) -> GetReplicationJobsError {
        GetReplicationJobsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReplicationJobsError {
    fn from(err: CredentialsError) -> GetReplicationJobsError {
        GetReplicationJobsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReplicationJobsError {
    fn from(err: HttpDispatchError) -> GetReplicationJobsError {
        GetReplicationJobsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReplicationJobsError {
    fn from(err: io::Error) -> GetReplicationJobsError {
        GetReplicationJobsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReplicationJobsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationJobsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationJobsError::InvalidParameter(ref cause) => cause,
            GetReplicationJobsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationJobsError::UnauthorizedOperation(ref cause) => cause,
            GetReplicationJobsError::Validation(ref cause) => cause,
            GetReplicationJobsError::Credentials(ref err) => err.description(),
            GetReplicationJobsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReplicationJobsError::ParseError(ref cause) => cause,
            GetReplicationJobsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetReplicationRuns
#[derive(Debug, PartialEq)]
pub enum GetReplicationRunsError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetReplicationRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> GetReplicationRunsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return GetReplicationRunsError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return GetReplicationRunsError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return GetReplicationRunsError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return GetReplicationRunsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetReplicationRunsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetReplicationRunsError {
    fn from(err: serde_json::error::Error) -> GetReplicationRunsError {
        GetReplicationRunsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetReplicationRunsError {
    fn from(err: CredentialsError) -> GetReplicationRunsError {
        GetReplicationRunsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetReplicationRunsError {
    fn from(err: HttpDispatchError) -> GetReplicationRunsError {
        GetReplicationRunsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetReplicationRunsError {
    fn from(err: io::Error) -> GetReplicationRunsError {
        GetReplicationRunsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetReplicationRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetReplicationRunsError {
    fn description(&self) -> &str {
        match *self {
            GetReplicationRunsError::InvalidParameter(ref cause) => cause,
            GetReplicationRunsError::MissingRequiredParameter(ref cause) => cause,
            GetReplicationRunsError::UnauthorizedOperation(ref cause) => cause,
            GetReplicationRunsError::Validation(ref cause) => cause,
            GetReplicationRunsError::Credentials(ref err) => err.description(),
            GetReplicationRunsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetReplicationRunsError::ParseError(ref cause) => cause,
            GetReplicationRunsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetServers
#[derive(Debug, PartialEq)]
pub enum GetServersError {
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl GetServersError {
    pub fn from_response(res: BufferedHttpResponse) -> GetServersError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "UnauthorizedOperationException" => {
                    return GetServersError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return GetServersError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return GetServersError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetServersError {
    fn from(err: serde_json::error::Error) -> GetServersError {
        GetServersError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetServersError {
    fn from(err: CredentialsError) -> GetServersError {
        GetServersError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetServersError {
    fn from(err: HttpDispatchError) -> GetServersError {
        GetServersError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetServersError {
    fn from(err: io::Error) -> GetServersError {
        GetServersError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetServersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetServersError {
    fn description(&self) -> &str {
        match *self {
            GetServersError::UnauthorizedOperation(ref cause) => cause,
            GetServersError::Validation(ref cause) => cause,
            GetServersError::Credentials(ref err) => err.description(),
            GetServersError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetServersError::ParseError(ref cause) => cause,
            GetServersError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ImportServerCatalog
#[derive(Debug, PartialEq)]
pub enum ImportServerCatalogError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>There are no connectors available.</p>
    NoConnectorsAvailable(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl ImportServerCatalogError {
    pub fn from_response(res: BufferedHttpResponse) -> ImportServerCatalogError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return ImportServerCatalogError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return ImportServerCatalogError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "NoConnectorsAvailableException" => {
                    return ImportServerCatalogError::NoConnectorsAvailable(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return ImportServerCatalogError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return ImportServerCatalogError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return ImportServerCatalogError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ImportServerCatalogError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ImportServerCatalogError {
    fn from(err: serde_json::error::Error) -> ImportServerCatalogError {
        ImportServerCatalogError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ImportServerCatalogError {
    fn from(err: CredentialsError) -> ImportServerCatalogError {
        ImportServerCatalogError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ImportServerCatalogError {
    fn from(err: HttpDispatchError) -> ImportServerCatalogError {
        ImportServerCatalogError::HttpDispatch(err)
    }
}
impl From<io::Error> for ImportServerCatalogError {
    fn from(err: io::Error) -> ImportServerCatalogError {
        ImportServerCatalogError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ImportServerCatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ImportServerCatalogError {
    fn description(&self) -> &str {
        match *self {
            ImportServerCatalogError::InvalidParameter(ref cause) => cause,
            ImportServerCatalogError::MissingRequiredParameter(ref cause) => cause,
            ImportServerCatalogError::NoConnectorsAvailable(ref cause) => cause,
            ImportServerCatalogError::OperationNotPermitted(ref cause) => cause,
            ImportServerCatalogError::UnauthorizedOperation(ref cause) => cause,
            ImportServerCatalogError::Validation(ref cause) => cause,
            ImportServerCatalogError::Credentials(ref err) => err.description(),
            ImportServerCatalogError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ImportServerCatalogError::ParseError(ref cause) => cause,
            ImportServerCatalogError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by LaunchApp
#[derive(Debug, PartialEq)]
pub enum LaunchAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl LaunchAppError {
    pub fn from_response(res: BufferedHttpResponse) -> LaunchAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return LaunchAppError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return LaunchAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return LaunchAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return LaunchAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return LaunchAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return LaunchAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return LaunchAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for LaunchAppError {
    fn from(err: serde_json::error::Error) -> LaunchAppError {
        LaunchAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for LaunchAppError {
    fn from(err: CredentialsError) -> LaunchAppError {
        LaunchAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for LaunchAppError {
    fn from(err: HttpDispatchError) -> LaunchAppError {
        LaunchAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for LaunchAppError {
    fn from(err: io::Error) -> LaunchAppError {
        LaunchAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for LaunchAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for LaunchAppError {
    fn description(&self) -> &str {
        match *self {
            LaunchAppError::InternalError(ref cause) => cause,
            LaunchAppError::InvalidParameter(ref cause) => cause,
            LaunchAppError::MissingRequiredParameter(ref cause) => cause,
            LaunchAppError::OperationNotPermitted(ref cause) => cause,
            LaunchAppError::UnauthorizedOperation(ref cause) => cause,
            LaunchAppError::Validation(ref cause) => cause,
            LaunchAppError::Credentials(ref err) => err.description(),
            LaunchAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            LaunchAppError::ParseError(ref cause) => cause,
            LaunchAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListApps
#[derive(Debug, PartialEq)]
pub enum ListAppsError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl ListAppsError {
    pub fn from_response(res: BufferedHttpResponse) -> ListAppsError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => return ListAppsError::InternalError(String::from(error_message)),
                "InvalidParameterException" => {
                    return ListAppsError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return ListAppsError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return ListAppsError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return ListAppsError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return ListAppsError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return ListAppsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListAppsError {
    fn from(err: serde_json::error::Error) -> ListAppsError {
        ListAppsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListAppsError {
    fn from(err: CredentialsError) -> ListAppsError {
        ListAppsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListAppsError {
    fn from(err: HttpDispatchError) -> ListAppsError {
        ListAppsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListAppsError {
    fn from(err: io::Error) -> ListAppsError {
        ListAppsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListAppsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAppsError {
    fn description(&self) -> &str {
        match *self {
            ListAppsError::InternalError(ref cause) => cause,
            ListAppsError::InvalidParameter(ref cause) => cause,
            ListAppsError::MissingRequiredParameter(ref cause) => cause,
            ListAppsError::OperationNotPermitted(ref cause) => cause,
            ListAppsError::UnauthorizedOperation(ref cause) => cause,
            ListAppsError::Validation(ref cause) => cause,
            ListAppsError::Credentials(ref err) => err.description(),
            ListAppsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListAppsError::ParseError(ref cause) => cause,
            ListAppsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutAppLaunchConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppLaunchConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl PutAppLaunchConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> PutAppLaunchConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return PutAppLaunchConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return PutAppLaunchConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return PutAppLaunchConfigurationError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return PutAppLaunchConfigurationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return PutAppLaunchConfigurationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutAppLaunchConfigurationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return PutAppLaunchConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutAppLaunchConfigurationError {
    fn from(err: serde_json::error::Error) -> PutAppLaunchConfigurationError {
        PutAppLaunchConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutAppLaunchConfigurationError {
    fn from(err: CredentialsError) -> PutAppLaunchConfigurationError {
        PutAppLaunchConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAppLaunchConfigurationError {
    fn from(err: HttpDispatchError) -> PutAppLaunchConfigurationError {
        PutAppLaunchConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAppLaunchConfigurationError {
    fn from(err: io::Error) -> PutAppLaunchConfigurationError {
        PutAppLaunchConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAppLaunchConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAppLaunchConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutAppLaunchConfigurationError::InternalError(ref cause) => cause,
            PutAppLaunchConfigurationError::InvalidParameter(ref cause) => cause,
            PutAppLaunchConfigurationError::MissingRequiredParameter(ref cause) => cause,
            PutAppLaunchConfigurationError::OperationNotPermitted(ref cause) => cause,
            PutAppLaunchConfigurationError::UnauthorizedOperation(ref cause) => cause,
            PutAppLaunchConfigurationError::Validation(ref cause) => cause,
            PutAppLaunchConfigurationError::Credentials(ref err) => err.description(),
            PutAppLaunchConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutAppLaunchConfigurationError::ParseError(ref cause) => cause,
            PutAppLaunchConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by PutAppReplicationConfiguration
#[derive(Debug, PartialEq)]
pub enum PutAppReplicationConfigurationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl PutAppReplicationConfigurationError {
    pub fn from_response(res: BufferedHttpResponse) -> PutAppReplicationConfigurationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return PutAppReplicationConfigurationError::InternalError(String::from(
                        error_message,
                    ));
                }
                "InvalidParameterException" => {
                    return PutAppReplicationConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return PutAppReplicationConfigurationError::MissingRequiredParameter(
                        String::from(error_message),
                    );
                }
                "OperationNotPermittedException" => {
                    return PutAppReplicationConfigurationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return PutAppReplicationConfigurationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return PutAppReplicationConfigurationError::Validation(
                        error_message.to_string(),
                    );
                }
                _ => {}
            }
        }
        return PutAppReplicationConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for PutAppReplicationConfigurationError {
    fn from(err: serde_json::error::Error) -> PutAppReplicationConfigurationError {
        PutAppReplicationConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for PutAppReplicationConfigurationError {
    fn from(err: CredentialsError) -> PutAppReplicationConfigurationError {
        PutAppReplicationConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for PutAppReplicationConfigurationError {
    fn from(err: HttpDispatchError) -> PutAppReplicationConfigurationError {
        PutAppReplicationConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for PutAppReplicationConfigurationError {
    fn from(err: io::Error) -> PutAppReplicationConfigurationError {
        PutAppReplicationConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for PutAppReplicationConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PutAppReplicationConfigurationError {
    fn description(&self) -> &str {
        match *self {
            PutAppReplicationConfigurationError::InternalError(ref cause) => cause,
            PutAppReplicationConfigurationError::InvalidParameter(ref cause) => cause,
            PutAppReplicationConfigurationError::MissingRequiredParameter(ref cause) => cause,
            PutAppReplicationConfigurationError::OperationNotPermitted(ref cause) => cause,
            PutAppReplicationConfigurationError::UnauthorizedOperation(ref cause) => cause,
            PutAppReplicationConfigurationError::Validation(ref cause) => cause,
            PutAppReplicationConfigurationError::Credentials(ref err) => err.description(),
            PutAppReplicationConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            PutAppReplicationConfigurationError::ParseError(ref cause) => cause,
            PutAppReplicationConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartAppReplication
#[derive(Debug, PartialEq)]
pub enum StartAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl StartAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> StartAppReplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return StartAppReplicationError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return StartAppReplicationError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return StartAppReplicationError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return StartAppReplicationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return StartAppReplicationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartAppReplicationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartAppReplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartAppReplicationError {
    fn from(err: serde_json::error::Error) -> StartAppReplicationError {
        StartAppReplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartAppReplicationError {
    fn from(err: CredentialsError) -> StartAppReplicationError {
        StartAppReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartAppReplicationError {
    fn from(err: HttpDispatchError) -> StartAppReplicationError {
        StartAppReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartAppReplicationError {
    fn from(err: io::Error) -> StartAppReplicationError {
        StartAppReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartAppReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAppReplicationError {
    fn description(&self) -> &str {
        match *self {
            StartAppReplicationError::InternalError(ref cause) => cause,
            StartAppReplicationError::InvalidParameter(ref cause) => cause,
            StartAppReplicationError::MissingRequiredParameter(ref cause) => cause,
            StartAppReplicationError::OperationNotPermitted(ref cause) => cause,
            StartAppReplicationError::UnauthorizedOperation(ref cause) => cause,
            StartAppReplicationError::Validation(ref cause) => cause,
            StartAppReplicationError::Credentials(ref err) => err.description(),
            StartAppReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartAppReplicationError::ParseError(ref cause) => cause,
            StartAppReplicationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StartOnDemandReplicationRun
#[derive(Debug, PartialEq)]
pub enum StartOnDemandReplicationRunError {
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You have exceeded the number of on-demand replication runs you can request in a 24-hour period.</p>
    ReplicationRunLimitExceeded(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl StartOnDemandReplicationRunError {
    pub fn from_response(res: BufferedHttpResponse) -> StartOnDemandReplicationRunError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InvalidParameterException" => {
                    return StartOnDemandReplicationRunError::InvalidParameter(String::from(
                        error_message,
                    ));
                }
                "MissingRequiredParameterException" => {
                    return StartOnDemandReplicationRunError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return StartOnDemandReplicationRunError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ReplicationRunLimitExceededException" => {
                    return StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(
                        String::from(error_message),
                    );
                }
                "UnauthorizedOperationException" => {
                    return StartOnDemandReplicationRunError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StartOnDemandReplicationRunError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StartOnDemandReplicationRunError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StartOnDemandReplicationRunError {
    fn from(err: serde_json::error::Error) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StartOnDemandReplicationRunError {
    fn from(err: CredentialsError) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StartOnDemandReplicationRunError {
    fn from(err: HttpDispatchError) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::HttpDispatch(err)
    }
}
impl From<io::Error> for StartOnDemandReplicationRunError {
    fn from(err: io::Error) -> StartOnDemandReplicationRunError {
        StartOnDemandReplicationRunError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StartOnDemandReplicationRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartOnDemandReplicationRunError {
    fn description(&self) -> &str {
        match *self {
            StartOnDemandReplicationRunError::InvalidParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::MissingRequiredParameter(ref cause) => cause,
            StartOnDemandReplicationRunError::OperationNotPermitted(ref cause) => cause,
            StartOnDemandReplicationRunError::ReplicationRunLimitExceeded(ref cause) => cause,
            StartOnDemandReplicationRunError::UnauthorizedOperation(ref cause) => cause,
            StartOnDemandReplicationRunError::Validation(ref cause) => cause,
            StartOnDemandReplicationRunError::Credentials(ref err) => err.description(),
            StartOnDemandReplicationRunError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StartOnDemandReplicationRunError::ParseError(ref cause) => cause,
            StartOnDemandReplicationRunError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by StopAppReplication
#[derive(Debug, PartialEq)]
pub enum StopAppReplicationError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl StopAppReplicationError {
    pub fn from_response(res: BufferedHttpResponse) -> StopAppReplicationError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return StopAppReplicationError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return StopAppReplicationError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return StopAppReplicationError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return StopAppReplicationError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return StopAppReplicationError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return StopAppReplicationError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return StopAppReplicationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for StopAppReplicationError {
    fn from(err: serde_json::error::Error) -> StopAppReplicationError {
        StopAppReplicationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for StopAppReplicationError {
    fn from(err: CredentialsError) -> StopAppReplicationError {
        StopAppReplicationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for StopAppReplicationError {
    fn from(err: HttpDispatchError) -> StopAppReplicationError {
        StopAppReplicationError::HttpDispatch(err)
    }
}
impl From<io::Error> for StopAppReplicationError {
    fn from(err: io::Error) -> StopAppReplicationError {
        StopAppReplicationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for StopAppReplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAppReplicationError {
    fn description(&self) -> &str {
        match *self {
            StopAppReplicationError::InternalError(ref cause) => cause,
            StopAppReplicationError::InvalidParameter(ref cause) => cause,
            StopAppReplicationError::MissingRequiredParameter(ref cause) => cause,
            StopAppReplicationError::OperationNotPermitted(ref cause) => cause,
            StopAppReplicationError::UnauthorizedOperation(ref cause) => cause,
            StopAppReplicationError::Validation(ref cause) => cause,
            StopAppReplicationError::Credentials(ref err) => err.description(),
            StopAppReplicationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            StopAppReplicationError::ParseError(ref cause) => cause,
            StopAppReplicationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by TerminateApp
#[derive(Debug, PartialEq)]
pub enum TerminateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl TerminateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> TerminateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return TerminateAppError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return TerminateAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return TerminateAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return TerminateAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return TerminateAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return TerminateAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return TerminateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for TerminateAppError {
    fn from(err: serde_json::error::Error) -> TerminateAppError {
        TerminateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for TerminateAppError {
    fn from(err: CredentialsError) -> TerminateAppError {
        TerminateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for TerminateAppError {
    fn from(err: HttpDispatchError) -> TerminateAppError {
        TerminateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for TerminateAppError {
    fn from(err: io::Error) -> TerminateAppError {
        TerminateAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for TerminateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for TerminateAppError {
    fn description(&self) -> &str {
        match *self {
            TerminateAppError::InternalError(ref cause) => cause,
            TerminateAppError::InvalidParameter(ref cause) => cause,
            TerminateAppError::MissingRequiredParameter(ref cause) => cause,
            TerminateAppError::OperationNotPermitted(ref cause) => cause,
            TerminateAppError::UnauthorizedOperation(ref cause) => cause,
            TerminateAppError::Validation(ref cause) => cause,
            TerminateAppError::Credentials(ref err) => err.description(),
            TerminateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            TerminateAppError::ParseError(ref cause) => cause,
            TerminateAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateApp
#[derive(Debug, PartialEq)]
pub enum UpdateAppError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl UpdateAppError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateAppError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return UpdateAppError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return UpdateAppError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return UpdateAppError::MissingRequiredParameter(String::from(error_message));
                }
                "OperationNotPermittedException" => {
                    return UpdateAppError::OperationNotPermitted(String::from(error_message));
                }
                "UnauthorizedOperationException" => {
                    return UpdateAppError::UnauthorizedOperation(String::from(error_message));
                }
                "ValidationException" => {
                    return UpdateAppError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateAppError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateAppError {
    fn from(err: serde_json::error::Error) -> UpdateAppError {
        UpdateAppError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateAppError {
    fn from(err: CredentialsError) -> UpdateAppError {
        UpdateAppError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateAppError {
    fn from(err: HttpDispatchError) -> UpdateAppError {
        UpdateAppError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateAppError {
    fn from(err: io::Error) -> UpdateAppError {
        UpdateAppError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateAppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAppError {
    fn description(&self) -> &str {
        match *self {
            UpdateAppError::InternalError(ref cause) => cause,
            UpdateAppError::InvalidParameter(ref cause) => cause,
            UpdateAppError::MissingRequiredParameter(ref cause) => cause,
            UpdateAppError::OperationNotPermitted(ref cause) => cause,
            UpdateAppError::UnauthorizedOperation(ref cause) => cause,
            UpdateAppError::Validation(ref cause) => cause,
            UpdateAppError::Credentials(ref err) => err.description(),
            UpdateAppError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateAppError::ParseError(ref cause) => cause,
            UpdateAppError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateReplicationJob
#[derive(Debug, PartialEq)]
pub enum UpdateReplicationJobError {
    /// <p>An internal error occurred.</p>
    InternalError(String),
    /// <p>A specified parameter is not valid.</p>
    InvalidParameter(String),
    /// <p>A required parameter is missing.</p>
    MissingRequiredParameter(String),
    /// <p>This operation is not allowed.</p>
    OperationNotPermitted(String),
    /// <p>The specified replication job does not exist.</p>
    ReplicationJobNotFound(String),
    /// <p>The specified server cannot be replicated.</p>
    ServerCannotBeReplicated(String),
    /// <p>The service is temporarily unavailable.</p>
    TemporarilyUnavailable(String),
    /// <p>You lack permissions needed to perform this operation. Check your IAM policies, and ensure that you are using the correct access keys.</p>
    UnauthorizedOperation(String),
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

impl UpdateReplicationJobError {
    pub fn from_response(res: BufferedHttpResponse) -> UpdateReplicationJobError {
        if let Ok(json) = from_slice::<SerdeJsonValue>(&res.body) {
            let raw_error_type = json
                .get("__type")
                .and_then(|e| e.as_str())
                .unwrap_or("Unknown");
            let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or("");

            let pieces: Vec<&str> = raw_error_type.split("#").collect();
            let error_type = pieces.last().expect("Expected error type");

            match *error_type {
                "InternalError" => {
                    return UpdateReplicationJobError::InternalError(String::from(error_message));
                }
                "InvalidParameterException" => {
                    return UpdateReplicationJobError::InvalidParameter(String::from(error_message));
                }
                "MissingRequiredParameterException" => {
                    return UpdateReplicationJobError::MissingRequiredParameter(String::from(
                        error_message,
                    ));
                }
                "OperationNotPermittedException" => {
                    return UpdateReplicationJobError::OperationNotPermitted(String::from(
                        error_message,
                    ));
                }
                "ReplicationJobNotFoundException" => {
                    return UpdateReplicationJobError::ReplicationJobNotFound(String::from(
                        error_message,
                    ));
                }
                "ServerCannotBeReplicatedException" => {
                    return UpdateReplicationJobError::ServerCannotBeReplicated(String::from(
                        error_message,
                    ));
                }
                "TemporarilyUnavailableException" => {
                    return UpdateReplicationJobError::TemporarilyUnavailable(String::from(
                        error_message,
                    ));
                }
                "UnauthorizedOperationException" => {
                    return UpdateReplicationJobError::UnauthorizedOperation(String::from(
                        error_message,
                    ));
                }
                "ValidationException" => {
                    return UpdateReplicationJobError::Validation(error_message.to_string());
                }
                _ => {}
            }
        }
        return UpdateReplicationJobError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateReplicationJobError {
    fn from(err: serde_json::error::Error) -> UpdateReplicationJobError {
        UpdateReplicationJobError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateReplicationJobError {
    fn from(err: CredentialsError) -> UpdateReplicationJobError {
        UpdateReplicationJobError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateReplicationJobError {
    fn from(err: HttpDispatchError) -> UpdateReplicationJobError {
        UpdateReplicationJobError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateReplicationJobError {
    fn from(err: io::Error) -> UpdateReplicationJobError {
        UpdateReplicationJobError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateReplicationJobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateReplicationJobError {
    fn description(&self) -> &str {
        match *self {
            UpdateReplicationJobError::InternalError(ref cause) => cause,
            UpdateReplicationJobError::InvalidParameter(ref cause) => cause,
            UpdateReplicationJobError::MissingRequiredParameter(ref cause) => cause,
            UpdateReplicationJobError::OperationNotPermitted(ref cause) => cause,
            UpdateReplicationJobError::ReplicationJobNotFound(ref cause) => cause,
            UpdateReplicationJobError::ServerCannotBeReplicated(ref cause) => cause,
            UpdateReplicationJobError::TemporarilyUnavailable(ref cause) => cause,
            UpdateReplicationJobError::UnauthorizedOperation(ref cause) => cause,
            UpdateReplicationJobError::Validation(ref cause) => cause,
            UpdateReplicationJobError::Credentials(ref err) => err.description(),
            UpdateReplicationJobError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UpdateReplicationJobError::ParseError(ref cause) => cause,
            UpdateReplicationJobError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the SMS API. SMS clients implement this trait.
pub trait ServerMigrationService {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError>;

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError>;

    /// <p>Deletes an existing application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError>;

    /// <p>Deletes existing launch configuration for an application.</p>
    fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> RusotoFuture<DeleteAppLaunchConfigurationResponse, DeleteAppLaunchConfigurationError>;

    /// <p>Deletes existing replication configuration for an application.</p>
    fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> RusotoFuture<
        DeleteAppReplicationConfigurationResponse,
        DeleteAppReplicationConfigurationError,
    >;

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError>;

    /// <p>Deletes all servers from your server catalog.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError>;

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError>;

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> RusotoFuture<GenerateChangeSetResponse, GenerateChangeSetError>;

    /// <p>Generates an Amazon CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> RusotoFuture<GenerateTemplateResponse, GenerateTemplateError>;

    /// <p>Retrieve information about an application.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError>;

    /// <p>Retrieves the application launch configuration associated with an application.</p>
    fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> RusotoFuture<GetAppLaunchConfigurationResponse, GetAppLaunchConfigurationError>;

    /// <p>Retrieves an application replication configuration associatd with an application.</p>
    fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> RusotoFuture<GetAppReplicationConfigurationResponse, GetAppReplicationConfigurationError>;

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError>;

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError>;

    /// <p>Describes the replication runs for the specified replication job.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError>;

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError>;

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers that you want to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError>;

    /// <p>Launches an application stack.</p>
    fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> RusotoFuture<LaunchAppResponse, LaunchAppError>;

    /// <p>Returns a list of summaries for all applications.</p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResponse, ListAppsError>;

    /// <p>Creates a launch configuration for an application.</p>
    fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> RusotoFuture<PutAppLaunchConfigurationResponse, PutAppLaunchConfigurationError>;

    /// <p>Creates or updates a replication configuration for an application.</p>
    fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> RusotoFuture<PutAppReplicationConfigurationResponse, PutAppReplicationConfigurationError>;

    /// <p>Starts replicating an application.</p>
    fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> RusotoFuture<StartAppReplicationResponse, StartAppReplicationError>;

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs you can request in a 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError>;

    /// <p>Stops replicating an application.</p>
    fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> RusotoFuture<StopAppReplicationResponse, StopAppReplicationError>;

    /// <p>Terminates the stack for an application.</p>
    fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> RusotoFuture<TerminateAppResponse, TerminateAppError>;

    /// <p>Updates an application.</p>
    fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> RusotoFuture<UpdateAppResponse, UpdateAppError>;

    /// <p>Updates the specified settings for the specified replication job.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError>;
}
/// A client for the SMS API.
#[derive(Clone)]
pub struct ServerMigrationServiceClient {
    client: Client,
    region: region::Region,
}

impl ServerMigrationServiceClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> ServerMigrationServiceClient {
        ServerMigrationServiceClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> ServerMigrationServiceClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        ServerMigrationServiceClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl ServerMigrationService for ServerMigrationServiceClient {
    /// <p>Creates an application. An application consists of one or more server groups. Each server group contain one or more servers.</p>
    fn create_app(
        &self,
        input: CreateAppRequest,
    ) -> RusotoFuture<CreateAppResponse, CreateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(CreateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a replication job. The replication job schedules periodic replication runs to replicate your server to AWS. Each replication run creates an Amazon Machine Image (AMI).</p>
    fn create_replication_job(
        &self,
        input: CreateReplicationJobRequest,
    ) -> RusotoFuture<CreateReplicationJobResponse, CreateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.CreateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<CreateReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes an existing application. Optionally deletes the launched stack associated with the application and all AWS SMS replication jobs for servers in the application.</p>
    fn delete_app(
        &self,
        input: DeleteAppRequest,
    ) -> RusotoFuture<DeleteAppResponse, DeleteAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DeleteAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes existing launch configuration for an application.</p>
    fn delete_app_launch_configuration(
        &self,
        input: DeleteAppLaunchConfigurationRequest,
    ) -> RusotoFuture<DeleteAppLaunchConfigurationResponse, DeleteAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAppLaunchConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Deletes existing replication configuration for an application.</p>
    fn delete_app_replication_configuration(
        &self,
        input: DeleteAppReplicationConfigurationRequest,
    ) -> RusotoFuture<
        DeleteAppReplicationConfigurationResponse,
        DeleteAppReplicationConfigurationError,
    > {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteAppReplicationConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAppReplicationConfigurationError::from_response(
                        response,
                    ))
                }))
            }
        })
    }

    /// <p>Deletes the specified replication job.</p> <p>After you delete a replication job, there are no further replication runs. AWS deletes the contents of the Amazon S3 bucket used to store AWS SMS artifacts. The AMIs created by the replication runs are not deleted.</p>
    fn delete_replication_job(
        &self,
        input: DeleteReplicationJobRequest,
    ) -> RusotoFuture<DeleteReplicationJobResponse, DeleteReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes all servers from your server catalog.</p>
    fn delete_server_catalog(
        &self,
    ) -> RusotoFuture<DeleteServerCatalogResponse, DeleteServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DeleteServerCatalog",
        );
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DeleteServerCatalogResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Disassociates the specified connector from AWS SMS.</p> <p>After you disassociate a connector, it is no longer available to support replication jobs.</p>
    fn disassociate_connector(
        &self,
        input: DisassociateConnectorRequest,
    ) -> RusotoFuture<DisassociateConnectorResponse, DisassociateConnectorError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.DisassociateConnector",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<DisassociateConnectorResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DisassociateConnectorError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Generates a target change set for a currently launched stack and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_change_set(
        &self,
        input: GenerateChangeSetRequest,
    ) -> RusotoFuture<GenerateChangeSetResponse, GenerateChangeSetError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateChangeSet",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GenerateChangeSetResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateChangeSetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Generates an Amazon CloudFormation template based on the current launch configuration and writes it to an Amazon S3 object in the customer’s Amazon S3 bucket.</p>
    fn generate_template(
        &self,
        input: GenerateTemplateRequest,
    ) -> RusotoFuture<GenerateTemplateResponse, GenerateTemplateError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GenerateTemplate",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GenerateTemplateResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GenerateTemplateError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieve information about an application.</p>
    fn get_app(&self, input: GetAppRequest) -> RusotoFuture<GetAppResponse, GetAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Retrieves the application launch configuration associated with an application.</p>
    fn get_app_launch_configuration(
        &self,
        input: GetAppLaunchConfigurationRequest,
    ) -> RusotoFuture<GetAppLaunchConfigurationResponse, GetAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAppLaunchConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Retrieves an application replication configuration associatd with an application.</p>
    fn get_app_replication_configuration(
        &self,
        input: GetAppReplicationConfigurationRequest,
    ) -> RusotoFuture<GetAppReplicationConfigurationResponse, GetAppReplicationConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetAppReplicationConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetAppReplicationConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the connectors registered with the AWS SMS.</p>
    fn get_connectors(
        &self,
        input: GetConnectorsRequest,
    ) -> RusotoFuture<GetConnectorsResponse, GetConnectorsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetConnectors",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetConnectorsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetConnectorsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the specified replication job or all of your replication jobs.</p>
    fn get_replication_jobs(
        &self,
        input: GetReplicationJobsRequest,
    ) -> RusotoFuture<GetReplicationJobsResponse, GetReplicationJobsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationJobs",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetReplicationJobsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationJobsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the replication runs for the specified replication job.</p>
    fn get_replication_runs(
        &self,
        input: GetReplicationRunsRequest,
    ) -> RusotoFuture<GetReplicationRunsResponse, GetReplicationRunsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetReplicationRuns",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetReplicationRunsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetReplicationRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the servers in your server catalog.</p> <p>Before you can describe your servers, you must import them using <a>ImportServerCatalog</a>.</p>
    fn get_servers(
        &self,
        input: GetServersRequest,
    ) -> RusotoFuture<GetServersResponse, GetServersError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.GetServers",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<GetServersResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetServersError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gathers a complete list of on-premises servers. Connectors must be installed and monitoring all servers that you want to import.</p> <p>This call returns immediately, but might take additional time to retrieve all the servers.</p>
    fn import_server_catalog(
        &self,
    ) -> RusotoFuture<ImportServerCatalogResponse, ImportServerCatalogError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ImportServerCatalog",
        );
        request.set_payload(Some(b"{}".to_vec()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ImportServerCatalogResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ImportServerCatalogError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Launches an application stack.</p>
    fn launch_app(
        &self,
        input: LaunchAppRequest,
    ) -> RusotoFuture<LaunchAppResponse, LaunchAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.LaunchApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<LaunchAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(LaunchAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Returns a list of summaries for all applications.</p>
    fn list_apps(&self, input: ListAppsRequest) -> RusotoFuture<ListAppsResponse, ListAppsError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.ListApps",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<ListAppsResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAppsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Creates a launch configuration for an application.</p>
    fn put_app_launch_configuration(
        &self,
        input: PutAppLaunchConfigurationRequest,
    ) -> RusotoFuture<PutAppLaunchConfigurationResponse, PutAppLaunchConfigurationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppLaunchConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutAppLaunchConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAppLaunchConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates or updates a replication configuration for an application.</p>
    fn put_app_replication_configuration(
        &self,
        input: PutAppReplicationConfigurationRequest,
    ) -> RusotoFuture<PutAppReplicationConfigurationResponse, PutAppReplicationConfigurationError>
    {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.PutAppReplicationConfiguration",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<PutAppReplicationConfigurationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(PutAppReplicationConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts replicating an application.</p>
    fn start_app_replication(
        &self,
        input: StartAppReplicationRequest,
    ) -> RusotoFuture<StartAppReplicationResponse, StartAppReplicationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartAppReplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(StartAppReplicationError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Starts an on-demand replication run for the specified replication job. This replication run starts immediately. This replication run is in addition to the ones already scheduled.</p> <p>There is a limit on the number of on-demand replications runs you can request in a 24-hour period.</p>
    fn start_on_demand_replication_run(
        &self,
        input: StartOnDemandReplicationRunRequest,
    ) -> RusotoFuture<StartOnDemandReplicationRunResponse, StartOnDemandReplicationRunError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StartOnDemandReplicationRun",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StartOnDemandReplicationRunResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(StartOnDemandReplicationRunError::from_response(response))
                }))
            }
        })
    }

    /// <p>Stops replicating an application.</p>
    fn stop_app_replication(
        &self,
        input: StopAppReplicationRequest,
    ) -> RusotoFuture<StopAppReplicationResponse, StopAppReplicationError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.StopAppReplication",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<StopAppReplicationResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopAppReplicationError::from_response(response))),
                )
            }
        })
    }

    /// <p>Terminates the stack for an application.</p>
    fn terminate_app(
        &self,
        input: TerminateAppRequest,
    ) -> RusotoFuture<TerminateAppResponse, TerminateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.TerminateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<TerminateAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(TerminateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates an application.</p>
    fn update_app(
        &self,
        input: UpdateAppRequest,
    ) -> RusotoFuture<UpdateAppResponse, UpdateAppError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateApp",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateAppResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateAppError::from_response(response))),
                )
            }
        })
    }

    /// <p>Updates the specified settings for the specified replication job.</p>
    fn update_replication_job(
        &self,
        input: UpdateReplicationJobRequest,
    ) -> RusotoFuture<UpdateReplicationJobResponse, UpdateReplicationJobError> {
        let mut request = SignedRequest::new("POST", "sms", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "AWSServerMigrationService_V2016_10_24.UpdateReplicationJob",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded.into_bytes()));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body.is_empty() || body == b"null" {
                        body = b"{}".to_vec();
                    }

                    serde_json::from_str::<UpdateReplicationJobResponse>(
                        String::from_utf8_lossy(body.as_ref()).as_ref(),
                    )
                    .unwrap()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateReplicationJobError::from_response(response))
                    }),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
