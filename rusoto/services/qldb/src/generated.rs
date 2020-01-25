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
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct CreateLedgerRequest {
    /// <p>The flag that prevents a ledger from being deleted by any user. If not provided on ledger creation, this feature is enabled (<code>true</code>) by default.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The name of the ledger that you want to create. The name must be unique among all of your ledgers in the current AWS Region.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>The permissions mode to assign to the ledger that you want to create.</p>
    #[serde(rename = "PermissionsMode")]
    pub permissions_mode: String,
    /// <p>The key-value pairs to add as tags to the ledger that you want to create. Tag keys are case sensitive. Tag values are case sensitive and can be null.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct CreateLedgerResponse {
    /// <p>The Amazon Resource Name (ARN) for the ledger.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time, in epoch time format, when the ledger was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The flag that prevents a ledger from being deleted by any user. If not provided on ledger creation, this feature is enabled (<code>true</code>) by default.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the ledger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DeleteLedgerRequest {
    /// <p>The name of the ledger that you want to delete.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeJournalS3ExportRequest {
    /// <p>The unique ID of the journal export job that you want to describe.</p>
    #[serde(rename = "ExportId")]
    pub export_id: String,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeJournalS3ExportResponse {
    /// <p>Information about the journal export job returned by a <code>DescribeJournalS3Export</code> request.</p>
    #[serde(rename = "ExportDescription")]
    pub export_description: JournalS3ExportDescription,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct DescribeLedgerRequest {
    /// <p>The name of the ledger that you want to describe.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct DescribeLedgerResponse {
    /// <p>The Amazon Resource Name (ARN) for the ledger.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time, in epoch time format, when the ledger was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The flag that prevents a ledger from being deleted by any user. If not provided on ledger creation, this feature is enabled (<code>true</code>) by default.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the ledger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ExportJournalToS3Request {
    /// <p>The exclusive end date and time for the range of journal contents that you want to export.</p> <p>The <code>ExclusiveEndTime</code> must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code> </p> <p>The <code>ExclusiveEndTime</code> must be less than or equal to the current UTC date and time.</p>
    #[serde(rename = "ExclusiveEndTime")]
    pub exclusive_end_time: f64,
    /// <p>The inclusive start date and time for the range of journal contents that you want to export.</p> <p>The <code>InclusiveStartTime</code> must be in <code>ISO 8601</code> date and time format and in Universal Coordinated Time (UTC). For example: <code>2019-06-13T21:36:34Z</code> </p> <p>The <code>InclusiveStartTime</code> must be before <code>ExclusiveEndTime</code>.</p> <p>If you provide an <code>InclusiveStartTime</code> that is before the ledger's <code>CreationDateTime</code>, Amazon QLDB defaults it to the ledger's <code>CreationDateTime</code>.</p>
    #[serde(rename = "InclusiveStartTime")]
    pub inclusive_start_time: f64,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal export job to do the following:</p> <ul> <li> <p>Write objects into your Amazon Simple Storage Service (Amazon S3) bucket.</p> </li> <li> <p>(Optional) Use your customer master key (CMK) in AWS Key Management Service (AWS KMS) for server-side encryption of your exported data.</p> </li> </ul></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    /// <p>The configuration settings of the Amazon S3 bucket destination for your export request.</p>
    #[serde(rename = "S3ExportConfiguration")]
    pub s3_export_configuration: S3ExportConfiguration,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ExportJournalToS3Response {
    /// <p>The unique ID that QLDB assigns to each journal export job.</p> <p>To describe your export request and check the status of the job, you can use <code>ExportId</code> to call <code>DescribeJournalS3Export</code>.</p>
    #[serde(rename = "ExportId")]
    pub export_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBlockRequest {
    /// <p>The location of the block that you want to request. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p> <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code> </p>
    #[serde(rename = "BlockAddress")]
    pub block_address: ValueHolder,
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p> <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code> </p>
    #[serde(rename = "DigestTipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_tip_address: Option<ValueHolder>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetBlockResponse {
    /// <p>The block data object in Amazon Ion format.</p>
    #[serde(rename = "Block")]
    pub block: ValueHolder,
    /// <p>The proof object in Amazon Ion format returned by a <code>GetBlock</code> request. A proof contains the list of hash values required to recalculate the specified digest using a Merkle tree, starting with the specified block.</p>
    #[serde(rename = "Proof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<ValueHolder>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetDigestRequest {
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetDigestResponse {
    /// <p>The 256-bit hash value representing the digest returned by a <code>GetDigest</code> request.</p>
    #[serde(rename = "Digest")]
    #[serde(
        deserialize_with = "::rusoto_core::serialization::SerdeBlob::deserialize_blob",
        serialize_with = "::rusoto_core::serialization::SerdeBlob::serialize_blob",
        default
    )]
    pub digest: bytes::Bytes,
    /// <p>The latest block location covered by the digest that you requested. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p>
    #[serde(rename = "DigestTipAddress")]
    pub digest_tip_address: ValueHolder,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetRevisionRequest {
    /// <p>The block location of the document revision to be verified. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p> <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:14}</code> </p>
    #[serde(rename = "BlockAddress")]
    pub block_address: ValueHolder,
    /// <p>The latest block location covered by the digest for which to request a proof. An address is an Amazon Ion structure that has two fields: <code>strandId</code> and <code>sequenceNo</code>.</p> <p>For example: <code>{strandId:"BlFTjlSXze9BIh1KOszcE3",sequenceNo:49}</code> </p>
    #[serde(rename = "DigestTipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest_tip_address: Option<ValueHolder>,
    /// <p>The unique ID of the document to be verified.</p>
    #[serde(rename = "DocumentId")]
    pub document_id: String,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct GetRevisionResponse {
    /// <p>The proof object in Amazon Ion format returned by a <code>GetRevision</code> request. A proof contains the list of hash values that are required to recalculate the specified digest using a Merkle tree, starting with the specified document revision.</p>
    #[serde(rename = "Proof")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof: Option<ValueHolder>,
    /// <p>The document revision data object in Amazon Ion format.</p>
    #[serde(rename = "Revision")]
    pub revision: ValueHolder,
}

/// <p>The information about a journal export job, including the ledger name, export ID, when it was created, current status, and its start and end time export parameters.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct JournalS3ExportDescription {
    /// <p>The exclusive end date and time for the range of journal contents that are specified in the original export request.</p>
    #[serde(rename = "ExclusiveEndTime")]
    pub exclusive_end_time: f64,
    /// <p>The date and time, in epoch time format, when the export job was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    #[serde(rename = "ExportCreationTime")]
    pub export_creation_time: f64,
    /// <p>The unique ID of the journal export job.</p>
    #[serde(rename = "ExportId")]
    pub export_id: String,
    /// <p>The inclusive start date and time for the range of journal contents that are specified in the original export request.</p>
    #[serde(rename = "InclusiveStartTime")]
    pub inclusive_start_time: f64,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "LedgerName")]
    pub ledger_name: String,
    /// <p><p>The Amazon Resource Name (ARN) of the IAM role that grants QLDB permissions for a journal export job to do the following:</p> <ul> <li> <p>Write objects into your Amazon Simple Storage Service (Amazon S3) bucket.</p> </li> <li> <p>(Optional) Use your customer master key (CMK) in AWS Key Management Service (AWS KMS) for server-side encryption of your exported data.</p> </li> </ul></p>
    #[serde(rename = "RoleArn")]
    pub role_arn: String,
    #[serde(rename = "S3ExportConfiguration")]
    pub s3_export_configuration: S3ExportConfiguration,
    /// <p>The current state of the journal export job.</p>
    #[serde(rename = "Status")]
    pub status: String,
}

/// <p>Information about a ledger, including its name, state, and when it was created.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct LedgerSummary {
    /// <p>The date and time, in epoch time format, when the ledger was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the ledger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJournalS3ExportsForLedgerRequest {
    /// <p>The maximum number of results to return in a single <code>ListJournalS3ExportsForLedger</code> request. (The actual number of results returned might be fewer.)</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
    /// <p>A pagination token, indicating that you want to retrieve the next page of results. If you received a value for <code>NextToken</code> in the response from a previous <code>ListJournalS3ExportsForLedger</code> call, then you should use that value as input here.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJournalS3ExportsForLedgerResponse {
    /// <p>The array of journal export job descriptions that are associated with the specified ledger.</p>
    #[serde(rename = "JournalS3Exports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_s3_exports: Option<Vec<JournalS3ExportDescription>>,
    /// <ul> <li> <p>If <code>NextToken</code> is empty, then the last page of results has been processed and there are no more results to be retrieved.</p> </li> <li> <p>If <code>NextToken</code> is <i>not</i> empty, then there are more results available. To retrieve the next page of results, use the value of <code>NextToken</code> in a subsequent <code>ListJournalS3ExportsForLedger</code> call.</p> </li> </ul>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListJournalS3ExportsRequest {
    /// <p>The maximum number of results to return in a single <code>ListJournalS3Exports</code> request. (The actual number of results returned might be fewer.)</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token, indicating that you want to retrieve the next page of results. If you received a value for <code>NextToken</code> in the response from a previous <code>ListJournalS3Exports</code> call, then you should use that value as input here.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListJournalS3ExportsResponse {
    /// <p>The array of journal export job descriptions for all ledgers that are associated with the current AWS account and Region.</p>
    #[serde(rename = "JournalS3Exports")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub journal_s3_exports: Option<Vec<JournalS3ExportDescription>>,
    /// <ul> <li> <p>If <code>NextToken</code> is empty, then the last page of results has been processed and there are no more results to be retrieved.</p> </li> <li> <p>If <code>NextToken</code> is <i>not</i> empty, then there are more results available. To retrieve the next page of results, use the value of <code>NextToken</code> in a subsequent <code>ListJournalS3Exports</code> call.</p> </li> </ul>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListLedgersRequest {
    /// <p>The maximum number of results to return in a single <code>ListLedgers</code> request. (The actual number of results returned might be fewer.)</p>
    #[serde(rename = "MaxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>A pagination token, indicating that you want to retrieve the next page of results. If you received a value for <code>NextToken</code> in the response from a previous <code>ListLedgers</code> call, then you should use that value as input here.</p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListLedgersResponse {
    /// <p>The array of ledger summaries that are associated with the current AWS account and Region.</p>
    #[serde(rename = "Ledgers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledgers: Option<Vec<LedgerSummary>>,
    /// <p><p>A pagination token, indicating whether there are more results available:</p> <ul> <li> <p>If <code>NextToken</code> is empty, then the last page of results has been processed and there are no more results to be retrieved.</p> </li> <li> <p>If <code>NextToken</code> is <i>not</i> empty, then there are more results available. To retrieve the next page of results, use the value of <code>NextToken</code> in a subsequent <code>ListLedgers</code> call.</p> </li> </ul></p>
    #[serde(rename = "NextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct ListTagsForResourceRequest {
    /// <p>The Amazon Resource Name (ARN) for which you want to list the tags. For example:</p> <p> <code>arn:aws:qldb:us-east-1:123456789012:ledger/exampleLedger</code> </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>The tags that are currently associated with the specified Amazon QLDB resource.</p>
    #[serde(rename = "Tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<::std::collections::HashMap<String, String>>,
}

/// <p>The encryption settings that are used by a journal export job to write data in an Amazon Simple Storage Service (Amazon S3) bucket.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3EncryptionConfiguration {
    /// <p>The Amazon Resource Name (ARN) for a customer master key (CMK) in AWS Key Management Service (AWS KMS).</p> <p>You must provide a <code>KmsKeyArn</code> if you specify <code>SSE_KMS</code> as the <code>ObjectEncryptionType</code>.</p> <p> <code>KmsKeyArn</code> is not required if you specify <code>SSE_S3</code> as the <code>ObjectEncryptionType</code>.</p>
    #[serde(rename = "KmsKeyArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    /// <p>The Amazon S3 object encryption type.</p> <p>To learn more about server-side encryption options in Amazon S3, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/serv-side-encryption.html">Protecting Data Using Server-Side Encryption</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    #[serde(rename = "ObjectEncryptionType")]
    pub object_encryption_type: String,
}

/// <p>The Amazon Simple Storage Service (Amazon S3) bucket location in which a journal export job writes the journal contents.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct S3ExportConfiguration {
    /// <p>The Amazon S3 bucket name in which a journal export job writes the journal contents.</p> <p>The bucket name must comply with the Amazon S3 bucket naming conventions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/BucketRestrictions.html">Bucket Restrictions and Limitations</a> in the <i>Amazon S3 Developer Guide</i>.</p>
    #[serde(rename = "Bucket")]
    pub bucket: String,
    /// <p>The encryption settings that are used by a journal export job to write data in an Amazon S3 bucket.</p>
    #[serde(rename = "EncryptionConfiguration")]
    pub encryption_configuration: S3EncryptionConfiguration,
    /// <p><p>The prefix for the Amazon S3 bucket in which a journal export job writes the journal contents.</p> <p>The prefix must comply with Amazon S3 key naming rules and restrictions. For more information, see <a href="https://docs.aws.amazon.com/AmazonS3/latest/dev/UsingMetadata.html">Object Key and Metadata</a> in the <i>Amazon S3 Developer Guide</i>.</p> <p>The following are examples of valid <code>Prefix</code> values:</p> <ul> <li> <p> <code>JournalExports-ForMyLedger/Testing/</code> </p> </li> <li> <p> <code>JournalExports</code> </p> </li> <li> <p> <code>My:Tests/</code> </p> </li> </ul></p>
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct TagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) to which you want to add the tags. For example:</p> <p> <code>arn:aws:qldb:us-east-1:123456789012:ledger/exampleLedger</code> </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The key-value pairs to add as tags to the specified QLDB resource. Tag keys are case sensitive. If you specify a key that already exists for the resource, your request fails and returns an error. Tag values are case sensitive and can be null.</p>
    #[serde(rename = "Tags")]
    pub tags: ::std::collections::HashMap<String, String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct TagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UntagResourceRequest {
    /// <p>The Amazon Resource Name (ARN) from which you want to remove the tags. For example:</p> <p> <code>arn:aws:qldb:us-east-1:123456789012:ledger/exampleLedger</code> </p>
    #[serde(rename = "ResourceArn")]
    pub resource_arn: String,
    /// <p>The list of tag keys that you want to remove.</p>
    #[serde(rename = "TagKeys")]
    pub tag_keys: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UntagResourceResponse {}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct UpdateLedgerRequest {
    /// <p>The flag that prevents a ledger from being deleted by any user. If not provided on ledger creation, this feature is enabled (<code>true</code>) by default.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(any(test, feature = "serialize_structs"), derive(Serialize))]
pub struct UpdateLedgerResponse {
    /// <p>The Amazon Resource Name (ARN) for the ledger.</p>
    #[serde(rename = "Arn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    /// <p>The date and time, in epoch time format, when the ledger was created. (Epoch time format is the number of seconds elapsed since 12:00:00 AM January 1, 1970 UTC.)</p>
    #[serde(rename = "CreationDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date_time: Option<f64>,
    /// <p>The flag that prevents a ledger from being deleted by any user. If not provided on ledger creation, this feature is enabled (<code>true</code>) by default.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    #[serde(rename = "DeletionProtection")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    /// <p>The name of the ledger.</p>
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// <p>The current status of the ledger.</p>
    #[serde(rename = "State")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// <p>A structure that can contain an Amazon Ion value in multiple encoding formats.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueHolder {
    /// <p>An Amazon Ion plaintext value contained in a <code>ValueHolder</code> structure.</p>
    #[serde(rename = "IonText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ion_text: Option<String>,
}

/// Errors returned by CreateLedger
#[derive(Debug, PartialEq)]
pub enum CreateLedgerError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>You have reached the limit on the maximum number of resources allowed.</p>
    LimitExceeded(String),
    /// <p>The specified resource already exists.</p>
    ResourceAlreadyExists(String),
    /// <p>The specified resource can't be modified at this time.</p>
    ResourceInUse(String),
}

impl CreateLedgerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateLedgerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(CreateLedgerError::InvalidParameter(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateLedgerError::LimitExceeded(err.msg))
                }
                "ResourceAlreadyExistsException" => {
                    return RusotoError::Service(CreateLedgerError::ResourceAlreadyExists(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(CreateLedgerError::ResourceInUse(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateLedgerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CreateLedgerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            CreateLedgerError::LimitExceeded(ref cause) => write!(f, "{}", cause),
            CreateLedgerError::ResourceAlreadyExists(ref cause) => write!(f, "{}", cause),
            CreateLedgerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for CreateLedgerError {}
/// Errors returned by DeleteLedger
#[derive(Debug, PartialEq)]
pub enum DeleteLedgerError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource can't be modified at this time.</p>
    ResourceInUse(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMet(String),
}

impl DeleteLedgerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteLedgerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DeleteLedgerError::InvalidParameter(err.msg))
                }
                "ResourceInUseException" => {
                    return RusotoError::Service(DeleteLedgerError::ResourceInUse(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DeleteLedgerError::ResourceNotFound(err.msg))
                }
                "ResourcePreconditionNotMetException" => {
                    return RusotoError::Service(DeleteLedgerError::ResourcePreconditionNotMet(
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
impl fmt::Display for DeleteLedgerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeleteLedgerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DeleteLedgerError::ResourceInUse(ref cause) => write!(f, "{}", cause),
            DeleteLedgerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            DeleteLedgerError::ResourcePreconditionNotMet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DeleteLedgerError {}
/// Errors returned by DescribeJournalS3Export
#[derive(Debug, PartialEq)]
pub enum DescribeJournalS3ExportError {
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribeJournalS3ExportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeJournalS3ExportError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeJournalS3ExportError::ResourceNotFound(
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
impl fmt::Display for DescribeJournalS3ExportError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeJournalS3ExportError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeJournalS3ExportError {}
/// Errors returned by DescribeLedger
#[derive(Debug, PartialEq)]
pub enum DescribeLedgerError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl DescribeLedgerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeLedgerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(DescribeLedgerError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(DescribeLedgerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeLedgerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DescribeLedgerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            DescribeLedgerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for DescribeLedgerError {}
/// Errors returned by ExportJournalToS3
#[derive(Debug, PartialEq)]
pub enum ExportJournalToS3Error {
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMet(String),
}

impl ExportJournalToS3Error {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ExportJournalToS3Error> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ResourceNotFoundException" => {
                    return RusotoError::Service(ExportJournalToS3Error::ResourceNotFound(err.msg))
                }
                "ResourcePreconditionNotMetException" => {
                    return RusotoError::Service(
                        ExportJournalToS3Error::ResourcePreconditionNotMet(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ExportJournalToS3Error {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ExportJournalToS3Error::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            ExportJournalToS3Error::ResourcePreconditionNotMet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ExportJournalToS3Error {}
/// Errors returned by GetBlock
#[derive(Debug, PartialEq)]
pub enum GetBlockError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMet(String),
}

impl GetBlockError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetBlockError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetBlockError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetBlockError::ResourceNotFound(err.msg))
                }
                "ResourcePreconditionNotMetException" => {
                    return RusotoError::Service(GetBlockError::ResourcePreconditionNotMet(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetBlockError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetBlockError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetBlockError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetBlockError::ResourcePreconditionNotMet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetBlockError {}
/// Errors returned by GetDigest
#[derive(Debug, PartialEq)]
pub enum GetDigestError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMet(String),
}

impl GetDigestError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetDigestError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetDigestError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetDigestError::ResourceNotFound(err.msg))
                }
                "ResourcePreconditionNotMetException" => {
                    return RusotoError::Service(GetDigestError::ResourcePreconditionNotMet(
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
impl fmt::Display for GetDigestError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetDigestError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetDigestError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetDigestError::ResourcePreconditionNotMet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetDigestError {}
/// Errors returned by GetRevision
#[derive(Debug, PartialEq)]
pub enum GetRevisionError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
    /// <p>The operation failed because a condition wasn't satisfied in advance.</p>
    ResourcePreconditionNotMet(String),
}

impl GetRevisionError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetRevisionError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(GetRevisionError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(GetRevisionError::ResourceNotFound(err.msg))
                }
                "ResourcePreconditionNotMetException" => {
                    return RusotoError::Service(GetRevisionError::ResourcePreconditionNotMet(
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
impl fmt::Display for GetRevisionError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GetRevisionError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            GetRevisionError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
            GetRevisionError::ResourcePreconditionNotMet(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for GetRevisionError {}
/// Errors returned by ListJournalS3Exports
#[derive(Debug, PartialEq)]
pub enum ListJournalS3ExportsError {}

impl ListJournalS3ExportsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListJournalS3ExportsError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListJournalS3ExportsError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListJournalS3ExportsError {}
/// Errors returned by ListJournalS3ExportsForLedger
#[derive(Debug, PartialEq)]
pub enum ListJournalS3ExportsForLedgerError {}

impl ListJournalS3ExportsForLedgerError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<ListJournalS3ExportsForLedgerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListJournalS3ExportsForLedgerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListJournalS3ExportsForLedgerError {}
/// Errors returned by ListLedgers
#[derive(Debug, PartialEq)]
pub enum ListLedgersError {}

impl ListLedgersError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListLedgersError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListLedgersError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {}
    }
}
impl Error for ListLedgersError {}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidParameter(
                        err.msg,
                    ))
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
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ListTagsForResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            ListTagsForResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for ListTagsForResourceError {}
/// Errors returned by TagResource
#[derive(Debug, PartialEq)]
pub enum TagResourceError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl TagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<TagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(TagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(TagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for TagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            TagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for TagResourceError {}
/// Errors returned by UntagResource
#[derive(Debug, PartialEq)]
pub enum UntagResourceError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl UntagResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UntagResourceError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(UntagResourceError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UntagResourceError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UntagResourceError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UntagResourceError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UntagResourceError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UntagResourceError {}
/// Errors returned by UpdateLedger
#[derive(Debug, PartialEq)]
pub enum UpdateLedgerError {
    /// <p>One or more parameters in the request aren't valid.</p>
    InvalidParameter(String),
    /// <p>The specified resource doesn't exist.</p>
    ResourceNotFound(String),
}

impl UpdateLedgerError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateLedgerError> {
        if let Some(err) = proto::json::Error::parse_rest(&res) {
            match err.typ.as_str() {
                "InvalidParameterException" => {
                    return RusotoError::Service(UpdateLedgerError::InvalidParameter(err.msg))
                }
                "ResourceNotFoundException" => {
                    return RusotoError::Service(UpdateLedgerError::ResourceNotFound(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateLedgerError {
    #[allow(unused_variables)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            UpdateLedgerError::InvalidParameter(ref cause) => write!(f, "{}", cause),
            UpdateLedgerError::ResourceNotFound(ref cause) => write!(f, "{}", cause),
        }
    }
}
impl Error for UpdateLedgerError {}
/// Trait representing the capabilities of the QLDB API. QLDB clients implement this trait.
#[async_trait]
pub trait Qldb {
    /// <p>Creates a new ledger in your AWS account.</p>
    async fn create_ledger(
        &self,
        input: CreateLedgerRequest,
    ) -> Result<CreateLedgerResponse, RusotoError<CreateLedgerError>>;

    /// <p>Deletes a ledger and all of its contents. This action is irreversible.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    async fn delete_ledger(
        &self,
        input: DeleteLedgerRequest,
    ) -> Result<(), RusotoError<DeleteLedgerError>>;

    /// <p>Returns information about a journal export job, including the ledger name, export ID, when it was created, current status, and its start and end time export parameters.</p> <p>If the export job with the given <code>ExportId</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p> <p>If the ledger with the given <code>Name</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p>
    async fn describe_journal_s3_export(
        &self,
        input: DescribeJournalS3ExportRequest,
    ) -> Result<DescribeJournalS3ExportResponse, RusotoError<DescribeJournalS3ExportError>>;

    /// <p>Returns information about a ledger, including its state and when it was created.</p>
    async fn describe_ledger(
        &self,
        input: DescribeLedgerRequest,
    ) -> Result<DescribeLedgerResponse, RusotoError<DescribeLedgerError>>;

    /// <p>Exports journal contents within a date and time range from a ledger into a specified Amazon Simple Storage Service (Amazon S3) bucket. The data is written as files in Amazon Ion format.</p> <p>If the ledger with the given <code>Name</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p> <p>If the ledger with the given <code>Name</code> is in <code>CREATING</code> status, then throws <code>ResourcePreconditionNotMetException</code>.</p> <p>You can initiate up to two concurrent journal export requests for each ledger. Beyond this limit, journal export requests throw <code>LimitExceededException</code>.</p>
    async fn export_journal_to_s3(
        &self,
        input: ExportJournalToS3Request,
    ) -> Result<ExportJournalToS3Response, RusotoError<ExportJournalToS3Error>>;

    /// <p>Returns a journal block object at a specified address in a ledger. Also returns a proof of the specified block for verification if <code>DigestTipAddress</code> is provided.</p> <p>If the specified ledger doesn't exist or is in <code>DELETING</code> status, then throws <code>ResourceNotFoundException</code>.</p> <p>If the specified ledger is in <code>CREATING</code> status, then throws <code>ResourcePreconditionNotMetException</code>.</p> <p>If no block exists with the specified address, then throws <code>InvalidParameterException</code>.</p>
    async fn get_block(
        &self,
        input: GetBlockRequest,
    ) -> Result<GetBlockResponse, RusotoError<GetBlockError>>;

    /// <p>Returns the digest of a ledger at the latest committed block in the journal. The response includes a 256-bit hash value and a block address.</p>
    async fn get_digest(
        &self,
        input: GetDigestRequest,
    ) -> Result<GetDigestResponse, RusotoError<GetDigestError>>;

    /// <p>Returns a revision data object for a specified document ID and block address. Also returns a proof of the specified revision for verification if <code>DigestTipAddress</code> is provided.</p>
    async fn get_revision(
        &self,
        input: GetRevisionRequest,
    ) -> Result<GetRevisionResponse, RusotoError<GetRevisionError>>;

    /// <p>Returns an array of journal export job descriptions for all ledgers that are associated with the current AWS account and Region.</p> <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that you can retrieve all the items by calling <code>ListJournalS3Exports</code> multiple times.</p>
    async fn list_journal_s3_exports(
        &self,
        input: ListJournalS3ExportsRequest,
    ) -> Result<ListJournalS3ExportsResponse, RusotoError<ListJournalS3ExportsError>>;

    /// <p>Returns an array of journal export job descriptions for a specified ledger.</p> <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that you can retrieve all the items by calling <code>ListJournalS3ExportsForLedger</code> multiple times.</p>
    async fn list_journal_s3_exports_for_ledger(
        &self,
        input: ListJournalS3ExportsForLedgerRequest,
    ) -> Result<
        ListJournalS3ExportsForLedgerResponse,
        RusotoError<ListJournalS3ExportsForLedgerError>,
    >;

    /// <p>Returns an array of ledger summaries that are associated with the current AWS account and Region.</p> <p>This action returns a maximum of 100 items and is paginated so that you can retrieve all the items by calling <code>ListLedgers</code> multiple times.</p>
    async fn list_ledgers(
        &self,
        input: ListLedgersRequest,
    ) -> Result<ListLedgersResponse, RusotoError<ListLedgersError>>;

    /// <p>Returns all tags for a specified Amazon QLDB resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>>;

    /// <p>Adds one or more tags to a specified Amazon QLDB resource.</p> <p>A resource can have up to 50 tags. If you try to create more than 50 tags for a resource, your request fails and returns an error.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>>;

    /// <p>Removes one or more tags from a specified Amazon QLDB resource. You can specify up to 50 tag keys to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>>;

    /// <p>Updates properties on a ledger.</p>
    async fn update_ledger(
        &self,
        input: UpdateLedgerRequest,
    ) -> Result<UpdateLedgerResponse, RusotoError<UpdateLedgerError>>;
}
/// A client for the QLDB API.
#[derive(Clone)]
pub struct QldbClient {
    client: Client,
    region: region::Region,
}

impl QldbClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> QldbClient {
        QldbClient {
            client: Client::shared(),
            region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> QldbClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        D: DispatchSignedRequest + Send + Sync + 'static,
    {
        QldbClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region,
        }
    }

    pub fn new_with_client(client: Client, region: region::Region) -> QldbClient {
        QldbClient { client, region }
    }
}

#[async_trait]
impl Qldb for QldbClient {
    /// <p>Creates a new ledger in your AWS account.</p>
    async fn create_ledger(
        &self,
        input: CreateLedgerRequest,
    ) -> Result<CreateLedgerResponse, RusotoError<CreateLedgerError>> {
        let request_uri = "/ledgers";

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<CreateLedgerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(CreateLedgerError::from_response(response))
        }
    }

    /// <p>Deletes a ledger and all of its contents. This action is irreversible.</p> <p>If deletion protection is enabled, you must first disable it before you can delete the ledger using the QLDB API or the AWS Command Line Interface (AWS CLI). You can disable it by calling the <code>UpdateLedger</code> operation to set the flag to <code>false</code>. The QLDB console disables deletion protection for you when you use it to delete a ledger.</p>
    async fn delete_ledger(
        &self,
        input: DeleteLedgerRequest,
    ) -> Result<(), RusotoError<DeleteLedgerError>> {
        let request_uri = format!("/ledgers/{name}", name = input.name);

        let mut request = SignedRequest::new("DELETE", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = ::std::mem::drop(response);

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DeleteLedgerError::from_response(response))
        }
    }

    /// <p>Returns information about a journal export job, including the ledger name, export ID, when it was created, current status, and its start and end time export parameters.</p> <p>If the export job with the given <code>ExportId</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p> <p>If the ledger with the given <code>Name</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p>
    async fn describe_journal_s3_export(
        &self,
        input: DescribeJournalS3ExportRequest,
    ) -> Result<DescribeJournalS3ExportResponse, RusotoError<DescribeJournalS3ExportError>> {
        let request_uri = format!(
            "/ledgers/{name}/journal-s3-exports/{export_id}",
            export_id = input.export_id,
            name = input.name
        );

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeJournalS3ExportResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeJournalS3ExportError::from_response(response))
        }
    }

    /// <p>Returns information about a ledger, including its state and when it was created.</p>
    async fn describe_ledger(
        &self,
        input: DescribeLedgerRequest,
    ) -> Result<DescribeLedgerResponse, RusotoError<DescribeLedgerError>> {
        let request_uri = format!("/ledgers/{name}", name = input.name);

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<DescribeLedgerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(DescribeLedgerError::from_response(response))
        }
    }

    /// <p>Exports journal contents within a date and time range from a ledger into a specified Amazon Simple Storage Service (Amazon S3) bucket. The data is written as files in Amazon Ion format.</p> <p>If the ledger with the given <code>Name</code> doesn't exist, then throws <code>ResourceNotFoundException</code>.</p> <p>If the ledger with the given <code>Name</code> is in <code>CREATING</code> status, then throws <code>ResourcePreconditionNotMetException</code>.</p> <p>You can initiate up to two concurrent journal export requests for each ledger. Beyond this limit, journal export requests throw <code>LimitExceededException</code>.</p>
    async fn export_journal_to_s3(
        &self,
        input: ExportJournalToS3Request,
    ) -> Result<ExportJournalToS3Response, RusotoError<ExportJournalToS3Error>> {
        let request_uri = format!("/ledgers/{name}/journal-s3-exports", name = input.name);

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ExportJournalToS3Response, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ExportJournalToS3Error::from_response(response))
        }
    }

    /// <p>Returns a journal block object at a specified address in a ledger. Also returns a proof of the specified block for verification if <code>DigestTipAddress</code> is provided.</p> <p>If the specified ledger doesn't exist or is in <code>DELETING</code> status, then throws <code>ResourceNotFoundException</code>.</p> <p>If the specified ledger is in <code>CREATING</code> status, then throws <code>ResourcePreconditionNotMetException</code>.</p> <p>If no block exists with the specified address, then throws <code>InvalidParameterException</code>.</p>
    async fn get_block(
        &self,
        input: GetBlockRequest,
    ) -> Result<GetBlockResponse, RusotoError<GetBlockError>> {
        let request_uri = format!("/ledgers/{name}/block", name = input.name);

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetBlockResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetBlockError::from_response(response))
        }
    }

    /// <p>Returns the digest of a ledger at the latest committed block in the journal. The response includes a 256-bit hash value and a block address.</p>
    async fn get_digest(
        &self,
        input: GetDigestRequest,
    ) -> Result<GetDigestResponse, RusotoError<GetDigestError>> {
        let request_uri = format!("/ledgers/{name}/digest", name = input.name);

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetDigestResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetDigestError::from_response(response))
        }
    }

    /// <p>Returns a revision data object for a specified document ID and block address. Also returns a proof of the specified revision for verification if <code>DigestTipAddress</code> is provided.</p>
    async fn get_revision(
        &self,
        input: GetRevisionRequest,
    ) -> Result<GetRevisionResponse, RusotoError<GetRevisionError>> {
        let request_uri = format!("/ledgers/{name}/revision", name = input.name);

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<GetRevisionResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(GetRevisionError::from_response(response))
        }
    }

    /// <p>Returns an array of journal export job descriptions for all ledgers that are associated with the current AWS account and Region.</p> <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that you can retrieve all the items by calling <code>ListJournalS3Exports</code> multiple times.</p>
    async fn list_journal_s3_exports(
        &self,
        input: ListJournalS3ExportsRequest,
    ) -> Result<ListJournalS3ExportsResponse, RusotoError<ListJournalS3ExportsError>> {
        let request_uri = "/journal-s3-exports";

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
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
                .deserialize::<ListJournalS3ExportsResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJournalS3ExportsError::from_response(response))
        }
    }

    /// <p>Returns an array of journal export job descriptions for a specified ledger.</p> <p>This action returns a maximum of <code>MaxResults</code> items, and is paginated so that you can retrieve all the items by calling <code>ListJournalS3ExportsForLedger</code> multiple times.</p>
    async fn list_journal_s3_exports_for_ledger(
        &self,
        input: ListJournalS3ExportsForLedgerRequest,
    ) -> Result<
        ListJournalS3ExportsForLedgerResponse,
        RusotoError<ListJournalS3ExportsForLedgerError>,
    > {
        let request_uri = format!("/ledgers/{name}/journal-s3-exports", name = input.name);

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
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
                .deserialize::<ListJournalS3ExportsForLedgerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListJournalS3ExportsForLedgerError::from_response(response))
        }
    }

    /// <p>Returns an array of ledger summaries that are associated with the current AWS account and Region.</p> <p>This action returns a maximum of 100 items and is paginated so that you can retrieve all the items by calling <code>ListLedgers</code> multiple times.</p>
    async fn list_ledgers(
        &self,
        input: ListLedgersRequest,
    ) -> Result<ListLedgersResponse, RusotoError<ListLedgersError>> {
        let request_uri = "/ledgers";

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut params = Params::new();
        if let Some(ref x) = input.max_results {
            params.put("max_results", x);
        }
        if let Some(ref x) = input.next_token {
            params.put("next_token", x);
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
                .deserialize::<ListLedgersResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListLedgersError::from_response(response))
        }
    }

    /// <p>Returns all tags for a specified Amazon QLDB resource.</p>
    async fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> Result<ListTagsForResourceResponse, RusotoError<ListTagsForResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("GET", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<ListTagsForResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(ListTagsForResourceError::from_response(response))
        }
    }

    /// <p>Adds one or more tags to a specified Amazon QLDB resource.</p> <p>A resource can have up to 50 tags. If you try to create more than 50 tags for a resource, your request fails and returns an error.</p>
    async fn tag_resource(
        &self,
        input: TagResourceRequest,
    ) -> Result<TagResourceResponse, RusotoError<TagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("POST", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<TagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(TagResourceError::from_response(response))
        }
    }

    /// <p>Removes one or more tags from a specified Amazon QLDB resource. You can specify up to 50 tag keys to remove.</p>
    async fn untag_resource(
        &self,
        input: UntagResourceRequest,
    ) -> Result<UntagResourceResponse, RusotoError<UntagResourceError>> {
        let request_uri = format!("/tags/{resource_arn}", resource_arn = input.resource_arn);

        let mut request = SignedRequest::new("DELETE", "qldb", &self.region, &request_uri);
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
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UntagResourceResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UntagResourceError::from_response(response))
        }
    }

    /// <p>Updates properties on a ledger.</p>
    async fn update_ledger(
        &self,
        input: UpdateLedgerRequest,
    ) -> Result<UpdateLedgerResponse, RusotoError<UpdateLedgerError>> {
        let request_uri = format!("/ledgers/{name}", name = input.name);

        let mut request = SignedRequest::new("PATCH", "qldb", &self.region, &request_uri);
        request.set_content_type("application/x-amz-json-1.1".to_owned());

        let encoded = Some(serde_json::to_vec(&input).unwrap());
        request.set_payload(encoded);

        let mut response = self
            .client
            .sign_and_dispatch(request)
            .await
            .map_err(RusotoError::from)?;
        if response.status.is_success() {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            let result = proto::json::ResponsePayload::new(&response)
                .deserialize::<UpdateLedgerResponse, _>()?;

            Ok(result)
        } else {
            let response = response.buffer().await.map_err(RusotoError::HttpDispatch)?;
            Err(UpdateLedgerError::from_response(response))
        }
    }
}
