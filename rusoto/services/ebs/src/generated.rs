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
/// <p>A block of data in an Amazon Elastic Block Store snapshot.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ChangedBlock {
    /// <p>The block index.</p>
    #[serde(rename = "BlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_index: Option<i64>,
    /// <p>The block token for the block index of the <code>FirstSnapshotId</code> specified in the <code>ListChangedBlocks</code> operation. This value is absent if the first snapshot does not have the changed block that is on the second snapshot.</p>
    #[serde(rename = "FirstBlockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_block_token: Option<String>,
    /// <p>The block token for the block index of the <code>SecondSnapshotId</code> specified in the <code>ListChangedBlocks</code> operation.</p>
    #[serde(rename = "SecondBlockToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_block_token: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CompleteSnapshotRequest {
    /// <p>The number of blocks that were written to the snapshot.</p>
    #[serde(rename = "ChangedBlocksCount")]
    pub changed_blocks_count: i64,
    /// <p>An aggregated Base-64 SHA256 checksum based on the checksums of each written block.</p> <p>To generate the aggregated checksum using the linear aggregation method, arrange the checksums for each written block in ascending order of their block index, concatenate them to form a single string, and then generate the checksum on the entire string using the SHA256 algorithm.</p>
    #[serde(rename = "Checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The aggregation method used to generate the checksum. Currently, the only supported aggregation method is <code>LINEAR</code>.</p>
    #[serde(rename = "ChecksumAggregationMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_aggregation_method: Option<String>,
    /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm is <code>SHA256</code>.</p>
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
    /// <p>The ID of the snapshot.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CompleteSnapshotResponse {
    /// <p>The status of the snapshot.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetSnapshotBlockRequest {
    /// <p>The block index of the block from which to get data.</p> <p>Obtain the <code>BlockIndex</code> by running the <code>ListChangedBlocks</code> or <code>ListSnapshotBlocks</code> operations.</p>
    #[serde(rename = "BlockIndex")]
    pub block_index: i64,
    /// <p>The block token of the block from which to get data.</p> <p>Obtain the <code>BlockToken</code> by running the <code>ListChangedBlocks</code> or <code>ListSnapshotBlocks</code> operations.</p>
    #[serde(rename = "BlockToken")]
    pub block_token: String,
    /// <p>The ID of the snapshot containing the block from which to get data.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GetSnapshotBlockResponse {
    /// <p>The data content of the block.</p>
    pub block_data: Option<bytes::Bytes>,
    /// <p>The checksum generated for the block, which is Base64 encoded.</p>
    pub checksum: Option<String>,
    /// <p>The algorithm used to generate the checksum for the block, such as SHA256.</p>
    pub checksum_algorithm: Option<String>,
    /// <p>The size of the data in the block.</p>
    pub data_length: Option<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListChangedBlocksRequest {
    /// <p><p>The ID of the first snapshot to use for the comparison.</p> <important> <p>The <code>FirstSnapshotID</code> parameter must be specified with a <code>SecondSnapshotId</code> parameter; otherwise, an error occurs.</p> </important></p>
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
    /// <p><p>The ID of the second snapshot to use for the comparison.</p> <important> <p>The <code>SecondSnapshotId</code> parameter must be specified with a <code>FirstSnapshotID</code> parameter; otherwise, an error occurs.</p> </important></p>
    #[serde(rename = "SecondSnapshotId")]
    pub second_snapshot_id: String,
    /// <p>The block index from which the comparison should start.</p> <p>The list in the response will start from this block index or the next valid block index in the snapshots.</p>
    #[serde(rename = "StartingBlockIndex")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_block_index: Option<i64>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The time when the <code>BlockToken</code> expires.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
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

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
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
    /// <p>The time when the <code>BlockToken</code> expires.</p>
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

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct PutSnapshotBlockRequest {
    /// <p>The data to write to the block.</p> <p>The block data is not signed as part of the Signature Version 4 signing process. As a result, you must generate and provide a Base64-encoded SHA256 checksum for the block data using the <b>x-amz-Checksum</b> header. Also, you must specify the checksum algorithm using the <b>x-amz-Checksum-Algorithm</b> header. The checksum that you provide is part of the Signature Version 4 signing process. It is validated against a checksum generated by Amazon EBS to ensure the validity and authenticity of the data. If the checksums do not correspond, the request fails. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapis-using-checksums"> Using checksums with the EBS direct APIs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[serde(rename = "BlockData")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub block_data: bytes::Bytes,
    /// <p>The block index of the block in which to write the data. A block index is a logical index in units of <code>512</code> KiB blocks. To identify the block index, divide the logical offset of the data in the logical volume by the block size (logical offset of data/<code>524288</code>). The logical offset of the data must be <code>512</code> KiB aligned.</p>
    #[serde(rename = "BlockIndex")]
    pub block_index: i64,
    /// <p>A Base64-encoded SHA256 checksum of the data. Only SHA256 checksums are supported.</p>
    #[serde(rename = "Checksum")]
    pub checksum: String,
    /// <p>The algorithm used to generate the checksum. Currently, the only supported algorithm is <code>SHA256</code>.</p>
    #[serde(rename = "ChecksumAlgorithm")]
    pub checksum_algorithm: String,
    /// <p>The size of the data to write to the block, in bytes. Currently, the only supported size is <code>524288</code>.</p> <p>Valid values: <code>524288</code> </p>
    #[serde(rename = "DataLength")]
    pub data_length: i64,
    /// <p>The progress of the write process, as a percentage.</p>
    #[serde(rename = "Progress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /// <p>The ID of the snapshot.</p>
    #[serde(rename = "SnapshotId")]
    pub snapshot_id: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct PutSnapshotBlockResponse {
    /// <p>The SHA256 checksum generated for the block data by Amazon EBS.</p>
    #[serde(rename = "Checksum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// <p>The algorithm used by Amazon EBS to generate the checksum.</p>
    #[serde(rename = "ChecksumAlgorithm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum_algorithm: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct StartSnapshotRequest {
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. Idempotency ensures that an API request completes only once. With an idempotent request, if the original request completes successfully. The subsequent retries with the same client token return the result from the original successful request and they have no additional effect.</p> <p>If you do not specify a client token, one is automatically generated by the AWS SDK.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-direct-api-idempotency.html"> Idempotency for StartSnapshot API</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[serde(rename = "ClientToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// <p>A description for the snapshot.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>Indicates whether to encrypt the snapshot. To create an encrypted snapshot, specify <code>true</code>. To create an unencrypted snapshot, omit this parameter.</p> <p>If you specify a value for <b>ParentSnapshotId</b>, omit this parameter.</p> <p>If you specify <code>true</code>, the snapshot is encrypted using the CMK specified using the <b>KmsKeyArn</b> parameter. If no value is specified for <b>KmsKeyArn</b>, the default CMK for your account is used. If no default CMK has been specified for your account, the AWS managed CMK is used. To set a default CMK for your account, use <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_ModifyEbsDefaultKmsKeyId.html"> ModifyEbsDefaultKmsKeyId</a>.</p> <p>If your account is enabled for encryption by default, you cannot set this parameter to <code>false</code>. In this case, you can omit this parameter.</p> <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapis-using-encryption"> Using encryption</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[serde(rename = "Encrypted")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Key Management Service (AWS KMS) customer master key (CMK) to be used to encrypt the snapshot. If you do not specify a CMK, the default AWS managed CMK is used.</p> <p>If you specify a <b>ParentSnapshotId</b>, omit this parameter; the snapshot will be encrypted using the same CMK that was used to encrypt the parent snapshot.</p> <p>If <b>Encrypted</b> is set to <code>true</code>, you must specify a CMK ARN. </p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The ID of the parent snapshot. If there is no parent snapshot, or if you are creating the first snapshot for an on-premises volume, omit this parameter.</p> <p>If your account is enabled for encryption by default, you cannot use an unencrypted snapshot as a parent snapshot. You must first create an encrypted copy of the parent snapshot using <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/API_CopySnapshot.html">CopySnapshot</a>.</p>
    #[serde(rename = "ParentSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_snapshot_id: Option<String>,
    /// <p>The tags to apply to the snapshot.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The amount of time (in minutes) after which the snapshot is automatically cancelled if:</p> <ul> <li> <p>No blocks are written to the snapshot.</p> </li> <li> <p>The snapshot is not completed after writing the last block of data.</p> </li> </ul> <p>If no value is specified, the timeout defaults to <code>60</code> minutes.</p>
    #[serde(rename = "Timeout")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<i64>,
    /// <p>The size of the volume, in GiB. The maximum size is <code>16384</code> GiB (16 TiB).</p>
    #[serde(rename = "VolumeSize")]
    pub volume_size: i64,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct StartSnapshotResponse {
    /// <p>The size of the blocks in the snapshot, in bytes.</p>
    #[serde(rename = "BlockSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_size: Option<i64>,
    /// <p>The description of the snapshot.</p>
    #[serde(rename = "Description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The Amazon Resource Name (ARN) of the AWS Key Management Service (AWS KMS) customer master key (CMK) used to encrypt the snapshot.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The AWS account ID of the snapshot owner.</p>
    #[serde(rename = "OwnerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    /// <p>The ID of the parent snapshot.</p>
    #[serde(rename = "ParentSnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_snapshot_id: Option<String>,
    /// <p>The ID of the snapshot.</p>
    #[serde(rename = "SnapshotId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    /// <p>The timestamp when the snapshot was created.</p>
    #[serde(rename = "StartTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    /// <p>The status of the snapshot.</p>
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// <p>The tags applied to the snapshot. You can specify up to 50 tags per snapshot. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/Using_Tags.html"> Tagging your Amazon EC2 resources</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    /// <p>The size of the volume, in GiB.</p>
    #[serde(rename = "VolumeSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i64>,
}

/// <p>Describes a tag.</p>
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Tag {
    /// <p>The key of the tag.</p>
    #[serde(rename = "Key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The value of the tag.</p>
    #[serde(rename = "Value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// Errors returned by CompleteSnapshot
#[derive(Debug, PartialEq)]
pub enum CompleteSnapshotError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl CompleteSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CompleteSnapshotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CompleteSnapshotError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(CompleteSnapshotError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(CompleteSnapshotError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(CompleteSnapshotError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(CompleteSnapshotError::ServiceQuotaExceeded(
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
impl fmt::Display for CompleteSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CompleteSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            CompleteSnapshotError::InternalServer(ref cause) => write!(f, "{}", cause),
            CompleteSnapshotError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            CompleteSnapshotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            CompleteSnapshotError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CompleteSnapshotError {}
/// Errors returned by GetSnapshotBlock
#[derive(Debug, PartialEq)]
pub enum GetSnapshotBlockError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl GetSnapshotBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetSnapshotBlockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetSnapshotBlockError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(GetSnapshotBlockError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(GetSnapshotBlockError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetSnapshotBlockError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(GetSnapshotBlockError::ServiceQuotaExceeded(
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
impl fmt::Display for GetSnapshotBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetSnapshotBlockError::AccessDenied(ref cause) => write!(f, "{}", cause),
            GetSnapshotBlockError::InternalServer(ref cause) => write!(f, "{}", cause),
            GetSnapshotBlockError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            GetSnapshotBlockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetSnapshotBlockError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetSnapshotBlockError {}
/// Errors returned by ListChangedBlocks
#[derive(Debug, PartialEq)]
pub enum ListChangedBlocksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl ListChangedBlocksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListChangedBlocksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListChangedBlocksError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListChangedBlocksError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(ListChangedBlocksError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListChangedBlocksError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(ListChangedBlocksError::ServiceQuotaExceeded(
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
impl fmt::Display for ListChangedBlocksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListChangedBlocksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListChangedBlocksError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListChangedBlocksError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            ListChangedBlocksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListChangedBlocksError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListChangedBlocksError {}
/// Errors returned by ListSnapshotBlocks
#[derive(Debug, PartialEq)]
pub enum ListSnapshotBlocksError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl ListSnapshotBlocksError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListSnapshotBlocksError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(ListSnapshotBlocksError::ServiceQuotaExceeded(
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
impl fmt::Display for ListSnapshotBlocksError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListSnapshotBlocksError::AccessDenied(ref cause) => write!(f, "{}", cause),
            ListSnapshotBlocksError::InternalServer(ref cause) => write!(f, "{}", cause),
            ListSnapshotBlocksError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            ListSnapshotBlocksError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ListSnapshotBlocksError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListSnapshotBlocksError {}
/// Errors returned by PutSnapshotBlock
#[derive(Debug, PartialEq)]
pub enum PutSnapshotBlockError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl PutSnapshotBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PutSnapshotBlockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PutSnapshotBlockError::AccessDenied(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(PutSnapshotBlockError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(PutSnapshotBlockError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(PutSnapshotBlockError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(PutSnapshotBlockError::ServiceQuotaExceeded(
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
impl fmt::Display for PutSnapshotBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PutSnapshotBlockError::AccessDenied(ref cause) => write!(f, "{}", cause),
            PutSnapshotBlockError::InternalServer(ref cause) => write!(f, "{}", cause),
            PutSnapshotBlockError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            PutSnapshotBlockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            PutSnapshotBlockError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for PutSnapshotBlockError {}
/// Errors returned by StartSnapshot
#[derive(Debug, PartialEq)]
pub enum StartSnapshotError {
    /// <p>You do not have sufficient access to perform this action.</p>
    AccessDenied(String),
    /// <p>You have reached the limit for concurrent API requests. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ebs-accessing-snapshot.html#ebsapi-performance">Optimizing performance of the EBS direct APIs</a> in the <i>Amazon Elastic Compute Cloud User Guide</i>.</p>
    ConcurrentLimitExceeded(String),
    /// <p>The request uses the same client token as a previous, but non-identical request.</p>
    Conflict(String),
    /// <p>An internal error has occurred.</p>
    InternalServer(String),
    /// <p>The number of API requests has exceed the maximum allowed API request throttling limit.</p>
    RequestThrottled(String),
    /// <p>The specified resource does not exist.</p>
    ResourceNotFound(String),
    /// <p>Your current service quotas do not allow you to perform this action.</p>
    ServiceQuotaExceeded(String),
}

impl StartSnapshotError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartSnapshotError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartSnapshotError::AccessDenied(err.msg))
                }
                "ConcurrentLimitExceededException" => {
                    return RusotoError::Service(StartSnapshotError::ConcurrentLimitExceeded(
                        err.msg,
                    ))
                }
                "ConflictException" => {
                    return RusotoError::Service(StartSnapshotError::Conflict(err.msg))
                }
                "InternalServerException" => {
                    return RusotoError::Service(StartSnapshotError::InternalServer(err.msg))
                }
                "RequestThrottledException" => {
                    return RusotoError::Service(StartSnapshotError::RequestThrottled(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(StartSnapshotError::ResourceNotFound(err.msg))
                }
                "ServiceQuotaExceededException" => {
                    return RusotoError::Service(StartSnapshotError::ServiceQuotaExceeded(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        RusotoError::Unknown(res)
    }
}
impl fmt::Display for StartSnapshotError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            StartSnapshotError::AccessDenied(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::ConcurrentLimitExceeded(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::Conflict(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::InternalServer(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::RequestThrottled(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            StartSnapshotError::ServiceQuotaExceeded(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for StartSnapshotError {}
/// Trait representing the capabilities of the Amazon EBS API. Amazon EBS clients implement this trait.
#[async_trait]
pub trait Ebs {
    /// <p>Seals and completes the snapshot after all of the required blocks of data have been written to it. Completing the snapshot changes the status to <code>completed</code>. You cannot write new blocks to a snapshot after it has been completed.</p>
    async fn complete_snapshot(
        &self,
        input: CompleteSnapshotRequest,
    ) -> Result<CompleteSnapshotResponse, RusotoError<CompleteSnapshotError>>;

    /// <p>Returns the data in a block in an Amazon Elastic Block Store snapshot.</p>
    async fn get_snapshot_block(
        &self,
        input: GetSnapshotBlockRequest,
    ) -> Result<GetSnapshotBlockResponse, RusotoError<GetSnapshotBlockError>>;

    /// <p>Returns information about the blocks that are different between two Amazon Elastic Block Store snapshots of the same volume/snapshot lineage.</p>
    async fn list_changed_blocks(
        &self,
        input: ListChangedBlocksRequest,
    ) -> Result<ListChangedBlocksResponse, RusotoError<ListChangedBlocksError>>;

    /// <p>Returns information about the blocks in an Amazon Elastic Block Store snapshot.</p>
    async fn list_snapshot_blocks(
        &self,
        input: ListSnapshotBlocksRequest,
    ) -> Result<ListSnapshotBlocksResponse, RusotoError<ListSnapshotBlocksError>>;

    /// <p>Writes a block of data to a snapshot. If the specified block contains data, the existing data is overwritten. The target snapshot must be in the <code>pending</code> state.</p> <p>Data written to a snapshot must be aligned with 512-byte sectors.</p>
    async fn put_snapshot_block(
        &self,
        input: PutSnapshotBlockRequest,
    ) -> Result<PutSnapshotBlockResponse, RusotoError<PutSnapshotBlockError>>;

    /// <p>Creates a new Amazon EBS snapshot. The new snapshot enters the <code>pending</code> state after the request completes. </p> <p>After creating the snapshot, use <a href="https://docs.aws.amazon.com/ebs/latest/APIReference/API_PutSnapshotBlock.html"> PutSnapshotBlock</a> to write blocks of data to the snapshot.</p>
    async fn start_snapshot(
        &self,
        input: StartSnapshotRequest,
    ) -> Result<StartSnapshotResponse, RusotoError<StartSnapshotError>>;
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
    /// <p>Seals and completes the snapshot after all of the required blocks of data have been written to it. Completing the snapshot changes the status to <code>completed</code>. You cannot write new blocks to a snapshot after it has been completed.</p>
    #[allow(unused_mut)]
    async fn complete_snapshot(
        &self,
        input: CompleteSnapshotRequest,
    ) -> Result<CompleteSnapshotResponse, RusotoError<CompleteSnapshotError>> {
        let request_uri = format!(
            "/snapshots/completion/{snapshot_id}",
            snapshot_id = input.snapshot_id
        );

        let mut request = SignedRequest::new("POST", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        request.add_header(
            "x-amz-ChangedBlocksCount",
            &input.changed_blocks_count.to_string(),
        );
        request.add_optional_header("x-amz-Checksum", input.checksum.as_ref());
        request.add_optional_header(
            "x-amz-Checksum-Aggregation-Method",
            input.checksum_aggregation_method.as_ref(),
        );
        request.add_optional_header(
            "x-amz-Checksum-Algorithm",
            input.checksum_algorithm.as_ref(),
        );

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 202 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CompleteSnapshotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CompleteSnapshotError::from_response(response))
        }
    }

    /// <p>Returns the data in a block in an Amazon Elastic Block Store snapshot.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;

            let mut result = GetSnapshotBlockResponse::default();
            result.block_data = Some(response.body);

            result.checksum = response.headers.remove("x-amz-Checksum");
            result.checksum_algorithm = response.headers.remove("x-amz-Checksum-Algorithm");
            result.data_length = response
                .headers
                .remove("x-amz-Data-Length")
                .map(|value| value.parse::<i64>().unwrap());

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetSnapshotBlockError::from_response(response))
        }
    }

    /// <p>Returns information about the blocks that are different between two Amazon Elastic Block Store snapshots of the same volume/snapshot lineage.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListChangedBlocksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListChangedBlocksError::from_response(response))
        }
    }

    /// <p>Returns information about the blocks in an Amazon Elastic Block Store snapshot.</p>
    #[allow(unused_mut)]
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
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListSnapshotBlocksResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListSnapshotBlocksError::from_response(response))
        }
    }

    /// <p>Writes a block of data to a snapshot. If the specified block contains data, the existing data is overwritten. The target snapshot must be in the <code>pending</code> state.</p> <p>Data written to a snapshot must be aligned with 512-byte sectors.</p>
    #[allow(unused_mut)]
    async fn put_snapshot_block(
        &self,
        input: PutSnapshotBlockRequest,
    ) -> Result<PutSnapshotBlockResponse, RusotoError<PutSnapshotBlockError>> {
        let request_uri = format!(
            "/snapshots/{snapshot_id}/blocks/{block_index}",
            block_index = input.block_index,
            snapshot_id = input.snapshot_id
        );

        let mut request = SignedRequest::new("PUT", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(input.block_data.to_owned());
        request.set_payload(encoded);
        request.add_header("x-amz-Checksum", &input.checksum.to_string());
        request.add_header(
            "x-amz-Checksum-Algorithm",
            &input.checksum_algorithm.to_string(),
        );
        request.add_header("x-amz-Data-Length", &input.data_length.to_string());
        request.add_optional_header("x-amz-Progress", input.progress.as_ref());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let mut result = proto::json::ResponsePayload::new(&response)
                .deserialize::<PutSnapshotBlockResponse, _>()?;
            result.checksum = response.headers.remove("x-amz-Checksum");
            result.checksum_algorithm = response.headers.remove("x-amz-Checksum-Algorithm");

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(PutSnapshotBlockError::from_response(response))
        }
    }

    /// <p>Creates a new Amazon EBS snapshot. The new snapshot enters the <code>pending</code> state after the request completes. </p> <p>After creating the snapshot, use <a href="https://docs.aws.amazon.com/ebs/latest/APIReference/API_PutSnapshotBlock.html"> PutSnapshotBlock</a> to write blocks of data to the snapshot.</p>
    #[allow(unused_mut)]
    async fn start_snapshot(
        &self,
        input: StartSnapshotRequest,
    ) -> Result<StartSnapshotResponse, RusotoError<StartSnapshotError>> {
        let request_uri = "/snapshots";

        let mut request = SignedRequest::new("POST", "ebs", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.as_u16() == 201 {
            let mut response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<StartSnapshotResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(StartSnapshotError::from_response(response))
        }
    }
}
