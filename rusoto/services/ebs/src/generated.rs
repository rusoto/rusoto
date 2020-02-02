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
use serde_json;
/// <p>A block of data in an Amazon Elastic Block Store snapshot.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct Block {
    /// <p>The block index.</p>
    #[serde(rename = "BlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<i64>,
    /// <p>The block token for the block index.</p>
    #[serde(rename = "BlockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_token: Option<String>,
}

/// <p>A block of data in an Amazon Elastic Block Store snapshot that is different from another snapshot of the same volume/snapshot lineage.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChangedBlock {
    /// <p>The block index.</p>
    #[serde(rename = "BlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<i64>,
    /// <p>The block token for the block index of the <code>first snapshot ID</code> specified in the <code>list changed blocks</code> operation. This value is absent if the first snapshot does not have the changed block that is on the second snapshot.</p>
    #[serde(rename = "FirstBlockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_block_token: Option<String>,
    /// <p>The block token for the block index of the <code>second snapshot ID</code> specified in the <code>list changed blocks</code> operation.</p>
    #[serde(rename = "SecondBlockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_block_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSnapshotBlockRequest {
    /// <p>The block index of the block from which to get data.</p> <p>Obtain the <code>block index</code> by running the <code>list changed blocks</code> or <code>list snapshot blocks</code> operations.</p>
    #[serde(rename = "BlockIndex")]
    pub block_index: i64,
    /// <p>The block token of the block from which to get data.</p> <p>Obtain the <code>block token</code> by running the <code>list changed blocks</code> or <code>list snapshot blocks</code> operations.</p>
    #[serde(rename = "BlockToken")]
    pub block_token: String,
    /// <p>The ID of the snapshot containing the block from which to get data.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct GetSnapshotBlockResponse {
    /// <p>The data content of the block.</p>
    pub block_data: Option<bytes::Bytes>,
    /// <p>The checksum generated for the block.</p>
    pub checksum: Option<String>,
    /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
    pub checksum_algorithm: Option<String>,
    /// <p>The size of the data in the block.</p>
    pub data_length: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChangedBlocksRequest {
    /// <p>The ID of the first snapshot to use for the comparison.</p>
    #[serde(rename = "FirstSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_snapshot_id: Option<String>,
    /// <p>The number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the second snapshot to use for the comparison.</p>
    #[serde(rename = "SecondSnapshotId")]
    pub second_snapshot_id: String,
    /// <p>The block index from which the comparison should start.</p> <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    #[serde(rename = "StartingBlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_block_index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListChangedBlocksResponse {
    /// <p>The size of the block.</p>
    #[serde(rename = "BlockSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
    /// <p>An array of objects containing information about the changed blocks.</p>
    #[serde(rename = "ChangedBlocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_blocks: Option<Vec<ChangedBlock>>,
    /// <p>The time when the <code>block token</code> expires.</p>
    #[serde(rename = "ExpiryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The size of the volume in GB.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListSnapshotBlocksRequest {
    /// <p>The number of results to return.</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The token to request the next page of results.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ID of the snapshot from which to get block indexes and block tokens.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
    /// <p>The block index from which the list should start. The list in the response will start from this block index or the next valid block index in the snapshot.</p>
    #[serde(rename = "StartingBlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_block_index: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListSnapshotBlocksResponse {
    /// <p>The size of the block.</p>
    #[serde(rename = "BlockSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
    /// <p>An array of objects containing information about the blocks.</p>
    #[serde(rename = "Blocks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<Block>>,
    /// <p>The time when the <code>block token</code> expires.</p>
    #[serde(rename = "ExpiryTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<f64>,
    /// <p>The token to use to retrieve the next page of results. This value is null when there are no more results to return.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The size of the volume in GB.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

/// Errors returned by GetSnapshotBlock
#[derive(Debug, PartialEq)]
pub enum GetSnapshotBlockError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl GetSnapshotBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSnapshotBlockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSnapshotBlockError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for GetSnapshotBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSnapshotBlockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSnapshotBlockError {}
/// Errors returned by ListChangedBlocks
#[derive(Debug, PartialEq)]
pub enum ListChangedBlocksError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl ListChangedBlocksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChangedBlocksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListChangedBlocksError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListChangedBlocksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListChangedBlocksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChangedBlocksError {}
/// Errors returned by ListSnapshotBlocks
#[derive(Debug, PartialEq)]
pub enum ListSnapshotBlocksError {
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
}

impl ListSnapshotBlocksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSnapshotBlocksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for ListSnapshotBlocksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSnapshotBlocksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSnapshotBlocksError {}
/// Trait representing the capabilities of the Amazon EBS API. Amazon EBS clients implement this trait.
#[async_trait]
pub trait Ebs {
    /// <p>Returns the data in a block in an Amazon Elastic Block Store snapshot.</p>
    async fn get_snapshot_block(
        &self,
        input: GetSnapshotBlockRequest,
    ) -> Result<GetSnapshotBlockResponse, RusotoError<GetSnapshotBlockError>>;

    /// <p>Returns the block indexes and block tokens for blocks that are different between two Amazon Elastic Block Store snapshots of the same volume/snapshot lineage.</p>
    async fn list_changed_blocks(
        &self,
        input: ListChangedBlocksRequest,
    ) -> Result<ListChangedBlocksResponse, RusotoError<ListChangedBlocksError>>;

    /// <p>Returns the block indexes and block tokens for blocks in an Amazon Elastic Block Store snapshot.</p>
    async fn list_snapshot_blocks(
        &self,
        input: ListSnapshotBlocksRequest,
    ) -> Result<ListSnapshotBlocksResponse, RusotoError<ListSnapshotBlocksError>>;
}
/// A client for the Amazon EBS API.
#[derive(Clone)]
pub struct EbsClient {
    client: Client,
    region: region::Region,
}

impl EbsClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> EbsClient {
        EbsClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> EbsClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        EbsClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> EbsClient {
        EbsClient { client, region }
    }
}

#[async_trait]
impl Ebs for EbsClient {
    /// <p>Returns the data in a block in an Amazon Elastic Block Store snapshot.</p>
    async fn get_snapshot_block(
        &self,
        input: GetSnapshotBlockRequest,
    ) -> Result<GetSnapshotBlockResponse, RusotoError<GetSnapshotBlockError>> {
        let request_uri = format!(
            "/snapshots/{snapshot_id}/blocks/{block_index}",
            block_index = input.block_index,
            snapshot_id = input.snapshot_id
        );

        let mut request = SignedRequest::new("GET", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        params.put("blockToken", &input.block_token);
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetSnapshotBlockResponse::default();
            result.block_data = Some(response.body);

            if let Some(checksum) = response.headers.get("x-amz-Checksum") {
                let value = checksum.to_owned();
                result.checksum = Some(value)
            };
            if let Some(checksum_algorithm) = response.headers.get("x-amz-Checksum-Algorithm") {
                let value = checksum_algorithm.to_owned();
                result.checksum_algorithm = Some(value)
            };
            if let Some(data_length) = response.headers.get("x-amz-Data-Length") {
                let value = data_length.to_owned();
                result.data_length = Some(value.parse::<i64>().unwrap())
            };

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSnapshotBlockError::from_response(response))
        }
    }

    /// <p>Returns the block indexes and block tokens for blocks that are different between two Amazon Elastic Block Store snapshots of the same volume/snapshot lineage.</p>
    async fn list_changed_blocks(
        &self,
        input: ListChangedBlocksRequest,
    ) -> Result<ListChangedBlocksResponse, RusotoError<ListChangedBlocksError>> {
        let request_uri = format!(
            "/snapshots/{second_snapshot_id}/changedblocks",
            second_snapshot_id = input.second_snapshot_id
        );

        let mut request = SignedRequest::new("GET", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.first_snapshot_id {
            params.put("firstSnapshotId", x);
        }
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("pageToken", x);
        }
        if let Some(ref x) = input.starting_block_index {
            params.put("startingBlockIndex", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListChangedBlocksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListChangedBlocksError::from_response(response))
        }
    }

    /// <p>Returns the block indexes and block tokens for blocks in an Amazon Elastic Block Store snapshot.</p>
    async fn list_snapshot_blocks(
        &self,
        input: ListSnapshotBlocksRequest,
    ) -> Result<ListSnapshotBlocksResponse, RusotoError<ListSnapshotBlocksError>> {
        let request_uri = format!(
            "/snapshots/{snapshot_id}/blocks",
            snapshot_id = input.snapshot_id
        );

        let mut request = SignedRequest::new("GET", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("maxResults", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("pageToken", x);
        }
        if let Some(ref x) = input.starting_block_index {
            params.put("startingBlockIndex", x);
        }
        request.set_params(params);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSnapshotBlocksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSnapshotBlocksError::from_response(response))
        }
    }
}
