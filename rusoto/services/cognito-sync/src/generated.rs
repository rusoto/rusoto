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
/// <p>The input for the BulkPublish operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct BulkPublishRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the BulkPublish operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct BulkPublishResponse {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
}

/// <p>Configuration options for configure Cognito streams.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CognitoStreams {
    /// <p>The ARN of the role Amazon Cognito can assume in order to publish to the stream. This role must grant access to Amazon Cognito (cognito-sync) to invoke PutRecord on your Cognito stream.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    /// <p>The name of the Cognito stream to receive updates. This stream must be in the developers account and in the same region as the identity pool.</p>
    #[serde(rename = "StreamName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_name: Option<String>,
    /// <p>Status of the Cognito streams. Valid values are: <p>ENABLED - Streaming of updates to identity pool is enabled.</p> <p>DISABLED - Streaming of updates to identity pool is disabled. Bulk publish will also fail if StreamingStatus is DISABLED.</p></p>
    #[serde(rename = "StreamingStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub streaming_status: Option<String>,
}

/// <p>A collection of data for an identity pool. An identity pool can have multiple datasets. A dataset is per identity and can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don&#39;t exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Dataset {
    /// <p>Date on which the dataset was created.</p>
    #[serde(rename = "CreationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    /// <p>Total size in bytes of the records in this dataset.</p>
    #[serde(rename = "DataStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    /// <p>A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (dash), and &#39;.&#39; (dot).</p>
    #[serde(rename = "DatasetName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<String>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>The device that made the last change to this dataset.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Date when the dataset was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Number of records in this dataset.</p>
    #[serde(rename = "NumRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_records: Option<i64>,
}

/// <p>A request to delete the specific dataset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteDatasetRequest {
    /// <p>A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (dash), and &#39;.&#39; (dot).</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to a successful DeleteDataset request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DeleteDatasetResponse {
    /// <p>A collection of data for an identity pool. An identity pool can have multiple datasets. A dataset is per identity and can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don&#39;t exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs.</p>
    #[serde(rename = "Dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

/// <p>A request for meta data about a dataset (creation date, number of records, size) by owner and dataset name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeDatasetRequest {
    /// <p>A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (dash), and &#39;.&#39; (dot).</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to a successful DescribeDataset request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeDatasetResponse {
    /// <p>Meta data for a collection of data for an identity. An identity can have multiple datasets. A dataset can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don&#39;t exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs.</p>
    #[serde(rename = "Dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

/// <p>A request for usage information about the identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityPoolUsageRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to a successful DescribeIdentityPoolUsage request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeIdentityPoolUsageResponse {
    /// <p>Information about the usage of the identity pool.</p>
    #[serde(rename = "IdentityPoolUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_usage: Option<IdentityPoolUsage>,
}

/// <p>A request for information about the usage of an identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeIdentityUsageRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The response to a successful DescribeIdentityUsage request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeIdentityUsageResponse {
    /// <p>Usage information for the identity.</p>
    #[serde(rename = "IdentityUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_usage: Option<IdentityUsage>,
}

/// <p>The input for the GetBulkPublishDetails operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetBulkPublishDetailsRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the GetBulkPublishDetails operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetBulkPublishDetailsResponse {
    /// <p>If BulkPublishStatus is SUCCEEDED, the time the last bulk publish operation completed.</p>
    #[serde(rename = "BulkPublishCompleteTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_complete_time: Option<f64>,
    /// <p>The date/time at which the last bulk publish was initiated.</p>
    #[serde(rename = "BulkPublishStartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_start_time: Option<f64>,
    /// <p>Status of the last bulk publish operation, valid values are: <p>NOT<em>STARTED - No bulk publish has been requested for this identity pool</p> <p>IN</em>PROGRESS - Data is being published to the configured stream</p> <p>SUCCEEDED - All data for the identity pool has been published to the configured stream</p> <p>FAILED - Some portion of the data has failed to publish, check FailureMessage for the cause.</p></p>
    #[serde(rename = "BulkPublishStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bulk_publish_status: Option<String>,
    /// <p>If BulkPublishStatus is FAILED this field will contain the error message that caused the bulk publish to fail.</p>
    #[serde(rename = "FailureMessage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
}

/// <p>A request for a list of the configured Cognito Events</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetCognitoEventsRequest {
    /// <p>The Cognito Identity Pool ID for the request</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The response from the GetCognitoEvents request</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetCognitoEventsResponse {
    /// <p>The Cognito Events returned from the GetCognitoEvents request</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The input for the GetIdentityPoolConfiguration operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetIdentityPoolConfigurationRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. This is the ID of the pool for which to return a configuration.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the GetIdentityPoolConfiguration operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetIdentityPoolConfigurationResponse {
    /// <p>Options to apply to this identity pool for Amazon Cognito streams.</p>
    #[serde(rename = "CognitoStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>Options to apply to this identity pool for push synchronization.</p>
    #[serde(rename = "PushSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

/// <p>Usage information for the identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IdentityPoolUsage {
    /// <p>Data storage information for the identity pool.</p>
    #[serde(rename = "DataStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>Date on which the identity pool was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>Number of sync sessions for the identity pool.</p>
    #[serde(rename = "SyncSessionsCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_sessions_count: Option<i64>,
}

/// <p>Usage information for the identity.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct IdentityUsage {
    /// <p>Total data storage for this identity.</p>
    #[serde(rename = "DataStorage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_storage: Option<i64>,
    /// <p>Number of datasets for the identity.</p>
    #[serde(rename = "DatasetCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_count: Option<i64>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_id: Option<String>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>Date on which the identity was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
}

/// <p>Request for a list of datasets for an identity.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListDatasetsRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>The maximum number of results to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returned for a successful ListDatasets request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListDatasetsResponse {
    /// <p>Number of datasets returned.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>A set of datasets.</p>
    #[serde(rename = "Datasets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub datasets: Option<Vec<Dataset>>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request for usage information on an identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListIdentityPoolUsageRequest {
    /// <p>The maximum number of results to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Returned for a successful ListIdentityPoolUsage request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListIdentityPoolUsageResponse {
    /// <p>Total number of identities for the identity pool.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>Usage information for the identity pools.</p>
    #[serde(rename = "IdentityPoolUsages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_usages: Option<Vec<IdentityPoolUsage>>,
    /// <p>The maximum number of results to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>A request for a list of records.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRecordsRequest {
    /// <p>A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (dash), and &#39;.&#39; (dot).</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>The last server sync count for this record.</p>
    #[serde(rename = "LastSyncCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_sync_count: Option<i64>,
    /// <p>The maximum number of results to be returned.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A token containing a session ID, identity ID, and expiration.</p>
    #[serde(rename = "SyncSessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_session_token: Option<String>,
}

/// <p>Returned for a successful ListRecordsRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRecordsResponse {
    /// <p>Total number of records.</p>
    #[serde(rename = "Count")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// <p>A boolean value specifying whether to delete the dataset locally.</p>
    #[serde(rename = "DatasetDeletedAfterRequestedSyncCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_deleted_after_requested_sync_count: Option<bool>,
    /// <p>Indicates whether the dataset exists.</p>
    #[serde(rename = "DatasetExists")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_exists: Option<bool>,
    /// <p>Server sync count for this dataset.</p>
    #[serde(rename = "DatasetSyncCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_sync_count: Option<i64>,
    /// <p>The user/device that made the last change to this record.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>Names of merged datasets.</p>
    #[serde(rename = "MergedDatasetNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_dataset_names: Option<Vec<String>>,
    /// <p>A pagination token for obtaining the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>A list of all records.</p>
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
    /// <p>A token containing a session ID, identity ID, and expiration.</p>
    #[serde(rename = "SyncSessionToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_session_token: Option<String>,
}

/// <p>Configuration options to be applied to the identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PushSync {
    /// <p>List of SNS platform application ARNs that could be used by clients.</p>
    #[serde(rename = "ApplicationArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arns: Option<Vec<String>>,
    /// <p>A role configured to allow Cognito to call SNS on behalf of the developer.</p>
    #[serde(rename = "RoleArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

/// <p>The basic data structure of a dataset.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Record {
    /// <p>The last modified date of the client device.</p>
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    /// <p>The key for the record.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The user/device that made the last change to this record.</p>
    #[serde(rename = "LastModifiedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by: Option<String>,
    /// <p>The date on which the record was last modified.</p>
    #[serde(rename = "LastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<f64>,
    /// <p>The server sync count for this record.</p>
    #[serde(rename = "SyncCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_count: Option<i64>,
    /// <p>The value for the record.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>An update operation for a record.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RecordPatch {
    /// <p>The last modified date of the client device.</p>
    #[serde(rename = "DeviceLastModifiedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    /// <p>The key associated with the record patch.</p>
    #[serde(rename = "Key")]
    pub key: String,
    /// <p>An operation, either replace or remove.</p>
    #[serde(rename = "Op")]
    pub op: String,
    /// <p>Last known server sync count for this record. Set to 0 if unknown.</p>
    #[serde(rename = "SyncCount")]
    pub sync_count: i64,
    /// <p>The value associated with the record patch.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>A request to RegisterDevice.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterDeviceRequest {
    /// <p>The unique ID for this identity.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. Here, the ID of the pool that the identity belongs to.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>The SNS platform type (e.g. GCM, SDM, APNS, APNS_SANDBOX).</p>
    #[serde(rename = "Platform")]
    pub platform: String,
    /// <p>The push token.</p>
    #[serde(rename = "Token")]
    pub token: String,
}

/// <p>Response to a RegisterDevice request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RegisterDeviceResponse {
    /// <p>The unique ID generated for this device by Cognito.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// <p>A request to configure Cognito Events</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetCognitoEventsRequest {
    /// <p>The events to configure</p>
    #[serde(rename = "Events")]
    pub events: ::std::collections::HashMap<String, String>,
    /// <p>The Cognito Identity Pool to use when configuring Cognito Events</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The input for the SetIdentityPoolConfiguration operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetIdentityPoolConfigurationRequest {
    /// <p>Options to apply to this identity pool for Amazon Cognito streams.</p>
    #[serde(rename = "CognitoStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. This is the ID of the pool to modify.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>Options to apply to this identity pool for push synchronization.</p>
    #[serde(rename = "PushSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

/// <p>The output for the SetIdentityPoolConfiguration operation</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SetIdentityPoolConfigurationResponse {
    /// <p>Options to apply to this identity pool for Amazon Cognito streams.</p>
    #[serde(rename = "CognitoStreams")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito.</p>
    #[serde(rename = "IdentityPoolId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    /// <p>Options to apply to this identity pool for push synchronization.</p>
    #[serde(rename = "PushSync")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub push_sync: Option<PushSync>,
}

/// <p>A request to SubscribeToDatasetRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubscribeToDatasetRequest {
    /// <p>The name of the dataset to subcribe to.</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>The unique ID generated for this device by Cognito.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>Unique ID for this identity.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which the identity belongs.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to a SubscribeToDataset request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SubscribeToDatasetResponse {}

/// <p>A request to UnsubscribeFromDataset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnsubscribeFromDatasetRequest {
    /// <p>The name of the dataset from which to unsubcribe.</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>The unique ID generated for this device by Cognito.</p>
    #[serde(rename = "DeviceId")]
    pub device_id: String,
    /// <p>Unique ID for this identity.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which this identity belongs.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to an UnsubscribeFromDataset request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UnsubscribeFromDatasetResponse {}

/// <p>A request to post updates to records or add and delete records for a dataset and user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateRecordsRequest {
    /// <p>Intended to supply a device ID that will populate the lastModifiedBy field referenced in other methods. The ClientContext field is not yet implemented.</p>
    #[serde(rename = "ClientContext")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_context: Option<String>,
    /// <p>A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, &#39;_&#39; (underscore), &#39;-&#39; (dash), and &#39;.&#39; (dot).</p>
    #[serde(rename = "DatasetName")]
    pub dataset_name: String,
    /// <p>The unique ID generated for this device by Cognito.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityId")]
    pub identity_id: String,
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
    /// <p>A list of patch operations.</p>
    #[serde(rename = "RecordPatches")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_patches: Option<Vec<RecordPatch>>,
    /// <p>The SyncSessionToken returned by a previous call to ListRecords for this dataset and identity.</p>
    #[serde(rename = "SyncSessionToken")]
    pub sync_session_token: String,
}

/// <p>Returned for a successful UpdateRecordsRequest.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct UpdateRecordsResponse {
    /// <p>A list of records that have been updated.</p>
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<Record>>,
}

/// Errors returned by BulkPublish
#[derive(Debug, PartialEq)]
pub enum BulkPublishError {
    /// <p>An exception thrown when a bulk publish operation is requested less than 24 hours after a previous bulk publish operation completed successfully.</p>
    AlreadyStreamed(String),
    /// <p>An exception thrown when there is an IN_PROGRESS bulk publish operation for the given identity pool.</p>
    DuplicateRequest(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
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

impl BulkPublishError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> BulkPublishError {
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
                "AlreadyStreamedException" => {
                    return BulkPublishError::AlreadyStreamed(String::from(error_message))
                }
                "DuplicateRequestException" => {
                    return BulkPublishError::DuplicateRequest(String::from(error_message))
                }
                "InternalErrorException" => {
                    return BulkPublishError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return BulkPublishError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return BulkPublishError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return BulkPublishError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return BulkPublishError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return BulkPublishError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for BulkPublishError {
    fn from(err: serde_json::error::Error) -> BulkPublishError {
        BulkPublishError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for BulkPublishError {
    fn from(err: CredentialsError) -> BulkPublishError {
        BulkPublishError::Credentials(err)
    }
}
impl From<HttpDispatchError> for BulkPublishError {
    fn from(err: HttpDispatchError) -> BulkPublishError {
        BulkPublishError::HttpDispatch(err)
    }
}
impl From<io::Error> for BulkPublishError {
    fn from(err: io::Error) -> BulkPublishError {
        BulkPublishError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for BulkPublishError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for BulkPublishError {
    fn description(&self) -> &str {
        match *self {
            BulkPublishError::AlreadyStreamed(ref cause) => cause,
            BulkPublishError::DuplicateRequest(ref cause) => cause,
            BulkPublishError::InternalError(ref cause) => cause,
            BulkPublishError::InvalidParameter(ref cause) => cause,
            BulkPublishError::NotAuthorized(ref cause) => cause,
            BulkPublishError::ResourceNotFound(ref cause) => cause,
            BulkPublishError::Validation(ref cause) => cause,
            BulkPublishError::Credentials(ref err) => err.description(),
            BulkPublishError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BulkPublishError::ParseError(ref cause) => cause,
            BulkPublishError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DeleteDataset
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if an update can&#39;t be applied because the resource was changed by another call and this would result in a conflict.</p>
    ResourceConflict(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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
                "InternalErrorException" => {
                    return DeleteDatasetError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DeleteDatasetError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return DeleteDatasetError::NotAuthorized(String::from(error_message))
                }
                "ResourceConflictException" => {
                    return DeleteDatasetError::ResourceConflict(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DeleteDatasetError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DeleteDatasetError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DeleteDatasetError::Validation(error_message.to_string())
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
            DeleteDatasetError::InternalError(ref cause) => cause,
            DeleteDatasetError::InvalidParameter(ref cause) => cause,
            DeleteDatasetError::NotAuthorized(ref cause) => cause,
            DeleteDatasetError::ResourceConflict(ref cause) => cause,
            DeleteDatasetError::ResourceNotFound(ref cause) => cause,
            DeleteDatasetError::TooManyRequests(ref cause) => cause,
            DeleteDatasetError::Validation(ref cause) => cause,
            DeleteDatasetError::Credentials(ref err) => err.description(),
            DeleteDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDatasetError::ParseError(ref cause) => cause,
            DeleteDatasetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeDataset
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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
                "InternalErrorException" => {
                    return DescribeDatasetError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DescribeDatasetError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return DescribeDatasetError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DescribeDatasetError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeDatasetError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeDatasetError::Validation(error_message.to_string())
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
            DescribeDatasetError::InternalError(ref cause) => cause,
            DescribeDatasetError::InvalidParameter(ref cause) => cause,
            DescribeDatasetError::NotAuthorized(ref cause) => cause,
            DescribeDatasetError::ResourceNotFound(ref cause) => cause,
            DescribeDatasetError::TooManyRequests(ref cause) => cause,
            DescribeDatasetError::Validation(ref cause) => cause,
            DescribeDatasetError::Credentials(ref err) => err.description(),
            DescribeDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDatasetError::ParseError(ref cause) => cause,
            DescribeDatasetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeIdentityPoolUsage
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityPoolUsageError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl DescribeIdentityPoolUsageError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeIdentityPoolUsageError {
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
                "InternalErrorException" => {
                    return DescribeIdentityPoolUsageError::InternalError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return DescribeIdentityPoolUsageError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "NotAuthorizedException" => {
                    return DescribeIdentityPoolUsageError::NotAuthorized(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return DescribeIdentityPoolUsageError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return DescribeIdentityPoolUsageError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return DescribeIdentityPoolUsageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeIdentityPoolUsageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeIdentityPoolUsageError {
    fn from(err: serde_json::error::Error) -> DescribeIdentityPoolUsageError {
        DescribeIdentityPoolUsageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeIdentityPoolUsageError {
    fn from(err: CredentialsError) -> DescribeIdentityPoolUsageError {
        DescribeIdentityPoolUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIdentityPoolUsageError {
    fn from(err: HttpDispatchError) -> DescribeIdentityPoolUsageError {
        DescribeIdentityPoolUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeIdentityPoolUsageError {
    fn from(err: io::Error) -> DescribeIdentityPoolUsageError {
        DescribeIdentityPoolUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeIdentityPoolUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityPoolUsageError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityPoolUsageError::InternalError(ref cause) => cause,
            DescribeIdentityPoolUsageError::InvalidParameter(ref cause) => cause,
            DescribeIdentityPoolUsageError::NotAuthorized(ref cause) => cause,
            DescribeIdentityPoolUsageError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityPoolUsageError::TooManyRequests(ref cause) => cause,
            DescribeIdentityPoolUsageError::Validation(ref cause) => cause,
            DescribeIdentityPoolUsageError::Credentials(ref err) => err.description(),
            DescribeIdentityPoolUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIdentityPoolUsageError::ParseError(ref cause) => cause,
            DescribeIdentityPoolUsageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by DescribeIdentityUsage
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityUsageError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl DescribeIdentityUsageError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> DescribeIdentityUsageError {
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
                "InternalErrorException" => {
                    return DescribeIdentityUsageError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return DescribeIdentityUsageError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return DescribeIdentityUsageError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return DescribeIdentityUsageError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return DescribeIdentityUsageError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return DescribeIdentityUsageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return DescribeIdentityUsageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for DescribeIdentityUsageError {
    fn from(err: serde_json::error::Error) -> DescribeIdentityUsageError {
        DescribeIdentityUsageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for DescribeIdentityUsageError {
    fn from(err: CredentialsError) -> DescribeIdentityUsageError {
        DescribeIdentityUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for DescribeIdentityUsageError {
    fn from(err: HttpDispatchError) -> DescribeIdentityUsageError {
        DescribeIdentityUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for DescribeIdentityUsageError {
    fn from(err: io::Error) -> DescribeIdentityUsageError {
        DescribeIdentityUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for DescribeIdentityUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeIdentityUsageError {
    fn description(&self) -> &str {
        match *self {
            DescribeIdentityUsageError::InternalError(ref cause) => cause,
            DescribeIdentityUsageError::InvalidParameter(ref cause) => cause,
            DescribeIdentityUsageError::NotAuthorized(ref cause) => cause,
            DescribeIdentityUsageError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityUsageError::TooManyRequests(ref cause) => cause,
            DescribeIdentityUsageError::Validation(ref cause) => cause,
            DescribeIdentityUsageError::Credentials(ref err) => err.description(),
            DescribeIdentityUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIdentityUsageError::ParseError(ref cause) => cause,
            DescribeIdentityUsageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetBulkPublishDetails
#[derive(Debug, PartialEq)]
pub enum GetBulkPublishDetailsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
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

impl GetBulkPublishDetailsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetBulkPublishDetailsError {
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
                "InternalErrorException" => {
                    return GetBulkPublishDetailsError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return GetBulkPublishDetailsError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return GetBulkPublishDetailsError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetBulkPublishDetailsError::ResourceNotFound(String::from(error_message))
                }
                "ValidationException" => {
                    return GetBulkPublishDetailsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetBulkPublishDetailsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetBulkPublishDetailsError {
    fn from(err: serde_json::error::Error) -> GetBulkPublishDetailsError {
        GetBulkPublishDetailsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetBulkPublishDetailsError {
    fn from(err: CredentialsError) -> GetBulkPublishDetailsError {
        GetBulkPublishDetailsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetBulkPublishDetailsError {
    fn from(err: HttpDispatchError) -> GetBulkPublishDetailsError {
        GetBulkPublishDetailsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetBulkPublishDetailsError {
    fn from(err: io::Error) -> GetBulkPublishDetailsError {
        GetBulkPublishDetailsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetBulkPublishDetailsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetBulkPublishDetailsError {
    fn description(&self) -> &str {
        match *self {
            GetBulkPublishDetailsError::InternalError(ref cause) => cause,
            GetBulkPublishDetailsError::InvalidParameter(ref cause) => cause,
            GetBulkPublishDetailsError::NotAuthorized(ref cause) => cause,
            GetBulkPublishDetailsError::ResourceNotFound(ref cause) => cause,
            GetBulkPublishDetailsError::Validation(ref cause) => cause,
            GetBulkPublishDetailsError::Credentials(ref err) => err.description(),
            GetBulkPublishDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBulkPublishDetailsError::ParseError(ref cause) => cause,
            GetBulkPublishDetailsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetCognitoEvents
#[derive(Debug, PartialEq)]
pub enum GetCognitoEventsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl GetCognitoEventsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetCognitoEventsError {
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
                "InternalErrorException" => {
                    return GetCognitoEventsError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return GetCognitoEventsError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return GetCognitoEventsError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return GetCognitoEventsError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return GetCognitoEventsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return GetCognitoEventsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetCognitoEventsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetCognitoEventsError {
    fn from(err: serde_json::error::Error) -> GetCognitoEventsError {
        GetCognitoEventsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetCognitoEventsError {
    fn from(err: CredentialsError) -> GetCognitoEventsError {
        GetCognitoEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetCognitoEventsError {
    fn from(err: HttpDispatchError) -> GetCognitoEventsError {
        GetCognitoEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetCognitoEventsError {
    fn from(err: io::Error) -> GetCognitoEventsError {
        GetCognitoEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetCognitoEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetCognitoEventsError {
    fn description(&self) -> &str {
        match *self {
            GetCognitoEventsError::InternalError(ref cause) => cause,
            GetCognitoEventsError::InvalidParameter(ref cause) => cause,
            GetCognitoEventsError::NotAuthorized(ref cause) => cause,
            GetCognitoEventsError::ResourceNotFound(ref cause) => cause,
            GetCognitoEventsError::TooManyRequests(ref cause) => cause,
            GetCognitoEventsError::Validation(ref cause) => cause,
            GetCognitoEventsError::Credentials(ref err) => err.description(),
            GetCognitoEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCognitoEventsError::ParseError(ref cause) => cause,
            GetCognitoEventsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by GetIdentityPoolConfiguration
#[derive(Debug, PartialEq)]
pub enum GetIdentityPoolConfigurationError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl GetIdentityPoolConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> GetIdentityPoolConfigurationError {
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
                "InternalErrorException" => {
                    return GetIdentityPoolConfigurationError::InternalError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return GetIdentityPoolConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "NotAuthorizedException" => {
                    return GetIdentityPoolConfigurationError::NotAuthorized(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return GetIdentityPoolConfigurationError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return GetIdentityPoolConfigurationError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return GetIdentityPoolConfigurationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return GetIdentityPoolConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for GetIdentityPoolConfigurationError {
    fn from(err: serde_json::error::Error) -> GetIdentityPoolConfigurationError {
        GetIdentityPoolConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for GetIdentityPoolConfigurationError {
    fn from(err: CredentialsError) -> GetIdentityPoolConfigurationError {
        GetIdentityPoolConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for GetIdentityPoolConfigurationError {
    fn from(err: HttpDispatchError) -> GetIdentityPoolConfigurationError {
        GetIdentityPoolConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for GetIdentityPoolConfigurationError {
    fn from(err: io::Error) -> GetIdentityPoolConfigurationError {
        GetIdentityPoolConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for GetIdentityPoolConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetIdentityPoolConfigurationError {
    fn description(&self) -> &str {
        match *self {
            GetIdentityPoolConfigurationError::InternalError(ref cause) => cause,
            GetIdentityPoolConfigurationError::InvalidParameter(ref cause) => cause,
            GetIdentityPoolConfigurationError::NotAuthorized(ref cause) => cause,
            GetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => cause,
            GetIdentityPoolConfigurationError::TooManyRequests(ref cause) => cause,
            GetIdentityPoolConfigurationError::Validation(ref cause) => cause,
            GetIdentityPoolConfigurationError::Credentials(ref err) => err.description(),
            GetIdentityPoolConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityPoolConfigurationError::ParseError(ref cause) => cause,
            GetIdentityPoolConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListDatasets
#[derive(Debug, PartialEq)]
pub enum ListDatasetsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the request is throttled.</p>
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
                "InternalErrorException" => {
                    return ListDatasetsError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListDatasetsError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return ListDatasetsError::NotAuthorized(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListDatasetsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return ListDatasetsError::Validation(error_message.to_string())
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
            ListDatasetsError::InternalError(ref cause) => cause,
            ListDatasetsError::InvalidParameter(ref cause) => cause,
            ListDatasetsError::NotAuthorized(ref cause) => cause,
            ListDatasetsError::TooManyRequests(ref cause) => cause,
            ListDatasetsError::Validation(ref cause) => cause,
            ListDatasetsError::Credentials(ref err) => err.description(),
            ListDatasetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDatasetsError::ParseError(ref cause) => cause,
            ListDatasetsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListIdentityPoolUsage
#[derive(Debug, PartialEq)]
pub enum ListIdentityPoolUsageError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl ListIdentityPoolUsageError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListIdentityPoolUsageError {
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
                "InternalErrorException" => {
                    return ListIdentityPoolUsageError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListIdentityPoolUsageError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return ListIdentityPoolUsageError::NotAuthorized(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListIdentityPoolUsageError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return ListIdentityPoolUsageError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListIdentityPoolUsageError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListIdentityPoolUsageError {
    fn from(err: serde_json::error::Error) -> ListIdentityPoolUsageError {
        ListIdentityPoolUsageError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListIdentityPoolUsageError {
    fn from(err: CredentialsError) -> ListIdentityPoolUsageError {
        ListIdentityPoolUsageError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListIdentityPoolUsageError {
    fn from(err: HttpDispatchError) -> ListIdentityPoolUsageError {
        ListIdentityPoolUsageError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListIdentityPoolUsageError {
    fn from(err: io::Error) -> ListIdentityPoolUsageError {
        ListIdentityPoolUsageError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListIdentityPoolUsageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListIdentityPoolUsageError {
    fn description(&self) -> &str {
        match *self {
            ListIdentityPoolUsageError::InternalError(ref cause) => cause,
            ListIdentityPoolUsageError::InvalidParameter(ref cause) => cause,
            ListIdentityPoolUsageError::NotAuthorized(ref cause) => cause,
            ListIdentityPoolUsageError::TooManyRequests(ref cause) => cause,
            ListIdentityPoolUsageError::Validation(ref cause) => cause,
            ListIdentityPoolUsageError::Credentials(ref err) => err.description(),
            ListIdentityPoolUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIdentityPoolUsageError::ParseError(ref cause) => cause,
            ListIdentityPoolUsageError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by ListRecords
#[derive(Debug, PartialEq)]
pub enum ListRecordsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl ListRecordsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> ListRecordsError {
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
                "InternalErrorException" => {
                    return ListRecordsError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return ListRecordsError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return ListRecordsError::NotAuthorized(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return ListRecordsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return ListRecordsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return ListRecordsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for ListRecordsError {
    fn from(err: serde_json::error::Error) -> ListRecordsError {
        ListRecordsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for ListRecordsError {
    fn from(err: CredentialsError) -> ListRecordsError {
        ListRecordsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for ListRecordsError {
    fn from(err: HttpDispatchError) -> ListRecordsError {
        ListRecordsError::HttpDispatch(err)
    }
}
impl From<io::Error> for ListRecordsError {
    fn from(err: io::Error) -> ListRecordsError {
        ListRecordsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for ListRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRecordsError {
    fn description(&self) -> &str {
        match *self {
            ListRecordsError::InternalError(ref cause) => cause,
            ListRecordsError::InvalidParameter(ref cause) => cause,
            ListRecordsError::NotAuthorized(ref cause) => cause,
            ListRecordsError::TooManyRequests(ref cause) => cause,
            ListRecordsError::Validation(ref cause) => cause,
            ListRecordsError::Credentials(ref err) => err.description(),
            ListRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRecordsError::ParseError(ref cause) => cause,
            ListRecordsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by RegisterDevice
#[derive(Debug, PartialEq)]
pub enum RegisterDeviceError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),

    InvalidConfiguration(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl RegisterDeviceError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> RegisterDeviceError {
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
                "InternalErrorException" => {
                    return RegisterDeviceError::InternalError(String::from(error_message))
                }
                "InvalidConfigurationException" => {
                    return RegisterDeviceError::InvalidConfiguration(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return RegisterDeviceError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return RegisterDeviceError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return RegisterDeviceError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return RegisterDeviceError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return RegisterDeviceError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return RegisterDeviceError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for RegisterDeviceError {
    fn from(err: serde_json::error::Error) -> RegisterDeviceError {
        RegisterDeviceError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for RegisterDeviceError {
    fn from(err: CredentialsError) -> RegisterDeviceError {
        RegisterDeviceError::Credentials(err)
    }
}
impl From<HttpDispatchError> for RegisterDeviceError {
    fn from(err: HttpDispatchError) -> RegisterDeviceError {
        RegisterDeviceError::HttpDispatch(err)
    }
}
impl From<io::Error> for RegisterDeviceError {
    fn from(err: io::Error) -> RegisterDeviceError {
        RegisterDeviceError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for RegisterDeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterDeviceError {
    fn description(&self) -> &str {
        match *self {
            RegisterDeviceError::InternalError(ref cause) => cause,
            RegisterDeviceError::InvalidConfiguration(ref cause) => cause,
            RegisterDeviceError::InvalidParameter(ref cause) => cause,
            RegisterDeviceError::NotAuthorized(ref cause) => cause,
            RegisterDeviceError::ResourceNotFound(ref cause) => cause,
            RegisterDeviceError::TooManyRequests(ref cause) => cause,
            RegisterDeviceError::Validation(ref cause) => cause,
            RegisterDeviceError::Credentials(ref err) => err.description(),
            RegisterDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterDeviceError::ParseError(ref cause) => cause,
            RegisterDeviceError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SetCognitoEvents
#[derive(Debug, PartialEq)]
pub enum SetCognitoEventsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl SetCognitoEventsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SetCognitoEventsError {
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
                "InternalErrorException" => {
                    return SetCognitoEventsError::InternalError(String::from(error_message))
                }
                "InvalidParameterException" => {
                    return SetCognitoEventsError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return SetCognitoEventsError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return SetCognitoEventsError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return SetCognitoEventsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return SetCognitoEventsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SetCognitoEventsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SetCognitoEventsError {
    fn from(err: serde_json::error::Error) -> SetCognitoEventsError {
        SetCognitoEventsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SetCognitoEventsError {
    fn from(err: CredentialsError) -> SetCognitoEventsError {
        SetCognitoEventsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetCognitoEventsError {
    fn from(err: HttpDispatchError) -> SetCognitoEventsError {
        SetCognitoEventsError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetCognitoEventsError {
    fn from(err: io::Error) -> SetCognitoEventsError {
        SetCognitoEventsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetCognitoEventsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetCognitoEventsError {
    fn description(&self) -> &str {
        match *self {
            SetCognitoEventsError::InternalError(ref cause) => cause,
            SetCognitoEventsError::InvalidParameter(ref cause) => cause,
            SetCognitoEventsError::NotAuthorized(ref cause) => cause,
            SetCognitoEventsError::ResourceNotFound(ref cause) => cause,
            SetCognitoEventsError::TooManyRequests(ref cause) => cause,
            SetCognitoEventsError::Validation(ref cause) => cause,
            SetCognitoEventsError::Credentials(ref err) => err.description(),
            SetCognitoEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetCognitoEventsError::ParseError(ref cause) => cause,
            SetCognitoEventsError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SetIdentityPoolConfiguration
#[derive(Debug, PartialEq)]
pub enum SetIdentityPoolConfigurationError {
    /// <p>Thrown if there are parallel requests to modify a resource.</p>
    ConcurrentModification(String),
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl SetIdentityPoolConfigurationError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SetIdentityPoolConfigurationError {
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
                "ConcurrentModificationException" => {
                    return SetIdentityPoolConfigurationError::ConcurrentModification(String::from(
                        error_message,
                    ))
                }
                "InternalErrorException" => {
                    return SetIdentityPoolConfigurationError::InternalError(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return SetIdentityPoolConfigurationError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "NotAuthorizedException" => {
                    return SetIdentityPoolConfigurationError::NotAuthorized(String::from(
                        error_message,
                    ))
                }
                "ResourceNotFoundException" => {
                    return SetIdentityPoolConfigurationError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return SetIdentityPoolConfigurationError::TooManyRequests(String::from(
                        error_message,
                    ))
                }
                "ValidationException" => {
                    return SetIdentityPoolConfigurationError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SetIdentityPoolConfigurationError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SetIdentityPoolConfigurationError {
    fn from(err: serde_json::error::Error) -> SetIdentityPoolConfigurationError {
        SetIdentityPoolConfigurationError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SetIdentityPoolConfigurationError {
    fn from(err: CredentialsError) -> SetIdentityPoolConfigurationError {
        SetIdentityPoolConfigurationError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SetIdentityPoolConfigurationError {
    fn from(err: HttpDispatchError) -> SetIdentityPoolConfigurationError {
        SetIdentityPoolConfigurationError::HttpDispatch(err)
    }
}
impl From<io::Error> for SetIdentityPoolConfigurationError {
    fn from(err: io::Error) -> SetIdentityPoolConfigurationError {
        SetIdentityPoolConfigurationError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SetIdentityPoolConfigurationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetIdentityPoolConfigurationError {
    fn description(&self) -> &str {
        match *self {
            SetIdentityPoolConfigurationError::ConcurrentModification(ref cause) => cause,
            SetIdentityPoolConfigurationError::InternalError(ref cause) => cause,
            SetIdentityPoolConfigurationError::InvalidParameter(ref cause) => cause,
            SetIdentityPoolConfigurationError::NotAuthorized(ref cause) => cause,
            SetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => cause,
            SetIdentityPoolConfigurationError::TooManyRequests(ref cause) => cause,
            SetIdentityPoolConfigurationError::Validation(ref cause) => cause,
            SetIdentityPoolConfigurationError::Credentials(ref err) => err.description(),
            SetIdentityPoolConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityPoolConfigurationError::ParseError(ref cause) => cause,
            SetIdentityPoolConfigurationError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by SubscribeToDataset
#[derive(Debug, PartialEq)]
pub enum SubscribeToDatasetError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),

    InvalidConfiguration(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl SubscribeToDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> SubscribeToDatasetError {
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
                "InternalErrorException" => {
                    return SubscribeToDatasetError::InternalError(String::from(error_message))
                }
                "InvalidConfigurationException" => {
                    return SubscribeToDatasetError::InvalidConfiguration(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return SubscribeToDatasetError::InvalidParameter(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return SubscribeToDatasetError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return SubscribeToDatasetError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return SubscribeToDatasetError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return SubscribeToDatasetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return SubscribeToDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for SubscribeToDatasetError {
    fn from(err: serde_json::error::Error) -> SubscribeToDatasetError {
        SubscribeToDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for SubscribeToDatasetError {
    fn from(err: CredentialsError) -> SubscribeToDatasetError {
        SubscribeToDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for SubscribeToDatasetError {
    fn from(err: HttpDispatchError) -> SubscribeToDatasetError {
        SubscribeToDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for SubscribeToDatasetError {
    fn from(err: io::Error) -> SubscribeToDatasetError {
        SubscribeToDatasetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for SubscribeToDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubscribeToDatasetError {
    fn description(&self) -> &str {
        match *self {
            SubscribeToDatasetError::InternalError(ref cause) => cause,
            SubscribeToDatasetError::InvalidConfiguration(ref cause) => cause,
            SubscribeToDatasetError::InvalidParameter(ref cause) => cause,
            SubscribeToDatasetError::NotAuthorized(ref cause) => cause,
            SubscribeToDatasetError::ResourceNotFound(ref cause) => cause,
            SubscribeToDatasetError::TooManyRequests(ref cause) => cause,
            SubscribeToDatasetError::Validation(ref cause) => cause,
            SubscribeToDatasetError::Credentials(ref err) => err.description(),
            SubscribeToDatasetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SubscribeToDatasetError::ParseError(ref cause) => cause,
            SubscribeToDatasetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UnsubscribeFromDataset
#[derive(Debug, PartialEq)]
pub enum UnsubscribeFromDatasetError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),

    InvalidConfiguration(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl UnsubscribeFromDatasetError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UnsubscribeFromDatasetError {
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
                "InternalErrorException" => {
                    return UnsubscribeFromDatasetError::InternalError(String::from(error_message))
                }
                "InvalidConfigurationException" => {
                    return UnsubscribeFromDatasetError::InvalidConfiguration(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return UnsubscribeFromDatasetError::InvalidParameter(String::from(
                        error_message,
                    ))
                }
                "NotAuthorizedException" => {
                    return UnsubscribeFromDatasetError::NotAuthorized(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UnsubscribeFromDatasetError::ResourceNotFound(String::from(
                        error_message,
                    ))
                }
                "TooManyRequestsException" => {
                    return UnsubscribeFromDatasetError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UnsubscribeFromDatasetError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UnsubscribeFromDatasetError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UnsubscribeFromDatasetError {
    fn from(err: serde_json::error::Error) -> UnsubscribeFromDatasetError {
        UnsubscribeFromDatasetError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UnsubscribeFromDatasetError {
    fn from(err: CredentialsError) -> UnsubscribeFromDatasetError {
        UnsubscribeFromDatasetError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UnsubscribeFromDatasetError {
    fn from(err: HttpDispatchError) -> UnsubscribeFromDatasetError {
        UnsubscribeFromDatasetError::HttpDispatch(err)
    }
}
impl From<io::Error> for UnsubscribeFromDatasetError {
    fn from(err: io::Error) -> UnsubscribeFromDatasetError {
        UnsubscribeFromDatasetError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UnsubscribeFromDatasetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnsubscribeFromDatasetError {
    fn description(&self) -> &str {
        match *self {
            UnsubscribeFromDatasetError::InternalError(ref cause) => cause,
            UnsubscribeFromDatasetError::InvalidConfiguration(ref cause) => cause,
            UnsubscribeFromDatasetError::InvalidParameter(ref cause) => cause,
            UnsubscribeFromDatasetError::NotAuthorized(ref cause) => cause,
            UnsubscribeFromDatasetError::ResourceNotFound(ref cause) => cause,
            UnsubscribeFromDatasetError::TooManyRequests(ref cause) => cause,
            UnsubscribeFromDatasetError::Validation(ref cause) => cause,
            UnsubscribeFromDatasetError::Credentials(ref err) => err.description(),
            UnsubscribeFromDatasetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UnsubscribeFromDatasetError::ParseError(ref cause) => cause,
            UnsubscribeFromDatasetError::Unknown(_) => "unknown error",
        }
    }
}
/// Errors returned by UpdateRecords
#[derive(Debug, PartialEq)]
pub enum UpdateRecordsError {
    /// <p>Indicates an internal service error.</p>
    InternalError(String),
    /// <p>The AWS Lambda function returned invalid output or an exception.</p>
    InvalidLambdaFunctionOutput(String),
    /// <p>Thrown when a request parameter does not comply with the associated constraints.</p>
    InvalidParameter(String),
    /// <p>AWS Lambda throttled your account, please contact AWS Support</p>
    LambdaThrottled(String),
    /// <p>Thrown when the limit on the number of objects or operations has been exceeded.</p>
    LimitExceeded(String),
    /// <p>Thrown when a user is not authorized to access the requested resource.</p>
    NotAuthorized(String),
    /// <p>Thrown if an update can&#39;t be applied because the resource was changed by another call and this would result in a conflict.</p>
    ResourceConflict(String),
    /// <p>Thrown if the resource doesn&#39;t exist.</p>
    ResourceNotFound(String),
    /// <p>Thrown if the request is throttled.</p>
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

impl UpdateRecordsError {
    // see boto RestJSONParser impl for parsing errors
    // https://github.com/boto/botocore/blob/4dff78c840403d1d17db9b3f800b20d3bd9fbf9f/botocore/parsers.py#L838-L850
    pub fn from_response(res: BufferedHttpResponse) -> UpdateRecordsError {
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
                "InternalErrorException" => {
                    return UpdateRecordsError::InternalError(String::from(error_message))
                }
                "InvalidLambdaFunctionOutputException" => {
                    return UpdateRecordsError::InvalidLambdaFunctionOutput(String::from(
                        error_message,
                    ))
                }
                "InvalidParameterException" => {
                    return UpdateRecordsError::InvalidParameter(String::from(error_message))
                }
                "LambdaThrottledException" => {
                    return UpdateRecordsError::LambdaThrottled(String::from(error_message))
                }
                "LimitExceededException" => {
                    return UpdateRecordsError::LimitExceeded(String::from(error_message))
                }
                "NotAuthorizedException" => {
                    return UpdateRecordsError::NotAuthorized(String::from(error_message))
                }
                "ResourceConflictException" => {
                    return UpdateRecordsError::ResourceConflict(String::from(error_message))
                }
                "ResourceNotFoundException" => {
                    return UpdateRecordsError::ResourceNotFound(String::from(error_message))
                }
                "TooManyRequestsException" => {
                    return UpdateRecordsError::TooManyRequests(String::from(error_message))
                }
                "ValidationException" => {
                    return UpdateRecordsError::Validation(error_message.to_string())
                }
                _ => {}
            }
        }
        return UpdateRecordsError::Unknown(res);
    }
}

impl From<serde_json::error::Error> for UpdateRecordsError {
    fn from(err: serde_json::error::Error) -> UpdateRecordsError {
        UpdateRecordsError::ParseError(err.description().to_string())
    }
}
impl From<CredentialsError> for UpdateRecordsError {
    fn from(err: CredentialsError) -> UpdateRecordsError {
        UpdateRecordsError::Credentials(err)
    }
}
impl From<HttpDispatchError> for UpdateRecordsError {
    fn from(err: HttpDispatchError) -> UpdateRecordsError {
        UpdateRecordsError::HttpDispatch(err)
    }
}
impl From<io::Error> for UpdateRecordsError {
    fn from(err: io::Error) -> UpdateRecordsError {
        UpdateRecordsError::HttpDispatch(HttpDispatchError::from(err))
    }
}
impl fmt::Display for UpdateRecordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateRecordsError {
    fn description(&self) -> &str {
        match *self {
            UpdateRecordsError::InternalError(ref cause) => cause,
            UpdateRecordsError::InvalidLambdaFunctionOutput(ref cause) => cause,
            UpdateRecordsError::InvalidParameter(ref cause) => cause,
            UpdateRecordsError::LambdaThrottled(ref cause) => cause,
            UpdateRecordsError::LimitExceeded(ref cause) => cause,
            UpdateRecordsError::NotAuthorized(ref cause) => cause,
            UpdateRecordsError::ResourceConflict(ref cause) => cause,
            UpdateRecordsError::ResourceNotFound(ref cause) => cause,
            UpdateRecordsError::TooManyRequests(ref cause) => cause,
            UpdateRecordsError::Validation(ref cause) => cause,
            UpdateRecordsError::Credentials(ref err) => err.description(),
            UpdateRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRecordsError::ParseError(ref cause) => cause,
            UpdateRecordsError::Unknown(_) => "unknown error",
        }
    }
}
/// Trait representing the capabilities of the Amazon Cognito Sync API. Amazon Cognito Sync clients implement this trait.
pub trait CognitoSync {
    /// <p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn bulk_publish(
        &self,
        input: BulkPublishRequest,
    ) -> RusotoFuture<BulkPublishResponse, BulkPublishError>;

    /// <p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> RusotoFuture<DeleteDatasetResponse, DeleteDatasetError>;

    /// <p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> RusotoFuture<DescribeDatasetResponse, DescribeDatasetError>;

    /// <p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn describe_identity_pool_usage(
        &self,
        input: DescribeIdentityPoolUsageRequest,
    ) -> RusotoFuture<DescribeIdentityPoolUsageResponse, DescribeIdentityPoolUsageError>;

    /// <p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn describe_identity_usage(
        &self,
        input: DescribeIdentityUsageRequest,
    ) -> RusotoFuture<DescribeIdentityUsageResponse, DescribeIdentityUsageError>;

    /// <p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_bulk_publish_details(
        &self,
        input: GetBulkPublishDetailsRequest,
    ) -> RusotoFuture<GetBulkPublishDetailsResponse, GetBulkPublishDetailsError>;

    /// <p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_cognito_events(
        &self,
        input: GetCognitoEventsRequest,
    ) -> RusotoFuture<GetCognitoEventsResponse, GetCognitoEventsError>;

    /// <p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_identity_pool_configuration(
        &self,
        input: GetIdentityPoolConfigurationRequest,
    ) -> RusotoFuture<GetIdentityPoolConfigurationResponse, GetIdentityPoolConfigurationError>;

    /// <p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>
    fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> RusotoFuture<ListDatasetsResponse, ListDatasetsError>;

    /// <p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>
    fn list_identity_pool_usage(
        &self,
        input: ListIdentityPoolUsageRequest,
    ) -> RusotoFuture<ListIdentityPoolUsageResponse, ListIdentityPoolUsageError>;

    /// <p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    fn list_records(
        &self,
        input: ListRecordsRequest,
    ) -> RusotoFuture<ListRecordsResponse, ListRecordsError>;

    /// <p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn register_device(
        &self,
        input: RegisterDeviceRequest,
    ) -> RusotoFuture<RegisterDeviceResponse, RegisterDeviceError>;

    /// <p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn set_cognito_events(
        &self,
        input: SetCognitoEventsRequest,
    ) -> RusotoFuture<(), SetCognitoEventsError>;

    /// <p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn set_identity_pool_configuration(
        &self,
        input: SetIdentityPoolConfigurationRequest,
    ) -> RusotoFuture<SetIdentityPoolConfigurationResponse, SetIdentityPoolConfigurationError>;

    /// <p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn subscribe_to_dataset(
        &self,
        input: SubscribeToDatasetRequest,
    ) -> RusotoFuture<SubscribeToDatasetResponse, SubscribeToDatasetError>;

    /// <p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn unsubscribe_from_dataset(
        &self,
        input: UnsubscribeFromDatasetRequest,
    ) -> RusotoFuture<UnsubscribeFromDatasetResponse, UnsubscribeFromDatasetError>;

    /// <p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn update_records(
        &self,
        input: UpdateRecordsRequest,
    ) -> RusotoFuture<UpdateRecordsResponse, UpdateRecordsError>;
}
/// A client for the Amazon Cognito Sync API.
pub struct CognitoSyncClient {
    client: Client,
    region: region::Region,
}

impl CognitoSyncClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> CognitoSyncClient {
        CognitoSyncClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CognitoSyncClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        CognitoSyncClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl CognitoSync for CognitoSyncClient {
    /// <p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn bulk_publish(
        &self,
        input: BulkPublishRequest,
    ) -> RusotoFuture<BulkPublishResponse, BulkPublishError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/bulkpublish",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
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
                    let result = serde_json::from_slice::<BulkPublishResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(BulkPublishError::from_response(response))),
                )
            }
        })
    }

    /// <p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> RusotoFuture<DeleteDatasetResponse, DeleteDatasetError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
            dataset_name = input.dataset_name,
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
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
                    let result = serde_json::from_slice::<DeleteDatasetResponse>(&body).unwrap();

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

    /// <p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> RusotoFuture<DescribeDatasetResponse, DescribeDatasetError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
            dataset_name = input.dataset_name,
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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

    /// <p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn describe_identity_pool_usage(
        &self,
        input: DescribeIdentityPoolUsageRequest,
    ) -> RusotoFuture<DescribeIdentityPoolUsageResponse, DescribeIdentityPoolUsageError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<DescribeIdentityPoolUsageResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeIdentityPoolUsageError::from_response(response))
                }))
            }
        })
    }

    /// <p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn describe_identity_usage(
        &self,
        input: DescribeIdentityUsageRequest,
    ) -> RusotoFuture<DescribeIdentityUsageResponse, DescribeIdentityUsageError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}",
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<DescribeIdentityUsageResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeIdentityUsageError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_bulk_publish_details(
        &self,
        input: GetBulkPublishDetailsRequest,
    ) -> RusotoFuture<GetBulkPublishDetailsResponse, GetBulkPublishDetailsError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/getBulkPublishDetails",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<GetBulkPublishDetailsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetBulkPublishDetailsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_cognito_events(
        &self,
        input: GetCognitoEventsRequest,
    ) -> RusotoFuture<GetCognitoEventsResponse, GetCognitoEventsError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/events",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
                    let result = serde_json::from_slice::<GetCognitoEventsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(GetCognitoEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn get_identity_pool_configuration(
        &self,
        input: GetIdentityPoolConfigurationRequest,
    ) -> RusotoFuture<GetIdentityPoolConfigurationResponse, GetIdentityPoolConfigurationError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/configuration",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<GetIdentityPoolConfigurationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(GetIdentityPoolConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>
    fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> RusotoFuture<ListDatasetsResponse, ListDatasetsError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets",
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
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

    /// <p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>
    fn list_identity_pool_usage(
        &self,
        input: ListIdentityPoolUsageRequest,
    ) -> RusotoFuture<ListIdentityPoolUsageResponse, ListIdentityPoolUsageError> {
        let request_uri = "/identitypools";

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
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
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result =
                        serde_json::from_slice::<ListIdentityPoolUsageResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListIdentityPoolUsageError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    fn list_records(
        &self,
        input: ListRecordsRequest,
    ) -> RusotoFuture<ListRecordsResponse, ListRecordsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/records", dataset_name = input.dataset_name, identity_id = input.identity_id, identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.last_sync_count {
            params.put("lastSyncCount", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("nextToken", x);
        }
        if let Some(ref x) = input.sync_session_token {
            params.put("syncSessionToken", x);
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
                    let result = serde_json::from_slice::<ListRecordsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRecordsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn register_device(
        &self,
        input: RegisterDeviceRequest,
    ) -> RusotoFuture<RegisterDeviceResponse, RegisterDeviceError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identity/{identity_id}/device",
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
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
                    let result = serde_json::from_slice::<RegisterDeviceResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(RegisterDeviceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn set_cognito_events(
        &self,
        input: SetCognitoEventsRequest,
    ) -> RusotoFuture<(), SetCognitoEventsError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/events",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let result = ::std::mem::drop(response);

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetCognitoEventsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    fn set_identity_pool_configuration(
        &self,
        input: SetIdentityPoolConfigurationRequest,
    ) -> RusotoFuture<SetIdentityPoolConfigurationResponse, SetIdentityPoolConfigurationError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/configuration",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
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
                    let result =
                        serde_json::from_slice::<SetIdentityPoolConfigurationResponse>(&body)
                            .unwrap();

                    result
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(SetIdentityPoolConfigurationError::from_response(response))
                }))
            }
        })
    }

    /// <p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn subscribe_to_dataset(
        &self,
        input: SubscribeToDatasetRequest,
    ) -> RusotoFuture<SubscribeToDatasetResponse, SubscribeToDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}", dataset_name = input.dataset_name, device_id = input.device_id, identity_id = input.identity_id, identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<SubscribeToDatasetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SubscribeToDatasetError::from_response(response))),
                )
            }
        })
    }

    /// <p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    fn unsubscribe_from_dataset(
        &self,
        input: UnsubscribeFromDatasetRequest,
    ) -> RusotoFuture<UnsubscribeFromDatasetResponse, UnsubscribeFromDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}", dataset_name = input.dataset_name, device_id = input.device_id, identity_id = input.identity_id, identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
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
                        serde_json::from_slice::<UnsubscribeFromDatasetResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UnsubscribeFromDatasetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    fn update_records(
        &self,
        input: UpdateRecordsRequest,
    ) -> RusotoFuture<UpdateRecordsResponse, UpdateRecordsError> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
            dataset_name = input.dataset_name,
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        if let Some(ref client_context) = input.client_context {
            request.add_header("x-amz-Client-Context", &client_context.to_string());
        }

        self.client.sign_and_dispatch(request, |response| {
            if response.status.as_u16() == 200 {
                Box::new(response.buffer().from_err().map(|response| {
                    let mut body = response.body;

                    if body == b"null" {
                        body = b"{}".to_vec();
                    }

                    debug!("Response body: {:?}", body);
                    debug!("Response status: {}", response.status);
                    let result = serde_json::from_slice::<UpdateRecordsResponse>(&body).unwrap();

                    result
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(UpdateRecordsError::from_response(response))),
                )
            }
        })
    }
}

#[cfg(test)]
mod protocol_tests {}
