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
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError};

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CancelJobRunRequest {
    /// <p>The ID of the job run to cancel.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The ID of the virtual cluster for which the job run will be canceled.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CancelJobRunResponse {
    /// <p>The output contains the ID of the cancelled job run.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The output contains the virtual cluster ID for which the job run is cancelled.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

/// <p>A configuration for CloudWatch monitoring. You can configure your jobs to send log information to CloudWatch Logs.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct CloudWatchMonitoringConfiguration {
    /// <p>The name of the log group for log publishing.</p>
    #[serde(rename = "logGroupName")]
    pub log_group_name: String,
    /// <p>The specified name prefix for log streams.</p>
    #[serde(rename = "logStreamNamePrefix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name_prefix: Option<String>,
}

/// <p>A configuration specification to be used when provisioning virtual clusters, which can include configurations for applications and software bundled with Amazon EMR on EKS. A configuration consists of a classification, properties, and optional nested configurations. A classification refers to an application-specific configuration file. Properties are the settings you want to change in that file.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Configuration {
    /// <p>The classification within a configuration.</p>
    #[serde(rename = "classification")]
    pub classification: String,
    /// <p>A list of additional configurations to apply within a configuration object.</p>
    #[serde(rename = "configurations")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
    /// <p>A set of properties specified within a configuration classification.</p>
    #[serde(rename = "properties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<::std::collections::HashMap<String, String>>,
}

/// <p>A configuration specification to be used to override existing configurations.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ConfigurationOverrides {
    /// <p>The configurations for the application running by the job run. </p>
    #[serde(rename = "applicationConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_configuration: Option<Vec<Configuration>>,
    /// <p>The configurations for monitoring.</p>
    #[serde(rename = "monitoringConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_configuration: Option<MonitoringConfiguration>,
}

/// <p>The information about the container used for a job run or a managed endpoint.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerInfo {
    /// <p>The information about the EKS cluster.</p>
    #[serde(rename = "eksInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_info: Option<EksInfo>,
}

/// <p>The information about the container provider.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ContainerProvider {
    /// <p>The ID of the container cluster.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The information about the container cluster.</p>
    #[serde(rename = "info")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<ContainerInfo>,
    /// <p>The type of the container provider. EKS is the only supported type as of now.</p>
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateManagedEndpointRequest {
    /// <p>The certificate ARN of the managed endpoint.</p>
    #[serde(rename = "certificateArn")]
    pub certificate_arn: String,
    /// <p>The client idempotency token for this create call.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The configuration settings that will be used to override existing configurations.</p>
    #[serde(rename = "configurationOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    /// <p>The ARN of the execution role.</p>
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The name of the managed endpoint.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The Amazon EMR release version.</p>
    #[serde(rename = "releaseLabel")]
    pub release_label: String,
    /// <p>The tags of the managed endpoint. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the managed endpoint.</p>
    #[serde(rename = "type")]
    pub type_: String,
    /// <p>The ID of the virtual cluster for which a managed endpoint is created.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateManagedEndpointResponse {
    /// <p>The output contains the ARN of the managed endpoint.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The output contains the ID of the managed endpoint.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The output contains the name of the managed endpoint.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The output contains the ID of the virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateVirtualClusterRequest {
    /// <p>The client token of the virtual cluster.</p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The container provider of the virtual cluster.</p>
    #[serde(rename = "containerProvider")]
    pub container_provider: ContainerProvider,
    /// <p>The specified name of the virtual cluster.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The tags assigned to the virtual cluster.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateVirtualClusterResponse {
    /// <p>This output contains the ARN of virtual cluster.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>This output contains the virtual cluster ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>This output contains the name of the virtual cluster.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteManagedEndpointRequest {
    /// <p>The ID of the managed endpoint.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The ID of the endpoint's virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteManagedEndpointResponse {
    /// <p>The output displays the ID of the managed endpoint.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The output displays the ID of the endpoint's virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteVirtualClusterRequest {
    /// <p>The ID of the virtual cluster that will be deleted.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteVirtualClusterResponse {
    /// <p>This output contains the ID of the virtual cluster that will be deleted. </p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJobRunRequest {
    /// <p>The ID of the job run request. </p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The ID of the virtual cluster for which the job run is submitted.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJobRunResponse {
    /// <p>The output displays information about a job run.</p>
    #[serde(rename = "jobRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_run: Option<JobRun>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeManagedEndpointRequest {
    /// <p>This output displays ID of the managed endpoint.</p>
    #[serde(rename = "id")]
    pub id: String,
    /// <p>The ID of the endpoint's virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeManagedEndpointResponse {
    /// <p>This output displays information about a managed endpoint.</p>
    #[serde(rename = "endpoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeVirtualClusterRequest {
    /// <p>The ID of the virtual cluster that will be described.</p>
    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeVirtualClusterResponse {
    /// <p>This output displays information about the specified virtual cluster.</p>
    #[serde(rename = "virtualCluster")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster: Option<VirtualCluster>,
}

/// <p>The information about the EKS cluster.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct EksInfo {
    /// <p>The namespaces of the EKS cluster.</p>
    #[serde(rename = "namespace")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// <p>This entity represents the endpoint that is managed by Amazon EMR on EKS.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Endpoint {
    /// <p>The ARN of the endpoint.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The certificate ARN of the endpoint.</p>
    #[serde(rename = "certificateArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    /// <p>The configuration settings that are used to override existing configurations for endpoints.</p>
    #[serde(rename = "configurationOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    /// <p>The date and time when the endpoint was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The execution role ARN of the endpoint.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The ID of the endpoint.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the endpoint.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The EMR release version to be used for the endpoint.</p>
    #[serde(rename = "releaseLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    /// <p>The security group configuration of the endpoint. </p>
    #[serde(rename = "securityGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group: Option<String>,
    /// <p>The server URL of the endpoint.</p>
    #[serde(rename = "serverUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_url: Option<String>,
    /// <p>The state of the endpoint.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The subnet IDs of the endpoint. </p>
    #[serde(rename = "subnetIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    /// <p>The tags of the endpoint. </p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The type of the endpoint.</p>
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// <p>The ID of the endpoint's virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

/// <p>Specify the driver that the job runs on.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct JobDriver {
    /// <p>The job driver parameters specified for spark submit.</p>
    #[serde(rename = "sparkSubmitJobDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit_job_driver: Option<SparkSubmitJobDriver>,
}

/// <p>This entity describes a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS. </p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JobRun {
    /// <p>The ARN of job run.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The client token used to start a job run.</p>
    #[serde(rename = "clientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>The configuration settings that are used to override default configuration.</p>
    #[serde(rename = "configurationOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    /// <p>The date and time when the job run was created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The user who created the job run.</p>
    #[serde(rename = "createdBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// <p>The execution role ARN of the job run.</p>
    #[serde(rename = "executionRoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    /// <p>The reasons why the job run has failed.</p>
    #[serde(rename = "failureReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    /// <p>The date and time when the job run has finished.</p>
    #[serde(rename = "finishedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<f64>,
    /// <p>The ID of the job run.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>Parameters of job driver for the job run.</p>
    #[serde(rename = "jobDriver")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_driver: Option<JobDriver>,
    /// <p>The name of the job run.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The release version of Amazon EMR.</p>
    #[serde(rename = "releaseLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_label: Option<String>,
    /// <p>The state of the job run. </p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>Additional details of the job run state.</p>
    #[serde(rename = "stateDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_details: Option<String>,
    /// <p>The assigned tags of the job run.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The ID of the job run's virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJobRunsRequest {
    /// <p>The date and time after which the job runs were submitted.</p>
    #[serde(rename = "createdAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>The date and time before which the job runs were submitted.</p>
    #[serde(rename = "createdBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>The maximum number of job runs that can be listed.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the job run.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The token for the next set of job runs to return.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The states of the job run.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// <p>The ID of the virtual cluster for which to list the job run. </p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJobRunsResponse {
    /// <p>This output lists information about the specified job runs.</p>
    #[serde(rename = "jobRuns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_runs: Option<Vec<JobRun>>,
    /// <p>This output displays the token for the next set of job runs.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListManagedEndpointsRequest {
    /// <p> The date and time after which the endpoints are created.</p>
    #[serde(rename = "createdAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>The date and time before which the endpoints are created.</p>
    #[serde(rename = "createdBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>The maximum number of managed endpoints that can be listed.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p> The token for the next set of managed endpoints to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The states of the managed endpoints.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
    /// <p>The types of the managed endpoints.</p>
    #[serde(rename = "types")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
    /// <p>The ID of the virtual cluster.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListManagedEndpointsResponse {
    /// <p>The managed endpoints to be listed.</p>
    #[serde(rename = "endpoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    /// <p> The token for the next set of endpoints to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN of tagged resources.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags assigned to resources.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListVirtualClustersRequest {
    /// <p>The container provider ID of the virtual cluster.</p>
    #[serde(rename = "containerProviderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider_id: Option<String>,
    /// <p>The container provider type of the virtual cluster. EKS is the only supported type as of now.</p>
    #[serde(rename = "containerProviderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider_type: Option<String>,
    /// <p>The date and time after which the virtual clusters are created.</p>
    #[serde(rename = "createdAfter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<f64>,
    /// <p>The date and time before which the virtual clusters are created.</p>
    #[serde(rename = "createdBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<f64>,
    /// <p>The maximum number of virtual clusters that can be listed.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token for the next set of virtual clusters to return. </p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The states of the requested virtual clusters.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListVirtualClustersResponse {
    /// <p>This output displays the token for the next set of virtual clusters.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>This output lists the specified virtual clusters.</p>
    #[serde(rename = "virtualClusters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_clusters: Option<Vec<VirtualCluster>>,
}

/// <p>Configuration setting for monitoring.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct MonitoringConfiguration {
    /// <p>Monitoring configurations for CloudWatch.</p>
    #[serde(rename = "cloudWatchMonitoringConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_monitoring_configuration: Option<CloudWatchMonitoringConfiguration>,
    /// <p>Monitoring configurations for the persistent application UI. </p>
    #[serde(rename = "persistentAppUI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_app_ui: Option<String>,
    /// <p>Amazon S3 configuration for monitoring log publishing.</p>
    #[serde(rename = "s3MonitoringConfiguration")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_3_monitoring_configuration: Option<S3MonitoringConfiguration>,
}

/// <p> Amazon S3 configuration for monitoring log publishing. You can configure your jobs to send log information to Amazon S3.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct S3MonitoringConfiguration {
    /// <p>Amazon S3 destination URI for log publishing.</p>
    #[serde(rename = "logUri")]
    pub log_uri: String,
}

/// <p>The information about job driver for Spark submit.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SparkSubmitJobDriver {
    /// <p>The entry point of job application.</p>
    #[serde(rename = "entryPoint")]
    pub entry_point: String,
    /// <p>The arguments for job application.</p>
    #[serde(rename = "entryPointArguments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point_arguments: Option<Vec<String>>,
    /// <p>The Spark submit parameters that are used for job runs.</p>
    #[serde(rename = "sparkSubmitParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spark_submit_parameters: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartJobRunRequest {
    /// <p>The client idempotency token of the job run request. </p>
    #[serde(rename = "clientToken")]
    pub client_token: String,
    /// <p>The configuration overrides for the job run.</p>
    #[serde(rename = "configurationOverrides")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<ConfigurationOverrides>,
    /// <p>The execution role ARN for the job run.</p>
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: String,
    /// <p>The job driver for the job run.</p>
    #[serde(rename = "jobDriver")]
    pub job_driver: JobDriver,
    /// <p>The name of the job run.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The Amazon EMR release version to use for the job run.</p>
    #[serde(rename = "releaseLabel")]
    pub release_label: String,
    /// <p>The tags assigned to job runs.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
    /// <p>The virtual cluster ID for which the job run request is submitted.</p>
    #[serde(rename = "virtualClusterId")]
    pub virtual_cluster_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartJobRunResponse {
    /// <p>This output lists the ARN of job run.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>This output displays the started job run ID.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>This output displays the name of the started job run.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>This output displays the virtual cluster ID for which the job run was submitted.</p>
    #[serde(rename = "virtualClusterId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_cluster_id: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The ARN of resources.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tags assigned to resources.</p>
    #[serde(rename = "tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The ARN of resources.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The tag keys of the resources.</p>
    #[serde(rename = "tagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

/// <p>This entity describes a virtual cluster. A virtual cluster is a Kubernetes namespace that Amazon EMR is registered with. Amazon EMR uses virtual clusters to run jobs and host endpoints. Multiple virtual clusters can be backed by the same physical cluster. However, each virtual cluster maps to one namespace on an EKS cluster. Virtual clusters do not create any active resources that contribute to your bill or that require lifecycle management outside the service.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct VirtualCluster {
    /// <p>The ARN of the virtual cluster.</p>
    #[serde(rename = "arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The container provider of the virtual cluster.</p>
    #[serde(rename = "containerProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_provider: Option<ContainerProvider>,
    /// <p>The date and time when the virtual cluster is created.</p>
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    /// <p>The ID of the virtual cluster.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>The name of the virtual cluster.</p>
    #[serde(rename = "name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The state of the virtual cluster.</p>
    #[serde(rename = "state")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// <p>The assigned tags of the virtual cluster.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// Errors returned by CancelJobRun
#[derive(Debug, PartialEq)]
pub enum CancelJobRunError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl CancelJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CancelJobRunError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CancelJobRunError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for CancelJobRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CancelJobRunError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CancelJobRunError {}
/// Errors returned by CreateManagedEndpoint
#[derive(Debug, PartialEq)]
pub enum CreateManagedEndpointError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateManagedEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateManagedEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateManagedEndpointError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateManagedEndpointError::ResourceNotFound(
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
impl fmt::Display for CreateManagedEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateManagedEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateManagedEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateManagedEndpointError {}
/// Errors returned by CreateVirtualCluster
#[derive(Debug, PartialEq)]
pub enum CreateVirtualClusterError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl CreateVirtualClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateVirtualClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(CreateVirtualClusterError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CreateVirtualClusterError::ResourceNotFound(
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
impl fmt::Display for CreateVirtualClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateVirtualClusterError::InternalServer(ref cause) => write!(f, "{}", cause),
            CreateVirtualClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateVirtualClusterError {}
/// Errors returned by DeleteManagedEndpoint
#[derive(Debug, PartialEq)]
pub enum DeleteManagedEndpointError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl DeleteManagedEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteManagedEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteManagedEndpointError::InternalServer(
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
impl fmt::Display for DeleteManagedEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteManagedEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteManagedEndpointError {}
/// Errors returned by DeleteVirtualCluster
#[derive(Debug, PartialEq)]
pub enum DeleteVirtualClusterError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl DeleteVirtualClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteVirtualClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DeleteVirtualClusterError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DeleteVirtualClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteVirtualClusterError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteVirtualClusterError {}
/// Errors returned by DescribeJobRun
#[derive(Debug, PartialEq)]
pub enum DescribeJobRunError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJobRunError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeJobRunError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeJobRunError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for DescribeJobRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJobRunError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeJobRunError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJobRunError {}
/// Errors returned by DescribeManagedEndpoint
#[derive(Debug, PartialEq)]
pub enum DescribeManagedEndpointError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeManagedEndpointError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeManagedEndpointError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeManagedEndpointError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeManagedEndpointError::ResourceNotFound(
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
impl fmt::Display for DescribeManagedEndpointError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeManagedEndpointError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeManagedEndpointError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeManagedEndpointError {}
/// Errors returned by DescribeVirtualCluster
#[derive(Debug, PartialEq)]
pub enum DescribeVirtualClusterError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl DescribeVirtualClusterError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeVirtualClusterError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(DescribeVirtualClusterError::InternalServer(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeVirtualClusterError::ResourceNotFound(
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
impl fmt::Display for DescribeVirtualClusterError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeVirtualClusterError::InternalServer(ref cause) => write!(f, "{}", cause),
            DescribeVirtualClusterError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeVirtualClusterError {}
/// Errors returned by ListJobRuns
#[derive(Debug, PartialEq)]
pub enum ListJobRunsError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl ListJobRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJobRunsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListJobRunsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListJobRunsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListJobRunsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListJobRunsError {}
/// Errors returned by ListManagedEndpoints
#[derive(Debug, PartialEq)]
pub enum ListManagedEndpointsError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl ListManagedEndpointsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListManagedEndpointsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListManagedEndpointsError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListManagedEndpointsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListManagedEndpointsError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListManagedEndpointsError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListTagsForResourceError::InternalServer(err.msg))
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
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by ListVirtualClusters
#[derive(Debug, PartialEq)]
pub enum ListVirtualClustersError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
}

impl ListVirtualClustersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListVirtualClustersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(ListVirtualClustersError::InternalServer(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListVirtualClustersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListVirtualClustersError::InternalServer(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListVirtualClustersError {}
/// Errors returned by StartJobRun
#[derive(Debug, PartialEq)]
pub enum StartJobRunError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl StartJobRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartJobRunError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(StartJobRunError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartJobRunError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartJobRunError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartJobRunError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartJobRunError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartJobRunError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(TagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
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
            TagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>This is an internal server exception.</p>
    InternalServer(String),
    /// <p>The specified resource was not found.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            #[allow(clippy::single_match)]
            match err.typ.as_str() {
                "InternalServerException" => {
                    return RusotoError::Service(UntagResourceError::InternalServer(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
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
            UntagResourceError::InternalServer(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Trait representing the capabilities of the Amazon EMR Containers API. Amazon EMR Containers clients implement this trait.
#[async_trait]
pub trait EmrContainers {
    /// <p>Cancels a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    async fn cancel_job_run(
        &self,
        input: CancelJobRunRequest,
    ) -> Result<CancelJobRunResponse, RusotoError<CancelJobRunError>>;

    /// <p>Creates a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    async fn create_managed_endpoint(
        &self,
        input: CreateManagedEndpointRequest,
    ) -> Result<CreateManagedEndpointResponse, RusotoError<CreateManagedEndpointError>>;

    /// <p>Creates a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    async fn create_virtual_cluster(
        &self,
        input: CreateVirtualClusterRequest,
    ) -> Result<CreateVirtualClusterResponse, RusotoError<CreateVirtualClusterError>>;

    /// <p>Deletes a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    async fn delete_managed_endpoint(
        &self,
        input: DeleteManagedEndpointRequest,
    ) -> Result<DeleteManagedEndpointResponse, RusotoError<DeleteManagedEndpointError>>;

    /// <p>Deletes a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    async fn delete_virtual_cluster(
        &self,
        input: DeleteVirtualClusterRequest,
    ) -> Result<DeleteVirtualClusterResponse, RusotoError<DeleteVirtualClusterError>>;

    /// <p>Displays detailed information about a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    async fn describe_job_run(
        &self,
        input: DescribeJobRunRequest,
    ) -> Result<DescribeJobRunResponse, RusotoError<DescribeJobRunError>>;

    /// <p>Displays detailed information about a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    async fn describe_managed_endpoint(
        &self,
        input: DescribeManagedEndpointRequest,
    ) -> Result<DescribeManagedEndpointResponse, RusotoError<DescribeManagedEndpointError>>;

    /// <p>Displays detailed information about a specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    async fn describe_virtual_cluster(
        &self,
        input: DescribeVirtualClusterRequest,
    ) -> Result<DescribeVirtualClusterResponse, RusotoError<DescribeVirtualClusterError>>;

    /// <p>Lists job runs based on a set of parameters. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    async fn list_job_runs(
        &self,
        input: ListJobRunsRequest,
    ) -> Result<ListJobRunsResponse, RusotoError<ListJobRunsError>>;

    /// <p>Lists managed endpoints based on a set of parameters. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    async fn list_managed_endpoints(
        &self,
        input: ListManagedEndpointsRequest,
    ) -> Result<ListManagedEndpointsResponse, RusotoError<ListManagedEndpointsError>>;

    /// <p>Lists the tags assigned to the resources.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Lists information about the specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    async fn list_virtual_clusters(
        &self,
        input: ListVirtualClustersRequest,
    ) -> Result<ListVirtualClustersResponse, RusotoError<ListVirtualClustersError>>;

    /// <p>Starts a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    async fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> Result<StartJobRunResponse, RusotoError<StartJobRunError>>;

    /// <p>Assigns tags to resources. A tag is a label that you assign to an AWS resource. Each tag consists of a key and an optional value, both of which you define. Tags enable you to categorize your AWS resources by attributes such as purpose, owner, or environment. When you have many resources of the same type, you can quickly identify a specific resource based on the tags you've assigned to it. For example, you can define a set of tags for your Amazon EMR on EKS clusters to help you track each cluster's owner and stack level. We recommend that you devise a consistent set of tag keys for each resource type. You can then search and filter the resources based on the tags that you add.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes tags from resources.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;
}
/// A client for the Amazon EMR Containers API.
#[derive(Clone)]
pub struct EmrContainersClient {
    client: Client,
    region: region::Region,
}

impl EmrContainersClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EmrContainersClient {
        EmrContainersClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EmrContainersClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EmrContainersClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EmrContainersClient {
        EmrContainersClient { client, region }
    }
}

#[async_trait]
impl EmrContainers for EmrContainersClient {
    /// <p>Cancels a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    #[allow(unused_mut)]
    async fn cancel_job_run(
        &self,
        input: CancelJobRunRequest,
    ) -> Result<CancelJobRunResponse, RusotoError<CancelJobRunError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/jobruns/{job_run_id}",
            job_run_id = input.id,
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request =
            SignedRequest::new("DELETE", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CancelJobRunResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CancelJobRunError::from_response(response))
        }
    }

    /// <p>Creates a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    #[allow(unused_mut)]
    async fn create_managed_endpoint(
        &self,
        input: CreateManagedEndpointRequest,
    ) -> Result<CreateManagedEndpointResponse, RusotoError<CreateManagedEndpointError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/endpoints",
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("POST", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateManagedEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateManagedEndpointError::from_response(response))
        }
    }

    /// <p>Creates a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    #[allow(unused_mut)]
    async fn create_virtual_cluster(
        &self,
        input: CreateVirtualClusterRequest,
    ) -> Result<CreateVirtualClusterResponse, RusotoError<CreateVirtualClusterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/virtualclusters";

        let mut request = SignedRequest::new("POST", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateVirtualClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateVirtualClusterError::from_response(response))
        }
    }

    /// <p>Deletes a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    #[allow(unused_mut)]
    async fn delete_managed_endpoint(
        &self,
        input: DeleteManagedEndpointRequest,
    ) -> Result<DeleteManagedEndpointResponse, RusotoError<DeleteManagedEndpointError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/endpoints/{endpoint_id}",
            endpoint_id = input.id,
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request =
            SignedRequest::new("DELETE", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteManagedEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteManagedEndpointError::from_response(response))
        }
    }

    /// <p>Deletes a virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    #[allow(unused_mut)]
    async fn delete_virtual_cluster(
        &self,
        input: DeleteVirtualClusterRequest,
    ) -> Result<DeleteVirtualClusterResponse, RusotoError<DeleteVirtualClusterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}",
            virtual_cluster_id = input.id
        );

        let mut request =
            SignedRequest::new("DELETE", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteVirtualClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteVirtualClusterError::from_response(response))
        }
    }

    /// <p>Displays detailed information about a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    #[allow(unused_mut)]
    async fn describe_job_run(
        &self,
        input: DescribeJobRunRequest,
    ) -> Result<DescribeJobRunResponse, RusotoError<DescribeJobRunError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/jobruns/{job_run_id}",
            job_run_id = input.id,
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJobRunResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJobRunError::from_response(response))
        }
    }

    /// <p>Displays detailed information about a managed endpoint. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    #[allow(unused_mut)]
    async fn describe_managed_endpoint(
        &self,
        input: DescribeManagedEndpointRequest,
    ) -> Result<DescribeManagedEndpointResponse, RusotoError<DescribeManagedEndpointError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/endpoints/{endpoint_id}",
            endpoint_id = input.id,
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeManagedEndpointResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeManagedEndpointError::from_response(response))
        }
    }

    /// <p>Displays detailed information about a specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    #[allow(unused_mut)]
    async fn describe_virtual_cluster(
        &self,
        input: DescribeVirtualClusterRequest,
    ) -> Result<DescribeVirtualClusterResponse, RusotoError<DescribeVirtualClusterError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}",
            virtual_cluster_id = input.id
        );

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeVirtualClusterResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeVirtualClusterError::from_response(response))
        }
    }

    /// <p>Lists job runs based on a set of parameters. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    #[allow(unused_mut)]
    async fn list_job_runs(
        &self,
        input: ListJobRunsRequest,
    ) -> Result<ListJobRunsResponse, RusotoError<ListJobRunsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/jobruns",
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.name {
            params.put("name", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("states", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListJobRunsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJobRunsError::from_response(response))
        }
    }

    /// <p>Lists managed endpoints based on a set of parameters. A managed endpoint is a gateway that connects EMR Studio to Amazon EMR on EKS so that EMR Studio can communicate with your virtual cluster.</p>
    #[allow(unused_mut)]
    async fn list_managed_endpoints(
        &self,
        input: ListManagedEndpointsRequest,
    ) -> Result<ListManagedEndpointsResponse, RusotoError<ListManagedEndpointsError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/endpoints",
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("states", item);
            }
        }
        if let Some(ref x) = input.types {
            for item in x.iter() {
                params.put("types", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListManagedEndpointsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListManagedEndpointsError::from_response(response))
        }
    }

    /// <p>Lists the tags assigned to the resources.</p>
    #[allow(unused_mut)]
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Lists information about the specified virtual cluster. Virtual cluster is a managed entity on Amazon EMR on EKS. You can create, describe, list and delete virtual clusters. They do not consume any additional resource in your system. A single virtual cluster maps to a single Kubernetes namespace. Given this relationship, you can model virtual clusters the same way you model Kubernetes namespaces to meet your requirements.</p>
    #[allow(unused_mut)]
    async fn list_virtual_clusters(
        &self,
        input: ListVirtualClustersRequest,
    ) -> Result<ListVirtualClustersResponse, RusotoError<ListVirtualClustersError>> {
        #![allow(clippy::needless_update)]
        let request_uri = "/virtualclusters";

        let mut request = SignedRequest::new("GET", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.container_provider_id {
            params.put("containerProviderId", x);
        }
        if let Some(ref x) = input.container_provider_type {
            params.put("containerProviderType", x);
        }
        if let Some(ref x) = input.created_after {
            params.put("createdAfter", x);
        }
        if let Some(ref x) = input.created_before {
            params.put("createdBefore", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.states {
            for item in x.iter() {
                params.put("states", item);
            }
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListVirtualClustersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListVirtualClustersError::from_response(response))
        }
    }

    /// <p>Starts a job run. A job run is a unit of work, such as a Spark jar, PySpark script, or SparkSQL query, that you submit to Amazon EMR on EKS.</p>
    #[allow(unused_mut)]
    async fn start_job_run(
        &self,
        input: StartJobRunRequest,
    ) -> Result<StartJobRunResponse, RusotoError<StartJobRunError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!(
            "/virtualclusters/{virtual_cluster_id}/jobruns",
            virtual_cluster_id = input.virtual_cluster_id
        );

        let mut request = SignedRequest::new("POST", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartJobRunResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartJobRunError::from_response(response))
        }
    }

    /// <p>Assigns tags to resources. A tag is a label that you assign to an AWS resource. Each tag consists of a key and an optional value, both of which you define. Tags enable you to categorize your AWS resources by attributes such as purpose, owner, or environment. When you have many resources of the same type, you can quickly identify a specific resource based on the tags you've assigned to it. For example, you can define a set of tags for your Amazon EMR on EKS clusters to help you track each cluster's owner and stack level. We recommend that you devise a consistent set of tag keys for each resource type. You can then search and filter the resources based on the tags that you add.</p>
    #[allow(unused_mut)]
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "emr-containers", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes tags from resources.</p>
    #[allow(unused_mut)]
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        #![allow(clippy::needless_update)]
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request =
            SignedRequest::new("DELETE", "emr-containers", &self.region, &request_uri);
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
        if response.status.is_success() {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }
}
