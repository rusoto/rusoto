
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

use std::fmt;
use std::error::Error;
use std::io;
use std::io::Read;

use rusoto_core::region;
use rusoto_core::request::{DispatchSignedRequest, HttpDispatchError};
use rusoto_core::credential::{CredentialsError, ProvideAwsCredentials};

use serde_json;
use rusoto_core::param::{Params, ServiceParams};
use rusoto_core::signature::SignedRequest;
use serde_json::from_str;
use serde_json::Value as SerdeJsonValue;
#[doc="The input for the BulkPublish operation."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct BulkPublishRequest {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="The output for the BulkPublish operation."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct BulkPublishResponse {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum BulkPublishStatus {
    Failed,
    InProgress,
    NotStarted,
    Succeeded,
}

impl Into<String> for BulkPublishStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for BulkPublishStatus {
    fn into(self) -> &'static str {
        match self {
            BulkPublishStatus::Failed => "FAILED",
            BulkPublishStatus::InProgress => "IN_PROGRESS",
            BulkPublishStatus::NotStarted => "NOT_STARTED",
            BulkPublishStatus::Succeeded => "SUCCEEDED",
        }
    }
}

impl ::std::str::FromStr for BulkPublishStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FAILED" => Ok(BulkPublishStatus::Failed),
            "IN_PROGRESS" => Ok(BulkPublishStatus::InProgress),
            "NOT_STARTED" => Ok(BulkPublishStatus::NotStarted),
            "SUCCEEDED" => Ok(BulkPublishStatus::Succeeded),
            _ => Err(()),
        }
    }
}

#[doc="Configuration options for configure Cognito streams."]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct CognitoStreams {
    #[doc="The ARN of the role Amazon Cognito can assume in order to publish to the stream. This role must grant access to Amazon Cognito (cognito-sync) to invoke PutRecord on your Cognito stream."]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
    #[doc="The name of the Cognito stream to receive updates. This stream must be in the developers account and in the same region as the identity pool."]
    #[serde(rename="StreamName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub stream_name: Option<String>,
    #[doc="Status of the Cognito streams. Valid values are: <p>ENABLED - Streaming of updates to identity pool is enabled.</p> <p>DISABLED - Streaming of updates to identity pool is disabled. Bulk publish will also fail if StreamingStatus is DISABLED.</p>"]
    #[serde(rename="StreamingStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub streaming_status: Option<String>,
}

#[doc="A collection of data for an identity pool. An identity pool can have multiple datasets. A dataset is per identity and can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don't exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Dataset {
    #[doc="Date on which the dataset was created."]
    #[serde(rename="CreationDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub creation_date: Option<f64>,
    #[doc="Total size in bytes of the records in this dataset."]
    #[serde(rename="DataStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_storage: Option<i64>,
    #[doc="A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.' (dot)."]
    #[serde(rename="DatasetName")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset_name: Option<String>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_id: Option<String>,
    #[doc="The device that made the last change to this dataset."]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Date when the dataset was last modified."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="Number of records in this dataset."]
    #[serde(rename="NumRecords")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub num_records: Option<i64>,
}

#[doc="A request to delete the specific dataset."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DeleteDatasetRequest {
    #[doc="A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.' (dot)."]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="Response to a successful DeleteDataset request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DeleteDatasetResponse {
    #[doc="A collection of data for an identity pool. An identity pool can have multiple datasets. A dataset is per identity and can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don't exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs."]
    #[serde(rename="Dataset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[doc="A request for meta data about a dataset (creation date, number of records, size) by owner and dataset name."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeDatasetRequest {
    #[doc="A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.' (dot)."]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="Response to a successful DescribeDataset request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeDatasetResponse {
    #[doc="Meta data for a collection of data for an identity. An identity can have multiple datasets. A dataset can be general or associated with a particular entity in an application (like a saved game). Datasets are automatically created if they don't exist. Data is synced by dataset, and a dataset can hold up to 1MB of key-value pairs."]
    #[serde(rename="Dataset")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset: Option<Dataset>,
}

#[doc="A request for usage information about the identity pool."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeIdentityPoolUsageRequest {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="Response to a successful DescribeIdentityPoolUsage request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeIdentityPoolUsageResponse {
    #[doc="Information about the usage of the identity pool."]
    #[serde(rename="IdentityPoolUsage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_usage: Option<IdentityPoolUsage>,
}

#[doc="A request for information about the usage of an identity pool."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct DescribeIdentityUsageRequest {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="The response to a successful DescribeIdentityUsage request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct DescribeIdentityUsageResponse {
    #[doc="Usage information for the identity."]
    #[serde(rename="IdentityUsage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_usage: Option<IdentityUsage>,
}

#[doc="The input for the GetBulkPublishDetails operation."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetBulkPublishDetailsRequest {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="The output for the GetBulkPublishDetails operation."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetBulkPublishDetailsResponse {
    #[doc="If BulkPublishStatus is SUCCEEDED, the time the last bulk publish operation completed."]
    #[serde(rename="BulkPublishCompleteTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bulk_publish_complete_time: Option<f64>,
    #[doc="The date/time at which the last bulk publish was initiated."]
    #[serde(rename="BulkPublishStartTime")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bulk_publish_start_time: Option<f64>,
    #[doc="Status of the last bulk publish operation, valid values are: <p>NOT_STARTED - No bulk publish has been requested for this identity pool</p> <p>IN_PROGRESS - Data is being published to the configured stream</p> <p>SUCCEEDED - All data for the identity pool has been published to the configured stream</p> <p>FAILED - Some portion of the data has failed to publish, check FailureMessage for the cause.</p>"]
    #[serde(rename="BulkPublishStatus")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub bulk_publish_status: Option<String>,
    #[doc="If BulkPublishStatus is FAILED this field will contain the error message that caused the bulk publish to fail."]
    #[serde(rename="FailureMessage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub failure_message: Option<String>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
}

#[doc="<p>A request for a list of the configured Cognito Events</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetCognitoEventsRequest {
    #[doc="<p>The Cognito Identity Pool ID for the request</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="<p>The response from the GetCognitoEvents request</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetCognitoEventsResponse {
    #[doc="<p>The Cognito Events returned from the GetCognitoEvents request</p>"]
    #[serde(rename="Events")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub events: Option<::std::collections::HashMap<String, String>>,
}

#[doc="<p>The input for the GetIdentityPoolConfiguration operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct GetIdentityPoolConfigurationRequest {
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. This is the ID of the pool for which to return a configuration.</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="<p>The output for the GetIdentityPoolConfiguration operation.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct GetIdentityPoolConfigurationResponse {
    #[doc="Options to apply to this identity pool for Amazon Cognito streams."]
    #[serde(rename="CognitoStreams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito.</p>"]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[doc="<p>Options to apply to this identity pool for push synchronization.</p>"]
    #[serde(rename="PushSync")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push_sync: Option<PushSync>,
}

#[doc="Usage information for the identity pool."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IdentityPoolUsage {
    #[doc="Data storage information for the identity pool."]
    #[serde(rename="DataStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_storage: Option<i64>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[doc="Date on which the identity pool was last modified."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="Number of sync sessions for the identity pool."]
    #[serde(rename="SyncSessionsCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sync_sessions_count: Option<i64>,
}

#[doc="Usage information for the identity."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct IdentityUsage {
    #[doc="Total data storage for this identity."]
    #[serde(rename="DataStorage")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub data_storage: Option<i64>,
    #[doc="Number of datasets for the identity."]
    #[serde(rename="DatasetCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset_count: Option<i64>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_id: Option<String>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[doc="Date on which the identity was last modified."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
}

#[doc="Request for a list of datasets for an identity."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListDatasetsRequest {
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[doc="The maximum number of results to be returned."]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="Returned for a successful ListDatasets request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListDatasetsResponse {
    #[doc="Number of datasets returned."]
    #[serde(rename="Count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,
    #[doc="A set of datasets."]
    #[serde(rename="Datasets")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub datasets: Option<Vec<Dataset>>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="A request for usage information on an identity pool."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListIdentityPoolUsageRequest {
    #[doc="The maximum number of results to be returned."]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="Returned for a successful ListIdentityPoolUsage request."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListIdentityPoolUsageResponse {
    #[doc="Total number of identities for the identity pool."]
    #[serde(rename="Count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,
    #[doc="Usage information for the identity pools."]
    #[serde(rename="IdentityPoolUsages")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_usages: Option<Vec<IdentityPoolUsage>>,
    #[doc="The maximum number of results to be returned."]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
}

#[doc="A request for a list of records."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct ListRecordsRequest {
    #[doc="A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.' (dot)."]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[doc="The last server sync count for this record."]
    #[serde(rename="LastSyncCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_sync_count: Option<i64>,
    #[doc="The maximum number of results to be returned."]
    #[serde(rename="MaxResults")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub max_results: Option<i64>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="A token containing a session ID, identity ID, and expiration."]
    #[serde(rename="SyncSessionToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sync_session_token: Option<String>,
}

#[doc="Returned for a successful ListRecordsRequest."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct ListRecordsResponse {
    #[doc="Total number of records."]
    #[serde(rename="Count")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub count: Option<i64>,
    #[doc="A boolean value specifying whether to delete the dataset locally."]
    #[serde(rename="DatasetDeletedAfterRequestedSyncCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset_deleted_after_requested_sync_count: Option<bool>,
    #[doc="Indicates whether the dataset exists."]
    #[serde(rename="DatasetExists")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset_exists: Option<bool>,
    #[doc="Server sync count for this dataset."]
    #[serde(rename="DatasetSyncCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub dataset_sync_count: Option<i64>,
    #[doc="The user/device that made the last change to this record."]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="Names of merged datasets."]
    #[serde(rename="MergedDatasetNames")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub merged_dataset_names: Option<Vec<String>>,
    #[doc="A pagination token for obtaining the next page of results."]
    #[serde(rename="NextToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub next_token: Option<String>,
    #[doc="A list of all records."]
    #[serde(rename="Records")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub records: Option<Vec<Record>>,
    #[doc="A token containing a session ID, identity ID, and expiration."]
    #[serde(rename="SyncSessionToken")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sync_session_token: Option<String>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Operation {
    Remove,
    Replace,
}

impl Into<String> for Operation {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for Operation {
    fn into(self) -> &'static str {
        match self {
            Operation::Remove => "remove",
            Operation::Replace => "replace",
        }
    }
}

impl ::std::str::FromStr for Operation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "remove" => Ok(Operation::Remove),
            "replace" => Ok(Operation::Replace),
            _ => Err(()),
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum Platform {
    Adm,
    Apns,
    ApnsSandbox,
    Gcm,
}

impl Into<String> for Platform {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for Platform {
    fn into(self) -> &'static str {
        match self {
            Platform::Adm => "ADM",
            Platform::Apns => "APNS",
            Platform::ApnsSandbox => "APNS_SANDBOX",
            Platform::Gcm => "GCM",
        }
    }
}

impl ::std::str::FromStr for Platform {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ADM" => Ok(Platform::Adm),
            "APNS" => Ok(Platform::Apns),
            "APNS_SANDBOX" => Ok(Platform::ApnsSandbox),
            "GCM" => Ok(Platform::Gcm),
            _ => Err(()),
        }
    }
}

#[doc="<p>Configuration options to be applied to the identity pool.</p>"]
#[derive(Default,Debug,Clone,Serialize,Deserialize)]
pub struct PushSync {
    #[doc="<p>List of SNS platform application ARNs that could be used by clients.</p>"]
    #[serde(rename="ApplicationArns")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub application_arns: Option<Vec<String>>,
    #[doc="<p>A role configured to allow Cognito to call SNS on behalf of the developer.</p>"]
    #[serde(rename="RoleArn")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub role_arn: Option<String>,
}

#[doc="The basic data structure of a dataset."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct Record {
    #[doc="The last modified date of the client device."]
    #[serde(rename="DeviceLastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    #[doc="The key for the record."]
    #[serde(rename="Key")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub key: Option<String>,
    #[doc="The user/device that made the last change to this record."]
    #[serde(rename="LastModifiedBy")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_by: Option<String>,
    #[doc="The date on which the record was last modified."]
    #[serde(rename="LastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_modified_date: Option<f64>,
    #[doc="The server sync count for this record."]
    #[serde(rename="SyncCount")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub sync_count: Option<i64>,
    #[doc="The value for the record."]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[doc="An update operation for a record."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RecordPatch {
    #[doc="The last modified date of the client device."]
    #[serde(rename="DeviceLastModifiedDate")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_last_modified_date: Option<f64>,
    #[doc="The key associated with the record patch."]
    #[serde(rename="Key")]
    pub key: String,
    #[doc="An operation, either replace or remove."]
    #[serde(rename="Op")]
    pub op: String,
    #[doc="Last known server sync count for this record. Set to 0 if unknown."]
    #[serde(rename="SyncCount")]
    pub sync_count: i64,
    #[doc="The value associated with the record patch."]
    #[serde(rename="Value")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub value: Option<String>,
}

#[doc="<p>A request to RegisterDevice.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct RegisterDeviceRequest {
    #[doc="<p>The unique ID for this identity.</p>"]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. Here, the ID of the pool that the identity belongs to.</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[doc="<p>The SNS platform type (e.g. GCM, SDM, APNS, APNS_SANDBOX).</p>"]
    #[serde(rename="Platform")]
    pub platform: String,
    #[doc="<p>The push token.</p>"]
    #[serde(rename="Token")]
    pub token: String,
}

#[doc="<p>Response to a RegisterDevice request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct RegisterDeviceResponse {
    #[doc="<p>The unique ID generated for this device by Cognito.</p>"]
    #[serde(rename="DeviceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_id: Option<String>,
}

#[doc="<p>A request to configure Cognito Events\"</p>\""]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetCognitoEventsRequest {
    #[doc="<p>The events to configure</p>"]
    #[serde(rename="Events")]
    pub events: ::std::collections::HashMap<String, String>,
    #[doc="<p>The Cognito Identity Pool to use when configuring Cognito Events</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="<p>The input for the SetIdentityPoolConfiguration operation.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SetIdentityPoolConfigurationRequest {
    #[doc="Options to apply to this identity pool for Amazon Cognito streams."]
    #[serde(rename="CognitoStreams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. This is the ID of the pool to modify.</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[doc="<p>Options to apply to this identity pool for push synchronization.</p>"]
    #[serde(rename="PushSync")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push_sync: Option<PushSync>,
}

#[doc="<p>The output for the SetIdentityPoolConfiguration operation</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SetIdentityPoolConfigurationResponse {
    #[doc="Options to apply to this identity pool for Amazon Cognito streams."]
    #[serde(rename="CognitoStreams")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub cognito_streams: Option<CognitoStreams>,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito.</p>"]
    #[serde(rename="IdentityPoolId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[doc="<p>Options to apply to this identity pool for push synchronization.</p>"]
    #[serde(rename="PushSync")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub push_sync: Option<PushSync>,
}


#[allow(non_camel_case_types)]
#[derive(Clone,Debug,Eq,PartialEq)]
pub enum StreamingStatus {
    Disabled,
    Enabled,
}

impl Into<String> for StreamingStatus {
    fn into(self) -> String {
        let s: &'static str = self.into();
        s.to_owned()
    }
}

impl Into<&'static str> for StreamingStatus {
    fn into(self) -> &'static str {
        match self {
            StreamingStatus::Disabled => "DISABLED",
            StreamingStatus::Enabled => "ENABLED",
        }
    }
}

impl ::std::str::FromStr for StreamingStatus {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "DISABLED" => Ok(StreamingStatus::Disabled),
            "ENABLED" => Ok(StreamingStatus::Enabled),
            _ => Err(()),
        }
    }
}

#[doc="<p>A request to SubscribeToDatasetRequest.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct SubscribeToDatasetRequest {
    #[doc="<p>The name of the dataset to subcribe to.</p>"]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="<p>The unique ID generated for this device by Cognito.</p>"]
    #[serde(rename="DeviceId")]
    pub device_id: String,
    #[doc="<p>Unique ID for this identity.</p>"]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which the identity belongs.</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="<p>Response to a SubscribeToDataset request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct SubscribeToDatasetResponse;

#[doc="<p>A request to UnsubscribeFromDataset.</p>"]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UnsubscribeFromDatasetRequest {
    #[doc="<p>The name of the dataset from which to unsubcribe.</p>"]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="<p>The unique ID generated for this device by Cognito.</p>"]
    #[serde(rename="DeviceId")]
    pub device_id: String,
    #[doc="<p>Unique ID for this identity.</p>"]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="<p>A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. The ID of the pool to which this identity belongs.</p>"]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
}

#[doc="<p>Response to an UnsubscribeFromDataset request.</p>"]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UnsubscribeFromDatasetResponse;

#[doc="A request to post updates to records or add and delete records for a dataset and user."]
#[derive(Default,Debug,Clone,Serialize)]
pub struct UpdateRecordsRequest {
    #[doc="Intended to supply a device ID that will populate the lastModifiedBy field referenced in other methods. The ClientContext field is not yet implemented."]
    #[serde(rename="ClientContext")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub client_context: Option<String>,
    #[doc="A string of up to 128 characters. Allowed characters are a-z, A-Z, 0-9, '_' (underscore), '-' (dash), and '.' (dot)."]
    #[serde(rename="DatasetName")]
    pub dataset_name: String,
    #[doc="<p>The unique ID generated for this device by Cognito.</p>"]
    #[serde(rename="DeviceId")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub device_id: Option<String>,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityId")]
    pub identity_id: String,
    #[doc="A name-spaced GUID (for example, us-east-1:23EC4050-6AEA-7089-A2DD-08002EXAMPLE) created by Amazon Cognito. GUID generation is unique within a region."]
    #[serde(rename="IdentityPoolId")]
    pub identity_pool_id: String,
    #[doc="A list of patch operations."]
    #[serde(rename="RecordPatches")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub record_patches: Option<Vec<RecordPatch>>,
    #[doc="The SyncSessionToken returned by a previous call to ListRecords for this dataset and identity."]
    #[serde(rename="SyncSessionToken")]
    pub sync_session_token: String,
}

#[doc="Returned for a successful UpdateRecordsRequest."]
#[derive(Default,Debug,Clone,Deserialize)]
pub struct UpdateRecordsResponse {
    #[doc="A list of records that have been updated."]
    #[serde(rename="Records")]
    #[serde(skip_serializing_if="Option::is_none")]
    pub records: Option<Vec<Record>>,
}

/// Errors returned by BulkPublish
#[derive(Debug, PartialEq)]
pub enum BulkPublishError {
    ///An exception thrown when a bulk publish operation is requested less than 24 hours after a previous bulk publish operation completed successfully.
    AlreadyStreamed(String),
    ///An exception thrown when there is an IN_PROGRESS bulk publish operation for the given identity pool.
    DuplicateRequest(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl BulkPublishError {
    pub fn from_body(body: &str) -> BulkPublishError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "AlreadyStreamedException" => {
                        BulkPublishError::AlreadyStreamed(String::from(error_message))
                    }
                    "DuplicateRequestException" => {
                        BulkPublishError::DuplicateRequest(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        BulkPublishError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        BulkPublishError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        BulkPublishError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        BulkPublishError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        BulkPublishError::Validation(error_message.to_string())
                    }
                    _ => BulkPublishError::Unknown(String::from(body)),
                }
            }
            Err(_) => BulkPublishError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for BulkPublishError {
    fn from(err: serde_json::error::Error) -> BulkPublishError {
        BulkPublishError::Unknown(err.description().to_string())
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
            BulkPublishError::ResourceNotFound(ref cause) => cause,
            BulkPublishError::InvalidParameter(ref cause) => cause,
            BulkPublishError::NotAuthorized(ref cause) => cause,
            BulkPublishError::Validation(ref cause) => cause,
            BulkPublishError::Credentials(ref err) => err.description(),
            BulkPublishError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            BulkPublishError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteDataset
#[derive(Debug, PartialEq)]
pub enum DeleteDatasetError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if an update can't be applied because the resource was changed by another call and this would result in a conflict.
    ResourceConflict(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DeleteDatasetError {
    pub fn from_body(body: &str) -> DeleteDatasetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DeleteDatasetError::InternalError(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        DeleteDatasetError::ResourceConflict(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DeleteDatasetError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DeleteDatasetError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DeleteDatasetError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DeleteDatasetError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DeleteDatasetError::Validation(error_message.to_string())
                    }
                    _ => DeleteDatasetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DeleteDatasetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DeleteDatasetError {
    fn from(err: serde_json::error::Error) -> DeleteDatasetError {
        DeleteDatasetError::Unknown(err.description().to_string())
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
            DeleteDatasetError::ResourceConflict(ref cause) => cause,
            DeleteDatasetError::TooManyRequests(ref cause) => cause,
            DeleteDatasetError::ResourceNotFound(ref cause) => cause,
            DeleteDatasetError::InvalidParameter(ref cause) => cause,
            DeleteDatasetError::NotAuthorized(ref cause) => cause,
            DeleteDatasetError::Validation(ref cause) => cause,
            DeleteDatasetError::Credentials(ref err) => err.description(),
            DeleteDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DeleteDatasetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeDataset
#[derive(Debug, PartialEq)]
pub enum DescribeDatasetError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeDatasetError {
    pub fn from_body(body: &str) -> DescribeDatasetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeDatasetError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeDatasetError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeDatasetError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeDatasetError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeDatasetError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeDatasetError::Validation(error_message.to_string())
                    }
                    _ => DescribeDatasetError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeDatasetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeDatasetError {
    fn from(err: serde_json::error::Error) -> DescribeDatasetError {
        DescribeDatasetError::Unknown(err.description().to_string())
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
            DescribeDatasetError::TooManyRequests(ref cause) => cause,
            DescribeDatasetError::ResourceNotFound(ref cause) => cause,
            DescribeDatasetError::InvalidParameter(ref cause) => cause,
            DescribeDatasetError::NotAuthorized(ref cause) => cause,
            DescribeDatasetError::Validation(ref cause) => cause,
            DescribeDatasetError::Credentials(ref err) => err.description(),
            DescribeDatasetError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            DescribeDatasetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentityPoolUsage
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityPoolUsageError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeIdentityPoolUsageError {
    pub fn from_body(body: &str) -> DescribeIdentityPoolUsageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeIdentityPoolUsageError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeIdentityPoolUsageError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => DescribeIdentityPoolUsageError::ResourceNotFound(String::from(error_message)),
                    "InvalidParameterException" => DescribeIdentityPoolUsageError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => {
                        DescribeIdentityPoolUsageError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeIdentityPoolUsageError::Validation(error_message.to_string())
                    }
                    _ => DescribeIdentityPoolUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeIdentityPoolUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeIdentityPoolUsageError {
    fn from(err: serde_json::error::Error) -> DescribeIdentityPoolUsageError {
        DescribeIdentityPoolUsageError::Unknown(err.description().to_string())
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
            DescribeIdentityPoolUsageError::TooManyRequests(ref cause) => cause,
            DescribeIdentityPoolUsageError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityPoolUsageError::InvalidParameter(ref cause) => cause,
            DescribeIdentityPoolUsageError::NotAuthorized(ref cause) => cause,
            DescribeIdentityPoolUsageError::Validation(ref cause) => cause,
            DescribeIdentityPoolUsageError::Credentials(ref err) => err.description(),
            DescribeIdentityPoolUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIdentityPoolUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeIdentityUsage
#[derive(Debug, PartialEq)]
pub enum DescribeIdentityUsageError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl DescribeIdentityUsageError {
    pub fn from_body(body: &str) -> DescribeIdentityUsageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        DescribeIdentityUsageError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        DescribeIdentityUsageError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        DescribeIdentityUsageError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        DescribeIdentityUsageError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        DescribeIdentityUsageError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        DescribeIdentityUsageError::Validation(error_message.to_string())
                    }
                    _ => DescribeIdentityUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => DescribeIdentityUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for DescribeIdentityUsageError {
    fn from(err: serde_json::error::Error) -> DescribeIdentityUsageError {
        DescribeIdentityUsageError::Unknown(err.description().to_string())
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
            DescribeIdentityUsageError::TooManyRequests(ref cause) => cause,
            DescribeIdentityUsageError::ResourceNotFound(ref cause) => cause,
            DescribeIdentityUsageError::InvalidParameter(ref cause) => cause,
            DescribeIdentityUsageError::NotAuthorized(ref cause) => cause,
            DescribeIdentityUsageError::Validation(ref cause) => cause,
            DescribeIdentityUsageError::Credentials(ref err) => err.description(),
            DescribeIdentityUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            DescribeIdentityUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetBulkPublishDetails
#[derive(Debug, PartialEq)]
pub enum GetBulkPublishDetailsError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetBulkPublishDetailsError {
    pub fn from_body(body: &str) -> GetBulkPublishDetailsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetBulkPublishDetailsError::InternalError(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetBulkPublishDetailsError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetBulkPublishDetailsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetBulkPublishDetailsError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetBulkPublishDetailsError::Validation(error_message.to_string())
                    }
                    _ => GetBulkPublishDetailsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetBulkPublishDetailsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetBulkPublishDetailsError {
    fn from(err: serde_json::error::Error) -> GetBulkPublishDetailsError {
        GetBulkPublishDetailsError::Unknown(err.description().to_string())
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
            GetBulkPublishDetailsError::ResourceNotFound(ref cause) => cause,
            GetBulkPublishDetailsError::InvalidParameter(ref cause) => cause,
            GetBulkPublishDetailsError::NotAuthorized(ref cause) => cause,
            GetBulkPublishDetailsError::Validation(ref cause) => cause,
            GetBulkPublishDetailsError::Credentials(ref err) => err.description(),
            GetBulkPublishDetailsError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetBulkPublishDetailsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetCognitoEvents
#[derive(Debug, PartialEq)]
pub enum GetCognitoEventsError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetCognitoEventsError {
    pub fn from_body(body: &str) -> GetCognitoEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        GetCognitoEventsError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        GetCognitoEventsError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        GetCognitoEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        GetCognitoEventsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        GetCognitoEventsError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        GetCognitoEventsError::Validation(error_message.to_string())
                    }
                    _ => GetCognitoEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetCognitoEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetCognitoEventsError {
    fn from(err: serde_json::error::Error) -> GetCognitoEventsError {
        GetCognitoEventsError::Unknown(err.description().to_string())
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
            GetCognitoEventsError::TooManyRequests(ref cause) => cause,
            GetCognitoEventsError::ResourceNotFound(ref cause) => cause,
            GetCognitoEventsError::InvalidParameter(ref cause) => cause,
            GetCognitoEventsError::NotAuthorized(ref cause) => cause,
            GetCognitoEventsError::Validation(ref cause) => cause,
            GetCognitoEventsError::Credentials(ref err) => err.description(),
            GetCognitoEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            GetCognitoEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by GetIdentityPoolConfiguration
#[derive(Debug, PartialEq)]
pub enum GetIdentityPoolConfigurationError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl GetIdentityPoolConfigurationError {
    pub fn from_body(body: &str) -> GetIdentityPoolConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => GetIdentityPoolConfigurationError::InternalError(String::from(error_message)),
                    "TooManyRequestsException" => GetIdentityPoolConfigurationError::TooManyRequests(String::from(error_message)),
                    "ResourceNotFoundException" => GetIdentityPoolConfigurationError::ResourceNotFound(String::from(error_message)),
                    "InvalidParameterException" => GetIdentityPoolConfigurationError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => GetIdentityPoolConfigurationError::NotAuthorized(String::from(error_message)),
                    "ValidationException" => {
                        GetIdentityPoolConfigurationError::Validation(error_message.to_string())
                    }
                    _ => GetIdentityPoolConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => GetIdentityPoolConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for GetIdentityPoolConfigurationError {
    fn from(err: serde_json::error::Error) -> GetIdentityPoolConfigurationError {
        GetIdentityPoolConfigurationError::Unknown(err.description().to_string())
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
            GetIdentityPoolConfigurationError::TooManyRequests(ref cause) => cause,
            GetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => cause,
            GetIdentityPoolConfigurationError::InvalidParameter(ref cause) => cause,
            GetIdentityPoolConfigurationError::NotAuthorized(ref cause) => cause,
            GetIdentityPoolConfigurationError::Validation(ref cause) => cause,
            GetIdentityPoolConfigurationError::Credentials(ref err) => err.description(),
            GetIdentityPoolConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            GetIdentityPoolConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListDatasets
#[derive(Debug, PartialEq)]
pub enum ListDatasetsError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListDatasetsError {
    pub fn from_body(body: &str) -> ListDatasetsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListDatasetsError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListDatasetsError::TooManyRequests(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListDatasetsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListDatasetsError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListDatasetsError::Validation(error_message.to_string())
                    }
                    _ => ListDatasetsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListDatasetsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListDatasetsError {
    fn from(err: serde_json::error::Error) -> ListDatasetsError {
        ListDatasetsError::Unknown(err.description().to_string())
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
            ListDatasetsError::TooManyRequests(ref cause) => cause,
            ListDatasetsError::InvalidParameter(ref cause) => cause,
            ListDatasetsError::NotAuthorized(ref cause) => cause,
            ListDatasetsError::Validation(ref cause) => cause,
            ListDatasetsError::Credentials(ref err) => err.description(),
            ListDatasetsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListDatasetsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListIdentityPoolUsage
#[derive(Debug, PartialEq)]
pub enum ListIdentityPoolUsageError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListIdentityPoolUsageError {
    pub fn from_body(body: &str) -> ListIdentityPoolUsageError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListIdentityPoolUsageError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListIdentityPoolUsageError::TooManyRequests(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListIdentityPoolUsageError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListIdentityPoolUsageError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListIdentityPoolUsageError::Validation(error_message.to_string())
                    }
                    _ => ListIdentityPoolUsageError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListIdentityPoolUsageError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListIdentityPoolUsageError {
    fn from(err: serde_json::error::Error) -> ListIdentityPoolUsageError {
        ListIdentityPoolUsageError::Unknown(err.description().to_string())
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
            ListIdentityPoolUsageError::TooManyRequests(ref cause) => cause,
            ListIdentityPoolUsageError::InvalidParameter(ref cause) => cause,
            ListIdentityPoolUsageError::NotAuthorized(ref cause) => cause,
            ListIdentityPoolUsageError::Validation(ref cause) => cause,
            ListIdentityPoolUsageError::Credentials(ref err) => err.description(),
            ListIdentityPoolUsageError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            ListIdentityPoolUsageError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRecords
#[derive(Debug, PartialEq)]
pub enum ListRecordsError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl ListRecordsError {
    pub fn from_body(body: &str) -> ListRecordsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        ListRecordsError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        ListRecordsError::TooManyRequests(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        ListRecordsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        ListRecordsError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        ListRecordsError::Validation(error_message.to_string())
                    }
                    _ => ListRecordsError::Unknown(String::from(body)),
                }
            }
            Err(_) => ListRecordsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for ListRecordsError {
    fn from(err: serde_json::error::Error) -> ListRecordsError {
        ListRecordsError::Unknown(err.description().to_string())
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
            ListRecordsError::TooManyRequests(ref cause) => cause,
            ListRecordsError::InvalidParameter(ref cause) => cause,
            ListRecordsError::NotAuthorized(ref cause) => cause,
            ListRecordsError::Validation(ref cause) => cause,
            ListRecordsError::Credentials(ref err) => err.description(),
            ListRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            ListRecordsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterDevice
#[derive(Debug, PartialEq)]
pub enum RegisterDeviceError {
    ///
    InvalidConfiguration(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl RegisterDeviceError {
    pub fn from_body(body: &str) -> RegisterDeviceError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidConfigurationException" => {
                        RegisterDeviceError::InvalidConfiguration(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        RegisterDeviceError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        RegisterDeviceError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        RegisterDeviceError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        RegisterDeviceError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        RegisterDeviceError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        RegisterDeviceError::Validation(error_message.to_string())
                    }
                    _ => RegisterDeviceError::Unknown(String::from(body)),
                }
            }
            Err(_) => RegisterDeviceError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for RegisterDeviceError {
    fn from(err: serde_json::error::Error) -> RegisterDeviceError {
        RegisterDeviceError::Unknown(err.description().to_string())
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
            RegisterDeviceError::InvalidConfiguration(ref cause) => cause,
            RegisterDeviceError::InternalError(ref cause) => cause,
            RegisterDeviceError::TooManyRequests(ref cause) => cause,
            RegisterDeviceError::ResourceNotFound(ref cause) => cause,
            RegisterDeviceError::InvalidParameter(ref cause) => cause,
            RegisterDeviceError::NotAuthorized(ref cause) => cause,
            RegisterDeviceError::Validation(ref cause) => cause,
            RegisterDeviceError::Credentials(ref err) => err.description(),
            RegisterDeviceError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            RegisterDeviceError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetCognitoEvents
#[derive(Debug, PartialEq)]
pub enum SetCognitoEventsError {
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetCognitoEventsError {
    pub fn from_body(body: &str) -> SetCognitoEventsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InternalErrorException" => {
                        SetCognitoEventsError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        SetCognitoEventsError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SetCognitoEventsError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SetCognitoEventsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        SetCognitoEventsError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SetCognitoEventsError::Validation(error_message.to_string())
                    }
                    _ => SetCognitoEventsError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetCognitoEventsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetCognitoEventsError {
    fn from(err: serde_json::error::Error) -> SetCognitoEventsError {
        SetCognitoEventsError::Unknown(err.description().to_string())
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
            SetCognitoEventsError::TooManyRequests(ref cause) => cause,
            SetCognitoEventsError::ResourceNotFound(ref cause) => cause,
            SetCognitoEventsError::InvalidParameter(ref cause) => cause,
            SetCognitoEventsError::NotAuthorized(ref cause) => cause,
            SetCognitoEventsError::Validation(ref cause) => cause,
            SetCognitoEventsError::Credentials(ref err) => err.description(),
            SetCognitoEventsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            SetCognitoEventsError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SetIdentityPoolConfiguration
#[derive(Debug, PartialEq)]
pub enum SetIdentityPoolConfigurationError {
    ///<p>Thrown if there are parallel requests to modify a resource.</p>
    ConcurrentModification(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SetIdentityPoolConfigurationError {
    pub fn from_body(body: &str) -> SetIdentityPoolConfigurationError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "ConcurrentModificationException" => SetIdentityPoolConfigurationError::ConcurrentModification(String::from(error_message)),
                    "InternalErrorException" => SetIdentityPoolConfigurationError::InternalError(String::from(error_message)),
                    "TooManyRequestsException" => SetIdentityPoolConfigurationError::TooManyRequests(String::from(error_message)),
                    "ResourceNotFoundException" => SetIdentityPoolConfigurationError::ResourceNotFound(String::from(error_message)),
                    "InvalidParameterException" => SetIdentityPoolConfigurationError::InvalidParameter(String::from(error_message)),
                    "NotAuthorizedException" => SetIdentityPoolConfigurationError::NotAuthorized(String::from(error_message)),
                    "ValidationException" => {
                        SetIdentityPoolConfigurationError::Validation(error_message.to_string())
                    }
                    _ => SetIdentityPoolConfigurationError::Unknown(String::from(body)),
                }
            }
            Err(_) => SetIdentityPoolConfigurationError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SetIdentityPoolConfigurationError {
    fn from(err: serde_json::error::Error) -> SetIdentityPoolConfigurationError {
        SetIdentityPoolConfigurationError::Unknown(err.description().to_string())
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
            SetIdentityPoolConfigurationError::TooManyRequests(ref cause) => cause,
            SetIdentityPoolConfigurationError::ResourceNotFound(ref cause) => cause,
            SetIdentityPoolConfigurationError::InvalidParameter(ref cause) => cause,
            SetIdentityPoolConfigurationError::NotAuthorized(ref cause) => cause,
            SetIdentityPoolConfigurationError::Validation(ref cause) => cause,
            SetIdentityPoolConfigurationError::Credentials(ref err) => err.description(),
            SetIdentityPoolConfigurationError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SetIdentityPoolConfigurationError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by SubscribeToDataset
#[derive(Debug, PartialEq)]
pub enum SubscribeToDatasetError {
    ///
    InvalidConfiguration(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl SubscribeToDatasetError {
    pub fn from_body(body: &str) -> SubscribeToDatasetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidConfigurationException" => {
                        SubscribeToDatasetError::InvalidConfiguration(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        SubscribeToDatasetError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        SubscribeToDatasetError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        SubscribeToDatasetError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        SubscribeToDatasetError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        SubscribeToDatasetError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        SubscribeToDatasetError::Validation(error_message.to_string())
                    }
                    _ => SubscribeToDatasetError::Unknown(String::from(body)),
                }
            }
            Err(_) => SubscribeToDatasetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for SubscribeToDatasetError {
    fn from(err: serde_json::error::Error) -> SubscribeToDatasetError {
        SubscribeToDatasetError::Unknown(err.description().to_string())
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
            SubscribeToDatasetError::InvalidConfiguration(ref cause) => cause,
            SubscribeToDatasetError::InternalError(ref cause) => cause,
            SubscribeToDatasetError::TooManyRequests(ref cause) => cause,
            SubscribeToDatasetError::ResourceNotFound(ref cause) => cause,
            SubscribeToDatasetError::InvalidParameter(ref cause) => cause,
            SubscribeToDatasetError::NotAuthorized(ref cause) => cause,
            SubscribeToDatasetError::Validation(ref cause) => cause,
            SubscribeToDatasetError::Credentials(ref err) => err.description(),
            SubscribeToDatasetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            SubscribeToDatasetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UnsubscribeFromDataset
#[derive(Debug, PartialEq)]
pub enum UnsubscribeFromDatasetError {
    ///
    InvalidConfiguration(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UnsubscribeFromDatasetError {
    pub fn from_body(body: &str) -> UnsubscribeFromDatasetError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "InvalidConfigurationException" => UnsubscribeFromDatasetError::InvalidConfiguration(String::from(error_message)),
                    "InternalErrorException" => {
                        UnsubscribeFromDatasetError::InternalError(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UnsubscribeFromDatasetError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UnsubscribeFromDatasetError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UnsubscribeFromDatasetError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        UnsubscribeFromDatasetError::NotAuthorized(String::from(error_message))
                    }
                    "ValidationException" => {
                        UnsubscribeFromDatasetError::Validation(error_message.to_string())
                    }
                    _ => UnsubscribeFromDatasetError::Unknown(String::from(body)),
                }
            }
            Err(_) => UnsubscribeFromDatasetError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UnsubscribeFromDatasetError {
    fn from(err: serde_json::error::Error) -> UnsubscribeFromDatasetError {
        UnsubscribeFromDatasetError::Unknown(err.description().to_string())
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
            UnsubscribeFromDatasetError::InvalidConfiguration(ref cause) => cause,
            UnsubscribeFromDatasetError::InternalError(ref cause) => cause,
            UnsubscribeFromDatasetError::TooManyRequests(ref cause) => cause,
            UnsubscribeFromDatasetError::ResourceNotFound(ref cause) => cause,
            UnsubscribeFromDatasetError::InvalidParameter(ref cause) => cause,
            UnsubscribeFromDatasetError::NotAuthorized(ref cause) => cause,
            UnsubscribeFromDatasetError::Validation(ref cause) => cause,
            UnsubscribeFromDatasetError::Credentials(ref err) => err.description(),
            UnsubscribeFromDatasetError::HttpDispatch(ref dispatch_error) => {
                dispatch_error.description()
            }
            UnsubscribeFromDatasetError::Unknown(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateRecords
#[derive(Debug, PartialEq)]
pub enum UpdateRecordsError {
    ///<p>AWS Lambda throttled your account, please contact AWS Support</p>
    LambdaThrottled(String),
    ///<p>The AWS Lambda function returned invalid output or an exception.</p>
    InvalidLambdaFunctionOutput(String),
    ///Indicates an internal service error.
    InternalError(String),
    ///Thrown if an update can't be applied because the resource was changed by another call and this would result in a conflict.
    ResourceConflict(String),
    ///Thrown if the request is throttled.
    TooManyRequests(String),
    ///Thrown if the resource doesn't exist.
    ResourceNotFound(String),
    ///Thrown when a request parameter does not comply with the associated constraints.
    InvalidParameter(String),
    ///Thrown when a user is not authorized to access the requested resource.
    NotAuthorized(String),
    ///Thrown when the limit on the number of objects or operations has been exceeded.
    LimitExceeded(String),
    /// An error occurred dispatching the HTTP request
    HttpDispatch(HttpDispatchError),
    /// An error was encountered with AWS credentials.
    Credentials(CredentialsError),
    /// A validation error occurred.  Details from AWS are provided.
    Validation(String),
    /// An unknown error occurred.  The raw HTTP response is provided.
    Unknown(String),
}


impl UpdateRecordsError {
    pub fn from_body(body: &str) -> UpdateRecordsError {
        match from_str::<SerdeJsonValue>(body) {
            Ok(json) => {
                let raw_error_type = json.get("__type")
                    .and_then(|e| e.as_str())
                    .unwrap_or("Unknown");
                let error_message = json.get("message").and_then(|m| m.as_str()).unwrap_or(body);

                let pieces: Vec<&str> = raw_error_type.split("#").collect();
                let error_type = pieces.last().expect("Expected error type");

                match *error_type {
                    "LambdaThrottledException" => {
                        UpdateRecordsError::LambdaThrottled(String::from(error_message))
                    }
                    "InvalidLambdaFunctionOutputException" => {
                        UpdateRecordsError::InvalidLambdaFunctionOutput(String::from(error_message))
                    }
                    "InternalErrorException" => {
                        UpdateRecordsError::InternalError(String::from(error_message))
                    }
                    "ResourceConflictException" => {
                        UpdateRecordsError::ResourceConflict(String::from(error_message))
                    }
                    "TooManyRequestsException" => {
                        UpdateRecordsError::TooManyRequests(String::from(error_message))
                    }
                    "ResourceNotFoundException" => {
                        UpdateRecordsError::ResourceNotFound(String::from(error_message))
                    }
                    "InvalidParameterException" => {
                        UpdateRecordsError::InvalidParameter(String::from(error_message))
                    }
                    "NotAuthorizedException" => {
                        UpdateRecordsError::NotAuthorized(String::from(error_message))
                    }
                    "LimitExceededException" => {
                        UpdateRecordsError::LimitExceeded(String::from(error_message))
                    }
                    "ValidationException" => {
                        UpdateRecordsError::Validation(error_message.to_string())
                    }
                    _ => UpdateRecordsError::Unknown(String::from(body)),
                }
            }
            Err(_) => UpdateRecordsError::Unknown(String::from(body)),
        }
    }
}

impl From<serde_json::error::Error> for UpdateRecordsError {
    fn from(err: serde_json::error::Error) -> UpdateRecordsError {
        UpdateRecordsError::Unknown(err.description().to_string())
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
            UpdateRecordsError::LambdaThrottled(ref cause) => cause,
            UpdateRecordsError::InvalidLambdaFunctionOutput(ref cause) => cause,
            UpdateRecordsError::InternalError(ref cause) => cause,
            UpdateRecordsError::ResourceConflict(ref cause) => cause,
            UpdateRecordsError::TooManyRequests(ref cause) => cause,
            UpdateRecordsError::ResourceNotFound(ref cause) => cause,
            UpdateRecordsError::InvalidParameter(ref cause) => cause,
            UpdateRecordsError::NotAuthorized(ref cause) => cause,
            UpdateRecordsError::LimitExceeded(ref cause) => cause,
            UpdateRecordsError::Validation(ref cause) => cause,
            UpdateRecordsError::Credentials(ref err) => err.description(),
            UpdateRecordsError::HttpDispatch(ref dispatch_error) => dispatch_error.description(),
            UpdateRecordsError::Unknown(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Cognito Sync API. Amazon Cognito Sync clients implement this trait.
pub trait CognitoSync {
    #[doc="<p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn bulk_publish(&self,
                    input: &BulkPublishRequest)
                    -> Result<BulkPublishResponse, BulkPublishError>;


    #[doc="<p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn delete_dataset(&self,
                      input: &DeleteDatasetRequest)
                      -> Result<DeleteDatasetResponse, DeleteDatasetError>;


    #[doc="<p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>"]
    fn describe_dataset(&self,
                        input: &DescribeDatasetRequest)
                        -> Result<DescribeDatasetResponse, DescribeDatasetError>;


    #[doc="<p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn describe_identity_pool_usage
        (&self,
         input: &DescribeIdentityPoolUsageRequest)
         -> Result<DescribeIdentityPoolUsageResponse, DescribeIdentityPoolUsageError>;


    #[doc="<p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn describe_identity_usage
        (&self,
         input: &DescribeIdentityUsageRequest)
         -> Result<DescribeIdentityUsageResponse, DescribeIdentityUsageError>;


    #[doc="<p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_bulk_publish_details
        (&self,
         input: &GetBulkPublishDetailsRequest)
         -> Result<GetBulkPublishDetailsResponse, GetBulkPublishDetailsError>;


    #[doc="<p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_cognito_events(&self,
                          input: &GetCognitoEventsRequest)
                          -> Result<GetCognitoEventsResponse, GetCognitoEventsError>;


    #[doc="<p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_identity_pool_configuration
        (&self,
         input: &GetIdentityPoolConfigurationRequest)
         -> Result<GetIdentityPoolConfigurationResponse, GetIdentityPoolConfigurationError>;


    #[doc="<p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>"]
    fn list_datasets(&self,
                     input: &ListDatasetsRequest)
                     -> Result<ListDatasetsResponse, ListDatasetsError>;


    #[doc="<p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>"]
    fn list_identity_pool_usage
        (&self,
         input: &ListIdentityPoolUsageRequest)
         -> Result<ListIdentityPoolUsageResponse, ListIdentityPoolUsageError>;


    #[doc="<p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>"]
    fn list_records(&self,
                    input: &ListRecordsRequest)
                    -> Result<ListRecordsResponse, ListRecordsError>;


    #[doc="<p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn register_device(&self,
                       input: &RegisterDeviceRequest)
                       -> Result<RegisterDeviceResponse, RegisterDeviceError>;


    #[doc="<p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn set_cognito_events(&self,
                          input: &SetCognitoEventsRequest)
                          -> Result<(), SetCognitoEventsError>;


    #[doc="<p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn set_identity_pool_configuration
        (&self,
         input: &SetIdentityPoolConfigurationRequest)
         -> Result<SetIdentityPoolConfigurationResponse, SetIdentityPoolConfigurationError>;


    #[doc="<p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn subscribe_to_dataset(&self,
                            input: &SubscribeToDatasetRequest)
                            -> Result<SubscribeToDatasetResponse, SubscribeToDatasetError>;


    #[doc="<p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn unsubscribe_from_dataset
        (&self,
         input: &UnsubscribeFromDatasetRequest)
         -> Result<UnsubscribeFromDatasetResponse, UnsubscribeFromDatasetError>;


    #[doc="<p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn update_records(&self,
                      input: &UpdateRecordsRequest)
                      -> Result<UpdateRecordsResponse, UpdateRecordsError>;
}
/// A client for the Amazon Cognito Sync API.
pub struct CognitoSyncClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    credentials_provider: P,
    region: region::Region,
    dispatcher: D,
}

impl<P, D> CognitoSyncClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    pub fn new(request_dispatcher: D, credentials_provider: P, region: region::Region) -> Self {
        CognitoSyncClient {
            credentials_provider: credentials_provider,
            region: region,
            dispatcher: request_dispatcher,
        }
    }
}

impl<P, D> CognitoSync for CognitoSyncClient<P, D>
    where P: ProvideAwsCredentials,
          D: DispatchSignedRequest
{
    #[doc="<p>Initiates a bulk publish of all existing datasets for an Identity Pool to the configured stream. Customers are limited to one successful bulk publish per 24 hours. Bulk publish is an asynchronous request, customers can see the status of the request via the GetBulkPublishDetails operation.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn bulk_publish(&self,
                    input: &BulkPublishRequest)
                    -> Result<BulkPublishResponse, BulkPublishError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/bulkpublish",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<BulkPublishResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(BulkPublishError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Deletes the specific dataset. The dataset will be deleted permanently, and the action can't be undone. Datasets that this dataset was merged with will no longer report the merge. Any subsequent operation on this dataset will result in a ResourceNotFoundException.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn delete_dataset(&self,
                      input: &DeleteDatasetRequest)
                      -> Result<DeleteDatasetResponse, DeleteDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
                                  dataset_name = input.dataset_name,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DeleteDatasetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DeleteDatasetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets meta data about a dataset by identity and dataset name. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>"]
    fn describe_dataset(&self,
                        input: &DescribeDatasetRequest)
                        -> Result<DescribeDatasetResponse, DescribeDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
                                  dataset_name = input.dataset_name,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeDatasetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeDatasetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets usage details (for example, data storage) about a particular identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn describe_identity_pool_usage
        (&self,
         input: &DescribeIdentityPoolUsageRequest)
         -> Result<DescribeIdentityPoolUsageResponse, DescribeIdentityPoolUsageError> {
        let request_uri = format!("/identitypools/{identity_pool_id}",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeIdentityPoolUsageResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeIdentityPoolUsageError::from_body(String::from_utf8_lossy(&body)
                                                                  .as_ref()))
            }
        }
    }


    #[doc="<p>Gets usage information for an identity, including number of datasets and data usage.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn describe_identity_usage
        (&self,
         input: &DescribeIdentityUsageRequest)
         -> Result<DescribeIdentityUsageResponse, DescribeIdentityUsageError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}",
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<DescribeIdentityUsageResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(DescribeIdentityUsageError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Get the status of the last BulkPublish operation for an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_bulk_publish_details
        (&self,
         input: &GetBulkPublishDetailsRequest)
         -> Result<GetBulkPublishDetailsResponse, GetBulkPublishDetailsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/getBulkPublishDetails",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetBulkPublishDetailsResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetBulkPublishDetailsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets the events and the corresponding Lambda functions associated with an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_cognito_events(&self,
                          input: &GetCognitoEventsRequest)
                          -> Result<GetCognitoEventsResponse, GetCognitoEventsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/events",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetCognitoEventsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetCognitoEventsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets the configuration settings of an identity pool.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn get_identity_pool_configuration
        (&self,
         input: &GetIdentityPoolConfigurationRequest)
         -> Result<GetIdentityPoolConfigurationResponse, GetIdentityPoolConfigurationError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/configuration",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("GET", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<GetIdentityPoolConfigurationResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(GetIdentityPoolConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Lists datasets for an identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListDatasets can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use the Cognito Identity credentials to make this API call.</p>"]
    fn list_datasets(&self,
                     input: &ListDatasetsRequest)
                     -> Result<ListDatasetsResponse, ListDatasetsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets",
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

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

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListDatasetsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListDatasetsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets a list of identity pools registered with Cognito.</p> <p>ListIdentityPoolUsage can only be called with developer credentials. You cannot make this API call with the temporary user credentials provided by Cognito Identity.</p>"]
    fn list_identity_pool_usage
        (&self,
         input: &ListIdentityPoolUsageRequest)
         -> Result<ListIdentityPoolUsageResponse, ListIdentityPoolUsageError> {
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

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListIdentityPoolUsageResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListIdentityPoolUsageError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Gets paginated records, optionally changed after a particular sync count for a dataset and identity. With Amazon Cognito Sync, each identity has access only to its own data. Thus, the credentials used to make this API call need to have access to the identity data.</p> <p>ListRecords can be called with temporary user credentials provided by Cognito Identity or with developer credentials. You should use Cognito Identity credentials to make this API call.</p>"]
    fn list_records(&self,
                    input: &ListRecordsRequest)
                    -> Result<ListRecordsResponse, ListRecordsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/records",
                                  dataset_name = input.dataset_name,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

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

        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<ListRecordsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(ListRecordsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Registers a device to receive push sync notifications.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn register_device(&self,
                       input: &RegisterDeviceRequest)
                       -> Result<RegisterDeviceResponse, RegisterDeviceError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identity/{identity_id}/device",
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<RegisterDeviceResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(RegisterDeviceError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets the AWS Lambda function for a given event type for an identity pool. This request only updates the key/value pair specified. Other key/values pairs are not updated. To remove a key value pair, pass a empty value for the particular key.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn set_cognito_events(&self,
                          input: &SetCognitoEventsRequest)
                          -> Result<(), SetCognitoEventsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/events",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {
                let result = ();


                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetCognitoEventsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Sets the necessary configuration for push sync.</p> <p>This API can only be called with developer credentials. You cannot call this API with the temporary user credentials provided by Cognito Identity.</p>"]
    fn set_identity_pool_configuration
        (&self,
         input: &SetIdentityPoolConfigurationRequest)
         -> Result<SetIdentityPoolConfigurationResponse, SetIdentityPoolConfigurationError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/configuration",
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);



        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SetIdentityPoolConfigurationResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SetIdentityPoolConfigurationError::from_body(String::from_utf8_lossy(&body)
                                                                     .as_ref()))
            }
        }
    }


    #[doc="<p>Subscribes to receive notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn subscribe_to_dataset(&self,
                            input: &SubscribeToDatasetRequest)
                            -> Result<SubscribeToDatasetResponse, SubscribeToDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}",
                                  dataset_name = input.dataset_name,
                                  device_id = input.device_id,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<SubscribeToDatasetResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(SubscribeToDatasetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Unsubscribes from receiving notifications when a dataset is modified by another device.</p> <p>This API can only be called with temporary credentials provided by Cognito Identity. You cannot call this API with developer credentials.</p>"]
    fn unsubscribe_from_dataset
        (&self,
         input: &UnsubscribeFromDatasetRequest)
         -> Result<UnsubscribeFromDatasetResponse, UnsubscribeFromDatasetError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}/subscriptions/{device_id}",
                                  dataset_name = input.dataset_name,
                                  device_id = input.device_id,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("DELETE", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());






        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UnsubscribeFromDatasetResponse>(&body)
                    .unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UnsubscribeFromDatasetError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }


    #[doc="<p>Posts updates to records and adds and deletes records for a dataset and user.</p> <p>The sync count in the record patch is your last known sync count for that record. The server will reject an UpdateRecords request with a ResourceConflictException if you try to patch a record with a new value but a stale sync count.</p> <p>For example, if the sync count on the server is 5 for a key called highScore and you try and submit a new highScore with sync count of 4, the request will be rejected. To obtain the current sync count for a record, call ListRecords. On a successful update of the record, the response returns the new sync count for that record. You should present that sync count the next time you try to update that same record. When the record does not exist, specify the sync count as 0.</p> <p>This API can be called with temporary user credentials provided by Cognito Identity or with developer credentials.</p>"]
    fn update_records(&self,
                      input: &UpdateRecordsRequest)
                      -> Result<UpdateRecordsResponse, UpdateRecordsError> {
        let request_uri = format!("/identitypools/{identity_pool_id}/identities/{identity_id}/datasets/{dataset_name}",
                                  dataset_name = input.dataset_name,
                                  identity_id = input.identity_id,
                                  identity_pool_id = input.identity_pool_id);

        let mut request = SignedRequest::new("POST", "cognito-sync", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());


        let encoded = Some(serde_json::to_vec(input).unwrap());
        request.set_payload(encoded);

        if let Some(ref client_context) = input.client_context {
            request.add_header("x-amz-Client-Context", &client_context.to_string());
        }


        request.sign_with_plus(&self.credentials_provider.credentials()?, true);
        let mut response = self.dispatcher.dispatch(&request)?;

        match response.status {
            ::hyper::status::StatusCode::Ok => {

                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));

                if body == b"{}" {
                    body = b"null".to_vec();
                }

                debug!("Response body: {:?}", body);
                debug!("Response status: {}", response.status);
                let result = serde_json::from_slice::<UpdateRecordsResponse>(&body).unwrap();



                Ok(result)
            }
            _ => {
                let mut body: Vec<u8> = Vec::new();
                try!(response.body.read_to_end(&mut body));
                Err(UpdateRecordsError::from_body(String::from_utf8_lossy(&body).as_ref()))
            }
        }
    }
}

#[cfg(test)]
mod protocol_tests {}
