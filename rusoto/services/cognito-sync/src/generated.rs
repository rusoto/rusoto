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

use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use serde_json;
/// <p>The input for the BulkPublish operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct BulkPublishRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the BulkPublish operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DeleteDatasetResponse {
    /// <p>A collection of data for an identity pool. An identity pool can have multiple datasets. A dataset is per identity and can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don&#39;t exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs.</p>
    #[serde(rename = "Dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

/// <p>A request for meta data about a dataset (creation date, number of records, size) by owner and dataset name.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeDatasetResponse {
    /// <p>Meta data for a collection of data for an identity. An identity can have multiple datasets. A dataset can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don&#39;t exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs.</p>
    #[serde(rename = "Dataset")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset: Option<Dataset>,
}

/// <p>A request for usage information about the identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeIdentityPoolUsageRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>Response to a successful DescribeIdentityPoolUsage request.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIdentityPoolUsageResponse {
    /// <p>Information about the usage of the identity pool.</p>
    #[serde(rename = "IdentityPoolUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_usage: Option<IdentityPoolUsage>,
}

/// <p>A request for information about the usage of an identity pool.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeIdentityUsageResponse {
    /// <p>Usage information for the identity.</p>
    #[serde(rename = "IdentityUsage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_usage: Option<IdentityUsage>,
}

/// <p>The input for the GetBulkPublishDetails operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBulkPublishDetailsRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the GetBulkPublishDetails operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetCognitoEventsRequest {
    /// <p>The Cognito Identity Pool ID for the request</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The response from the GetCognitoEvents request</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetCognitoEventsResponse {
    /// <p>The Cognito Events returned from the GetCognitoEvents request</p>
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The input for the GetIdentityPoolConfiguration operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetIdentityPoolConfigurationRequest {
    /// <p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. This is the ID of the pool for which to return a configuration.</p>
    #[serde(rename = "IdentityPoolId")]
    pub identity_pool_id: String,
}

/// <p>The output for the GetIdentityPoolConfiguration operation.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct RegisterDeviceResponse {
    /// <p>The unique ID generated for this device by Cognito.</p>
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

/// <p>A request to configure Cognito Events</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct SubscribeToDatasetResponse {}

/// <p>A request to UnsubscribeFromDataset.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UnsubscribeFromDatasetResponse {}

/// <p>A request to post updates to records or add and delete records for a dataset and user.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
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
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
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
}

impl BulkPublishError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<BulkPublishError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AlreadyStreamedException" => {
                    return RusotoError::Service(BulkPublishError::AlreadyStreamed(err.msg))
                }
                "DuplicateRequestException" => {
                    return RusotoError::Service(BulkPublishError::DuplicateRequest(err.msg))
                }
                "InternalErrorException" => {
                    return RusotoError::Service(BulkPublishError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(BulkPublishError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(BulkPublishError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(BulkPublishError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for BulkPublishError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BulkPublishError::AlreadyStreamed(ref cause) => write!(f, "{}", cause),
            BulkPublishError::DuplicateRequest(ref cause) => write!(f, "{}", cause),
            BulkPublishError::InternalError(ref cause) => write!(f, "{}", cause),
            BulkPublishError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            BulkPublishError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            BulkPublishError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for BulkPublishError {}
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
}

impl DeleteDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DeleteDatasetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteDatasetError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DeleteDatasetError::NotAuthorized(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(DeleteDatasetError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteDatasetError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DeleteDatasetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteDatasetError::InternalError(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteDatasetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteDatasetError {}
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
}

impl DescribeDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeDatasetError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeDatasetError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeDatasetError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeDatasetError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeDatasetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeDatasetError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeDatasetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeDatasetError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeDatasetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeDatasetError {}
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
}

impl DescribeIdentityPoolUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIdentityPoolUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityPoolUsageError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityPoolUsageError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityPoolUsageError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityPoolUsageError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityPoolUsageError::TooManyRequests(
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
impl fmt::Display for DescribeIdentityPoolUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIdentityPoolUsageError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeIdentityPoolUsageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeIdentityPoolUsageError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeIdentityPoolUsageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIdentityPoolUsageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIdentityPoolUsageError {}
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
}

impl DescribeIdentityUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeIdentityUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(DescribeIdentityUsageError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeIdentityUsageError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(DescribeIdentityUsageError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeIdentityUsageError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(DescribeIdentityUsageError::TooManyRequests(
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
impl fmt::Display for DescribeIdentityUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeIdentityUsageError::InternalError(ref cause) => write!(f, "{}", cause),
            DescribeIdentityUsageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeIdentityUsageError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            DescribeIdentityUsageError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DescribeIdentityUsageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeIdentityUsageError {}
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
}

impl GetBulkPublishDetailsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBulkPublishDetailsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetBulkPublishDetailsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetBulkPublishDetailsError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetBulkPublishDetailsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBulkPublishDetailsError::ResourceNotFound(
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
impl fmt::Display for GetBulkPublishDetailsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBulkPublishDetailsError::InternalError(ref cause) => write!(f, "{}", cause),
            GetBulkPublishDetailsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetBulkPublishDetailsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetBulkPublishDetailsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBulkPublishDetailsError {}
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
}

impl GetCognitoEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetCognitoEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetCognitoEventsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(GetCognitoEventsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetCognitoEventsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetCognitoEventsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(GetCognitoEventsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetCognitoEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetCognitoEventsError::InternalError(ref cause) => write!(f, "{}", cause),
            GetCognitoEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetCognitoEventsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetCognitoEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetCognitoEventsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetCognitoEventsError {}
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
}

impl GetIdentityPoolConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<GetIdentityPoolConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(GetIdentityPoolConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        GetIdentityPoolConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(GetIdentityPoolConfigurationError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        GetIdentityPoolConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        GetIdentityPoolConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetIdentityPoolConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetIdentityPoolConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            GetIdentityPoolConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityPoolConfigurationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            GetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            GetIdentityPoolConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetIdentityPoolConfigurationError {}
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
}

impl ListDatasetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListDatasetsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListDatasetsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListDatasetsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListDatasetsError::NotAuthorized(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListDatasetsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListDatasetsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListDatasetsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListDatasetsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListDatasetsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListDatasetsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListDatasetsError {}
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
}

impl ListIdentityPoolUsageError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListIdentityPoolUsageError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListIdentityPoolUsageError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListIdentityPoolUsageError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListIdentityPoolUsageError::NotAuthorized(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListIdentityPoolUsageError::TooManyRequests(
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
impl fmt::Display for ListIdentityPoolUsageError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListIdentityPoolUsageError::InternalError(ref cause) => write!(f, "{}", cause),
            ListIdentityPoolUsageError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListIdentityPoolUsageError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListIdentityPoolUsageError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListIdentityPoolUsageError {}
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
}

impl ListRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRecordsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(ListRecordsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(ListRecordsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(ListRecordsError::NotAuthorized(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(ListRecordsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListRecordsError::InternalError(ref cause) => write!(f, "{}", cause),
            ListRecordsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListRecordsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            ListRecordsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListRecordsError {}
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
}

impl RegisterDeviceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<RegisterDeviceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(RegisterDeviceError::InternalError(err.msg))
                }
                "InvalidConfigurationException" => {
                    return RusotoError::Service(RegisterDeviceError::InvalidConfiguration(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(RegisterDeviceError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(RegisterDeviceError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(RegisterDeviceError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(RegisterDeviceError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterDeviceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RegisterDeviceError::InternalError(ref cause) => write!(f, "{}", cause),
            RegisterDeviceError::InvalidConfiguration(ref cause) => write!(f, "{}", cause),
            RegisterDeviceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            RegisterDeviceError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            RegisterDeviceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            RegisterDeviceError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for RegisterDeviceError {}
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
}

impl SetCognitoEventsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetCognitoEventsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SetCognitoEventsError::InternalError(err.msg))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SetCognitoEventsError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetCognitoEventsError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SetCognitoEventsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SetCognitoEventsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetCognitoEventsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetCognitoEventsError::InternalError(ref cause) => write!(f, "{}", cause),
            SetCognitoEventsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SetCognitoEventsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetCognitoEventsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SetCognitoEventsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetCognitoEventsError {}
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
}

impl SetIdentityPoolConfigurationError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<SetIdentityPoolConfigurationError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ConcurrentModificationException" => {
                    return RusotoError::Service(
                        SetIdentityPoolConfigurationError::ConcurrentModification(err.msg),
                    )
                }
                "InternalErrorException" => {
                    return RusotoError::Service(SetIdentityPoolConfigurationError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(
                        SetIdentityPoolConfigurationError::InvalidParameter(err.msg),
                    )
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SetIdentityPoolConfigurationError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(
                        SetIdentityPoolConfigurationError::ResourceNotFound(err.msg),
                    )
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(
                        SetIdentityPoolConfigurationError::TooManyRequests(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetIdentityPoolConfigurationError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SetIdentityPoolConfigurationError::ConcurrentModification(ref cause) => {
                write!(f, "{}", cause)
            }
            SetIdentityPoolConfigurationError::InternalError(ref cause) => write!(f, "{}", cause),
            SetIdentityPoolConfigurationError::InvalidParameter(ref cause) => {
                write!(f, "{}", cause)
            }
            SetIdentityPoolConfigurationError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => {
                write!(f, "{}", cause)
            }
            SetIdentityPoolConfigurationError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SetIdentityPoolConfigurationError {}
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
}

impl SubscribeToDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubscribeToDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(SubscribeToDatasetError::InternalError(err.msg))
                }
                "InvalidConfigurationException" => {
                    return RusotoError::Service(SubscribeToDatasetError::InvalidConfiguration(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(SubscribeToDatasetError::InvalidParameter(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(SubscribeToDatasetError::NotAuthorized(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(SubscribeToDatasetError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(SubscribeToDatasetError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SubscribeToDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SubscribeToDatasetError::InternalError(ref cause) => write!(f, "{}", cause),
            SubscribeToDatasetError::InvalidConfiguration(ref cause) => write!(f, "{}", cause),
            SubscribeToDatasetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            SubscribeToDatasetError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            SubscribeToDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            SubscribeToDatasetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for SubscribeToDatasetError {}
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
}

impl UnsubscribeFromDatasetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnsubscribeFromDatasetError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::InternalError(
                        err.msg,
                    ))
                }
                "InvalidConfigurationException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::InvalidConfiguration(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::InvalidParameter(
                        err.msg,
                    ))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::NotAuthorized(
                        err.msg,
                    ))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::ResourceNotFound(
                        err.msg,
                    ))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UnsubscribeFromDatasetError::TooManyRequests(
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
impl fmt::Display for UnsubscribeFromDatasetError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UnsubscribeFromDatasetError::InternalError(ref cause) => write!(f, "{}", cause),
            UnsubscribeFromDatasetError::InvalidConfiguration(ref cause) => write!(f, "{}", cause),
            UnsubscribeFromDatasetError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UnsubscribeFromDatasetError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UnsubscribeFromDatasetError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UnsubscribeFromDatasetError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UnsubscribeFromDatasetError {}
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
}

impl UpdateRecordsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateRecordsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InternalErrorException" => {
                    return RusotoError::Service(UpdateRecordsError::InternalError(err.msg))
                }
                "InvalidLambdaFunctionOutputException" => {
                    return RusotoError::Service(UpdateRecordsError::InvalidLambdaFunctionOutput(
                        err.msg,
                    ))
                }
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateRecordsError::InvalidParameter(err.msg))
                }
                "LambdaThrottledException" => {
                    return RusotoError::Service(UpdateRecordsError::LambdaThrottled(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(UpdateRecordsError::LimitExceeded(err.msg))
                }
                "NotAuthorizedException" => {
                    return RusotoError::Service(UpdateRecordsError::NotAuthorized(err.msg))
                }
                "ResourceConflictException" => {
                    return RusotoError::Service(UpdateRecordsError::ResourceConflict(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateRecordsError::ResourceNotFound(err.msg))
                }
                "TooManyRequestsException" => {
                    return RusotoError::Service(UpdateRecordsError::TooManyRequests(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateRecordsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateRecordsError::InternalError(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::InvalidLambdaFunctionOutput(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::LambdaThrottled(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::NotAuthorized(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::ResourceConflict(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            UpdateRecordsError::TooManyRequests(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateRecordsError {}
/// Trait representing the capabilities of the Amazon Cognito Sync API. Amazon Cognito Sync clients implement this trait.
#[async_trait]
pub trait CognitoSync {
    /// <p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn bulk_publish(
        &self,
        input: BulkPublishRequest,
    ) -> Result<BulkPublishResponse, RusotoError<BulkPublishError>>;

    /// <p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<DeleteDatasetResponse, RusotoError<DeleteDatasetError>>;

    /// <p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>>;

    /// <p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn describe_identity_pool_usage(
        &self,
        input: DescribeIdentityPoolUsageRequest,
    ) -> Result<DescribeIdentityPoolUsageResponse, RusotoError<DescribeIdentityPoolUsageError>>;

    /// <p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn describe_identity_usage(
        &self,
        input: DescribeIdentityUsageRequest,
    ) -> Result<DescribeIdentityUsageResponse, RusotoError<DescribeIdentityUsageError>>;

    /// <p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_bulk_publish_details(
        &self,
        input: GetBulkPublishDetailsRequest,
    ) -> Result<GetBulkPublishDetailsResponse, RusotoError<GetBulkPublishDetailsError>>;

    /// <p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_cognito_events(
        &self,
        input: GetCognitoEventsRequest,
    ) -> Result<GetCognitoEventsResponse, RusotoError<GetCognitoEventsError>>;

    /// <p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_identity_pool_configuration(
        &self,
        input: GetIdentityPoolConfigurationRequest,
    ) -> Result<GetIdentityPoolConfigurationResponse, RusotoError<GetIdentityPoolConfigurationError>>;

    /// <p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>>;

    /// <p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>
    async fn list_identity_pool_usage(
        &self,
        input: ListIdentityPoolUsageRequest,
    ) -> Result<ListIdentityPoolUsageResponse, RusotoError<ListIdentityPoolUsageError>>;

    /// <p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    async fn list_records(
        &self,
        input: ListRecordsRequest,
    ) -> Result<ListRecordsResponse, RusotoError<ListRecordsError>>;

    /// <p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn register_device(
        &self,
        input: RegisterDeviceRequest,
    ) -> Result<RegisterDeviceResponse, RusotoError<RegisterDeviceError>>;

    /// <p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn set_cognito_events(
        &self,
        input: SetCognitoEventsRequest,
    ) -> Result<(), RusotoError<SetCognitoEventsError>>;

    /// <p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn set_identity_pool_configuration(
        &self,
        input: SetIdentityPoolConfigurationRequest,
    ) -> Result<SetIdentityPoolConfigurationResponse, RusotoError<SetIdentityPoolConfigurationError>>;

    /// <p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn subscribe_to_dataset(
        &self,
        input: SubscribeToDatasetRequest,
    ) -> Result<SubscribeToDatasetResponse, RusotoError<SubscribeToDatasetError>>;

    /// <p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn unsubscribe_from_dataset(
        &self,
        input: UnsubscribeFromDatasetRequest,
    ) -> Result<UnsubscribeFromDatasetResponse, RusotoError<UnsubscribeFromDatasetError>>;

    /// <p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn update_records(
        &self,
        input: UpdateRecordsRequest,
    ) -> Result<UpdateRecordsResponse, RusotoError<UpdateRecordsError>>;
}
/// A client for the Amazon Cognito Sync API.
#[derive(Clone)]
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
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> CognitoSyncClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        CognitoSyncClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> CognitoSyncClient {
        CognitoSyncClient { client, region }
    }
}

#[async_trait]
impl CognitoSync for CognitoSyncClient {
    /// <p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn bulk_publish(
        &self,
        input: BulkPublishRequest,
    ) -> Result<BulkPublishResponse, RusotoError<BulkPublishError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/bulkpublish",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<BulkPublishResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(BulkPublishError::from_response(response))
        }
    }

    /// <p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn delete_dataset(
        &self,
        input: DeleteDatasetRequest,
    ) -> Result<DeleteDatasetResponse, RusotoError<DeleteDatasetError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
            dataset_name = input.dataset_name,
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DeleteDatasetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteDatasetError::from_response(response))
        }
    }

    /// <p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    async fn describe_dataset(
        &self,
        input: DescribeDatasetRequest,
    ) -> Result<DescribeDatasetResponse, RusotoError<DescribeDatasetError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
            dataset_name = input.dataset_name,
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeDatasetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeDatasetError::from_response(response))
        }
    }

    /// <p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn describe_identity_pool_usage(
        &self,
        input: DescribeIdentityPoolUsageRequest,
    ) -> Result<DescribeIdentityPoolUsageResponse, RusotoError<DescribeIdentityPoolUsageError>>
    {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIdentityPoolUsageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIdentityPoolUsageError::from_response(response))
        }
    }

    /// <p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn describe_identity_usage(
        &self,
        input: DescribeIdentityUsageRequest,
    ) -> Result<DescribeIdentityUsageResponse, RusotoError<DescribeIdentityUsageError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identities/{identity_id}",
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeIdentityUsageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeIdentityUsageError::from_response(response))
        }
    }

    /// <p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_bulk_publish_details(
        &self,
        input: GetBulkPublishDetailsRequest,
    ) -> Result<GetBulkPublishDetailsResponse, RusotoError<GetBulkPublishDetailsError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/getBulkPublishDetails",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBulkPublishDetailsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBulkPublishDetailsError::from_response(response))
        }
    }

    /// <p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_cognito_events(
        &self,
        input: GetCognitoEventsRequest,
    ) -> Result<GetCognitoEventsResponse, RusotoError<GetCognitoEventsError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/events",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetCognitoEventsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetCognitoEventsError::from_response(response))
        }
    }

    /// <p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn get_identity_pool_configuration(
        &self,
        input: GetIdentityPoolConfigurationRequest,
    ) -> Result<GetIdentityPoolConfigurationResponse, RusotoError<GetIdentityPoolConfigurationError>>
    {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/configuration",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetIdentityPoolConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetIdentityPoolConfigurationError::from_response(response))
        }
    }

    /// <p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>
    async fn list_datasets(
        &self,
        input: ListDatasetsRequest,
    ) -> Result<ListDatasetsResponse, RusotoError<ListDatasetsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListDatasetsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListDatasetsError::from_response(response))
        }
    }

    /// <p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>
    async fn list_identity_pool_usage(
        &self,
        input: ListIdentityPoolUsageRequest,
    ) -> Result<ListIdentityPoolUsageResponse, RusotoError<ListIdentityPoolUsageError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListIdentityPoolUsageResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListIdentityPoolUsageError::from_response(response))
        }
    }

    /// <p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>
    async fn list_records(
        &self,
        input: ListRecordsRequest,
    ) -> Result<ListRecordsResponse, RusotoError<ListRecordsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListRecordsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListRecordsError::from_response(response))
        }
    }

    /// <p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn register_device(
        &self,
        input: RegisterDeviceRequest,
    ) -> Result<RegisterDeviceResponse, RusotoError<RegisterDeviceError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/identity/{identity_id}/device",
            identity_id = input.identity_id,
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<RegisterDeviceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(RegisterDeviceError::from_response(response))
        }
    }

    /// <p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn set_cognito_events(
        &self,
        input: SetCognitoEventsRequest,
    ) -> Result<(), RusotoError<SetCognitoEventsError>> {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/events",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SetCognitoEventsError::from_response(response))
        }
    }

    /// <p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>
    async fn set_identity_pool_configuration(
        &self,
        input: SetIdentityPoolConfigurationRequest,
    ) -> Result<SetIdentityPoolConfigurationResponse, RusotoError<SetIdentityPoolConfigurationError>>
    {
        let request_uri = format!(
            "/identitypools/{identity_pool_id}/configuration",
            identity_pool_id = input.identity_pool_id
        );

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SetIdentityPoolConfigurationResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SetIdentityPoolConfigurationError::from_response(response))
        }
    }

    /// <p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn subscribe_to_dataset(
        &self,
        input: SubscribeToDatasetRequest,
    ) -> Result<SubscribeToDatasetResponse, RusotoError<SubscribeToDatasetError>> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}", dataset_name = input.dataset_name, device_id = input.device_id, identity_id = input.identity_id, identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<SubscribeToDatasetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(SubscribeToDatasetError::from_response(response))
        }
    }

    /// <p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>
    async fn unsubscribe_from_dataset(
        &self,
        input: UnsubscribeFromDatasetRequest,
    ) -> Result<UnsubscribeFromDatasetResponse, RusotoError<UnsubscribeFromDatasetError>> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}", dataset_name = input.dataset_name, device_id = input.device_id, identity_id = input.identity_id, identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UnsubscribeFromDatasetResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UnsubscribeFromDatasetError::from_response(response))
        }
    }

    /// <p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>
    async fn update_records(
        &self,
        input: UpdateRecordsRequest,
    ) -> Result<UpdateRecordsResponse, RusotoError<UpdateRecordsError>> {
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

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 200 {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateRecordsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateRecordsError::from_response(response))
        }
    }
}
